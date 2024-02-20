#![feature(restricted_std)]
#![feature(io_error_more)]
#![feature(asm_experimental_arch)]

use std::ffi::CString;
use core::panic::PanicInfo;

mod allocator;

pub mod console;
pub mod debug;
pub mod dfs;
pub mod joypad;

#[derive(Debug)]
pub enum LibDragonError {
    DfsError { error_code: i32 },
    IoError { error: std::io::Error },
}

pub type Result<T> = std::result::Result<T, LibDragonError>;

extern "C" {
    static _gp: ::std::os::raw::c_int;

    // keep rust optimizer from removing the entry point
    fn _start() -> !;

    fn __getreent() -> *mut libdragon_sys::_reent;
}

fn get_stderr() -> *mut libdragon_sys::__FILE {
    unsafe { (*__getreent())._stderr }
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
        let oldgp: *const ::std::os::raw::c_void;
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

pub fn setup_panic() {
    std::panic::set_hook(Box::new(|info: &PanicInfo| {
        let (file, line) = match info.location() {
            Some(location) => {
                (CString::new(location.file()).unwrap(), location.line())
            },
            _ => (CString::new("<unknown>").unwrap(), 0)
        };
    
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            CString::new(*s).unwrap()
        } else {
            CString::new("<unknown>").unwrap()
        };
    
        let failedexpr = CString::new("Rust panic!").unwrap();
        let fmt = CString::new("%s").unwrap();
    
        unsafe {
            libdragon_sys::debug_assert_func_f(file.as_ptr(), line as i32, std::ptr::null(), failedexpr.as_ptr(), fmt.as_ptr(), msg.as_ptr());
        }
    }));

    let _ = std::panic::catch_unwind(|| {
        eprintln!("unwind!");
    });
}
