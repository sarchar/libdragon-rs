
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
