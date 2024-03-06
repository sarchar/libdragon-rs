#![no_std]
#![feature(asm_experimental_arch)]
#![feature(panic_info_message)]

use core::arch::asm;

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

pub use paste::paste;

mod allocator;
mod panic;

pub mod audio;
pub mod console;
pub mod debug;
pub mod dfs;
pub mod display;
pub mod gl;
pub mod glu;
pub mod graphics;
pub mod joypad;
pub mod rdp;
pub mod rdpq;
pub mod rspq;
pub mod sprite;
pub mod timer;

#[derive(Debug)]
pub enum LibDragonError {
    DfsError { error: dfs::DfsError },
    ErrnoError { errno: u32 },
    Utf8Error { error: core::str::Utf8Error },
}

impl From<core::str::Utf8Error> for LibDragonError {
    fn from(error: core::str::Utf8Error) -> Self {
        Self::Utf8Error { error: error }
    }
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

pub trait N64Pointer<'a> {
    fn uncached_mut(&'a mut self) -> &'a mut Self;
    fn uncached_ref(&'a self) -> &'a Self;
    fn physical_ref(&'a self) -> &'a Self;
    fn cache_hit_invalidate(&self, write_back: bool);
}

impl<'a, T> N64Pointer<'a> for [T] {
    fn uncached_mut(&'a mut self) -> &'a mut Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts_mut(((self.as_mut_ptr() as u32) | 0x2000_0000) as *mut T, len)
        }
    }

    fn uncached_ref(&'a self) -> &'a Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts(((self.as_ptr() as u32) | 0x2000_0000) as *const T, len)
        }
    }

    fn physical_ref(&'a self) -> &'a Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts(((self.as_ptr() as u32) & !0xE000_0000) as *const T, len)
        }
    }

    fn cache_hit_invalidate(&self, write_back: bool) {
        let size = self.len() * ::core::mem::size_of::<T>();
        data_cache_hit_invalidate(self.as_ptr(), size, write_back);
    }
}

pub fn uncached_mut<'a, T>(v: &'a mut T) -> &'a mut T {
    unsafe {
        &mut *((((v as *mut T as u32) & 0x1FFF_FFFF) | 0xA000_0000) as *mut T)
    }
}

pub fn uncached_ref<'a, T>(v: &'a T) -> &'a T {
    unsafe {
        &*((((v as *const T as u32) & 0x1FFF_FFFF) | 0xA000_0000) as *const T)
    }
}

pub fn physical_ref<'a, T>(v: &'a T) -> &'a T {
    unsafe {
        &*(((v as *const T as u32) & !0xE000_0000) as *const T)
    }
}

pub fn data_cache_hit_invalidate<T>(v: *const T, size: usize, write_back: bool) {
    let base = v as u32;
    assert!(base & 0x0F == 0, "address must be a multiple of 16");
    assert!(size & 0x0F == 0, "size must be a multiple of 16");
    for addr in (base..(base + size as u32)).step_by(16) {
        if write_back {
            unsafe { 
                asm!(".set noat", 
                     "cache (5<<2)|1, 0({reg})", 
                     reg = in(reg) addr) 
            };
        } else {
            unsafe { 
                asm!(".set noat", 
                     "cache (4<<2)|1, 0({reg})", 
                     reg = in(reg) addr) 
            };
        }
    }
}
