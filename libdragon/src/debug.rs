#![allow(non_upper_case_globals)]

use crate::*;

pub const FEATURE_LOG_USB     : u32 = 1 << 0;
pub const FEATURE_LOG_ISVIEWER: u32 = 1 << 1;
pub const FEATURE_LOG_SD      : u32 = 1 << 2;
pub const FEATURE_FILE_SD     : u32 = 1 << 3;
pub const FEATURE_ALL         : u32 = 0xFF;

pub fn init_usblog() -> bool {
    unsafe {
        libdragon_sys::debug_init_usblog()
    }
}

pub fn init_isviewer() -> bool {
    unsafe {
        libdragon_sys::debug_init_isviewer()
    }
}

pub fn init_sdlog(filename: &str, openfmt: &str) -> bool {
    let cfilename = CString::new(filename).unwrap();
    let copenfmt = CString::new(openfmt).unwrap();
    unsafe {
        libdragon_sys::debug_init_sdlog(cfilename.as_ptr(), copenfmt.as_ptr()) 
    }
}

pub fn init_sdfs(prefix: &str, npart: i32) -> bool {
    let cprefix = CString::new(prefix).unwrap();
    unsafe {
        libdragon_sys::debug_init_sdfs(cprefix.as_ptr(), npart)
    }
}

pub fn close_sdfs() {
    unsafe {
        libdragon_sys::debug_close_sdfs();
    }
}

pub fn init_features(features: u32) -> bool {
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


