#![no_std]
#![no_main]

use libdragon::*;

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init_features(debug::FEATURE_LOG_ISVIEWER);

    console::init();
    console::set_render_mode(console::RenderMode::Manual);

    joypad::init();
    
    eprintln!("Hello from rust! Here's a number: {}", 5u32);

    let fmt_inputs = |inputs: &joypad::Inputs| {
        let mut s = format!("Stick: {:+04},{:+04} C-Stick: {:+04},{:+04} L-Trig:{:03} R-Trig:{:03}\n", 
                            inputs.stick_x, inputs.stick_y, inputs.cstick_x, inputs.cstick_y, 
                            inputs.analog_l, inputs.analog_r); 
        s.push_str(&format!("D-U:{} D-D:{} D-L:{} D-R:{} C-U:{} C-D:{} C-L:{} C-R:{}\n",
                            inputs.btn.d_up as u8, inputs.btn.d_down as u8, inputs.btn.d_left as u8,
                            inputs.btn.d_right as u8, inputs.btn.c_up as u8, inputs.btn.c_down as u8,
                            inputs.btn.c_left as u8, inputs.btn.c_right as u8));
        s.push_str(&format!("A:{} B:{} X:{} Y:{} L:{} R:{} Z:{} Start:{}\n",
                            inputs.btn.a as u8, inputs.btn.b as u8, inputs.btn.x as u8, 
                            inputs.btn.y as u8, inputs.btn.l as u8, inputs.btn.r as u8, 
                            inputs.btn.z as u8, inputs.btn.start as u8));
        s
    };

    loop {
        console::clear();

        joypad::poll();

        joypad::foreach(|port| {
            let style = port.get_style();
            let accessory_type = port.get_accessory_type();
            let rumble = match port.get_rumble_supported() {
                true => match port.get_rumble_active() {
                    true => "Active",
                    false => "Idle",
                },
                false => "Unsupported",
            };
            let inputs = port.get_inputs();

            println!("Port {} Style: {:?} Pak: {:?} Rumble: {} Inputs: {}", port.port, style, accessory_type, rumble, fmt_inputs(&inputs));
            println!("Dir:{:?}", port.get_direction(joypad::TwoD::RightHand));
        });

        console::render();
    }
}

