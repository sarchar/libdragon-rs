#![no_main]
#![feature(restricted_std)]

use libdragon::*;
use libdragon::{println, eprintln};

use libdragon::debug;
use libdragon::console::{self, RenderMode};

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init_features(debug::FEATURE_LOG_ISVIEWER);

    console::init();
    console::set_render_mode(RenderMode::Manual);
    
    eprintln!("Hello from rust! Here's a number: {}", 5u32);

    loop {
        console::clear();
        println!("Hello, rust!");
        console::render();
    }
}

