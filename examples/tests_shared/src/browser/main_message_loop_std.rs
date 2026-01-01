use super::main_message_loop::*;
use cef::*;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::HWND;

pub struct MainMessageLoopStd;

impl MainMessageLoopStd {
    pub fn new() -> Arc<Mutex<Option<Box<dyn MainMessageLoop>>>> {
        set_main_message_loop(Some(Box::new(MainMessageLoopStd)));
        get_main_message_loop()
    }
}

impl Drop for MainMessageLoopStd {
    fn drop(&mut self) {
        set_main_message_loop(None);
    }
}

impl MainMessageLoop for MainMessageLoopStd {
    fn run(&mut self) -> i32 {
        run_message_loop();
        0
    }

    fn quit(&mut self) {
        quit_message_loop();
    }

    fn post_task(&mut self, task: Option<&mut Task>) {
        post_task(ThreadId::UI, task);
    }

    fn run_tasks_on_current_thread(&self) -> bool {
        currently_on(ThreadId::UI) != 0
    }

    #[cfg(target_os = "windows")]
    fn set_current_modeless_dialog(&mut self, _hwnd: HWND) {
        // Nothing to do here. The Chromium message loop implementation will internally route
        // dialog messages.
    }
}
