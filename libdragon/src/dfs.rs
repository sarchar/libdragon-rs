use crate::*;

const DFS_ESUCCESS: i32 = libdragon_sys::DFS_ESUCCESS as i32;

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
    pub fn open(path: &str, mode: &str) -> Result<File> {
        let cpath = CString::new(path).unwrap();
        let cmode = CString::new(mode).unwrap();
        let fp = unsafe {
            libdragon_sys::fopen(cpath.as_ptr(), cmode.as_ptr())
        };
    
        if fp == std::ptr::null_mut() {
            Err(LibDragonError::IoError {
                error: std::io::Error::new(std::io::ErrorKind::NotFound, path),
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
            let mut stat_data: ::std::mem::MaybeUninit<libdragon_sys::stat> = ::std::mem::MaybeUninit::uninit();
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
