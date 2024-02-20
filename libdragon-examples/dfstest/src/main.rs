#![no_main]
#![feature(restricted_std)]

use std::io::{Read, Seek};

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

    eprintln!("hello.txt at ${:08X}", dfs::rom_addr("hello.txt"));
    let fp = dfs::File::open("rom://hello.txt", "r");
    eprintln!("fp {:?}", fp);
    let mut fp = fp.unwrap();
    eprintln!("file size = {}", fp.size().unwrap());

    let mut buf = vec![0u8; 256];
    let _ = fp.read(buf.as_mut_slice()).unwrap();
    let _ = fp.rewind();
    let sz = fp.read(buf.as_mut_slice()).unwrap();
    let content = String::from_utf8_lossy(&buf[..sz]).into_owned();
    eprintln!("fp is at {}, eof = {}", fp.tell().unwrap(), fp.eof().unwrap());

    loop {
        console::clear();
        println!("Read {} bytes: {}", sz, content);
        console::render();
    }
}

