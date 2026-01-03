use super::{main_message_loop::*, main_message_loop_std::*};
use cef::*;
use std::{
    sync::{Arc, Mutex, OnceLock, Weak},
    time::Duration,
};
#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::HWND;

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
use win::MainMessageLoopExternalPumpInner;

#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
use mac::MainMessageLoopExternalPumpInner;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::MainMessageLoopExternalPumpInner;

/// Special timer delay placeholder value. Intentionally 32-bit for Windows and
/// OS X platform API compatibility.
const TIMER_DELAY_PLACEHOLDER: i32 = i32::MAX;

// The maximum number of milliseconds we're willing to wait between calls to
// [MainMessageLoopExternalPump::do_work].
const MAX_TIMER_DELAY: i64 = 1000 / 30; // 30fps

static INSTANCE: OnceLock<Weak<Mutex<MainMessageLoopExternalPump>>> = OnceLock::new();

pub fn get_main_message_loop() -> Option<Arc<Mutex<MainMessageLoopExternalPump>>> {
    INSTANCE.get()?.upgrade()
}

pub fn set_main_message_loop(
    main_message_loop: Option<Arc<Mutex<MainMessageLoopExternalPump>>>,
) -> Option<Arc<Mutex<MainMessageLoopExternalPump>>> {
    let main_message_loop = main_message_loop
        .as_ref()
        .map(Arc::downgrade)
        .unwrap_or_default();
    if let Err(instance) = INSTANCE.set(main_message_loop) {
        instance.upgrade()
    } else {
        None
    }
}

pub struct MainMessageLoopExternalPump {
    standard_message_loop: Arc<Mutex<Option<Box<dyn MainMessageLoop>>>>,
    is_active: bool,
    reentrancy_detected: bool,
    inner: MainMessageLoopExternalPumpInner,
}

impl MainMessageLoopExternalPump {
    pub fn new() -> Arc<Mutex<Self>> {
        let external_pump = Arc::new_cyclic(|weak_ref| {
            Mutex::new(Self {
                standard_message_loop: MainMessageLoopStd::new_message_loop_std(),
                is_active: false,
                reentrancy_detected: false,
                inner: MainMessageLoopExternalPumpInner::new(weak_ref),
            })
        });
        set_main_message_loop(Some(external_pump.clone()));
        external_pump
    }

    /// Called from [BrowserProcessHandler::on_schedule_message_pump_work] on any thread.
    /// The platform subclass must implement this method and schedule a call to
    /// [MainMessageLoopExternalPump::on_schedule_work] on the main application thread.
    pub fn on_schedule_message_pump_work(&mut self, delay: i64) {
        self.inner.on_schedule_message_pump_work(delay);
    }

    fn on_schedule_work(&mut self, delay: i64) {
        assert!(currently_on_main_thread());

        if delay == i64::from(TIMER_DELAY_PLACEHOLDER) && self.is_timer_pending() {
            // Don't set the maximum timer requested from DoWork() if a timer event is
            // currently pending.
            return;
        }

        self.kill_timer();

        let delay = if delay <= 0 {
            // Execute the work immediately.
            self.do_work();
            0
        } else {
            // Never wait longer than the maximum allowed time.
            delay.min(MAX_TIMER_DELAY)
        };

        // Results in call to on_timer_timeout after the specified delay.
        self.set_timer(delay);
    }

    fn on_timer_timeout(&mut self) {
        assert!(currently_on_main_thread());
        self.kill_timer();
        self.do_work();
    }

    /// Control the pending work timer in the platform subclass. Only called on
    /// the main application thread.
    fn set_timer(&mut self, delay: i64) {
        assert!(!self.is_timer_pending());
        assert!(delay > 0);
        self.inner.set_timer(delay);
    }

    /// Control the pending work timer in the platform subclass. Only called on
    /// the main application thread.
    fn kill_timer(&mut self) {
        self.inner.kill_timer();
    }

    /// Control the pending work timer in the platform subclass. Only called on
    /// the main application thread.
    fn is_timer_pending(&self) -> bool {
        self.inner.is_timer_pending()
    }

    /// Handle work processing.
    fn do_work(&mut self) {
        let was_reentrant = self.perform_message_loop_work();
        if was_reentrant {
            self.on_schedule_message_pump_work(0);
        } else if !self.is_timer_pending() {
            self.on_schedule_message_pump_work(i64::from(TIMER_DELAY_PLACEHOLDER));
        }
    }

    fn perform_message_loop_work(&mut self) -> bool {
        if self.is_active {
            // When do_message_loop_work is called there may be various callbacks
            // (such as paint and IPC messages) that result in additional calls to this
            // method. If re-entrancy is detected we must repost a request again to the
            // owner thread to ensure that the discarded call is executed in the future.
            self.reentrancy_detected = true;
            return false;
        }

        self.reentrancy_detected = false;

        self.is_active = true;
        do_message_loop_work();
        self.is_active = false;

        // `reentrancy_detected` may have changed due to re-entrant calls to this
        // method.
        self.reentrancy_detected
    }
}

impl MainMessageLoop for MainMessageLoopExternalPump {
    fn run(&mut self) -> i32 {
        if !self.inner.on_run() {
            return 0;
        }

        self.kill_timer();

        // We need to run the message pump until it is idle. However we don't have
        // that information here so we run the message loop "for a while".
        for _ in 0..10 {
            // Do some work.
            do_message_loop_work();

            // Sleep to allow the CEF proc to do work.
            std::thread::sleep(Duration::from_millis(50));
        }

        0
    }

    fn quit(&mut self) {
        self.inner.on_quit();
    }

    fn post_task(&mut self, task: Option<&mut Task>) {
        let Ok(mut standard_message_loop) = self.standard_message_loop.lock() else {
            return;
        };
        let Some(standard_message_loop) = standard_message_loop.as_mut() else {
            return;
        };
        standard_message_loop.post_task(task);
    }

    fn run_tasks_on_current_thread(&self) -> bool {
        let Ok(standard_message_loop) = self.standard_message_loop.lock() else {
            return false;
        };
        let Some(standard_message_loop) = standard_message_loop.as_ref() else {
            return false;
        };
        standard_message_loop.run_tasks_on_current_thread()
    }

    #[cfg(target_os = "windows")]
    fn set_current_modeless_dialog(&mut self, hwnd: HWND) {
        let Ok(mut standard_message_loop) = self.standard_message_loop.lock() else {
            return;
        };
        let Some(standard_message_loop) = standard_message_loop.as_mut() else {
            return;
        };
        standard_message_loop.set_current_modeless_dialog(hwnd);
    }
}
