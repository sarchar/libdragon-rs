#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use libdragon::*;

use libdragon::dfs::DfsPathBuf;
use libdragon::display::{Resolution, BitDepth, Gamma, FilterOptions};
use libdragon::graphics::Graphics;
use libdragon::sprite::Sprite;
use libdragon::timer::Timer;

#[no_mangle]
extern "C" fn main() -> ! {
    debug::init_features(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    display::init(Resolution::_320x240, BitDepth::Bpp16, 2, Gamma::None, FilterOptions::Resample);
    dfs::init(None).unwrap_or_else(|e| panic!("Could not initialize filesystem: {:?}", e));
    rdpq::init();
    joypad::init();
    timer::init();

    let mudkip     = Sprite::load(DfsPathBuf::from("rom:/mudkip.sprite")).unwrap();
    let earthbound = Sprite::load(DfsPathBuf::from("rom:/earthbound.sprite")).unwrap();
    let plane      = Sprite::load(DfsPathBuf::from("rom:/plane.sprite")).unwrap();

    let animcounter = Arc::new(AtomicU32::new(0));
    let animcounter_clone = animcounter.clone();
    let _timer = Timer::new(timer::make_ticks(1000000 / 30), timer::Mode::Continuous, Box::new(move |_| {
        animcounter_clone.store(animcounter_clone.load(Ordering::SeqCst)+1, Ordering::SeqCst);
    }));

    let mut mode = 0;
    loop {
        let a = animcounter.load(Ordering::SeqCst);
        let mut g = Graphics::new(display::get());

        g.fill_screen(0xFFFF_FFFF);
        g.set_color(0, 0xFFFF_FFFF);

        match mode {
            0 => {
                g.draw_text(20, 20, "Software spritemap test");
                g.draw_sprite_trans(20, 50, &plane);
                g.draw_sprite_trans(50, 50, &mudkip);
                g.draw_sprite_stride(20, 100, &earthbound, if ((a / 15) & 1) != 0 { 1 } else { 0 });
                g.draw_sprite_stride(50, 100, &earthbound, (((a / 8) & 7) * 2) as i32);
                g.surface().show();
            },
            1 => {
                g.draw_text(20, 20, "Hardware spritemap test");
                let disp = g.finish();

                rdpq::set_mode_copy(true);
                rdpq::attach(Some(&disp), None);
                
                rdp::load_texture(0, 0, rdp::Mirror::Disabled, &plane);
                rdp::draw_sprite(0, 20, 50, rdp::Mirror::Disabled);

                for i in 0..4 {
                    rdp::load_texture_stride(0, 0, rdp::Mirror::Disabled, &mudkip, i);
                    rdp::draw_sprite(0, 50 + (20 * (i % 2)), 50 + (20 * (i / 2)), rdp::Mirror::Disabled);
                }

                rdp::load_texture_stride(0, 0, rdp::Mirror::Disabled, &earthbound, if ((a / 15) & 1) != 0 { 1 } else { 0 });
                rdp::draw_sprite(0, 20, 100, rdp::Mirror::Disabled);

                rdp::load_texture_stride(0, 0, rdp::Mirror::Disabled, &earthbound, (((a / 8) & 0x7) * 2) as i32);
                rdp::draw_sprite(0, 50, 100, rdp::Mirror::Disabled);

                rdpq::detach_wait();
                disp.show();
            },
            _ => panic!(),
        }

        joypad::poll();

        let keys = joypad::Port::get_port_1().get_buttons_pressed();
        if keys.a {
            mode = 1 - mode;
        }
    }
}

