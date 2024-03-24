#![no_std]
#![no_main]

use libdragon::*;

use dfs::Read;
use display::{Resolution, BitDepth, Gamma, FilterOptions};
use graphics::{make_color, Graphics};
use sprite::Sprite;

fn read_sprite(filename: &str) -> Sprite {
    let mut fp = dfs::File::open(dfs::PathBuf::from(filename), "r").unwrap();
    let size = fp.size().unwrap();
    let mut buf = Vec::new();
    buf.resize(size, 0u8);
    if fp.read(buf.as_mut_slice()).unwrap() != size {
        panic!("could not read sprite {}", filename);
    }
    return Sprite::from_data_raw(buf);
}

#[no_mangle]
extern "C" fn main() -> ! {
    let mut res = Resolution::_320x240;
    let mut bit = BitDepth::Bpp32;

    // enable ISViewer, so eprintln calls are displayed there
    debug::init(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    display::init(res, bit, 2, Gamma::None, FilterOptions::Resample);
    dfs::init(None).unwrap_or_else(|e| panic!("Could not initialize filesystem: {:?}", e));

    let mario = read_sprite("rom://mario.sprite");
    let mariotrans = read_sprite("rom://mariotrans.sprite");
    let mario16 = read_sprite("rom://mario16.sprite");
    let mariotrans16 = read_sprite("rom://mariotrans16.sprite");

    /* Alpha test sprites */
    let red = read_sprite("rom://red.sprite");
    let green = read_sprite("rom://green.sprite");
    let blue = read_sprite("rom://blue.sprite");

    /* Trans test sprites */
    let red16 = read_sprite("rom://red16.sprite");
    let green16 = read_sprite("rom://green16.sprite");
    let blue16 = read_sprite("rom://blue16.sprite");

    joypad::init();
    
    loop {
        let mut g = Graphics::new(display::get());

        g.fill_screen(0);
        g.draw_text(20, 20, &format!("Video mode: {}", unsafe { *(0x8000_0300 as *const u32) }));
        g.draw_text(20, 30, &format!("Status: {:08X}", unsafe { *(0xA440_0000 as *const u32) }));

        /* Full bright color */
        g.draw_box(20, 40, 20, 20, make_color(255, 0, 0, 255));
        g.draw_box(50, 40, 20, 20, make_color(0, 255, 0, 255));
        g.draw_box(80, 40, 20, 20, make_color(0, 0, 255, 255));
        g.draw_box(110, 40, 20, 20, make_color(255, 255, 255, 255));

        /* 2/3 bright color */
        g.draw_box(20, 60, 20, 20, make_color(171, 0, 0, 255));
        g.draw_box(50, 60, 20, 20, make_color(0, 171, 0, 255));
        g.draw_box(80, 60, 20, 20, make_color(0, 0, 171, 255));
        g.draw_box(110, 60, 20, 20, make_color(171, 171, 171, 255));

        /* 1/3 bright color */
        g.draw_box(20, 80, 20, 20, make_color(85, 0, 0, 255));
        g.draw_box(50, 80, 20, 20, make_color(0, 85, 0, 255));
        g.draw_box(80, 80, 20, 20, make_color(0, 0, 85, 255));
        g.draw_box(110, 80, 20, 20, make_color(85, 85, 85, 255));

        /* Shade box */
        for i in 0..=255 as u8 {
            g.draw_box(20 + i as i32, 120, 1, 20, make_color(i, i, i, 255));
        }

        /* Display sprite (16bpp ones will only display in 16bpp mode, same with 32bpp) */
        g.draw_sprite(20, 150, &mario);
        g.draw_sprite_trans(150, 150, &mariotrans);
        
        g.draw_sprite(20, 150, &mario16);
        g.draw_sprite_trans(150, 150, &mariotrans16);

        /* 32BPP alpha blending test */
        g.draw_sprite_trans(150, 20, &red);
        g.draw_sprite_trans(170, 20, &green);
        g.draw_sprite_trans(160, 30, &blue);

        /* 16BPP trans test */
        g.draw_sprite_trans(150, 20, &red16);
        g.draw_sprite_trans(170, 20, &green16);
        g.draw_sprite_trans(160, 30, &blue16);

        g.surface().unwrap().show();

        joypad::poll();

        let keys = joypad::Port::get_port_1().get_buttons_pressed();
        if keys.d_up {
            display::close();
            res = Resolution::_640x480;
            display::init(res, bit, 2, Gamma::None, FilterOptions::Resample);
        }
        if keys.d_down {
            display::close();
            res = Resolution::_320x240;
            display::init(res, bit, 2, Gamma::None, FilterOptions::Resample);
        }
        if keys.d_left {
            display::close();
            bit = BitDepth::Bpp16;
            display::init(res, bit, 2, Gamma::None, FilterOptions::Resample);
        }
        if keys.d_right {
            display::close();
            bit = BitDepth::Bpp32;
            display::init(res, bit, 2, Gamma::None, FilterOptions::Resample);
        }
    }
}

