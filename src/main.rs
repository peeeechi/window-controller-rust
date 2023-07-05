use core::ffi::{c_char, c_int, c_uint, c_void};
use std::ptr::null;
// extern crate libc;

type Error = u32;
type Hwnd = *mut c_void;
type Char = c_char;
type Int = c_int;
type Uint = c_uint;
type Lpcstr = *const Char;
#[link(name = "kernel32")]
extern "stdcall" {
    fn GetLastError() -> Error;
}

#[link(name = "user32")]
extern "stdcall" {
    fn FindWindow(lpClassName: Lpcstr, lpWindowName: Lpcstr) -> Hwnd;

    fn GetWindowText(hWnd: Hwnd) -> Int;
}

pub fn get_window_text() {
    unsafe {
        let t = FindWindow(null(), null());
        println!("{:?}", t);
    }
}

fn main() {
    get_window_text();
}

// trait ToResult: Sized {
//     n to_result(&self) -> Result<Self, Error>;
// }
//
// impl ToResult for *const c_void {
//     fn to_result(&self) -> Result<*const c_void, Error> {
//         if *self == ptr::null() {
//             unsafe { Err(GetLastError()) }
//         } else {
//             Ok(*self)
//         }
//     }
// }
//
// trait IntoNullTerminatedU16 {
//     fn to_nullterminated_u16(&self) -> Vec<u16>;
// }
//
// impl IntoNullTerminatedU16 for str {
//     fn to_nullterminated_u16(&self) -> Vec<u16> {
//         self.encode_utf16().chain(Some(0)).collect()
//     }
// }
