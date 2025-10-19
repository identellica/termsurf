use super::*;
use crate::browser::util_win::*;
use std::{
    mem, ptr,
    sync::{Mutex, Weak},
};
use windows_sys::{w, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

const MSG_HAVE_WORK: u32 = WM_USER + 1;

pub struct MainMessageLoopExternalPumpInner {
    timer_pending: bool,
    main_thread_target: Option<usize>,
}

impl MainMessageLoopExternalPumpInner {
    pub fn new(pump: &Weak<Mutex<MainMessageLoopExternalPump>>) -> Self {
        let main_thread_target = {
            const WINDOW_CLASS_NAME: *const u16 = w!("MainMessageLoopExternalPump");

            let instance = get_code_module_handle();
            unsafe {
                let wcex = WNDCLASSEXW {
                    cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
                    lpfnWndProc: Some(Self::window_proc),
                    hInstance: instance,
                    lpszClassName: WINDOW_CLASS_NAME,
                    ..mem::zeroed()
                };
                RegisterClassExW(&wcex);

                // Create the message handling window.
                let main_thread_target = CreateWindowExW(
                    0,
                    WINDOW_CLASS_NAME,
                    ptr::null(),
                    WS_OVERLAPPEDWINDOW,
                    0,
                    0,
                    0,
                    0,
                    HWND_MESSAGE,
                    ptr::null_mut(),
                    instance,
                    ptr::null(),
                );
                assert!(!main_thread_target.is_null());
                main_thread_target
            }
        };

        set_user_data(main_thread_target, Some(pump.clone()));

        Self {
            timer_pending: false,
            main_thread_target: Some(main_thread_target as usize),
        }
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            MSG_HAVE_WORK | WM_TIMER => {
                if let Some(message_loop) =
                    get_user_data::<Weak<Mutex<MainMessageLoopExternalPump>>>(hwnd)
                {
                    if let Some(message_loop) = message_loop.upgrade() {
                        if let Ok(mut message_loop) = message_loop.lock() {
                            if message == MSG_HAVE_WORK {
                                let delay = lparam as i64;
                                message_loop.on_schedule_work(delay);
                            } else {
                                message_loop.on_timer_timeout();
                            }
                        }
                    }
                }
            }
            WM_DESTROY => {
                let _ = set_user_data::<Weak<Mutex<MainMessageLoopExternalPump>>>(hwnd, None);
            }
            _ => {}
        }
        DefWindowProcW(hwnd, message, wparam, lparam)
    }

    pub fn on_run(&mut self) -> bool {
        let mut msg = Default::default();
        unsafe {
            while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        true
    }

    pub fn on_quit(&mut self) {
        unsafe {
            PostMessageW(ptr::null_mut(), WM_QUIT, 0, 0);
        }
    }

    pub fn on_schedule_message_pump_work(&mut self, delay: i64) {
        // This method may be called on any thread.
        unsafe {
            if let Some(main_thread_target) = self.main_thread_target {
                PostMessageW(
                    main_thread_target as HWND,
                    MSG_HAVE_WORK,
                    0,
                    delay as LPARAM,
                );
            }
        }
    }

    pub fn set_timer(&mut self, delay: i64) {
        self.timer_pending = true;
        if let Some(main_thread_target) = self.main_thread_target {
            unsafe {
                SetTimer(main_thread_target as HWND, 1, delay as u32, None);
            }
        }
    }

    pub fn kill_timer(&mut self) {
        if self.timer_pending {
            if let Some(main_thread_target) = self.main_thread_target {
                unsafe {
                    KillTimer(main_thread_target as HWND, 1);
                }
            }
            self.timer_pending = false;
        }
    }

    pub fn is_timer_pending(&self) -> bool {
        self.timer_pending
    }
}
