use std::{ffi::c_void, mem, ptr, sync::OnceLock};
use windows_sys::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::{
        LibraryLoader::{
            GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        },
        Performance::*,
        SystemServices::*,
    },
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

pub fn get_time_now() -> u64 {
    static FREQUENCY: OnceLock<f64> = OnceLock::new();

    let frequency = FREQUENCY.get_or_init(|| {
        let mut frequency = 0;
        unsafe { QueryPerformanceFrequency(&mut frequency) };
        frequency.max(1) as f64 / 1000000.0
    });

    let mut current_time = 0;
    unsafe { QueryPerformanceCounter(&mut current_time) };
    ((current_time as f64 / *frequency) as i64).max(0) as u64
}

pub fn set_user_data_ptr(hwnd: HWND, data: *mut c_void) -> *mut c_void {
    unsafe {
        SetLastError(ERROR_SUCCESS);
        let result = SetWindowLongPtrW(hwnd, GWLP_USERDATA, data as isize);
        assert!(result != 0 || GetLastError() == ERROR_SUCCESS);
        result as *mut _
    }
}

pub fn set_user_data<T>(hwnd: HWND, data: Option<T>) -> Option<Box<T>> {
    let ptr: *mut T = set_user_data_ptr(
        hwnd,
        data.map(|data| Box::into_raw(Box::new(data)).cast())
            .unwrap_or(ptr::null_mut()),
    )
    .cast();
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(Box::from_raw(ptr)) }
    }
}

pub fn get_user_data<'a, T>(hwnd: HWND) -> Option<&'a mut T> {
    unsafe {
        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut T;
        ptr.as_mut()
    }
}

pub fn set_window_proc_ptr(hwnd: HWND, proc: WNDPROC) -> WNDPROC {
    Some(unsafe {
        let old = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
        assert_ne!(old, 0);
        if let Some(proc) = proc {
            let result = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, mem::transmute(proc));
            assert!(result != 0 || GetLastError() == ERROR_SUCCESS);
        }
        mem::transmute(old)
    })
}

#[repr(i32)]
#[derive(Default, Debug)]
pub enum EventFlag {
    #[default]
    None = 0,
    CapsLockOn = 1 << 0,
    ShiftDown = 1 << 1,
    ControlDown = 1 << 2,
    AltDown = 1 << 3,
    LeftMouseButton = 1 << 4,
    MiddleMouseButton = 1 << 5,
    RightMouseButton = 1 << 6,
    CommandDown = 1 << 7,
    NumLockOn = 1 << 8,
    IsKeyPad = 1 << 9,
    IsLeft = 1 << 10,
    IsRight = 1 << 11,
    AltGrDown = 1 << 12,
    IsRepeat = 1 << 13,
    PrecisionScrollingDelta = 1 << 14,
    ScrollByPage = 1 << 15,
}

pub fn get_resource_string(id: u32) -> String {
    let mut buffer = [0u16; 100];
    String::from_utf16_lossy(unsafe {
        let len = LoadStringW(
            get_code_module_handle(),
            id,
            buffer.as_mut_ptr(),
            buffer.len() as i32,
        ) as usize;
        &buffer[..len]
    })
}

pub fn get_cef_mouse_modifiers(wparam: WPARAM) -> i32 {
    let mut modifiers = 0;

    if wparam & MK_CONTROL as usize != 0 {
        modifiers |= EventFlag::ControlDown as i32;
    }
    if wparam & MK_SHIFT as usize != 0 {
        modifiers |= EventFlag::ShiftDown as i32;
    }
    if is_key_down(VK_MENU as _) {
        modifiers |= EventFlag::AltDown as i32;
    }
    if wparam & MK_LBUTTON as usize != 0 {
        modifiers |= EventFlag::LeftMouseButton as i32;
    }
    if wparam & MK_MBUTTON as usize != 0 {
        modifiers |= EventFlag::MiddleMouseButton as i32;
    }
    if wparam & MK_RBUTTON as usize != 0 {
        modifiers |= EventFlag::RightMouseButton as i32;
    }

    // Low bit set from GetKeyState indicates "toggled".
    unsafe {
        if GetKeyState(VK_NUMLOCK as i32) & 1 != 0 {
            modifiers |= EventFlag::NumLockOn as i32;
        }
        if GetKeyState(VK_CAPITAL as i32) & 1 != 0 {
            modifiers |= EventFlag::CapsLockOn as i32;
        }
    }

    modifiers
}

pub fn get_cef_keyboard_modifiers(wparam: WPARAM, lparam: LPARAM) -> i32 {
    let mut modifiers = 0;

    if is_key_down(VK_SHIFT as _) {
        modifiers |= EventFlag::ShiftDown as i32;
    }
    if is_key_down(VK_CONTROL as _) {
        modifiers |= EventFlag::ControlDown as i32;
    }
    if is_key_down(VK_MENU as _) {
        modifiers |= EventFlag::AltDown as i32;
    }

    // Low bit set from GetKeyState indicates "toggled".
    unsafe {
        if GetKeyState(VK_NUMLOCK as i32) & 1 != 0 {
            modifiers |= EventFlag::NumLockOn as i32;
        }
        if GetKeyState(VK_CAPITAL as i32) & 1 != 0 {
            modifiers |= EventFlag::CapsLockOn as i32;
        }
    }

    match wparam as VIRTUAL_KEY {
        VK_RETURN => {
            if (lparam >> 16) & KF_EXTENDED as isize != 0 {
                modifiers |= EventFlag::IsKeyPad as i32;
            }
        }
        VK_INSERT | VK_DELETE | VK_HOME | VK_END | VK_PRIOR | VK_NEXT | VK_UP | VK_DOWN
        | VK_LEFT | VK_RIGHT => {
            if (lparam >> 16) & KF_EXTENDED as isize == 0 {
                modifiers |= EventFlag::IsKeyPad as i32;
            }
        }
        VK_NUMLOCK | VK_NUMPAD0 | VK_NUMPAD1 | VK_NUMPAD2 | VK_NUMPAD3 | VK_NUMPAD4
        | VK_NUMPAD5 | VK_NUMPAD6 | VK_NUMPAD7 | VK_NUMPAD8 | VK_NUMPAD9 | VK_DIVIDE
        | VK_MULTIPLY | VK_SUBTRACT | VK_ADD | VK_DECIMAL | VK_CLEAR => {
            modifiers |= EventFlag::IsKeyPad as i32;
        }
        VK_SHIFT => {
            if is_key_down(VK_LSHIFT as _) {
                modifiers |= EventFlag::IsLeft as i32;
            } else if is_key_down(VK_RSHIFT as _) {
                modifiers |= EventFlag::IsRight as i32;
            }
        }
        VK_CONTROL => {
            if is_key_down(VK_LCONTROL as _) {
                modifiers |= EventFlag::IsLeft as i32;
            } else if is_key_down(VK_RCONTROL as _) {
                modifiers |= EventFlag::IsRight as i32;
            }
        }
        VK_MENU => {
            if is_key_down(VK_LMENU as _) {
                modifiers |= EventFlag::IsLeft as i32;
            } else if is_key_down(VK_RMENU as _) {
                modifiers |= EventFlag::IsRight as i32;
            }
        }
        VK_LWIN => {
            modifiers |= EventFlag::IsLeft as i32;
        }
        VK_RWIN => {
            modifiers |= EventFlag::IsRight as i32;
        }
        _ => {}
    }

    modifiers
}

pub fn is_key_down(wparam: WPARAM) -> bool {
    unsafe {
        let key_state = GetKeyState(wparam as i32) as u16;
        key_state & 0x8000 != 0
    }
}

pub fn get_device_scale_factor() -> f32 {
    static SCALE_FACTOR: OnceLock<f32> = OnceLock::new();

    *SCALE_FACTOR.get_or_init(|| unsafe {
        let screen_dc = GetDC(ptr::null_mut());
        let dpi_x = GetDeviceCaps(screen_dc, LOGPIXELSX as i32);
        ReleaseDC(ptr::null_mut(), screen_dc);
        dpi_x as f32 / 96.0
    })
}

pub fn get_code_module_handle() -> HINSTANCE {
    let mut module: HMODULE = ptr::null_mut();
    unsafe {
        let result = GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            get_code_module_handle as *const _,
            &mut module,
        );
        assert_ne!(result, 0);
    }
    module
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_time_now() {
        let time = get_time_now();
        assert!(time > 0);
    }
}
