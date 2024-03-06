#![no_std]
#![no_main]

use libdragon::*;
use libdragon::dfs::DfsPathBuf;
use libdragon::display::{Resolution, BitDepth, Gamma, FilterOptions};
use libdragon::graphics::Graphics;

use audio::mixer;
use audio::wav64::Wav64;

const CHANNEL_SFX1 : i32 = 0;
const CHANNEL_SFX2 : i32 = 1;
const CHANNEL_MUSIC: i32 = 2;

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init_features(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);
    joypad::init();

    display::init(Resolution::_512x240, BitDepth::Bpp16, 3, Gamma::None, FilterOptions::Resample);

    dfs::init(None).unwrap_or_else(|e| panic!("Could not initialize filesystem: {:?}", e));
    
    audio::init(44100, 4);
    mixer::init(16);  // Initialize up to 16 channels

	// Bump maximum frequency of music channel to 128k.
	// The default is the same of the output frequency (44100), but we want to
	// let user increase it.
	mixer::ch_set_limits(CHANNEL_MUSIC, 0, 128000.0, 0);

	let mut sfx_cannon = Wav64::open(DfsPathBuf::from("rom:/cannon.wav64")).unwrap();
	
	let mut sfx_laser = Wav64::open(DfsPathBuf::from("rom:/laser.wav64")).unwrap();
    sfx_laser.set_loop(true);

	let mut sfx_monosample = Wav64::open(DfsPathBuf::from("rom:/monosample8.wav64")).unwrap();
	sfx_monosample.set_loop(true);

    let mut music = false;
    let mut music_frequency = sfx_monosample.wave().frequency();

    loop {
        let mut g = Graphics::new(display::get());
        g.fill_screen(0);
        g.draw_text(200-75, 10, "Audio mixer test");
        g.draw_text(200-70, 20, "v1.0 - by Rasky");
        g.draw_text(200-85, 30, "Rust port by Sarchar");
		g.draw_text(50, 60, "A - Play cannon");
		g.draw_text(50, 70, "B - Play laser (keep pressed)");
		g.draw_text(50, 80, "Z - Start / stop background music");
		g.draw_text(70, 90, "L/R - Change music frequency");
		g.draw_text(50, 140, "Music courtesy of MishtaLu / indiegamemusic.com");
        g.finish().show();

        joypad::poll();
        let ckeys = joypad::Port::get_port_1().get_buttons_pressed();

        if ckeys.a {
            sfx_cannon.play(CHANNEL_SFX1);
        }
        if ckeys.b {
            sfx_laser.play(CHANNEL_SFX2);
            mixer::ch_set_vol(CHANNEL_SFX2, 0.25, 0.25);
        }
        if ckeys.z {
            music = !music;
            if music {
                sfx_monosample.play(CHANNEL_MUSIC);
                music_frequency = sfx_monosample.wave().frequency();
            } else {
                mixer::ch_stop(CHANNEL_MUSIC);
            }
        }
        if music && music_frequency >= 8000.0 && ckeys.l {
            music_frequency /= 1.1;
            mixer::ch_set_freq(CHANNEL_MUSIC, music_frequency);
        }
        if music && music_frequency <= 128000.0 && ckeys.r {
            music_frequency *= 1.1;
            mixer::ch_set_freq(CHANNEL_MUSIC, music_frequency);
        }

        let ckeys = joypad::Port::get_port_1().get_buttons_released();
        if ckeys.b {
            mixer::ch_stop(CHANNEL_SFX2);
        }

        mixer::try_play();
    }
}

