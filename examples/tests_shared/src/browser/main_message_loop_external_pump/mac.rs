use super::*;
use objc2::{define_class, msg_send, rc::Retained, sel, AnyThread, DefinedClass};
use objc2_app_kit::{NSApp, NSEventTrackingRunLoopMode};
use objc2_foundation::{
    MainThreadMarker, NSNumber, NSObject, NSObjectNSThreadPerformAdditions, NSObjectProtocol,
    NSRunLoop, NSRunLoopCommonModes, NSThread, NSTimer,
};
use std::sync::{Mutex, Weak};

define_class! {
    #[unsafe(super(NSObject))]
    #[ivars = Weak<Mutex<MainMessageLoopExternalPump>>]
    struct EventHandler;

    impl EventHandler {
        #[unsafe(method(scheduleWork:))]
        fn schedule_work(&self, delay_ms: &NSNumber) {
            let Ok(delay_ms) = i64::try_from(delay_ms.integerValue()) else {
                return;
            };

            let Some(pump) = self.ivars().upgrade() else {
                return;
            };
            let Ok(mut pump) = pump.lock() else {
                return;
            };

            pump.on_schedule_work(delay_ms);
        }

        #[unsafe(method(timerTimeout:))]
        fn timer_timeout(&self, _: &NSTimer) {
            let Some(pump) = self.ivars().upgrade() else {
                return;
            };
            let Ok(mut pump) = pump.lock() else {
                return;
            };

            pump.on_timer_timeout();
        }
    }

    unsafe impl NSObjectProtocol for EventHandler {}
}

impl EventHandler {
    fn new(pump: Weak<Mutex<MainMessageLoopExternalPump>>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(pump);
        unsafe { msg_send![super(this), init] }
    }
}

pub struct MainMessageLoopExternalPumpInner {
    owner_thread: Retained<NSThread>,
    timer: Option<Retained<NSTimer>>,
    event_handler: Retained<EventHandler>,
}

unsafe impl Send for MainMessageLoopExternalPumpInner {}

impl MainMessageLoopExternalPumpInner {
    pub fn new(pump: &Weak<Mutex<MainMessageLoopExternalPump>>) -> Self {
        let event_handler = EventHandler::new(pump.clone());
        Self {
            owner_thread: NSThread::currentThread(),
            timer: None,
            event_handler,
        }
    }

    pub fn on_run(&mut self) -> bool {
        let Some(mtm) = MainThreadMarker::new() else {
            return false;
        };
        NSApp(mtm).run();
        true
    }

    pub fn on_quit(&mut self) {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        NSApp(mtm).stop(None);
    }

    pub fn on_schedule_message_pump_work(&mut self, delay: i64) {
        // This method may be called on any thread.
        let delay = isize::try_from(delay).unwrap_or(isize::MAX);
        let number = NSNumber::numberWithInteger(delay);
        unsafe {
            self.event_handler
                .performSelector_onThread_withObject_waitUntilDone(
                    sel!(scheduleWork:),
                    &self.owner_thread,
                    Some(&number),
                    false,
                );
        }
    }

    pub fn set_timer(&mut self, delay: i64) {
        let delay_s = delay as f64 / 1000.0;
        let timer = unsafe {
            NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                delay_s,
                &self.event_handler,
                sel!(timerTimeout:),
                None,
                false,
            )
        };

        // Add the timer to default and tracking runloop modes.
        let owner_runloop = NSRunLoop::currentRunLoop();
        unsafe {
            owner_runloop.addTimer_forMode(&timer, NSRunLoopCommonModes);
            owner_runloop.addTimer_forMode(&timer, NSEventTrackingRunLoopMode);
        }

        self.timer = Some(timer);
    }

    pub fn kill_timer(&mut self) {
        let Some(timer) = self.timer.take() else {
            return;
        };
        timer.invalidate();
    }

    pub fn is_timer_pending(&self) -> bool {
        self.timer.is_some()
    }
}
