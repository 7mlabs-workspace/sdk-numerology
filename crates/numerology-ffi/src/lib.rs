//! UTF-8 C ABI. Release returned result pointers exactly once with numerology_free.
use std::ffi::{c_char, CStr, CString};
fn owned(result: String) -> *mut c_char {
    CString::new(result).unwrap().into_raw()
}
fn failure(code: &str, message: &str) -> String {
    format!("{{\"ok\":false,\"error\":{{\"code\":\"{code}\",\"field\":\"request\",\"message\":\"{message}\"}}}}")
}
#[no_mangle]
pub extern "C" fn numerology_abi_version() -> u32 {
    1
}
/// Borrowed static version string: never free this pointer.
#[no_mangle]
pub extern "C" fn numerology_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}
/// # Safety
/// Input must be null or point to at least len readable bytes for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn numerology_calculate_json_len(
    input: *const u8,
    len: usize,
) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        if input.is_null() {
            return failure("NULL_INPUT", "Input pointer is null");
        }
        if len > numerology_core::MAX_REQUEST_BYTES {
            return numerology_core::calculate_json(
                &" ".repeat(numerology_core::MAX_REQUEST_BYTES + 1),
            );
        }
        let bytes = unsafe { std::slice::from_raw_parts(input, len) };
        match std::str::from_utf8(bytes) {
            Ok(s) => numerology_core::calculate_json(s),
            Err(_) => failure("INVALID_UTF8", "Input must be UTF-8"),
        }
    })
    .unwrap_or_else(|_| failure("INTERNAL_ERROR", "Calculation failed"));
    owned(result)
}
/// # Safety
/// Input must be null or point to a valid NUL-terminated string for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn numerology_calculate_json(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return unsafe { numerology_calculate_json_len(std::ptr::null(), 0) };
    }
    let bytes = unsafe { CStr::from_ptr(input) }.to_bytes();
    unsafe { numerology_calculate_json_len(bytes.as_ptr(), bytes.len()) }
}
/// # Safety
/// Pointer must be null or returned by a calculate function and not previously freed.
#[no_mangle]
pub unsafe extern "C" fn numerology_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    unsafe fn call(data: &[u8]) -> String {
        let p = unsafe { numerology_calculate_json_len(data.as_ptr(), data.len()) };
        let s = unsafe { CStr::from_ptr(p) }.to_str().unwrap().to_string();
        unsafe { numerology_free(p) };
        s
    }
    #[test]
    fn bytes_and_nul() {
        for bytes in [b"null".as_slice(), b"{}\0ignored", b"\xff"] {
            let s = unsafe { call(bytes) };
            assert!(s.contains("\\\"ok\\\":false") || s.contains("\"ok\":false"));
        }
        unsafe { numerology_free(std::ptr::null_mut()) };
    }
    #[test]
    fn null_and_limits() {
        let p = unsafe { numerology_calculate_json_len(std::ptr::null(), 0) };
        assert!(unsafe { CStr::from_ptr(p) }
            .to_str()
            .unwrap()
            .contains("NULL_INPUT"));
        unsafe { numerology_free(p) };
        assert!(unsafe { call(&vec![b' '; 16385]) }.contains("REQUEST_TOO_LARGE"));
    }
}
