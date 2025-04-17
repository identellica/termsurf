//! String module

use cef_dll_sys::{
    _cef_string_list_t, _cef_string_map_t, _cef_string_multimap_t, _cef_string_utf16_t,
    _cef_string_utf8_t, _cef_string_wide_t,
};
use std::{
    fmt::{self, Display, Formatter},
    mem,
    ptr::{self, NonNull},
    slice,
};

use crate::CefString;

struct UserFreeData<T>(Option<NonNull<T>>);

impl<T> Default for UserFreeData<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> From<*mut T> for UserFreeData<T> {
    fn from(value: *mut T) -> Self {
        Self(NonNull::new(value))
    }
}

impl<T> From<UserFreeData<T>> for *mut T {
    fn from(value: UserFreeData<T>) -> Self {
        let mut value = value;
        mem::take(&mut value.0)
            .map(NonNull::as_ptr)
            .unwrap_or(ptr::null_mut())
    }
}

impl Clone for UserFreeData<_cef_string_utf8_t> {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().and_then(|value| unsafe {
            let data = NonNull::new(cef_dll_sys::cef_string_userfree_utf8_alloc())?;
            if cef_dll_sys::cef_string_utf8_set(
                value.as_ref().str_,
                value.as_ref().length,
                data.as_ptr(),
                1,
            ) == 0
            {
                cef_dll_sys::cef_string_userfree_utf8_free(data.as_ptr());
                None
            } else {
                Some(data)
            }
        }))
    }
}

impl Clone for UserFreeData<_cef_string_utf16_t> {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().and_then(|value| unsafe {
            let data = NonNull::new(cef_dll_sys::cef_string_userfree_utf16_alloc())?;
            if cef_dll_sys::cef_string_utf16_set(
                value.as_ref().str_,
                value.as_ref().length,
                data.as_ptr(),
                1,
            ) == 0
            {
                cef_dll_sys::cef_string_userfree_utf16_free(data.as_ptr());
                None
            } else {
                Some(data)
            }
        }))
    }
}

impl Clone for UserFreeData<_cef_string_wide_t> {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().and_then(|value| unsafe {
            let data = NonNull::new(cef_dll_sys::cef_string_userfree_wide_alloc())?;
            if cef_dll_sys::cef_string_wide_set(
                value.as_ref().str_,
                value.as_ref().length,
                data.as_ptr(),
                1,
            ) == 0
            {
                cef_dll_sys::cef_string_userfree_wide_free(data.as_ptr());
                None
            } else {
                Some(data)
            }
        }))
    }
}

#[derive(Clone, Default)]
pub struct CefStringUserfreeUtf8(UserFreeData<_cef_string_utf8_t>);

impl From<*mut _cef_string_utf8_t> for CefStringUserfreeUtf8 {
    fn from(value: *mut _cef_string_utf8_t) -> Self {
        Self(value.into())
    }
}

impl From<CefStringUserfreeUtf8> for *mut _cef_string_utf8_t {
    fn from(value: CefStringUserfreeUtf8) -> Self {
        let mut value = value;
        mem::take(&mut value.0).into()
    }
}

impl From<&CefStringUserfreeUtf8> for Option<&_cef_string_utf8_t> {
    fn from(value: &CefStringUserfreeUtf8) -> Self {
        value.0 .0.as_ref().map(|value| unsafe { value.as_ref() })
    }
}

impl Drop for CefStringUserfreeUtf8 {
    fn drop(&mut self) {
        let value: *mut _cef_string_utf8_t = mem::take(&mut self.0).into();
        if !value.is_null() {
            unsafe {
                cef_dll_sys::cef_string_userfree_utf8_free(value);
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct CefStringUserfreeUtf16(UserFreeData<_cef_string_utf16_t>);

impl From<*mut _cef_string_utf16_t> for CefStringUserfreeUtf16 {
    fn from(value: *mut _cef_string_utf16_t) -> Self {
        Self(value.into())
    }
}

impl From<CefStringUserfreeUtf16> for *mut _cef_string_utf16_t {
    fn from(value: CefStringUserfreeUtf16) -> Self {
        let mut value = value;
        mem::take(&mut value.0).into()
    }
}

impl From<&CefStringUserfreeUtf16> for Option<&_cef_string_utf16_t> {
    fn from(value: &CefStringUserfreeUtf16) -> Self {
        value.0 .0.as_ref().map(|value| unsafe { value.as_ref() })
    }
}

impl Drop for CefStringUserfreeUtf16 {
    fn drop(&mut self) {
        let value: *mut _cef_string_utf16_t = mem::take(&mut self.0).into();
        if !value.is_null() {
            unsafe {
                cef_dll_sys::cef_string_userfree_utf16_free(value);
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct CefStringUserfreeWide(UserFreeData<_cef_string_wide_t>);

impl From<*mut _cef_string_wide_t> for CefStringUserfreeWide {
    fn from(value: *mut _cef_string_wide_t) -> Self {
        Self(value.into())
    }
}

impl From<CefStringUserfreeWide> for *mut _cef_string_wide_t {
    fn from(value: CefStringUserfreeWide) -> Self {
        let mut value = value;
        mem::take(&mut value.0).into()
    }
}

impl From<&CefStringUserfreeWide> for Option<&_cef_string_wide_t> {
    fn from(value: &CefStringUserfreeWide) -> Self {
        value.0 .0.as_ref().map(|value| unsafe { value.as_ref() })
    }
}

impl From<*const _cef_string_utf16_t> for CefStringUserfreeWide {
    fn from(value: *const _cef_string_utf16_t) -> Self {
        Self(UserFreeData(unsafe {
            value.as_ref().and_then(|value| {
                let slice = slice::from_raw_parts(value.str_ as *const _, value.length);
                NonNull::new(cef_dll_sys::cef_string_userfree_wide_alloc()).and_then(|data| {
                    if cef_dll_sys::cef_string_utf16_to_wide(
                        slice.as_ptr() as *const _,
                        slice.len(),
                        data.as_ptr(),
                    ) == 0
                    {
                        cef_dll_sys::cef_string_userfree_wide_free(data.as_ptr());
                        None
                    } else {
                        Some(data)
                    }
                })
            })
        }))
    }
}

impl From<&CefStringUtf16> for CefStringUserfreeWide {
    fn from(value: &CefStringUtf16) -> Self {
        let value: *const _cef_string_utf16_t = value.into();
        Self::from(value)
    }
}

impl Drop for CefStringUserfreeWide {
    fn drop(&mut self) {
        let value: *mut _cef_string_wide_t = mem::take(&mut self.0).into();
        if !value.is_null() {
            unsafe {
                cef_dll_sys::cef_string_userfree_wide_free(value);
            }
        }
    }
}

enum CefStringData<T> {
    Borrowed(Option<T>),
    BorrowedMut(Option<NonNull<T>>),
    Clear(Option<T>),
}

impl<T> Clone for CefStringData<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        let data: Option<&T> = self.into();
        let data = data.map(ptr::from_ref).unwrap_or(ptr::null());
        data.into()
    }
}

impl<T> Default for CefStringData<T> {
    fn default() -> Self {
        Self::Borrowed(None)
    }
}

impl<T> From<*const T> for CefStringData<T>
where
    T: Copy,
{
    fn from(value: *const T) -> Self {
        Self::Borrowed(unsafe { value.as_ref() }.copied())
    }
}

impl<T> From<*mut T> for CefStringData<T> {
    fn from(value: *mut T) -> Self {
        Self::BorrowedMut(NonNull::new(value))
    }
}

impl<'a, T> Into<Option<&'a T>> for &'a CefStringData<T> {
    fn into(self) -> Option<&'a T> {
        match self {
            CefStringData::Borrowed(value) | CefStringData::Clear(value) => value.as_ref(),
            CefStringData::BorrowedMut(value) => {
                value.as_ref().map(|value| unsafe { value.as_ref() })
            }
        }
    }
}

impl<'a, T> Into<Option<&'a mut T>> for &'a mut CefStringData<T> {
    fn into(self) -> Option<&'a mut T> {
        match self {
            CefStringData::BorrowedMut(value) => {
                value.as_mut().map(|value| unsafe { value.as_mut() })
            }
            _ => None,
        }
    }
}

/// See [_cef_string_utf8_t] for more documentation.
#[derive(Clone)]
pub struct CefStringUtf8(CefStringData<_cef_string_utf8_t>);

impl Drop for CefStringUtf8 {
    fn drop(&mut self) {
        if let CefStringData::Clear(mut value) = &mut self.0 {
            if let Some(mut value) = mem::take(&mut value) {
                unsafe {
                    cef_dll_sys::cef_string_utf8_clear(&mut value);
                }
            }
        }
    }
}

impl From<&str> for CefStringUtf8 {
    fn from(value: &str) -> Self {
        Self(CefStringData::Clear(unsafe {
            let mut data = mem::zeroed();
            if cef_dll_sys::cef_string_utf8_set(
                value.as_bytes().as_ptr() as *const _,
                value.as_bytes().len(),
                &mut data,
                1,
            ) == 0
            {
                None
            } else {
                Some(data)
            }
        }))
    }
}

impl From<&CefStringUserfreeUtf8> for CefStringUtf8 {
    fn from(value: &CefStringUserfreeUtf8) -> Self {
        let value: Option<&_cef_string_utf8_t> = value.into();
        Self(CefStringData::Clear(value.and_then(|value| unsafe {
            let mut data = mem::zeroed();
            if cef_dll_sys::cef_string_utf8_set(value.str_, value.length, &mut data, 1) == 0 {
                None
            } else {
                Some(data)
            }
        })))
    }
}

impl From<*const _cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: *const _cef_string_utf8_t) -> Self {
        Self(value.into())
    }
}

impl From<*mut _cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: *mut _cef_string_utf8_t) -> Self {
        Self(value.into())
    }
}

impl From<&CefStringUtf8> for *const _cef_string_utf8_t {
    fn from(value: &CefStringUtf8) -> Self {
        let data: Option<&_cef_string_utf8_t> = (&value.0).into();
        data.map(ptr::from_ref).unwrap_or(ptr::null())
    }
}

impl From<&mut CefStringUtf8> for *mut _cef_string_utf8_t {
    fn from(value: &mut CefStringUtf8) -> Self {
        match &mut value.0 {
            CefStringData::BorrowedMut(value) => value.map(|value| value.as_ptr()),
            _ => None,
        }
        .unwrap_or(ptr::null_mut())
    }
}

impl From<_cef_string_utf8_t> for CefStringUtf8 {
    fn from(value: _cef_string_utf8_t) -> Self {
        Self(CefStringData::Borrowed(Some(value)))
    }
}

impl From<CefStringUtf8> for _cef_string_utf8_t {
    fn from(value: CefStringUtf8) -> Self {
        match value.0 {
            CefStringData::Borrowed(value) => value,
            _ => None,
        }
        .unwrap_or(unsafe { mem::zeroed() })
    }
}

impl CefStringUtf8 {
    pub fn as_str(&self) -> Option<&str> {
        let data: Option<&_cef_string_utf8_t> = (&self.0).into();
        let (str_, length) = data.map(|value| (value.str_, value.length))?;
        Some(unsafe {
            let slice = slice::from_raw_parts(str_ as *const _, length);
            std::str::from_utf8_unchecked(slice)
        })
    }

    pub fn as_slice(&self) -> Option<&[u8]> {
        let data: Option<&_cef_string_utf8_t> = (&self.0).into();
        let (str_, length) = data.map(|value| (value.str_, value.length))?;
        Some(unsafe { slice::from_raw_parts(str_ as *const _, length) })
    }

    pub fn try_set(&mut self, value: &str) -> bool {
        let CefStringData::BorrowedMut(Some(data)) = &mut self.0 else {
            return false;
        };

        unsafe {
            assert_ne!(value.as_ptr(), data.as_ref().str_ as *const _);
            cef_dll_sys::cef_string_utf8_clear(data.as_ptr());
            cef_dll_sys::cef_string_utf8_set(
                value.as_ptr() as *const _,
                value.len(),
                data.as_ptr(),
                1,
            ) != 0
        }
    }
}

impl From<&CefStringUtf16> for CefStringUtf8 {
    fn from(value: &CefStringUtf16) -> Self {
        Self(CefStringData::Clear(unsafe {
            value.as_slice().and_then(|value| {
                let mut data = mem::zeroed();
                if cef_dll_sys::cef_string_utf16_to_utf8(
                    value.as_ptr() as *const _,
                    value.len(),
                    &mut data,
                ) == 0
                {
                    None
                } else {
                    Some(data)
                }
            })
        }))
    }
}

impl From<&CefStringWide> for CefStringUtf8 {
    fn from(value: &CefStringWide) -> Self {
        Self(CefStringData::Clear(unsafe {
            value.as_slice().and_then(|value| {
                let mut data = mem::zeroed();
                if cef_dll_sys::cef_string_wide_to_utf8(
                    value.as_ptr() as *const _,
                    value.len(),
                    &mut data,
                ) == 0
                {
                    None
                } else {
                    Some(data)
                }
            })
        }))
    }
}

impl Display for CefStringUtf8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(value) = self.as_str() {
            write!(f, "{value}")
        } else {
            Ok(())
        }
    }
}

/// See [_cef_string_utf16_t] for more documentation.
#[derive(Clone, Default)]
pub struct CefStringUtf16(CefStringData<_cef_string_utf16_t>);

impl Drop for CefStringUtf16 {
    fn drop(&mut self) {
        if let CefStringData::Clear(mut value) = &mut self.0 {
            if let Some(mut value) = mem::take(&mut value) {
                unsafe {
                    cef_dll_sys::cef_string_utf16_clear(&mut value);
                }
            }
        }
    }
}

impl From<&str> for CefStringUtf16 {
    fn from(value: &str) -> Self {
        Self(CefStringData::Clear(unsafe {
            let mut data = mem::zeroed();
            if cef_dll_sys::cef_string_utf8_to_utf16(
                value.as_bytes().as_ptr() as *const _,
                value.as_bytes().len(),
                &mut data,
            ) == 0
            {
                None
            } else {
                Some(data)
            }
        }))
    }
}

impl From<&CefStringUserfreeUtf16> for CefStringUtf16 {
    fn from(value: &CefStringUserfreeUtf16) -> Self {
        let value: Option<&_cef_string_utf16_t> = value.into();
        if value.is_none() {
            eprintln!("Invalid UTF-16 string");
        }
        Self(CefStringData::Clear(value.and_then(|value| unsafe {
            let mut data = mem::zeroed();
            if cef_dll_sys::cef_string_utf16_set(value.str_, value.length, &mut data, 1) == 0 {
                None
            } else {
                Some(data)
            }
        })))
    }
}

impl From<*const _cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: *const _cef_string_utf16_t) -> Self {
        Self(value.into())
    }
}

impl From<*mut _cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: *mut _cef_string_utf16_t) -> Self {
        Self(value.into())
    }
}

impl From<&CefStringUtf16> for *const _cef_string_utf16_t {
    fn from(value: &CefStringUtf16) -> Self {
        let data: Option<&_cef_string_utf16_t> = (&value.0).into();
        data.map(ptr::from_ref).unwrap_or(ptr::null())
    }
}

impl From<&mut CefStringUtf16> for *mut _cef_string_utf16_t {
    fn from(value: &mut CefStringUtf16) -> Self {
        match &mut value.0 {
            CefStringData::BorrowedMut(value) => value.map(|value| value.as_ptr()),
            _ => None,
        }
        .unwrap_or(ptr::null_mut())
    }
}

impl From<_cef_string_utf16_t> for CefStringUtf16 {
    fn from(value: _cef_string_utf16_t) -> Self {
        Self(CefStringData::Borrowed(Some(value)))
    }
}

impl From<CefStringUtf16> for _cef_string_utf16_t {
    fn from(value: CefStringUtf16) -> Self {
        match value.0 {
            CefStringData::Borrowed(value) => value,
            _ => None,
        }
        .unwrap_or(unsafe { mem::zeroed() })
    }
}

impl CefStringUtf16 {
    pub fn as_slice(&self) -> Option<&[u16]> {
        let data: Option<&_cef_string_utf16_t> = (&self.0).into();
        let (str_, length) = data.map(|value| (value.str_, value.length))?;
        Some(unsafe { slice::from_raw_parts(str_ as *const _, length) })
    }

    pub fn try_set(&mut self, value: &str) -> bool {
        let CefStringData::BorrowedMut(Some(data)) = &mut self.0 else {
            return false;
        };

        unsafe {
            cef_dll_sys::cef_string_utf16_clear(data.as_ptr());
            cef_dll_sys::cef_string_utf8_to_utf16(
                value.as_ptr() as *const _,
                value.len(),
                data.as_ptr(),
            ) != 0
        }
    }
}

impl From<&CefStringUtf8> for CefStringUtf16 {
    fn from(value: &CefStringUtf8) -> Self {
        Self(CefStringData::Clear(unsafe {
            value.as_str().and_then(|value| {
                let mut data = mem::zeroed();
                if cef_dll_sys::cef_string_utf8_to_utf16(
                    value.as_bytes().as_ptr() as *const _,
                    value.as_bytes().len(),
                    &mut data,
                ) == 0
                {
                    None
                } else {
                    Some(data)
                }
            })
        }))
    }
}

impl From<&CefStringWide> for CefStringUtf16 {
    fn from(value: &CefStringWide) -> Self {
        Self(CefStringData::Clear(unsafe {
            value.as_slice().and_then(|value| {
                let mut data = mem::zeroed();
                if cef_dll_sys::cef_string_wide_to_utf16(
                    value.as_ptr() as *const _,
                    value.len(),
                    &mut data,
                ) == 0
                {
                    None
                } else {
                    Some(data)
                }
            })
        }))
    }
}

impl Display for CefStringUtf16 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = CefStringUtf8::from(self);
        if let Some(value) = value.as_str() {
            write!(f, "{value}")
        } else {
            eprintln!("Invalid UTF-16 string");
            Ok(())
        }
    }
}

/// See [_cef_string_wide_t] for more documentation.
#[derive(Clone, Default)]
pub struct CefStringWide(CefStringData<_cef_string_wide_t>);

impl Drop for CefStringWide {
    fn drop(&mut self) {
        if let CefStringData::Clear(mut value) = &mut self.0 {
            if let Some(mut value) = mem::take(&mut value) {
                unsafe {
                    cef_dll_sys::cef_string_wide_clear(&mut value);
                }
            }
        }
    }
}

impl From<&str> for CefStringWide {
    fn from(value: &str) -> Self {
        Self(CefStringData::Clear(unsafe {
            let mut data = mem::zeroed();
            if cef_dll_sys::cef_string_utf8_to_wide(
                value.as_bytes().as_ptr() as *const _,
                value.as_bytes().len(),
                &mut data,
            ) == 0
            {
                None
            } else {
                Some(data)
            }
        }))
    }
}

impl From<&CefStringUserfreeWide> for CefStringWide {
    fn from(value: &CefStringUserfreeWide) -> Self {
        let value: Option<&_cef_string_wide_t> = value.into();
        Self(CefStringData::Clear(value.and_then(|value| unsafe {
            let mut data = mem::zeroed();
            if cef_dll_sys::cef_string_wide_set(value.str_, value.length, &mut data, 1) == 0 {
                None
            } else {
                Some(data)
            }
        })))
    }
}

impl From<*const _cef_string_wide_t> for CefStringWide {
    fn from(value: *const _cef_string_wide_t) -> Self {
        Self(value.into())
    }
}

impl From<*mut _cef_string_wide_t> for CefStringWide {
    fn from(value: *mut _cef_string_wide_t) -> Self {
        Self(value.into())
    }
}

impl From<&CefStringWide> for *const _cef_string_wide_t {
    fn from(value: &CefStringWide) -> Self {
        let data: Option<&_cef_string_wide_t> = (&value.0).into();
        data.map(ptr::from_ref).unwrap_or(ptr::null())
    }
}

impl From<&mut CefStringWide> for *mut _cef_string_wide_t {
    fn from(value: &mut CefStringWide) -> Self {
        match &mut value.0 {
            CefStringData::BorrowedMut(value) => value.map(|value| value.as_ptr()),
            _ => None,
        }
        .unwrap_or(ptr::null_mut())
    }
}

impl From<_cef_string_wide_t> for CefStringWide {
    fn from(value: _cef_string_wide_t) -> Self {
        Self(CefStringData::Borrowed(Some(value)))
    }
}

impl From<CefStringWide> for _cef_string_wide_t {
    fn from(value: CefStringWide) -> Self {
        match value.0 {
            CefStringData::Borrowed(value) => value,
            _ => None,
        }
        .unwrap_or(unsafe { mem::zeroed() })
    }
}

impl CefStringWide {
    pub fn as_slice(&self) -> Option<&[i32]> {
        let data: Option<&_cef_string_wide_t> = (&self.0).into();
        let (str_, length) = data.map(|value| (value.str_, value.length))?;
        Some(unsafe { slice::from_raw_parts(str_ as *const _, length) })
    }

    pub fn try_set(&mut self, value: &str) -> bool {
        let CefStringData::BorrowedMut(Some(data)) = &mut self.0 else {
            return false;
        };

        unsafe {
            cef_dll_sys::cef_string_wide_clear(data.as_ptr());
            cef_dll_sys::cef_string_utf8_to_wide(
                value.as_ptr() as *const _,
                value.len(),
                data.as_ptr(),
            ) != 0
        }
    }
}

impl From<&CefStringUtf8> for CefStringWide {
    fn from(value: &CefStringUtf8) -> Self {
        Self(CefStringData::Clear(unsafe {
            value.as_str().and_then(|value| {
                let mut data = mem::zeroed();
                if cef_dll_sys::cef_string_utf8_to_wide(
                    value.as_bytes().as_ptr() as *const _,
                    value.as_bytes().len(),
                    &mut data,
                ) == 0
                {
                    None
                } else {
                    Some(data)
                }
            })
        }))
    }
}

impl From<&CefStringUtf16> for CefStringWide {
    fn from(value: &CefStringUtf16) -> Self {
        Self(CefStringData::Clear(unsafe {
            value.as_slice().and_then(|value| {
                let mut data = mem::zeroed();
                if cef_dll_sys::cef_string_utf16_to_wide(
                    value.as_ptr() as *const _,
                    value.len(),
                    &mut data,
                ) == 0
                {
                    None
                } else {
                    Some(data)
                }
            })
        }))
    }
}

impl Display for CefStringWide {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = CefStringUtf8::from(self);
        if let Some(value) = value.as_str() {
            write!(f, "{value}")
        } else {
            Ok(())
        }
    }
}

/// See [_cef_string_list_t] for more documentation.
pub struct CefStringList(*mut _cef_string_list_t);

impl Drop for CefStringList {
    fn drop(&mut self) {
        unsafe {
            self.0
                .as_mut()
                .map(|value| cef_dll_sys::cef_string_list_free(value));
        }
    }
}

impl From<*mut _cef_string_list_t> for CefStringList {
    fn from(value: *mut _cef_string_list_t) -> Self {
        Self(value)
    }
}

impl From<&mut CefStringList> for *mut _cef_string_list_t {
    fn from(value: &mut CefStringList) -> Self {
        value.0
    }
}

impl IntoIterator for CefStringList {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let list = unsafe { self.0.as_mut() };
        list.map(|list| {
            let count = unsafe { cef_dll_sys::cef_string_list_size(list) };
            (0..count)
                .filter_map(|i| unsafe {
                    let mut value = mem::zeroed();
                    (cef_dll_sys::cef_string_list_value(list, i, &mut value) > 0).then_some(value)
                })
                .map(|value| {
                    CefStringUtf8::from(&CefString::from(ptr::from_ref(&value))).to_string()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
        .into_iter()
    }
}

/// See [_cef_string_map_t] for more documentation.
pub struct CefStringMap(*mut _cef_string_map_t);

impl Drop for CefStringMap {
    fn drop(&mut self) {
        unsafe {
            self.0
                .as_mut()
                .map(|value| cef_dll_sys::cef_string_map_free(value));
        }
    }
}

impl From<*mut _cef_string_map_t> for CefStringMap {
    fn from(value: *mut _cef_string_map_t) -> Self {
        Self(value)
    }
}

impl From<&mut CefStringMap> for *mut _cef_string_map_t {
    fn from(value: &mut CefStringMap) -> Self {
        value.0
    }
}

impl IntoIterator for CefStringMap {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let map = unsafe { self.0.as_mut() };
        map.map(|map| {
            let count = unsafe { cef_dll_sys::cef_string_map_size(map) };
            (0..count)
                .filter_map(|i| unsafe {
                    let mut key = mem::zeroed();
                    let mut value = mem::zeroed();
                    (cef_dll_sys::cef_string_map_key(map, i, &mut key) > 0
                        && cef_dll_sys::cef_string_map_value(map, i, &mut value) > 0)
                        .then_some((key, value))
                })
                .map(|(key, value)| {
                    (
                        CefStringUtf8::from(&CefString::from(ptr::from_ref(&key))).to_string(),
                        CefStringUtf8::from(&CefString::from(ptr::from_ref(&value))).to_string(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
        .into_iter()
    }
}

/// See [_cef_string_multimap_t] for more documentation.
pub struct CefStringMultimap(*mut _cef_string_multimap_t);

impl Drop for CefStringMultimap {
    fn drop(&mut self) {
        unsafe {
            self.0
                .as_mut()
                .map(|value| cef_dll_sys::cef_string_multimap_free(value));
        }
    }
}

impl From<*mut _cef_string_multimap_t> for CefStringMultimap {
    fn from(value: *mut _cef_string_multimap_t) -> Self {
        Self(value)
    }
}

impl From<&mut CefStringMultimap> for *mut _cef_string_multimap_t {
    fn from(value: &mut CefStringMultimap) -> Self {
        value.0
    }
}
