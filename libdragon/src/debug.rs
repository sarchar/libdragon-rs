#![allow(non_upper_case_globals)]

use crate::*;

/// Flag to activate the USB logging channel.
///
/// See [libdragon_sys::DEBUG_FEATURE_LOG_USB] for details.
pub const FEATURE_LOG_USB     : u32 = libdragon_sys::DEBUG_FEATURE_LOG_USB as u32;
/// Flag to activate the ISViewer logging channel.
///
/// See [libdragon_sys::DEBUG_FEATURE_LOG_ISVIEWER] for details.
pub const FEATURE_LOG_ISVIEWER: u32 = libdragon_sys::DEBUG_FEATURE_LOG_ISVIEWER as u32;
/// Flag to activate the logging on CompactFlash/SD card
///
/// See [libdragon_sys::DEBUG_FEATURE_LOG_SD] for details.
pub const FEATURE_LOG_SD      : u32 = libdragon_sys::DEBUG_FEATURE_LOG_SD as u32;
/// Flag to activate filesystem access to files on CompactFlash/SD.
///
/// See [libdragon_sys::DEBUG_FEATURE_FILE_SD] for details.
pub const FEATURE_FILE_SD     : u32 = libdragon_sys::DEBUG_FEATURE_FILE_SD as u32;
/// Flag to activate all supported debugging features
///
/// See [libdragon_sys::DEBUG_FEATURE_ALL] for details.
pub const FEATURE_ALL         : u32 = libdragon_sys::DEBUG_FEATURE_ALL as u32;

/// Initialize USB logging.
pub fn init_usblog() -> bool {
    unsafe {
        libdragon_sys::debug_init_usblog()
    }
}

/// Initialize ISViewer logging.
pub fn init_isviewer() -> bool {
    unsafe {
        libdragon_sys::debug_init_isviewer()
    }
}

/// Initialize SD logging.
pub fn init_sdlog(filename: &str, openfmt: &str) -> bool {
    let cfilename = CString::new(filename).unwrap();
    let copenfmt = CString::new(openfmt).unwrap();
    unsafe {
        libdragon_sys::debug_init_sdlog(cfilename.as_ptr(), copenfmt.as_ptr()) 
    }
}

/// Initialize SD filesystem
pub fn init_sdfs(prefix: &str, npart: i32) -> bool {
    let cprefix = CString::new(prefix).unwrap();
    unsafe {
        libdragon_sys::debug_init_sdfs(cprefix.as_ptr(), npart)
    }
}

/// Shutdown SD filesystem.
pub fn close_sdfs() {
    unsafe {
        libdragon_sys::debug_close_sdfs();
    }
}

/// Initialize debugging features of libdragon.
///
/// See [`debug_init`](libdragon_sys::debug_init)
pub fn init(features: u32) -> bool {
    let mut ok = false;
    if (features & FEATURE_LOG_USB) != 0 {
        ok = init_usblog() || ok;
    }
    if (features & FEATURE_LOG_ISVIEWER) != 0 {
        ok = init_isviewer() || ok;
    }
    if (features & FEATURE_FILE_SD) != 0 {
        ok = init_sdfs("sd:/", 0) || ok;
    }
    if (features & FEATURE_LOG_SD) != 0 {
        ok = init_sdlog("sd:/libdragon.log", "a") || ok;
    }
    ok
}


/// Do a hexdump of the specified buffer via `debugf` (see LibDragon debug.h)
///
/// See [`debug_hexdump`](libdragon_sys::debug_hexdump) for details.
pub fn hexdump<T>(buf: &[T]) {
    unsafe {
        libdragon_sys::debug_hexdump(buf.as_ptr() as *const ::core::ffi::c_void, 
                                     (buf.len() * ::core::mem::size_of::<T>()) as i32);
    }
}

/// Dump a backtrace (call stack) via `debugf`
///
/// See [`debug_backtrace`](libdragon_sys::debug_backtrace)
pub fn backtrace() { unsafe { libdragon_sys::debug_backtrace(); } }

/// Underlying implementation function for assert() and `assertf` in LibDragon
///
/// See [`debug_assert_func_f`](libdragon_sys::debug_assert_func_f)
pub fn debug_assert_func_f(file: &str, line: u32, func: &str, failedexpr: &str, msg: &str) -> ! {
    let c_file  = CString::new(file).unwrap();
    let c_func  = CString::new(func).unwrap();
    let c_fexpr = CString::new(failedexpr).unwrap();
    let c_msg   = CString::new(msg).unwrap();
    let fmt_str = CString::new("%s").unwrap();
    unsafe {
        libdragon_sys::debug_assert_func_f(c_file.as_ptr(), line as i32, c_func.as_ptr(), c_fexpr.as_ptr(), fmt_str.as_ptr(), c_msg.as_ptr())
    }
}




