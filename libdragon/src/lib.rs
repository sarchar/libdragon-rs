#![feature(restricted_std)]

mod allocator;

pub mod console;
pub mod debug;

extern "C" {
    // keep rust optimizer from removing the entry point
    fn _start() -> !;

    fn __getreent() -> *mut libdragon_sys::_reent;
}

fn get_stderr() -> *mut libdragon_sys::__FILE {
    unsafe { (*__getreent())._stderr }
}

pub fn libdragon_fprintf(msg: &str) -> i32 {
    unsafe {
        libdragon_sys::fprintf(get_stderr(), "%s\0".as_ptr() as *const i8, msg.as_ptr()) as i32
    }
}

pub fn libdragon_printf(msg: &str) -> i32 {
    unsafe {
        libdragon_sys::printf("%s\0".as_ptr() as *const i8, msg.as_ptr()) as i32
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


