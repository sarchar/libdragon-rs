#![no_std]
#![no_main]

use libdragon::*;
use libdragon::dfs::{Dir, DfsPathBuf};
use libdragon::display::{Resolution, BitDepth, Gamma, FilterOptions};
use libdragon::graphics::Graphics;

use audio::mixer;
use audio::xm64::Xm64;
use audio::ym64::Ym64;

#[no_mangle]
extern "C" fn main() -> ! {
    // enable ISViewer, so eprintln calls are displayed there
    debug::init(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);
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
    let mut xm: Option<Xm64> = None;
    let mut ym: Option<Ym64> = None;
    let mut ym_playing = false;

    loop {
        if xm.is_none() && ym.is_none() {
            let slower = music_files[play_index].to_lowercase();
            if slower.ends_with(".xm64") {
                let mut newxm = Xm64::open(DfsPathBuf::from(&music_files[play_index])).unwrap();
                newxm.set_loop(true);
                newxm.play(0);
                xm = Some(newxm);
            } else if slower.ends_with(".ym64") {
                let mut newym = Ym64::open(DfsPathBuf::from(&music_files[play_index])).unwrap();
                newym.play(0);
                ym_playing = true;
                ym = Some(newym);
            } else {
                panic!("can't play {}", music_files[play_index]);
            }
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

        if let Some(xmr) = xm.as_ref() {
            let mut pat = 0;
            let mut row = 0;
            let mut secs = 0.0;
            xmr.tell(&mut pat, &mut row, &mut secs);
            g.draw_text(30, 70, &format!("Pattern: {:2} Row: {:3} Secs: {}", pat, row, secs));
        } else if let Some(ymr) = ym.as_ref() {
            let mut pos = 0;
            let mut secs = 0.0;
            ymr.tell(&mut pos, &mut secs);
            g.draw_text(40, 70, &format!("Position: {:2} Secs: {}", pos, secs));
        }
        g.finish().show();

        joypad::poll();
        let pressed = joypad::Port::get_port_1().get_buttons_pressed();

        if pressed.r {
            play_index = core::cmp::min(play_index + 1, music_files.len() - 1);
            xm = None;
            ym = None;
        }

        if pressed.l {
            play_index = core::cmp::max(play_index - 1, 0);
            xm = None;
            ym = None;
        }

        if pressed.start {
            if let Some(xmr) = xm.as_mut() {
                if xmr.playing() { xmr.stop(); } else { xmr.play(0); };
            } else if let Some(ymr) = ym.as_mut() {
                if ym_playing { ymr.stop(); } else { ymr.play(0); };
                ym_playing = !ym_playing;
            }
        }
    }
}

