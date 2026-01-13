use super::*;
use glib::*;
use std::{
    io::{self, Read, Write},
    os::fd::AsRawFd,
    sync::{Arc, Mutex, Weak},
    thread,
};

/// Return a timeout suitable for the glib loop, -1 to block forever,
/// 0 to return right away, or a timeout in milliseconds from now.
fn get_time_interval_milliseconds(cef_time: &cef::Time) -> i32 {
    let mut time = 0.0;
    time_to_doublet(Some(cef_time), Some(&mut time));
    if time == 0.0 {
        return -1;
    }

    let mut cef_now = Default::default();
    time_now(Some(&mut cef_now));
    let mut now = 0.0;
    time_to_doublet(Some(&cef_now), Some(&mut now));

    // Be careful here. CefTime has a precision of microseconds, but we want a
    // value in milliseconds. If there are 5.5ms left, should the delay be 5 or
    // 6?  It should be 6 to avoid executing delayed work too early.
    let interval = (time - now).ceil() * 1000.0;
    let interval = interval as i32;

    // If this value is negative, then we need to run delayed work soon.
    interval.max(0)
}

fn handle_eintr<T>(mut callback: impl FnMut() -> io::Result<T>) -> io::Result<T> {
    loop {
        match callback() {
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            result => break result,
        }
    }
}

pub struct MainMessageLoopExternalPumpInner {
    should_quit: bool,
    main_context: MainContext,
    work_source: Source,
    timer_source: Option<Source>,
    delayed_work_time: Arc<Mutex<Option<cef::Time>>>,
    wakeup_pipe_write: io::PipeWriter,
}

impl Drop for MainMessageLoopExternalPumpInner {
    fn drop(&mut self) {
        self.work_source.destroy();
    }
}

impl MainMessageLoopExternalPumpInner {
    pub fn new(pump: &Weak<Mutex<MainMessageLoopExternalPump>>) -> Self {
        let (mut wakeup_pipe_read, wakeup_pipe_write) =
            io::pipe().expect("Failed to create wakeup pipe");
        let delayed_work_time = Arc::new(Mutex::new(None));
        let main_context = MainContext::default();
        let work_source = {
            let pump = pump.clone();
            let delayed_work_time = delayed_work_time.clone();
            unix_fd_source_new(
                wakeup_pipe_read.as_raw_fd(),
                IOCondition::IN,
                None,
                Priority::DEFAULT_IDLE,
                move |raw_fd, condition| {
                    let Some(pump) = pump.upgrade() else {
                        return ControlFlow::Break;
                    };
                    let Ok(mut pump) = pump.lock() else {
                        return ControlFlow::Break;
                    };

                    // We usually have a single message on the wakeup pipe, since we are only
                    // signaled when the queue went from empty to non-empty, but there can be
                    // two messages if a task posted a task, hence we read at most two bytes.
                    // The glib poll will tell us whether there was data, so this read shouldn't
                    // block.
                    if condition.contains(IOCondition::IN) {
                        assert_eq!(wakeup_pipe_read.as_raw_fd(), raw_fd);

                        let mut buffer = [0; 16];
                        let size = handle_eintr(|| wakeup_pipe_read.read(&mut buffer))
                            .expect("Error reading from the wakeup pipe.");

                        match size {
                            16 => {
                                let mut delay_ms = [0; 8];
                                delay_ms.copy_from_slice(&buffer[..8]);
                                pump.on_schedule_work(i64::from_ne_bytes(delay_ms));
                                delay_ms.copy_from_slice(&buffer[8..]);
                                pump.on_schedule_work(i64::from_ne_bytes(delay_ms));
                            }
                            8..16 => {
                                let mut delay_ms = [0; 8];
                                delay_ms.copy_from_slice(&buffer[..8]);
                                pump.on_schedule_work(i64::from_ne_bytes(delay_ms));
                            }
                            _ => {}
                        }
                    }

                    if let Ok(delayed_work_time) = delayed_work_time.lock() {
                        let delay = delayed_work_time
                            .as_ref()
                            .map(get_time_interval_milliseconds)
                            .unwrap_or_default();

                        if delay == 0 {
                            // The timer has expired. That condition will stay true until we process
                            // that delayed work, so we don't need to record this differently.
                            pump.on_timer_timeout();
                        }
                    }

                    ControlFlow::Continue
                },
            )
        };
        work_source.attach(Some(&main_context));

        Self {
            should_quit: false,
            main_context,
            work_source,
            timer_source: None,
            delayed_work_time,
            wakeup_pipe_write,
        }
    }

    pub fn on_run(&mut self) -> bool {
        // We really only do a single task for each iteration of the loop. If we
        // have done something, assume there is likely something more to do. This
        // will mean that we don't block on the message pump until there was nothing
        // more to do. We also set this to true to make sure not to block on the
        // first iteration of the loop.
        let mut more_work_is_plausible = true;

        // We run our own loop instead of using g_main_loop_quit in one of the
        // callbacks. This is so we only quit our own loops, and we don't quit
        // nested loops run by others.
        loop {
            // Don't block if we think we have more work to do.
            let block = !more_work_is_plausible;

            more_work_is_plausible = self.main_context.iteration(block);
            if self.should_quit {
                break;
            }
        }

        // We need to run the message pump until it is idle. However we don't have
        // that information here so we run the message loop "for a while".
        for _ in 0..10 {
            // Do some work.
            do_message_loop_work();

            // Sleep to allow the CEF proc to do work.
            thread::sleep(Duration::from_micros(50000));
        }

        false
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn on_schedule_message_pump_work(&mut self, delay: i64) {
        let buffer = delay.to_ne_bytes();
        let size = handle_eintr(|| self.wakeup_pipe_write.write(&buffer)).unwrap_or_default();
        assert_eq!(
            size, 8,
            "Could not write to the UI message loop wakeup pipe!"
        );
    }

    pub fn set_timer(&mut self, delay: i64) {
        assert!(delay > 0);

        let mut delayed_work_time = self
            .delayed_work_time
            .lock()
            .expect("Failed to lock delayed_work_time member");

        let mut cef_now = Default::default();
        time_now(Some(&mut cef_now));
        let mut now = 0.0;
        time_to_doublet(Some(&cef_now), Some(&mut now));

        let time = now + delay as f64 / 1000.0;
        let mut cef_time = Default::default();
        if time_from_doublet(time, Some(&mut cef_time)) == 0 {
            panic!("Failed to convert time to CEF time");
        }

        *delayed_work_time = Some(cef_time);

        if let Some(timer_source) = self.timer_source.take() {
            self.work_source.remove_child_source(&timer_source);
        }

        let timer_source = timeout_source_new(
            Duration::from_millis(delay.max(0).unsigned_abs()),
            None,
            Priority::DEFAULT_IDLE,
            || ControlFlow::Continue,
        );
        self.work_source.add_child_source(&timer_source);
    }

    pub fn kill_timer(&mut self) {
        let mut delayed_work_time = self
            .delayed_work_time
            .lock()
            .expect("Failed to lock delayed_work_time member");

        *delayed_work_time = None;

        if let Some(timer_source) = self.timer_source.take() {
            self.work_source.remove_child_source(&timer_source);
        }
    }

    pub fn is_timer_pending(&self) -> bool {
        let delayed_work_time = self
            .delayed_work_time
            .lock()
            .expect("Failed to lock delayed_work_time member");
        let delay = delayed_work_time
            .as_ref()
            .map(get_time_interval_milliseconds)
            .unwrap_or_default();
        delay > 0
    }
}
