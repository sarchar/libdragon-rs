#![no_std]
#![no_main]

use core_maths::*;
use libdragon::*;

use core::f32::consts::PI;

const PLAYBACK_RATE: i32 = 44100;

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init_features(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    console::init();
    console::set_debug(true);

    joypad::init();
    
    audio::init(PLAYBACK_RATE, 4);

    let mut tsamps: u64 = 0;
    let mut freq: f32 = 440.0;
    let mut vol: f32 = 0.5;
    loop {
        if audio::can_write() {
            audio::write(|buf| {
                let to_write = buf.len() / 2;
                for i in 0..to_write {
                    let v = (vol*(i16::MAX as f32) * (2.0 * PI * freq * ((tsamps as f32) / (PLAYBACK_RATE as f32))).sin()) as i16;
                    buf[2*i+0] = v;
                    buf[2*i+1] = v;
                    tsamps += 1;
                }
            });
        }

        joypad::poll();
        let port = joypad::Port::get_port_1();
        let pressed = port.get_buttons_pressed();
        if pressed.c_up {
            freq *= 2.0;
            println!("freq = {}", freq);
        }
        if pressed.c_down {
            freq *= 0.5;
            println!("freq = {}", freq);
        }
        if pressed.c_right {
            freq *= 1.06;
            println!("freq = {}", freq);
        }
        if pressed.c_left {
            freq /= 1.06;
            println!("freq = {}", freq);
        }
        if pressed.d_up {
            vol = f32::min(vol + 0.1, 1.0);
            println!("vol = {}", vol);
        }
        if pressed.d_down {
            vol = f32::max(vol - 0.1, 0.0);
            println!("vol = {}", vol);
        }
    }
}

