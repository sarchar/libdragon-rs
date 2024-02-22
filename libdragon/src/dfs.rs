use crate::*;

pub use embedded_io::Read;

const DFS_ESUCCESS: i32 = libdragon_sys::DFS_ESUCCESS as i32;

const DT_REG: i32 = libdragon_sys::DT_REG as i32;
const DT_DIR: i32 = libdragon_sys::DT_DIR as i32;

pub type DfsPathBuf = typed_path::PathBuf<typed_path::UnixEncoding>;

#[derive(Debug, Copy, Clone)]
pub enum DfsError {
    NoFS,
    NotFound,
    BrokenPipe,
    InvalidInput
}

impl embedded_io::Error for DfsError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            DfsError::NoFS => embedded_io::ErrorKind::Other,
            DfsError::NotFound => embedded_io::ErrorKind::NotFound,
            DfsError::BrokenPipe => embedded_io::ErrorKind::BrokenPipe,
            DfsError::InvalidInput => embedded_io::ErrorKind::InvalidInput,
        }
    }
}


pub fn init(base_fs_loc: Option<u32>) -> Result<()> {
    let s = unsafe {
        libdragon_sys::dfs_init(base_fs_loc.unwrap_or(libdragon_sys::DFS_DEFAULT_LOCATION))
    };

    match s {
        DFS_ESUCCESS => Ok(()),
        _ => {
            Err(LibDragonError::DfsError { error: DfsError::NoFS })
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
    pub fn open(path: &DfsPathBuf, mode: &str) -> Result<File> {
        // no as_bytes() on nintendo64
        let path_bytes: &[u8] = path.as_bytes();
        let cpath = CString::new(path_bytes).unwrap();
        let cmode = CString::new(mode).unwrap();
        let fp = unsafe {
            libdragon_sys::fopen(cpath.as_ptr(), cmode.as_ptr())
        };
    
        if fp == core::ptr::null_mut() {
            Err(LibDragonError::DfsError {
                error: DfsError::NotFound
            })
        } else {
            Ok(File {
                fp: Some(fp)
            })
        }
    }

    pub fn close(&mut self) {
        if let Some(fp) = core::mem::replace(&mut self.fp, None) {
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
                Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
            } else {
                Ok(r as u64)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::BrokenPipe })
        }
    }

    pub fn eof(&self) -> Result<bool> {
        if let Some(ref fp) = self.fp {
            let r = unsafe {
                libdragon_sys::feof(*fp)
            };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
            } else {
                Ok(r != 0)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::BrokenPipe })
        }
    }

    pub fn size(&self) -> Result<usize> {
        if let Some(ref fp) = self.fp {
            let mut stat_data: core::mem::MaybeUninit<libdragon_sys::stat> = core::mem::MaybeUninit::uninit();
            let r = unsafe {
                libdragon_sys::fstat(libdragon_sys::fileno(*fp), stat_data.as_mut_ptr())
            };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
            } else {
                Ok(unsafe { stat_data.assume_init() }.st_size as usize)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::BrokenPipe })
        }
    }
}

impl embedded_io::ErrorType for File {
    type Error = DfsError;
}

impl Drop for File {
    fn drop(&mut self) {
        self.close();
    }
}

impl embedded_io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> core::result::Result<usize, Self::Error> {
        if let Some(ref fp) = self.fp {
            let max_size = buf.len() as u32;
            let read_size = unsafe { 
                let buf_ptr = buf.as_mut_ptr() as *mut ::core::ffi::c_void;
                libdragon_sys::fread(buf_ptr, 1, max_size, *fp)
            };
            Ok(read_size as usize)
        } else {
            Err(DfsError::BrokenPipe)
        }
    }
}

impl embedded_io::Seek for File {
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> core::result::Result<u64, Self::Error> {
        if let Some(ref fp) = self.fp {
            let (seek_pos, offset) = match pos {
                embedded_io::SeekFrom::Start(n)   => (libdragon_sys::SEEK_SET, n as i64),
                embedded_io::SeekFrom::End(n)     => (libdragon_sys::SEEK_END, n),
                embedded_io::SeekFrom::Current(n) => (libdragon_sys::SEEK_CUR, n),
            };
            let r = unsafe {
                libdragon_sys::fseek(*fp, offset as ::core::ffi::c_long, seek_pos as ::core::ffi::c_int)
            };
            if r < 0 {
                Err(DfsError::InvalidInput)
            } else {
                Ok(r as u64)
            }
        } else {
            Err(DfsError::BrokenPipe)
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
        let mut dir: core::mem::MaybeUninit<libdragon_sys::dir_t> = core::mem::MaybeUninit::uninit();
        let s = unsafe {
            libdragon_sys::dir_findfirst(cpath.as_ptr(), dir.as_mut_ptr())
        };
        if s < 0 {
            Err(LibDragonError::DfsError { error: DfsError::NotFound })
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
                libdragon_sys::ENOENT => Err(LibDragonError::DfsError { error: DfsError::NotFound }),
                errno @ _ => Err(LibDragonError::ErrnoError {
                    errno: errno
                }),
            }
        } else {
            Ok(())
        }
    }

    pub fn d_name(&self) -> DfsPathBuf {
        let slice = unsafe { CStr::from_ptr(self.dir.d_name.as_ptr()) };
        let mut pb = DfsPathBuf::new();
        pb.set_file_name(slice.to_bytes());
        pb
    }

    pub fn d_type(&self) -> Result<EntryType> {
        match self.dir.d_type {
            DT_REG => Ok(EntryType::File),
            DT_DIR => Ok(EntryType::Directory),
            _ => Err(LibDragonError::DfsError { error: DfsError::NotFound }),
        }
    }
}
