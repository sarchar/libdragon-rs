#![no_main]
#![feature(restricted_std)]

use libdragon::*;
use libdragon::{println, eprintln};

use libdragon::{controller, debug};
use libdragon::console::{self, RenderMode};

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init_features(debug::FEATURE_LOG_ISVIEWER);

    console::init();
    console::set_render_mode(RenderMode::Manual);

    controller::init();
    
    eprintln!("Hello from rust! Here's a number: {}", 5u32);

    loop {
        console::clear();
        println!("Hello, rust!");

        controller::scan();

        let present = controller::get_present();
        let keys_down = controller::get_keys_pressed();

        for i in 0..3 {
            if present[i] { 
                let _ = controller::get_accessories_present();
                let acc = controller::identify_accessory(i);

                println!("keys_down[{i}] = {:?}, acc = {:?}", keys_down[i], acc); 
                eprintln!("keys_down[{i}] = {:?}, acc = {:?}", keys_down[i], acc); 

                if keys_down[i].A {
                    controller::rumble_start(i);
                }

                if keys_down[i].B {
                    controller::rumble_stop(i);
                }
            }
        }

        console::render();
    }
}

