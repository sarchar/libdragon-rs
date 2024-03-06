#![no_std]
#![no_main]

use libdragon::*;
use libdragon::dfs::{Dir, DfsPathBuf};
use libdragon::display::{Resolution, BitDepth, Gamma, FilterOptions};
use libdragon::graphics::Graphics;

use audio::mixer;
use audio::xm64::Xm64;

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init_features(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);
    joypad::init();

    display::init(Resolution::_320x240, BitDepth::Bpp16, 2, Gamma::None, FilterOptions::Resample);

    dfs::init(None).unwrap_or_else(|e| panic!("Could not initialize filesystem: {:?}", e));
    
    audio::init(44100, 4);
    mixer::init(16);  // Initialize up to 16 channels

    let mut music_files = vec![];
    let mut dir = Dir::findfirst("rom:/").unwrap();
    loop {
        if dir.d_type().unwrap() == dfs::EntryType::File {
            let mf = dir.d_name();
            let mfs = mf.as_path().to_string_lossy();
            eprintln!("found: {}", mfs);
            music_files.push(format!("rom:/{}", mfs));
        }

        match dir.findnext() {
            Err(_) => break,
            Ok(_) => {},
        }
    }

    let mut play_index = 0;
    let mut xm = None;

    loop {
        if xm.is_none() {
            let mut newxm = Xm64::open(DfsPathBuf::from(&music_files[play_index])).unwrap();
            newxm.set_loop(true);
            newxm.play(0);
            xm = Some(newxm);
            eprintln!("Started playing {}", music_files[play_index]);
        }

        // audio::write blocks if the output buffer is full, so we check first
        if audio::can_write() {
            audio::write(|buf| {
                mixer::poll(buf);
            });
        }

        let mut g = Graphics::new(display::get());
        g.fill_screen(0);
        g.draw_text(10, 10, &format!("{} songs found", music_files.len()));
        g.draw_text(10, 20, &format!("Playing #{}: {}", play_index, music_files[play_index]));
        g.draw_text(10, 40, "L/R to change song");

        let mut pat = 0;
        let mut row = 0;
        let mut secs = 0.0;
        xm.as_ref().unwrap().tell(&mut pat, &mut row, &mut secs);
        g.draw_text(30, 70, &format!("Pattern: {:2} Row: {:3} Secs: {}", pat, row, secs));
        g.finish().show();

        joypad::poll();
        let pressed = joypad::Port::get_port_1().get_buttons_pressed();

        if pressed.r {
            play_index = core::cmp::min(play_index + 1, music_files.len() - 1);
            //xm.stop();
            xm = None;
        }

        if pressed.l {
            play_index = core::cmp::max(play_index - 1, 0);
            //xm.stop();
            xm = None;
        }

        if pressed.start {
            if xm.as_mut().unwrap().playing() { xm.as_mut().unwrap().stop(); } else { xm.as_mut().unwrap().play(0); };
        }
    }
}

