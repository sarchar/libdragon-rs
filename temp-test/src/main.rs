#![no_main]
#![feature(restricted_std)]

use libdragon::*;
use libdragon::{println, eprintln};

#[no_mangle]
extern "C" fn main() -> ! {
    unsafe {
        debug_init_isviewer();
        console_init();
        console_set_render_mode(RENDER_MANUAL as i32);
    }
    
    eprintln!("Hello from rust! Here's a number: {}      ", 5u32);

    loop {
        unsafe { console_clear(); }
        println!("Hello, rust!");
        unsafe { console_render(); }
    }
}

