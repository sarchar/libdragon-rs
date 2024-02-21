use crate::*;

const DFS_ESUCCESS: i32 = libdragon_sys::DFS_ESUCCESS as i32;

const DT_REG: i32 = libdragon_sys::DT_REG as i32;
const DT_DIR: i32 = libdragon_sys::DT_DIR as i32;

pub fn init(base_fs_loc: Option<u32>) -> Result<()> {
    let s = unsafe {
        libdragon_sys::dfs_init(base_fs_loc.unwrap_or(libdragon_sys::DFS_DEFAULT_LOCATION))
    };

    match s {
        DFS_ESUCCESS => Ok(()),
        err @ _ => {
            Err(LibDragonError::DfsError { error_code: err })
        }
    }
}

pub fn rom_addr(path: &str) -> u32 {
    let cpath = CString::new(path).unwrap();
    unsafe {
        libdragon_sys::dfs_rom_addr(cpath.as_ptr())
    }
}

#[derive(Debug)]
pub struct File {
    fp: Option<*mut libdragon_sys::FILE>,
}

impl File {
    pub fn open(path: OsString, mode: &str) -> Result<File> {
        // no as_bytes() on nintendo64
        let osstr: &OsStr = &path;
        let path_bytes: &[u8] = unsafe { std::mem::transmute(osstr) };
        let cpath = CString::new(path_bytes).unwrap();
        let cmode = CString::new(mode).unwrap();
        let fp = unsafe {
            libdragon_sys::fopen(cpath.as_ptr(), cmode.as_ptr())
        };
    
        if fp == std::ptr::null_mut() {
            let pathbuf = std::path::PathBuf::from(path);
            Err(LibDragonError::IoError {
                error: std::io::Error::new(std::io::ErrorKind::NotFound, format!("{}", pathbuf.display())),
            })
        } else {
            Ok(File {
                fp: Some(fp)
            })
        }
    }

    pub fn close(&mut self) {
        if let Some(fp) = std::mem::replace(&mut self.fp, None) {
            unsafe { 
                libdragon_sys::fclose(fp);
            }
        }
    }

    pub fn tell(&self) -> Result<u64> {
        if let Some(ref fp) = self.fp {
            let r = unsafe {
                libdragon_sys::ftell(*fp)
            };
            if r < 0 {
                Err(LibDragonError::IoError {
                    error: std::io::Error::new(std::io::ErrorKind::BrokenPipe, "TODO: tell")
                })
            } else {
                Ok(r as u64)
            }
        } else {
            Err(LibDragonError::IoError {
                error: std::io::Error::new(std::io::ErrorKind::NotFound, "TODO: tell")
            })
        }
    }

    pub fn eof(&self) -> Result<bool> {
        if let Some(ref fp) = self.fp {
            let r = unsafe {
                libdragon_sys::feof(*fp)
            };
            if r < 0 {
                Err(LibDragonError::IoError {
                    error: std::io::Error::new(std::io::ErrorKind::BrokenPipe, "TODO: eof")
                })
            } else {
                Ok(r != 0)
            }
        } else {
            Err(LibDragonError::IoError {
                error: std::io::Error::new(std::io::ErrorKind::NotFound, "TODO: eof")
            })
        }
    }

    pub fn size(&self) -> Result<u64> {
        if let Some(ref fp) = self.fp {
            let mut stat_data: std::mem::MaybeUninit<libdragon_sys::stat> = std::mem::MaybeUninit::uninit();
            let r = unsafe {
                libdragon_sys::fstat(libdragon_sys::fileno(*fp), stat_data.as_mut_ptr())
            };
            if r < 0 {
                Err(LibDragonError::IoError {
                    error: std::io::Error::new(std::io::ErrorKind::BrokenPipe, "TODO: size")
                })
            } else {
                Ok(unsafe { stat_data.assume_init() }.st_size as u64)
            }
        } else {
            Err(LibDragonError::IoError {
                error: std::io::Error::new(std::io::ErrorKind::NotFound, "TODO: size")
            })
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        self.close();
    }
}

impl std::io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(ref fp) = self.fp {
            let max_size = buf.len() as u32;
            let read_size = unsafe { 
                let buf_ptr = buf.as_mut_ptr() as *mut ::std::os::raw::c_void;
                libdragon_sys::fread(buf_ptr, 1, max_size, *fp)
            };
            Ok(read_size as usize)
        } else {
            Ok(0)
        }
    }
}

impl std::io::Seek for File {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        if let Some(ref fp) = self.fp {
            let (seek_pos, offset) = match pos {
                std::io::SeekFrom::Start(n)   => (libdragon_sys::SEEK_SET, n as i64),
                std::io::SeekFrom::End(n)     => (libdragon_sys::SEEK_END, n),
                std::io::SeekFrom::Current(n) => (libdragon_sys::SEEK_CUR, n),
            };
            let r = unsafe {
                libdragon_sys::fseek(*fp, offset as ::std::os::raw::c_long, seek_pos as ::std::os::raw::c_int)
            };
            if r < 0 {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotSeekable,
                    "TODO: use the correct error"
                ))
            } else {
                Ok(r as u64)
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "could not seek"
            ))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntryType {
    File,
    Directory
}

pub struct Dir<'a> {
    path: &'a str,
    dir: libdragon_sys::dir_t,
}

impl<'a> Dir<'a> {
    pub fn findfirst(path: &'a str) -> Result<Self> {
        let cpath = CString::new(path).unwrap();
        let mut dir: std::mem::MaybeUninit<libdragon_sys::dir_t> = std::mem::MaybeUninit::uninit();
        let s = unsafe {
            libdragon_sys::dir_findfirst(cpath.as_ptr(), dir.as_mut_ptr())
        };
        if s < 0 {
            Err(LibDragonError::IoError {
                error: std::io::Error::new(std::io::ErrorKind::NotFound, path),
            })
        } else {
            Ok(Self {
                path: path,
                dir: unsafe { dir.assume_init() }
            })
        }
    }

    pub fn findnext(&mut self) -> Result<()> {
        let cpath = CString::new(self.path).unwrap();
        let s = unsafe {
            libdragon_sys::dir_findnext(cpath.as_ptr(), (&mut self.dir) as *mut libdragon_sys::dir_t)
        };
        if s < 0 {
            match get_errno() {
                libdragon_sys::ENOENT => Err(LibDragonError::IoError {
                    error: std::io::Error::new(std::io::ErrorKind::NotFound, "No more entries"),
                }),
                errno @ _ => Err(LibDragonError::ErrnoError {
                    errno: errno
                }),
            }
        } else {
            Ok(())
        }
    }

    pub fn d_name(&self) -> std::path::PathBuf {
        let slice = unsafe { CStr::from_ptr(self.dir.d_name.as_ptr()) };
        // so I guess from_bytes is only availble on cfg(unix)
        // let osstr = OsStr::from_bytes(slice.to_bytes());
        let osstr: &OsStr = unsafe { std::mem::transmute(slice.to_bytes()) };
        osstr.into()
    }

    pub fn d_type(&self) -> Result<EntryType> {
        match self.dir.d_type {
            DT_REG => Ok(EntryType::File),
            DT_DIR => Ok(EntryType::Directory),
            _ => Err(LibDragonError::IoError {
                        error: std::io::Error::new(std::io::ErrorKind::NotFound, "invalid d_type value")
            })
        }
    }
}
