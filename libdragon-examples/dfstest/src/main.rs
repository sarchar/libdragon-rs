#![no_main]
#![feature(restricted_std)]

use std::io::Read;
use std::path::PathBuf;

use libdragon::*;
#[allow(unused_imports)]
use libdragon::{print, println, eprintln};

use libdragon::{joypad, debug, dfs};
use libdragon::console::{self, RenderMode};

const MAX_LIST: usize = 20;

#[derive(Debug)]
struct Entry {
    filename: PathBuf,
    filetype: dfs::EntryType,
}

struct DemoDir {
    current_path: String,
    entries: Vec<Entry>,
}

impl DemoDir {
    fn new() -> Self {
        let mut ret = Self {
            current_path: String::from("rom://"),
            entries: Vec::new(),
        };

        ret.populate();
        ret
    }

    fn change_directory(&mut self, path: &str) {
        eprintln!("Changing directory to: {}", path);
        self.current_path = String::from(path);
        self.populate();
    }

    fn populate(&mut self) {
        self.entries.clear();

        let dir = dfs::Dir::findfirst(&self.current_path);
        if let Err(e) = dir {
            println!("Could not read directory {} (err {:?})", self.current_path, e);
            return;
        }

        let mut dir = dir.unwrap();
        loop {
            self.entries.push(Entry {
                filename: dir.d_name(),
                filetype: dir.d_type().unwrap(),
            });

            match dir.findnext() {
                Err(e) => {
                    println!("Error reading directory: {:?}", e);
                    break;
                }
                _ => {}, // continue scanning
            }
        }

        self.sort();
    }

    fn sort(&mut self) {
        self.entries.sort_by(|a, b| {
            if a.filetype == dfs::EntryType::Directory && b.filetype != dfs::EntryType::Directory { return std::cmp::Ordering::Less; }
            if a.filetype != dfs::EntryType::Directory && b.filetype == dfs::EntryType::Directory { return std::cmp::Ordering::Greater; }
            return a.filename.partial_cmp(&b.filename).unwrap()
        })
    }

    fn display(&self, cursor: usize, page: usize, max: usize) {
        let max = std::cmp::min(max, self.entries.len());
        if max == 0 {
            println!("No files in this dir...");
            return;
        }

        let cursor = std::cmp::min(cursor, (page + max) - 1);
        let cursor = std::cmp::max(cursor, page);

        for i in page..(page + max) {
            if i == cursor {
                print!("> ");
            } else {
                print!("  ");
            }

            let entry = &self.entries[i];
            match entry.filetype {
                dfs::EntryType::Directory => {
                    println!("[{}]", entry.filename.display());
                },
                dfs::EntryType::File => {
                    println!("{}", entry.filename.display());
                },
            }
        }
    }

    fn new_scroll_pos(&self, cursor: usize, page: usize, max: usize) -> (usize, usize) {
        let max = std::cmp::min(max, self.entries.len());

        let cursor = std::cmp::max(0, std::cmp::min(cursor, self.entries.len() - 1));

        // scrolled up
        let page = std::cmp::min(page, cursor);

        // scrolled down
        let page = if cursor >= (page + max) {
            (cursor - max) + 1
        } else {
            page
        };

        return (cursor, page)
    }
}

#[no_mangle]
extern "C" fn main() -> ! {
    libdragon::init_panic_hook();

    // enable ISViewer, so eprintln calls are displayed there
    // initialize access to the sd card if it exists
    debug::init_features(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_FILE_SD);

    console::init();
    console::set_render_mode(RenderMode::Manual);

    joypad::init();
    
    if let Err(e) = dfs::init(None) {
        println!("Filesystem failed to start!");
        println!("Error: {:?}", e);
        loop {}
    }

    // std::io::Read and std::io::Seek are implemented on dfs::File ->
    //
    // let content = {
    //     eprintln!("hello.txt at ${:08X}", dfs::rom_addr("hello.txt"));
    //     let fp = dfs::File::open("rom://hello.txt".into(), "r");
    //     eprintln!("fp {:?}", fp);
    //     let mut fp = fp.unwrap();
    //     eprintln!("file size = {}", fp.size().unwrap());

    //     let mut buf = vec![0u8; 256];
    //     let _ = fp.read(buf.as_mut_slice()).unwrap();
    //     let _ = fp.rewind();
    //     let sz = fp.read(buf.as_mut_slice()).unwrap();
    //     let content = String::from_utf8_lossy(&buf[..sz]).into_owned();
    //     eprintln!("fp is at {}, eof = {}", fp.tell().unwrap(), fp.eof().unwrap());
    //     content
    // };

    let joy1 = joypad::Port::get_port_1();

    let mut dir = DemoDir::new();
    let mut page = 0;
    let mut cursor = 0;
    loop {
        console::clear();
        dir.display(cursor, page, MAX_LIST);
        console::render();

        joypad::poll();
        let keys = joy1.get_buttons_pressed();
        if keys.d_up {
            (cursor, page) = dir.new_scroll_pos(cursor - 1, page, MAX_LIST);
        }

        if keys.d_down {
            (cursor, page) = dir.new_scroll_pos(cursor + 1, page, MAX_LIST);
        }

        if keys.c_right && dir.entries[cursor].filetype == dfs::EntryType::File {
            let path = std::path::PathBuf::from(dir.current_path.clone())
                                    .join(dir.entries[cursor].filename.clone());
            let res = dfs::File::open(path.clone().into_os_string(), "r");
            match res {
                Err(e) => {
                    println!("Failed to open {}: {:?}", path.display(), e);
                }
                Ok(mut file) => {
                    println!("Hold A to scroll");
                    let mut buf = vec![0u8; 1024];
                    loop {
                        match file.read(buf.as_mut_slice()) {
                            Ok(nread) => {
                                if nread == 0 { break; }
                                let s = String::from_utf8((&buf[0..nread]).to_vec()).unwrap();
                                let lines = s.split("\n");
                                for line in lines {
                                    println!("{}", line);
                                    console::render();

                                    wait_ms(100);
                                    joypad::poll();
                                    while !joy1.get_buttons().a {
                                        wait_ms(10);
                                        joypad::poll();
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error reading file: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }

            println!("Press B to quit");
            console::render();
            joypad::poll();
            while !joy1.get_buttons().b {
                wait_ms(10);
                joypad::poll();
            }

            continue;
        }

        if keys.l {
            dir.change_directory("sd://");
            page = 0;
            cursor = 0;
        }

        if keys.r {
            dir.change_directory("rom://");
            page = 0;
            cursor = 0;
        }

        if keys.a && dir.entries[cursor].filetype == dfs::EntryType::Directory {
            let path = std::path::PathBuf::from(dir.current_path.clone())
                                    .join(dir.entries[cursor].filename.clone());
            dir.change_directory(path.into_os_string().into_string().unwrap().as_str());
            page = 0;
            cursor = 0;
        }

        if keys.b {
            let mut path = std::path::PathBuf::from(dir.current_path.clone());
            if path.pop() {
                // PathBuf doesn't understand rom://, so we fix it up
                let mut s = path.into_os_string().into_string().unwrap();
                if s == "rom:" || s == "sd:" {
                    s.push_str("//");
                } 

                // if we popped too high, don't change directory
                if s != "" {
                    dir.change_directory(s.as_str());
                    page = 0;
                    cursor = 0;
                }
            }
        }
    }
}

