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

// re-export libdragon_sys, as things will be changing and there always may be a need for direct access
pub use libdragon_sys;

//#[macro_use] extern crate function_name;
pub use function_name::named;

#[doc(hidden)]
pub use paste::paste;

mod allocator;
mod panic;

/// Asset subsystem
pub mod asset;
/// Audio - mixer, wav64, etc.
pub mod audio;
/// Console emulator
pub mod console;
/// Debugging Support
pub mod debug;
/// Dragon Filesystem
pub mod dfs;
/// Display subsystem
pub mod display;
/// OpenGL support
pub mod gl;
/// GLU helper functions
pub mod glu;
/// Graphics (lines, text, etc.)
pub mod graphics;
/// Input support
pub mod joypad;
/// Direct RDP commands
pub mod rdp;
/// RDPQ module
pub mod rdpq;
/// RSP low-level hardware library
pub mod rsp;
/// RSPQ module
pub mod rspq;
/// Sprites (2D renderable objects)
pub mod sprite;
/// Surface buffers used to draw images
pub mod surface;
/// System timer support
pub mod timer;
/// Throttling engine
pub mod throttle;

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

/// [Register] provides volatile access to a memory region
///
/// Use the [`read()`](Register::read) and [`write()`](Register::write) functions to read
/// and write to a given memory address.
pub struct Register<T: Copy, const SIZE: usize = 1> {
    address: *mut T
}

impl<T: Copy, const SIZE: usize> Register<T, SIZE> {
    /// Write a single piece of data `T` to the address specified by the register
    pub fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(self.address, value); }
    }

    /// Write a slice of data of type `T` to the address specified by the register
    ///
    /// `offset` is in element counts, not bytes
    pub fn write_slice(&mut self, data: &[T], offset: usize) {
        assert!((offset + data.len()) <= SIZE, "overflow memory write");
        unsafe {
            for i in 0..data.len() {
                let addr = (self.address as usize + (offset + i) * ::core::mem::size_of::<T>()) as *mut _;
                core::ptr::write_volatile(addr, data[i]);
            }
        }
    }

    /// Read a single piece of data `T` from the address specified by the register
    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(self.address) }
    }

    /// Read an array of data of type `T` from the address specified by the register
    pub fn read_slice(&self, len: usize, offset: usize) -> Vec<T> {
        assert!((offset + len) <= SIZE, "overflow memory read");
        let mut storage = Vec::with_capacity(len);
        unsafe {
            for i in 0..len {
                let addr = (self.address as usize + (offset + i) * ::core::mem::size_of::<T>()) as *mut _;
                storage[i] = core::ptr::read_volatile(addr);
            }
        }
        storage
    }
}


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

#[doc(hidden)]
pub fn libdragon_fprintf(msg: &str) -> i32 {
    let c_str   = CString::new(msg).unwrap();
    let fmt_str = CString::new("%s").unwrap();
    unsafe {
        libdragon_sys::fprintf(get_stderr(), fmt_str.as_ptr(), c_str.as_ptr()) as i32
    }
}

#[doc(hidden)]
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
#[doc(hidden)]
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

pub fn boot_consoletype() -> i32 {
    unsafe { libdragon_sys::__boot_consoletype }
}

#[inline(always)]
pub fn rsp_frequency() -> u64 {
    if boot_consoletype() != 0 { 96000000 } else { 62500000 }
}

#[inline(always)]
pub fn cpu_frequency() -> u64 {
    if boot_consoletype() != 0 { 144000000 } else { 93750000 }
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
