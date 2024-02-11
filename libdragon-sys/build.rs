use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command};

use execute::Execute;
use filetime::FileTime;

fn main() -> () {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libdragon_dir = src_dir.clone().join("libdragon");

    // tell cargo that if build-toolchain.sh script changes we should re-run this script
    println!("cargo:rerun-if-changed=libdragon/tools/build-toolchain.sh");
    println!("cargo:rerun-if-changed=libdragon");

    // configure N64_INST for building libdragon and the toolchain
    let toolchain_dir = src_dir.clone().join("toolchain");
    env::set_var("N64_INST", toolchain_dir.display().to_string());
    println!("cargo:rustc-env=N64_INST={}", toolchain_dir.display());
    eprintln!("N64_INST={}", toolchain_dir.display());

    // create the build directory under out/
    let build_toolchain_dir = out_dir.clone().join("build-toolchain");
    fs::create_dir_all(&build_toolchain_dir).expect("Error creating build directory");

    // if {out}/mips64-libdragon-elf/bin/... doesn't exist OR if
    // <xyz> is newer than <xyz>, build toolchain
    // build the toolchain. execute from out for the build
    let build_toolchain_script = libdragon_dir.clone().join("tools").join("build-toolchain.sh");
    let gcc = toolchain_dir.clone().join("bin").join("mips64-libdragon-elf-gcc");
    let need_toolchain_build = !fs::metadata(gcc).is_ok_and(|metadata| {
        let gcc_time = FileTime::from_last_modification_time(&metadata);
        let build_script_time = FileTime::from_last_modification_time(&fs::metadata(build_toolchain_script.clone()).unwrap());
        build_script_time <= gcc_time
    });
    if need_toolchain_build {
        let mut build_toolchain = Command::new("bash");
        build_toolchain.arg(build_toolchain_script.into_os_string());
        build_toolchain.current_dir(build_toolchain_dir.into_os_string());
        let output = build_toolchain.execute_output().unwrap();
        if let Some(exit_code) = output.status.code() {
            if exit_code != 0 {
                eprintln!("There was an error building the mips64-libdragon-elf toolchain");
                exit(1);
            }
        } else {
            eprintln!("Build incomplete");
            exit(1);
        }
    }

    // build libdragon
    let mut make = Command::new("make");
    make.arg("-C").arg(libdragon_dir.clone().into_os_string());
    if make.execute_check_exit_status_code(0).is_err() {
        eprintln!("There was an error building libdragon");
        exit(1);
    }

    // install libdragon and tools
    let mut install = Command::new("make");
    install.arg("-C").arg(libdragon_dir.clone().into_os_string()).arg("install").arg("tools-install");
    if install.execute_check_exit_status_code(0).is_err() {
        eprintln!("There was an error installing libdragon");
        exit(1);
    }

    // link against libdragon.a and libdragonsys.a
    println!("cargo:rustc-link-search=native={}/mips64-libdragon-elf/lib", toolchain_dir.display());
    println!("cargo:rustc-link-lib=static=dragon");
    println!("cargo:rustc-link-lib=static=dragonsys");

    println!("cargo:rustc-link-search=native={}/lib/gcc/mips64-libdragon-elf/13.2.0", toolchain_dir.display());
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=g");
    println!("cargo:rustc-link-lib=static=nosys");
    println!("cargo:rustc-link-lib=static=gcc");

    let bindings = bindgen::Builder::default()
                        .clang_arg(format!("-I{}/mips64-libdragon-elf/include", toolchain_dir.display()))
                        .header("wrapper.h")
                        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                        .generate()
                        .expect("Unable to generate a binding");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");

    // set vars for parent crates
    println!("cargo:linker_script={}/linker.ld", src_dir.display());
    println!("cargo:objcopy={}/bin/mips64-libdragon-elf-objcopy", toolchain_dir.display());
    println!("cargo:n64tool={}/bin/n64tool", toolchain_dir.display());
    println!("cargo:header={}/mips64-libdragon-elf/lib/header", toolchain_dir.display());
    println!("cargo:chksum64={}/bin/chksum64", toolchain_dir.display());
}
