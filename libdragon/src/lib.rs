#![no_std]
#![feature(asm_experimental_arch)]
#![feature(panic_info_message)]

use cstr_core::{CStr, CString};

// Re-exports of common types and macros
extern crate alloc;
pub use alloc::borrow::ToOwned;
pub use alloc::string::String;
pub use alloc::string::ToString;
pub use alloc::vec;
pub use alloc::vec::Vec;
pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::sync::Arc;

mod allocator;
mod panic;

pub mod console;
pub mod debug;
pub mod dfs;
pub mod display;
pub mod graphics;
pub mod joypad;
pub mod rdp;
pub mod rdpq;
pub mod rspq;
pub mod timer;

#[derive(Debug)]
pub enum LibDragonError {
    DfsError { error: dfs::DfsError },
    ErrnoError { errno: u32 },
}

pub type Result<T> = core::result::Result<T, LibDragonError>;

extern "C" {
    static _gp: ::core::ffi::c_int;

    // keep rust optimizer from removing the entry point
    fn _start() -> !;

    fn __getreent() -> *mut libdragon_sys::_reent;
}

fn get_errno() -> u32 {
    unsafe { (*__getreent())._errno as u32 }
}

fn get_stderr() -> *mut libdragon_sys::__FILE {
    unsafe { (*__getreent())._stderr }
}

pub fn wait_ms(ms: u32) {
    unsafe {
        libdragon_sys::wait_ms(ms as ::core::ffi::c_ulong);
    }
}

pub fn libdragon_fprintf(msg: &str) -> i32 {
    let c_str   = CString::new(msg).unwrap();
    let fmt_str = CString::new("%s").unwrap();
    unsafe {
        libdragon_sys::fprintf(get_stderr(), fmt_str.as_ptr(), c_str.as_ptr()) as i32
    }
}

pub fn libdragon_printf(msg: &str) -> i32 {
    let c_str   = CString::new(msg).unwrap();
    let fmt_str = CString::new("%s").unwrap();
    unsafe {
        libdragon_sys::printf(fmt_str.as_ptr(), c_str.as_ptr()) as i32
    }
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ({ let _ = libdragon_fprintf(&format!($($arg)*)); });
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({ let _ = libdragon_printf(&format!($($arg)*)); });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// LLVM clobbers $gp, which is used by the C code.  This is an _attempt_ at fixing
// $gp before each and every libdragon call.  There's still a possibility that llvm
// will change gp before the library call but this should make it less likely.
// TODO TODO TODO fix gpopt !
#[macro_export]
macro_rules! protect_gp {
    ( $($s:stmt);* ) => {
        let oldgp: *const ::core::ffi::c_void;
        unsafe { 
            let gp = &crate::_gp;
            asm!(".set noat", "move {0}, $gp", "move $gp, {1}", out(reg) oldgp, in(reg) gp);
        }
        let r = (||{
            $($s)*
        })();
        unsafe {
            asm!(".set noat", "move $gp, {0}", in(reg) oldgp);
        }
        r
    }
}

pub fn __boot_consoletype() -> i32 {
    unsafe { libdragon_sys::__boot_consoletype }
}

#[inline(always)]
pub fn rsp_frequency() -> u64 {
    if __boot_consoletype() != 0 { 96000000 } else { 62500000 }
}

#[inline(always)]
pub fn cpu_frequency() -> u64 {
    if __boot_consoletype() != 0 { 144000000 } else { 93750000 }
}

#[inline(always)]
pub fn disable_interrupts() {
    unsafe { libdragon_sys::disable_interrupts(); }
}

#[inline(always)]
pub fn enable_interrupts() {
    unsafe { libdragon_sys::enable_interrupts(); }
}
