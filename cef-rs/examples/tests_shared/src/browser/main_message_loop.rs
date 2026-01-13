use cef::*;
use std::{
    mem,
    sync::{Arc, Mutex, OnceLock},
};

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::HWND;

pub type MainMessageLoopRef = Option<Box<dyn MainMessageLoop>>;

static INSTANCE: OnceLock<Arc<Mutex<MainMessageLoopRef>>> = OnceLock::new();

pub fn get_main_message_loop() -> Arc<Mutex<MainMessageLoopRef>> {
    INSTANCE.get_or_init(|| Arc::new(Mutex::new(None))).clone()
}

pub fn set_main_message_loop(
    mut main_message_loop: MainMessageLoopRef,
) -> Option<Box<dyn MainMessageLoop>> {
    let instance = get_main_message_loop();
    let Ok(mut instance) = instance.lock() else {
        return main_message_loop;
    };
    mem::swap(&mut *instance, &mut main_message_loop);
    main_message_loop
}

pub fn currently_on_main_thread() -> bool {
    let instance = get_main_message_loop();
    let Ok(instance) = instance.lock() else {
        return false;
    };
    let Some(instance) = instance.as_ref() else {
        return false;
    };
    instance.run_tasks_on_current_thread()
}

pub fn main_post_task(task: Option<&mut Task>) {
    let instance = get_main_message_loop();
    let Ok(mut instance) = instance.lock() else {
        return;
    };
    let Some(instance) = instance.as_mut() else {
        return;
    };
    instance.post_task(task);
}

pub fn main_post_once(closure: Box<dyn Send + FnOnce()>) {
    let instance = get_main_message_loop();
    let Ok(mut instance) = instance.lock() else {
        return;
    };
    let Some(instance) = instance.as_mut() else {
        return;
    };
    instance.post_once(closure);
}

pub fn main_post_repeating(closure: Box<dyn Send + FnMut()>) {
    let instance = get_main_message_loop();
    let Ok(mut instance) = instance.lock() else {
        return;
    };
    let Some(instance) = instance.as_mut() else {
        return;
    };
    instance.post_repeating(closure);
}

pub type OnceClosureCallback = Arc<Mutex<Option<Box<dyn Send + FnOnce()>>>>;

wrap_task! {
    struct OnceClosure {
        closure: OnceClosureCallback,
    }

    impl Task {
        fn execute(&self) {
            let Ok(mut closure) = self.closure.lock() else {
                return;
            };
            let Some(closure) = closure.take() else {
                return;
            };
            closure();
        }
    }
}

pub type RepeatingClosureCallback = Arc<Mutex<Option<Box<dyn Send + FnMut()>>>>;

wrap_task! {
    struct RepeatingClosure {
        closure: RepeatingClosureCallback,
    }

    impl Task {
        fn execute(&self) {
            let Ok(mut closure) = self.closure.lock() else {
                return;
            };
            let Some(closure) = closure.as_mut() else {
                return;
            };
            closure();
        }
    }
}

pub trait MainMessageLoop: Send {
    /// Run the message loop. The thread that this method is called on will be considered the main
    /// thread. This blocks until [MainMessageLoop::quit] is called.
    fn run(&mut self) -> i32;

    /// Quit the message loop.
    fn quit(&mut self);

    /// Post a task for execution on the main message loop.
    fn post_task(&mut self, task: Option<&mut Task>);

    /// Returns true if this message loop runs tasks on the current thread.
    fn run_tasks_on_current_thread(&self) -> bool;

    #[cfg(target_os = "windows")]
    fn set_current_modeless_dialog(&mut self, hwnd: HWND);

    /// Post a closure for execution on the main message loop.
    fn post_once(&mut self, closure: Box<dyn Send + FnOnce()>) {
        let mut task = OnceClosure::new(Arc::new(Mutex::new(Some(closure))));
        self.post_task(Some(&mut task));
    }

    /// Post a closure for execution on the main message loop.
    fn post_repeating(&mut self, closure: Box<dyn Send + FnMut()>) {
        let mut task = RepeatingClosure::new(Arc::new(Mutex::new(Some(closure))));
        self.post_task(Some(&mut task));
    }
}
