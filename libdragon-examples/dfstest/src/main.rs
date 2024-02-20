#![no_main]
#![feature(restricted_std)]

use std::io::Read;

use libdragon::*;
#[allow(unused_imports)]
use libdragon::{println, eprintln};

use libdragon::{joypad, debug, dfs};
use libdragon::console::{self, RenderMode};

#[no_mangle]
extern "C" fn main() -> ! {
    libdragon::setup_panic();

    // enable ISViewer, so eprintln calls are displayed there
    debug::init_features(debug::FEATURE_LOG_ISVIEWER);

    console::init();
    console::set_render_mode(RenderMode::Manual);

    joypad::init();
    
    let r = dfs::init(None);//.unwrap_or_else(|e| panic!("Could not initialize the filesystem: {e:?}"));
    eprintln!("dfs::init(None) = {:?}", r);

    let fp = dfs::File::open("rom://hello.txt", "r");
    eprintln!("fp {:?}", fp);
    let mut fp = fp.unwrap();

    let mut buf = vec![0u8; 256];
    let sz = fp.read(buf.as_mut_slice()).unwrap();
    let content = String::from_utf8_lossy(&buf[..sz]).into_owned();

    loop {
        console::clear();
        println!("Read {} bytes: {}", sz, content);
        console::render();
    }
}

