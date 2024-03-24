#![no_std]
#![no_main]

#[allow(unused_imports)]
use libdragon::*;

use graphics::{make_color, Graphics};
use display::{Resolution, BitDepth, Gamma, FilterOptions};

const WIDTH: [i32; 6] = [320, 640, 256, 512, 512, 640];
const HEIGHT: [i32; 6] = [240, 480, 240, 480, 240, 240];

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    display::init(Resolution::_320x240, BitDepth::Bpp32, 2, Gamma::None, FilterOptions::Resample);

    joypad::init();
    
    let print_text = |g: &mut Graphics, msg: &str, x, y| {
        g.draw_text(x * 8, y * 8, msg);
    };

    let mut res = 0;
    loop {
        let mut g = Graphics::new(display::get());

        let color = make_color(0xCC, 0xCC, 0xCC, 0xFF);
        g.fill_screen(color);

        let color = make_color(0xFF, 0xFF, 0xFF, 0xFF);
        g.draw_line(0           , 0            , WIDTH[res]-1, 0            , color);
        g.draw_line(WIDTH[res]-1, 0            , WIDTH[res]-1, HEIGHT[res]-1, color);
        g.draw_line(WIDTH[res]-1, HEIGHT[res]-1, 0           , HEIGHT[res]-1, color);
        g.draw_line(0           , HEIGHT[res]-1, 0           , 0            , color);

        g.draw_line(0, 0            , WIDTH[res]-1, HEIGHT[res]-1, color);
        g.draw_line(0, HEIGHT[res]-1, WIDTH[res]-1, 0            , color);

        graphics::set_color(make_color(0, 0, 0, 0xFF), 0);
        
        print_text(&mut g, "Video Resolution Test", WIDTH[res]/16 - 10, 3);
        match res {
            0 => print_text(&mut g, "320x240p", WIDTH[res]/16 - 3, 5),
            1 => print_text(&mut g, "640x480i", WIDTH[res]/16 - 3, 5),
            2 => print_text(&mut g, "256x240p", WIDTH[res]/16 - 3, 5),
            3 => print_text(&mut g, "512x480i", WIDTH[res]/16 - 3, 5),
            4 => print_text(&mut g, "512x240p", WIDTH[res]/16 - 3, 5),
            5 => print_text(&mut g, "640x240p", WIDTH[res]/16 - 3, 5),
            _ => panic!("invalid"),
        }
        let fps = display::get_fps();
        print_text(&mut g, &format!("{}", fps), WIDTH[res]/16 - 3, 10);

        for j in 0..8 {
            let msg = format!("Line {}", j);
            print_text(&mut g, &msg, 3, j);
            let msg = format!("Line {}", HEIGHT[res]/8 - j - 1);
            print_text(&mut g, &msg, 3, HEIGHT[res]/8 - j - 1);
        }

        print_text(&mut g, "0123456789", 0, 16);
        print_text(&mut g, "9876543210", WIDTH[res]/8 - 10, 16);

        g.surface().unwrap().show();

        joypad::poll();

        let buttons = joypad::Port::get_port_1().get_buttons_pressed();
        if buttons.a {
            let next_mode = match res {
                0 => Resolution::_640x480,
                1 => Resolution::_256x240,
                2 => Resolution::_512x480,
                3 => Resolution::_512x240,
                4 => Resolution::_640x240,
                5 => Resolution::_320x240,
                _ => panic!("invalid"),
            };
            res = (res + 1) % 6;

            display::close();
            display::init(next_mode, BitDepth::Bpp32, 2, Gamma::None, FilterOptions::Resample);
        }
    }
}

