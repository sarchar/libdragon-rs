#![no_std]
#![no_main]

#[allow(unused_imports)]
use libdragon::*;

use libdragon::display::{Resolution, BitDepth, Gamma, FilterOptions};

const FONT_PACIFICO: u8 = 1;
const FONT_ZEROVELOCITY: u8 = 2;

const TEXT: &str = "Two $02households$01, both alike in dignity,\n\
                    In $02fair Verona$01, where we lay our scene,\n\
                    From ancient grudge break to new $02mutiny$01,\n\
                    Where $02civil blood$01 makes civil hands unclean.\n\
                    From forth the fatal loins of these $02two foes$01\n\
                    A pair of $02star-cross'd lovers$01 take their life;\n";

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);
    joypad::init();

    // Initialize peripherals
    dfs::init(None).unwrap_or_else(|e| panic!("Could not initialize filesystem: {:?}", e));
    display::init(Resolution::_320x240, BitDepth::Bpp16, 3, Gamma::None, FilterOptions::Resample);
    rdpq::init();
    
    let mut fnt1 = rdpq::Font::load("rom:/Pacifico.font64");
    let mut fnt5 = rdpq::Font::load("rom:/FerriteCoreDX.font64");
    fnt1.style(0, rdpq::FontStyle {
        color: graphics::rgba32(0xED, 0xAE, 0x49, 0xFF),
    });
    fnt5.style(0, rdpq::FontStyle {
        color: graphics::rgba32(0x82, 0x30, 0x38, 0xFF),
    });
    rdpq::text_register_font(FONT_PACIFICO, &fnt1);
    rdpq::text_register_font(FONT_ZEROVELOCITY, &fnt5);

    let mut box_width = 262i32;
    let mut box_height = 150i32;
    let mut drawn_chars = 0;

    let mut frame = 0;
    loop {
        frame += 1;
        if (frame % 4) == 0 {
            drawn_chars = core::cmp::min(drawn_chars + 1, TEXT.len());
        }

        joypad::poll();
        let keys = joypad::Port::get_port_1().get_buttons_held();
        if keys.c_up { box_height += 2; }
        if keys.c_down { box_height -= 2; }
        if keys.c_left { box_width += 2; }
        if keys.c_right { box_width -= 2; }
        box_height = core::cmp::max(1, box_height);
        box_width = core::cmp::max(1, box_width);

        let disp = display::get();
        rdpq::attach_clear(&disp, None);
        rdpq::set_mode_fill(graphics::rgba32(0x30, 0x63, 0x38, 0xFF));
        rdpq::fill_rectangle((320-box_width)/2, (240-box_height)/2, (320+box_width)/2, (240+box_height)/2);
        let _nbytes = rdpq::text_print(rdpq::TextParms {
            // .line_spacing = -3,
            align : rdpq::Align::Left,
            valign: rdpq::VAlign::Center,
            width : box_width as i16,
            height: box_height as i16,
            wrap  : rdpq::TextWrap::Word,
            ..Default::default()
        }, FONT_PACIFICO, ((320-box_width)/2) as f32, ((240-box_height)/2) as f32, TEXT);

        disable_interrupts();
        let t0 = timer::ticks();
        //...
        let nbytes = 0;
        let t1 = timer::ticks();
        enable_interrupts();

        eprintln!("rdpq::text_print: {} us ({}x{}) - {} bytes\n", timer::micros(t1-t0), box_width, box_height, nbytes);

        rdpq::detach_show();
    }
}

