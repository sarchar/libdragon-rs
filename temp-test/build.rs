use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Store DEP_LIBDRAGON_SYS_* env vars in an .env file
    let mut vars = vec![];
    env::vars().for_each(|var| {
        if var.0.starts_with("DEP_LIBDRAGON_SYS_") {
            vars.push(var);
        }
    });

    let mut fp = File::create(src_dir.join(".libdragon-sys-env"))?;
    for var in vars {
        writeln!(&mut fp, "{}={}", var.0, var.1)?;
    }

    // Store path to the elf in the .env file
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    writeln!(&mut fp, "ELF_FILE={}", out_dir.join("..").join("..").join("..").canonicalize()?.join(pkg_name).display())?;

    // Pass the linker script to the linker
    println!("cargo:rustc-link-arg=-T{}", env::var("DEP_LIBDRAGON_SYS_LINKER_SCRIPT").unwrap());

    Ok(())
}
