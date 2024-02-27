use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub type Result<T> = std::io::Result<T>;

#[derive(Debug, Clone, Default)]
pub struct Build {
    env_filename: Option<String>,
    just_filename: Option<String>,
    game_name: Option<String>,
    rsp_compile: bool,
}

impl Build {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_env_file(&mut self, filename: &str) -> &mut Self {
        self.env_filename = Some(filename.to_owned());
        self
    }

    pub fn set_just_file(&mut self, filename: &str) -> &mut Self {
        self.just_filename = Some(filename.to_owned());
        self
    }

    pub fn enable_rsp_compile(&mut self) -> &mut Self {
        self.rsp_compile = true;
        self
    }

    pub fn set_game_name(&mut self, name: &str) -> &mut Self {
        self.game_name = Some(name.to_owned());
        self
    }

    pub fn build(&mut self) -> Result<()> {
        let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        if let Some(ref env_filename) = &self.env_filename {
            // Store DEP_LIBDRAGON_SYS_* env vars in the .env file
            let mut vars = vec![];
            env::vars().for_each(|var| {
                if var.0.starts_with("DEP_LIBDRAGON_SYS_") {
                    vars.push(var);
                }
            });

            // Store path to the elf in the .env file
            let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
            vars.push(("ELF_FILE".to_owned(), 
                       format!("{}", out_dir.join("..").join("..").join("..").canonicalize()?.join(pkg_name).display())));

            // Export N64_INST
            vars.push(("N64_INST".to_owned(), env::var("DEP_LIBDRAGON_SYS_N64_INST").unwrap()));

            // Export game name
            let game_name = self.game_name.clone().unwrap_or("GAME".to_owned());
            vars.push(("GAME_NAME".to_owned(), game_name));

            let mut fp = File::create(src_dir.join(&env_filename))?;
            for var in vars {
                writeln!(&mut fp, "{}={}", var.0, var.1)?;
            }
        }

        // Copy the justfile over to the local directory
        if let Some(ref just_filename) = &self.just_filename {
            let mut fp = File::create(src_dir.join(&just_filename))?;
            write!(&mut fp, "{}", JUSTFILE_CONTENT)?;
        }

        // Pass the linker script to the linker
        println!("cargo:rustc-link-arg=-T{}", env::var("DEP_LIBDRAGON_SYS_LINKER_SCRIPT").unwrap());

        Ok(())
    }
}

const JUSTFILE_CONTENT: &str = include_str!("Justfile.inc");
