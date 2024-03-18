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
/// DMA controller
pub mod dma;
/// OpenGL support
pub mod gl;
/// GLU helper functions
pub mod glu;
/// Graphics (lines, text, etc.)
pub mod graphics;
/// Joybus Subsystem
pub mod joybus;
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
    AccessoryIoError { error: joybus::AccessoryIoStatus },
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

/// eprint implementation that displays messages to the LibDragon debug log
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ({ let _ = libdragon_fprintf(&format!($($arg)*)); });
}

/// eprintln implementation that displays messages to the LibDragon debug log
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

/// print implementation that displays messages to the LibDragon console
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({ let _ = libdragon_printf(&format!($($arg)*)); });
}

/// println implementation that displays messages to the LibDragon console
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

/// Indicates whether we are running on a vanillia N64 or iQue player.
///
/// See [`__boot_consoletype`](libdragon_sys::__boot_consoletype) for details.
pub fn boot_consoletype() -> i32 { unsafe { libdragon_sys::__boot_consoletype } }

/// Frequency of the RCP
#[inline(always)]
pub fn rsp_frequency() -> u32 { if boot_consoletype() != 0 { 96000000 } else { 62500000 } }

/// Frequency of the MIPS R4300 CPU
#[inline(always)]
pub fn cpu_frequency() -> u32 { if boot_consoletype() != 0 { 144000000 } else { 93750000 } }

/// Pointer to the cached and non-mapped memory start address
pub const KSEG0_START_ADDR: *mut ::core::ffi::c_void = 0x8000_0000 as *mut _;

/// This trait adds features to slices that allow you to get uncached references to the memory.
pub trait N64Pointer<'a> {
    fn cached_mut(&'a mut self) -> &'a mut Self;
    fn cached_ref(&'a self) -> &'a Self;
    fn uncached_mut(&'a mut self) -> &'a mut Self;
    fn uncached_ref(&'a self) -> &'a Self;
    fn physical_ref(&'a self) -> &'a Self;
    fn cache_hit_invalidate(&self, write_back: bool);
}

impl<'a, T> N64Pointer<'a> for [T] {
    /// Return a mutable reference to the slice using cached memory access
    #[inline]
    fn cached_mut(&'a mut self) -> &'a mut Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts_mut((((self.as_mut_ptr() as u32) & 0x1FFF_FFFF) | 0x8000_0000) as *mut T, len)
        }
    }

    /// Return a reference to the slice using cached memory access
    #[inline]
    fn cached_ref(&'a self) -> &'a Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts((((self.as_ptr() as u32) & 0x1FFF_FFFF) | 0x8000_0000) as *const T, len)
        }
    }

    /// Return a mutable reference to the slice using uncached memory access
    #[inline]
    fn uncached_mut(&'a mut self) -> &'a mut Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts_mut((((self.as_mut_ptr() as u32) & 0x1FFF_FFFF) | 0xA000_0000) as *mut T, len)
        }
    }

    /// Return a reference to the slice using uncached memory access
    #[inline]
    fn uncached_ref(&'a self) -> &'a Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts((((self.as_ptr() as u32) & 0x1FFF_FFFF) | 0xA000_0000) as *const T, len)
        }
    }

    /// Return a reference to the slice in the physical memory space
    #[inline]
    fn physical_ref(&'a self) -> &'a Self {
        let len = self.len();
        unsafe {
            ::core::slice::from_raw_parts(((self.as_ptr() as u32) & !0xE000_0000) as *const T, len)
        }
    }

    ///
    fn cache_hit_invalidate(&self, write_back: bool) {
        let size = self.len() * ::core::mem::size_of::<T>();
        data_cache_hit_invalidate(self.as_ptr(), size, write_back);
    }
}

/// Return a mutable reference to the object using cached memory access
#[inline]
pub fn cached_mut<'a, T>(v: &'a mut T) -> &'a mut T {
    unsafe {
        &mut *((((v as *mut T as u32) & 0x1FFF_FFFF) | 0x8000_0000) as *mut T)
    }
}

/// Return a reference to the object using cached memory access
#[inline]
pub fn cached_ref<'a, T>(v: &'a T) -> &'a T {
    unsafe {
        &*((((v as *const T as u32) & 0x1FFF_FFFF) | 0x8000_0000) as *const T)
    }
}

/// Return a mutable reference to the object using uncached memory access
#[inline]
pub fn uncached_mut<'a, T>(v: &'a mut T) -> &'a mut T {
    unsafe {
        &mut *((((v as *mut T as u32) & 0x1FFF_FFFF) | 0xA000_0000) as *mut T)
    }
}

/// Return a reference to the object using uncached memory access
#[inline]
pub fn uncached_ref<'a, T>(v: &'a T) -> &'a T {
    unsafe {
        &*((((v as *const T as u32) & 0x1FFF_FFFF) | 0xA000_0000) as *const T)
    }
}

/// Return a reference to the object in the physical memory space
#[inline]
pub fn physical_ref<'a, T>(v: &'a T) -> &'a T {
    unsafe {
        &*(((v as *const T as u32) & !0xE000_0000) as *const T)
    }
}

/// Symbol at the start of code (start of ROM contents after header)
#[inline]
pub fn libdragon_text_start<T>() -> &'static mut [T] { 
    unsafe { 
        core::slice::from_raw_parts_mut(libdragon_sys::__libdragon_text_start.as_mut_ptr() as *mut _, 0) 
    } 
}

/// Symbol at the end of code, data, and sdata (set by the linker)
#[inline]
pub fn rom_end<T>() -> &'static mut [T] { 
    unsafe { 
        core::slice::from_raw_parts_mut(libdragon_sys::__rom_end.as_mut_ptr() as *mut _, 0) 
    } 
}

/// Symbol at the end of code, data, sdata, and bss (set by the linker)
#[inline]
pub fn bss_end<T>() -> &'static mut [T] { 
    unsafe { 
        core::slice::from_raw_parts_mut(libdragon_sys::__bss_end.as_mut_ptr() as *mut _, 0) 
    } 
}

/// Pointer to the start of heap memory
#[inline] pub fn heap_start_addr<T>() -> &'static mut [T] { bss_end() }

/// Memory barrier
#[inline(always)] pub fn memory_barrier() { unsafe { core::arch::asm!(""); } }

/// Ticks helper functions.
pub mod ticks {
    //! For anything not self-explanatory, see LibDragon's n64sys.h header file for documetation
    //! and details.

    /// Returns the 32-bit hardware tick counter
    /// 
    /// See [`TICKS_READ`](libdragon_sys::TICKS_READ) for details.
    #[inline(always)] pub fn read() -> u32 { /*cop0::count()*/0 }

    /// Number of updates to the count register per second
    ///
    /// See [`TICKS_PER_SECOND`](libdragon_sys::TICKS_PER_SECOND) for details.
    #[inline(always)] pub fn per_second() -> u32 { crate::cpu_frequency() / 2 }

    /// Calculate the time passed between two ticks
    ///
    /// See [`TICKS_DISTANCE`](libdragon_sys::TICKS_DISTANCE) for details.
    #[inline(always)] pub fn distance(from: u32, to: u32) -> i32 { to.wrapping_sub(from) as i32 }

    /// Return how much time ahs passed since the instant t0
    ///
    /// See [`TICKS_SINCE`](libdragon_sys::TICKS_SINCE) for details
    #[inline(always)] pub fn since(t0: u32) -> i32 { distance(t0, read()) }

    /// Returns true if "t1" is before "t2"
    ///
    /// See [`TICKS_BEFORE`](libdragon_sys::TICKS_BEFORE) for details
    #[inline(always)] pub fn before(t1: u32, t2: u32) -> bool { distance(t1, t2) > 0 }

    /// Returns equivalent count ticks for the given milliseconds.
    ///
    /// See [`TICKS_FROM_MS`](libdragon_sys::TICKS_FROM_MS) for details.
    #[inline(always)] pub fn from_ms(val: u32) -> u32 { val * per_second() / 1000 }

    /// Returns equivalent count ticks for the given microseconds.
    ///
    /// See [`TICKS_FROM_US`](libdragon_sys::TICKS_FROM_US) for details.
    #[inline(always)] pub fn from_us(val: u32) -> u32 { val * (8 * per_second() / 1000000) / 8 }

    /// Returns equivalent microseconds for the given ticks.
    ///
    /// See [`TICKS_TO_US`](libdragon_sys::TICKS_TO_US) for details.
    #[inline(always)] pub fn to_us(val: u32) -> u32 { (val * 8) / ((8 * per_second()) / 1000000) }

    /// Returns equivalent milliseconds for the given ticks.
    ///
    /// See [`TICKS_TO_US`](libdragon_sys::TICKS_TO_US) for details.
    #[inline(always)] pub fn to_ms(val: u32) -> u32 { val / (per_second() / 1000) }

    /// Return a 64-bit ticks counter that never overflows
    ///
    /// See [`get_ticks`](libdragon_sys::get_ticks) for details.
    #[inline] pub fn get() -> u64 { unsafe { libdragon_sys::get_ticks() } }

    /// Read the number of microseconds since system startup
    ///
    /// See [`get_ticks_us`](libdragon_sys::get_ticks_us) for details.
    #[inline] pub fn get_us() -> u64 { unsafe { libdragon_sys::get_ticks_us() } }

    /// Read the number of milliseconds since system startup
    ///
    /// See [`get_ticks_ms`](libdragon_sys::get_ticks_ms) for details.
    #[inline] pub fn get_ms() -> u64 { unsafe { libdragon_sys::get_ticks_ms() } }

    /// Spin wait until the number of ticks have elapsed
    ///
    /// See [`wait_ticks`](libdragon_sys::wait_ticks) for details.
    #[inline] pub fn wait(wait: u32) { unsafe { libdragon_sys::wait_ticks(wait); } }

    /// Spin wait until the number of milliseconds have elapsed
    ///
    /// See [`wait_ms`](libdragon_sys::wait_ms) for details.
    #[inline] pub fn wait_ms(wait_ms: u32) { unsafe { libdragon_sys::wait_ms(wait_ms); } }
}

/// See [`sys_bbplayer`](libdragon_sys::sys_bbplayer) for details.
#[inline] pub fn sys_bbplayer() -> bool { unsafe { libdragon_sys::sys_bbplayer() } }

/// Force a complete halt of all processors
/// 
/// See [`die`](libdragon_sys::die) for details
#[inline(always)] pub fn die() -> ! { unsafe { libdragon_sys::die(); } }

/// Rust implementation for
/// [`data_cache_hit_(writeback_)invalidate`](libdragon_sys::data_cache_hit_writeback_invalidate]
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

/// See
/// [`data_cache_index_writeback_invalidate`](libdragon_sys::data_cache_index_writeback_invalidate)
/// for details.
#[inline] pub fn data_cache_index_writeback_invalidate<T>(ptr: &mut [T]) {
    let size = ptr.len() * ::core::mem::size_of::<T>();
    unsafe { libdragon_sys::data_cache_index_writeback_invalidate(ptr.as_ptr() as *mut _, size as u32); }
}

/// See
/// [`data_cache_writeback_invalidate_all`](libdragon_sys::data_cache_writeback_invalidate_all)
/// for details.
#[inline] pub fn data_cache_writeback_invalidate_all() { unsafe { libdragon_sys::data_cache_writeback_invalidate_all(); } }

/// Rust implementation for
/// [`inst_cache_hit_(writeback_)invalidate`](libdragon_sys::inst_cache_hit_writeback_invalidate]
pub fn inst_cache_hit_invalidate<T>(v: *const T, size: usize, write_back: bool) {
    let base = v as u32;
    assert!(base & 0x0F == 0, "address must be a multiple of 16");
    assert!(size & 0x0F == 0, "size must be a multiple of 16");
    for addr in (base..(base + size as u32)).step_by(16) {
        if write_back {
            unsafe { 
                asm!(".set noat", 
                     "cache (5<<2)|0, 0({reg})", 
                     reg = in(reg) addr) 
            };
        } else {
            unsafe { 
                asm!(".set noat", 
                     "cache (4<<2)|0, 0({reg})", 
                     reg = in(reg) addr) 
            };
        }
    }
}

/// See
/// [`inst_cache_index_invalidate`](libdragon_sys::inst_cache_index_invalidate)
/// for details.
#[inline] pub fn inst_cache_index_invalidate<T>(ptr: &mut [T]) {
    let size = ptr.len() * ::core::mem::size_of::<T>();
    unsafe { libdragon_sys::inst_cache_index_invalidate(ptr.as_ptr() as *mut _, size as u32); }
}

/// See
/// [`inst_cache_invalidate_all`](libdragon_sys::inst_cache_invalidate_all)
/// for details.
#[inline] pub fn inst_cache_invalidate_all() { unsafe { libdragon_sys::inst_cache_invalidate_all(); } }

/// Get amount of available memory.
///
/// See [`get_memory_size`](libdragon_sys::get_memory_size)for details.
#[inline] pub fn get_memory_size() -> usize { unsafe { libdragon_sys::get_memory_size() as usize } }

/// Is expansion pak in use
///
/// See [`is_memory_expanded`](libdragon_sys::is_memory_expanded)for details.
#[inline] pub fn is_memory_expanded() -> usize { unsafe { libdragon_sys::is_memory_expanded() as usize } }

/// Allocate a buffer that will be accessed as uncached memory
///
/// See [`malloc_uncached`](libdragon_sys::malloc_uncached)for details.
#[inline] pub unsafe fn malloc_uncached<T>(size: usize) -> *mut T { libdragon_sys::malloc_uncached(size) as *mut _ }

/// Allocate a buffer that will be accessed as uncached memory, specifying alignment.
///
/// See [`malloc_uncached_aligned`](libdragon_sys::malloc_uncached_aligned)for details.
#[inline] pub unsafe fn malloc_uncached_aligned<T>(align: i32, size: usize) -> *mut T { libdragon_sys::malloc_uncached_aligned(align, size) as *mut _ }

/// Free an uncached memory buffer
///
/// See [`free_uncached`](libdragon_sys::free_uncached)for details.
#[inline] pub unsafe fn free_uncached<T>(ptr: *mut T) { libdragon_sys::free_uncached(ptr as *mut _); }

/// Type of TV video output
///
/// See [`tv_type_t`](libdragon_sys::tv_type_t) for details.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TvType {
    Pal,
    Ntsc,
    Mpal,
}

impl From<u32> for TvType {
    fn from(val: u32) -> TvType {
        match val {
            libdragon_sys::tv_type_t_TV_PAL => TvType::Pal,
            libdragon_sys::tv_type_t_TV_NTSC => TvType::Ntsc,
            libdragon_sys::tv_type_t_TV_MPAL => TvType::Mpal,
            _ => panic!("invalid value"),
        }
    }
}

/// Is system NTSC/PAL/MPAL
///
/// See [`get_tv_type`](libdragon_sys::get_tv_type) for details.
#[inline] pub fn get_tv_type() -> TvType { unsafe { libdragon_sys::get_tv_type().into() } }

/// Reset type
///
/// See [`reset_type_t`](libdragon_sys::reset_type_t) for details.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResetType {
    Cold,
    Warm,
}

impl From<u32> for ResetType {
    fn from(val: u32) -> ResetType {
        match val {
            libdragon_sys::reset_type_t_RESET_COLD => ResetType::Cold,
            libdragon_sys::reset_type_t_RESET_WARM => ResetType::Warm,
            _ => panic!("invalid value"),
        }
    }
}

/// Get reset type
///
/// See [`sys_reset_type`](libdragon_sys::sys_reset_type) for details.
#[inline] pub fn sys_reset_type() -> TvType { unsafe { libdragon_sys::sys_reset_type().into() } }

/// Read a 8-bit value from memory at the given 64-bit virtual address
///
/// See [`mem_read8`](libdragon_sys::mem_read8) for details
#[inline] pub unsafe fn mem_read8(vaddr: u64) -> u8 { (vaddr as *const u8).read_volatile() }

/// Read a 16-bit value from memory at the given 64-bit virtual address
///
/// See [`mem_read16`](libdragon_sys::mem_read16) for details
#[inline] pub unsafe fn mem_read16(vaddr: u64) -> u16 { (vaddr as *const u16).read_volatile() }

/// Read a 32-bit value from memory at the given 64-bit virtual address
///
/// See [`mem_read32`](libdragon_sys::mem_read32) for details
#[inline] pub unsafe fn mem_read32(vaddr: u64) -> u32 { (vaddr as *const u32).read_volatile() }

/// Read a 64-bit value from memory at the given 64-bit virtual address
///
/// See [`mem_read64`](libdragon_sys::mem_read64) for details
#[inline] pub unsafe fn mem_read64(vaddr: u64) -> u64 { (vaddr as *const u64).read_volatile() }

/// Interrupt Controller
pub mod interrupts {
    #![allow(non_snake_case)]
    use crate::Box;
    use crate::paste;

    type InterruptCallback = Box<dyn FnMut() + 'static + Sync + Send>;

    macro_rules! int_handler {
        ($lower_name:ident, $upper_name:ident) => {
            paste! {
                static mut [< $upper_name _CALLBACK >]: Option<InterruptCallback> = None;
                /// Register an interrupt handler.
                ///
                /// See LibDragon's interrupt.h for details.
                pub fn [<register_ $lower_name _handler>](cb: InterruptCallback) { 
                    unsafe { 
                        [<$upper_name _CALLBACK>] = Some(cb); 
                        libdragon_sys::[<register_ $upper_name _handler>](Some([<_ $lower_name _handler>])); 
                    } 
                }
                /// Unregister an interrupt handler.
                ///
                /// See LibDragon's interrupt.h for details.
                pub fn [<unregister_ $lower_name _handler>]() { 
                    unsafe { 
                        [<$upper_name _CALLBACK>] = None; 
                        libdragon_sys::[<unregister_ $upper_name _handler>](Some([<_ $lower_name _handler>])); 
                    } 
                }
                extern "C" fn [<_ $lower_name _handler>]() { 
                    let cb = unsafe { [<$upper_name _CALLBACK>].as_mut().unwrap() }; 
                    cb(); 
                }
            }
        };
    }

    int_handler!(ai, AI);
    int_handler!(vi, VI);
    int_handler!(pi, PI);
    int_handler!(dp, DP);
    int_handler!(si, SI);
    int_handler!(sp, SP);
    int_handler!(ti, TI);
    int_handler!(cart, CART);
    int_handler!(reset, RESET);

    /// Enable or disable the AI interrupt
    pub fn set_AI_interrupt(active: bool) { unsafe { libdragon_sys::set_AI_interrupt(active as i32); } }
    /// Enable or disable the VI interrupt
    pub fn set_VI_interrupt(active: bool, line: u32) { unsafe { libdragon_sys::set_VI_interrupt(active as i32, line); } }
    /// Enable or disable the PI interrupt
    pub fn set_PI_interrupt(active: bool) { unsafe { libdragon_sys::set_PI_interrupt(active as i32); } }
    /// Enable or disable the DP interrupt
    pub fn set_DP_interrupt(active: bool) { unsafe { libdragon_sys::set_DP_interrupt(active as i32); } }
    /// Enable or disable the SI interrupt
    pub fn set_SI_interrupt(active: bool) { unsafe { libdragon_sys::set_SI_interrupt(active as i32); } }
    /// Enable or disable the SP interrupt
    pub fn set_SP_interrupt(active: bool) { unsafe { libdragon_sys::set_SP_interrupt(active as i32); } }
    /// Enable or disable the TI interrupt
    pub fn set_TI_interrupt(active: bool) { unsafe { libdragon_sys::set_TI_interrupt(active as i32); } }
    /// Enable or disable the CART interrupt
    pub fn set_CART_interrupt(active: bool) { unsafe { libdragon_sys::set_CART_interrupt(active as i32); } }
    /// Enable or disable the RESET interrupt
    pub fn set_RESET_interrupt(active: bool) { unsafe { libdragon_sys::set_RESET_interrupt(active as i32); } }

    /// Disable interrupts systemwide
    ///
    /// See [`disable_interrupts`](libdragon_sys::disable_interrupts) for details.
    #[inline(always)] pub fn disable() { unsafe { libdragon_sys::disable_interrupts(); } }
    
    /// Enable interrupts systemwide
    ///
    /// See [`enable_interrupts`](libdragon_sys::enable_interrupts) for details.
    #[inline(always)] pub fn enable() { unsafe { libdragon_sys::enable_interrupts(); } }

    /// Check whether the RESET button was pressed and how long we are into the reset process.
    ///
    /// See [`exception_reset_time`](libdragon_sys::exception_reset_time) for details.
    #[inline] pub fn exception_reset_time() -> u32 { unsafe { libdragon_sys::exception_reset_time() } }

    /// State of interrupts on the system
    ///
    /// See [`interrupt_state_t`](libdragon_sys::interrupt_state_t) for details.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum InterruptsState { Uninitialized, Disabled, Enabled }

    impl From<libdragon_sys::interrupt_state_t> for InterruptsState {
        fn from(val: libdragon_sys::interrupt_state_t) -> InterruptsState {
            match val {
                libdragon_sys::interrupt_state_t_INTERRUPTS_UNINITIALIZED => InterruptsState::Uninitialized,
                libdragon_sys::interrupt_state_t_INTERRUPTS_DISABLED => InterruptsState::Disabled,
                libdragon_sys::interrupt_state_t_INTERRUPTS_ENABLED => InterruptsState::Enabled,
                _ => panic!("invalid value"),
            }
        }
    }

    /// Return the current state of interrupts
    ///
    /// See [`get_interrupts_state`](libdragon_sys::get_interrupts_state) for details.
    #[inline] pub fn get_state() -> InterruptsState { unsafe { libdragon_sys::get_interrupts_state().into() } }
}

/// Guaranteed length of the reset time.
///
/// See [`RESET_TIME_LENGTH`](libdragon_sys::RESET_TIME_LENGTH) for details.
#[inline(always)] pub fn reset_time_length() -> u32 { ticks::from_ms(200) }

