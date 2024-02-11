#![feature(restricted_std)]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};

pub use libdragon_sys::*;

pub fn get_stderr() -> *const u8 {
    // 0x488 == offset of stderr in _reent
    unsafe { __getreent().wrapping_add(0x488) }
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => (unsafe { fprintf(get_stderr() as *mut __sFILE, "%s\0".as_ptr() as *const i8, format!($($arg)*).as_ptr()) });
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (unsafe { printf("%s\0".as_ptr() as *const i8, format!($($arg)*).as_ptr()) });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


pub struct LibdragonAllocator;

#[global_allocator]
pub static ALLOCATOR: LibdragonAllocator = LibdragonAllocator {};

use core::ffi::c_void;

unsafe impl GlobalAlloc for LibdragonAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let _align = layout.align();

        malloc(size as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }
}

extern "C" {
    // keep rust optimizer from removing the entry point
    fn _start() -> !;

    fn __getreent() -> *const u8;
}

