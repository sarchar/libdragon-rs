use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use execute::Execute;

use glob::glob;

pub type Result<T> = std::io::Result<T>;

#[derive(Debug, Clone, Default)]
pub struct Build {
    env_filename: Option<String>,
    just_filename: Option<String>,
    game_name: Option<String>,
    rom_compression_level: u32,
    rsp_compile: bool,
}

impl Build {
    pub fn new() -> Self {
        let mut default = Self::default();
        default.rom_compression_level = 1;
        default
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

    pub fn set_rom_compression_level(&mut self, rom_compression_level: u32) -> &mut Self {
        assert!(rom_compression_level <= 3);
        self.rom_compression_level = rom_compression_level;
        self
    }

    pub fn build(&mut self) -> Result<()> {
        let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        if self.rsp_compile {
            self.compile_rsp_sources(&src_dir, &out_dir);
        }

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

            // Export rom compression level
            vars.push(("ROM_COMPRESSION_LEVEL".to_owned(), format!("{}", self.rom_compression_level)));

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

    pub fn get_toolchain_program(program: &str) -> String {
        format!("{}{}", env::var("DEP_LIBDRAGON_SYS_TOOLCHAIN_BIN").unwrap(), program)
    }

    pub fn compile_rsp_sources(&mut self, src_dir: &PathBuf, out_dir: &PathBuf) {
        for entry in glob(format!("{}/src/**/rsp*.S", src_dir.display()).as_str()).unwrap() {
            match entry {
                Err(e) => panic!("error: {:?}", e),
                Ok(rsp_file) => {
                    self.compile_rsp_source(src_dir, out_dir, rsp_file);
                },
            }
        }
    }

    pub fn compile_rsp_source(&mut self, src_dir: &PathBuf, out_dir: &PathBuf, rsp_file: PathBuf) {
        // tell cargo to rebuild if file changed
        println!("cargo:rerun-if-changed={}", rsp_file.display());

        // strip the source path from the filename
        let no_prefix = rsp_file.strip_prefix(src_dir.clone()).unwrap();

        // append the no_prefix path to the out directory
        let mut outfile = out_dir.clone();
        outfile.push(no_prefix);
        outfile.set_extension("o");

        let mut mapfile = out_dir.clone();
        mapfile.push(no_prefix);
        mapfile.set_extension("map");

        // make sure output directory exists
        let _ = std::fs::create_dir(outfile.clone().parent().unwrap());


        let gcc = Self::get_toolchain_program("gcc");
        let mut compile = Command::new(&gcc);
        compile.arg("-march=mips1")
               .arg("-mabi=32").arg("-Wa,-32")
               .arg("-Wa,--fatal-warnings")
               .arg("-Wl,-melf32bmip")
               .arg(format!("-I{}", env::var("DEP_LIBDRAGON_SYS_N64_INCLUDEDIR").unwrap()))
               .arg(format!("-L{}", env::var("DEP_LIBDRAGON_SYS_N64_LIBDIR").unwrap()))
               .arg("-nostartfiles")
               .arg(format!("-Wl,-T{}", env::var("DEP_LIBDRAGON_SYS_RSP_LINKER_SCRIPT").unwrap()))
               .arg("-Wl,--gc-sections")
               .arg(format!("-Wl,-Map={}", mapfile.display()))
               .arg("-o").arg(format!("{}", outfile.display()))
               .arg(format!("{}", rsp_file.display()))
               ;
        eprintln!("compiling {:?}: {:?}", no_prefix, compile);
        let output = compile.execute_output().unwrap();
        match output.status.code() {
            Some(exit_code) => {
                if exit_code != 0 {
                    eprintln!("Error assembling {:?}:", rsp_file);
                    std::io::stdout().write_all(&output.stdout).unwrap();
                    std::io::stderr().write_all(&output.stderr).unwrap();
                }
            },
            None => panic!("build incomplete"),
        }

        let filestem = rsp_file.file_stem().unwrap().to_string_lossy();

        // Extract text and data segments
        let objcopy = Self::get_toolchain_program("objcopy");
        for segment in ["text", "data"] {
            let mut segfile_bin = outfile.clone();
            segfile_bin.set_extension(format!("{}.bin", segment));

            let mut extract_seg = Command::new(&objcopy);
            extract_seg.arg("-O").arg("binary")
                       .arg("-j").arg(format!(".{}", segment))
                       .arg(format!("{}", outfile.display()))
                       .arg(format!("{}", segfile_bin.display()));

            eprintln!("running {:?}", extract_seg);
            let output = extract_seg.execute_output().unwrap();
            match output.status.code() {
                Some(exit_code) => {
                    if exit_code != 0 {
                        eprintln!("Error in objdump: {:?}:", rsp_file);
                        std::io::stdout().write_all(&output.stdout).unwrap();
                        std::io::stderr().write_all(&output.stderr).unwrap();
                    }
                },
                None => panic!("build incomplete"),
            }

            // Create the symbol prefix from the filename
            let ospath = segfile_bin.clone().into_os_string();
            let fullpath = ospath.to_string_lossy();
            let symprefix = String::from(&*fullpath).replace("/", "_").replace(".", "_").replace("-", "_");

            let mut segfile_o = outfile.clone();
            segfile_o.set_extension(format!("{}.o", segment));
            let mut convert_seg = Command::new(&objcopy);
            convert_seg.arg("-I").arg("binary")
                       .arg("-O").arg("elf32-ntradbigmips")
                       .arg("-B").arg("mips4300")
                       .arg("--redefine-sym").arg(format!("_binary_{}_start={}_{}_start", symprefix, filestem, segment))
                       .arg("--redefine-sym").arg(format!("_binary_{}_end={}_{}_end", symprefix, filestem, segment))
                       .arg("--redefine-sym").arg(format!("_binary_{}_size={}_{}_size", symprefix, filestem, segment))
                       .arg("--set-section-alignment").arg(".data=8")
                       .arg("--rename-section").arg(".text=.data")
                       .arg(format!("{}", segfile_bin.display()))
                       .arg(format!("{}", segfile_o.display()));

            eprintln!("running {:?}", convert_seg);
            let output = convert_seg.execute_output().unwrap();
            match output.status.code() {
                Some(exit_code) => {
                    if exit_code != 0 {
                        eprintln!("Error in objdump: {:?}:", rsp_file);
                        std::io::stdout().write_all(&output.stdout).unwrap();
                        std::io::stderr().write_all(&output.stderr).unwrap();
                    }
                },
                None => panic!("build incomplete"),
            }
        }

        // link the segments 
        let mut textsegment_o = outfile.clone();
        textsegment_o.set_extension("text.o");
        let mut datasegment_o = outfile.clone();
        datasegment_o.set_extension("data.o");

        let ld = Self::get_toolchain_program("ld");
        let mut link = Command::new(&ld);
        link.arg("-relocatable")
            .arg(format!("{}", textsegment_o.display()))
            .arg(format!("{}", datasegment_o.display()))
            .arg("-o").arg(format!("{}", outfile.display()));
        eprintln!("running {:?}", link);
        let output = link.execute_output().unwrap();
        match output.status.code() {
            Some(exit_code) => {
                if exit_code != 0 {
                    eprintln!("Error in objdump: {:?}:", rsp_file);
                    std::io::stdout().write_all(&output.stdout).unwrap();
                    std::io::stderr().write_all(&output.stderr).unwrap();
                }
            },
            None => panic!("build incomplete"),
        }
        
        // tell rustc to link this object
        println!("cargo:rustc-link-arg={}", outfile.display());
    }
}

const JUSTFILE_CONTENT: &str = include_str!("Justfile.inc");
