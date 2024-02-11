use std::env;

fn main() -> () {
    // Pass on DEP_LIBDRAGON_SYS_* env vars
    env::vars().for_each(|var| {
        if var.0.starts_with("DEP_LIBDRAGON_SYS_") {
            println!("cargo:sys_{}={}", &var.0[18..], var.1);
        }
    });
}
