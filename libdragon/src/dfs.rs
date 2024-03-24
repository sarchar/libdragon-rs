use crate::*;

#[doc(hidden)]
pub use embedded_io::{Read, Seek};

#[doc(hidden)]
const DFS_ESUCCESS: i32 = libdragon_sys::DFS_ESUCCESS as i32;

#[doc(hidden)]
const DFS_EBADINPUT: i32 = libdragon_sys::DFS_EBADINPUT as i32;

#[doc(hidden)]
const DFS_ENOFILE: i32 = libdragon_sys::DFS_ENOFILE as i32;

#[doc(hidden)]
const DFS_EBADFS: i32 = libdragon_sys::DFS_EBADFS as i32;

#[doc(hidden)]
const DFS_ENFILE: i32 = libdragon_sys::DFS_ENFILE as i32;

#[doc(hidden)]
const DFS_EBADHANDLE: i32 = libdragon_sys::DFS_EBADHANDLE as i32;

#[doc(hidden)]
const FLAGS_FILE: i32 = libdragon_sys::FLAGS_FILE as i32;

#[doc(hidden)]
const FLAGS_DIR: i32 = libdragon_sys::FLAGS_DIR as i32;

#[doc(hidden)]
const FLAGS_EOF: i32 = libdragon_sys::FLAGS_EOF as i32;

/// Maximum filename length
/// See [`MAX_FILENAME_LEN`](libdragon_sys::MAX_FILENAME_LEN)
pub const MAX_FILENAME_LEN: usize = libdragon_sys::MAX_FILENAME_LEN as usize;

/// Maximum depth of directories supported
/// See [`MAX_DIRECTORY_DEPTH`](libdragon_sys::MAX_DIRECTORY_DEPTH)
pub const MAX_DIRECTORY_DEPTH: usize = libdragon_sys::MAX_DIRECTORY_DEPTH as usize;

const DT_REG: i32 = libdragon_sys::DT_REG as i32;
const DT_DIR: i32 = libdragon_sys::DT_DIR as i32;

/// Type alias to a [typed_path::PathBuf] with encoding [typed_path::UnixEncoding].
///
/// [PathBuf] is used in place of where `std::path::PathBuf` might have been used.
pub type PathBuf = typed_path::PathBuf<typed_path::UnixEncoding>;

/// Type alias to a [typed_path::Path] with encoding [typed_path::UnixEncoding].
///
/// [Path] is used in place of where `std::path::Path` might have been used, and in particular
/// the `AsRef<Path>` equivelents.
pub type Path = typed_path::Path<typed_path::UnixEncoding>;

/// Mapping from the various LibDragon DFS_E* types into an enum
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DfsError {
    /// Bad filesystem
    NoFS,
    /// File does not exist
    NotFound,
    /// Invalid file handle, file closed, or lost
    BadHandle,
    /// Input parameters invalid
    InvalidInput,
    /// Too many open files
    NumFiles,
}

impl embedded_io::Error for DfsError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            DfsError::NoFS => embedded_io::ErrorKind::Other,
            DfsError::NotFound => embedded_io::ErrorKind::NotFound,
            DfsError::BadHandle => embedded_io::ErrorKind::BrokenPipe,
            DfsError::InvalidInput => embedded_io::ErrorKind::InvalidInput,
            DfsError::NumFiles => embedded_io::ErrorKind::OutOfMemory,
        }
    }
}

impl From<i32> for DfsError {
    fn from(val: i32) -> DfsError {
        match val {
            DFS_EBADINPUT  => DfsError::InvalidInput,
            DFS_ENOFILE    => DfsError::NotFound,
            DFS_EBADFS     => DfsError::NoFS,
            DFS_ENFILE     => DfsError::NumFiles,
            DFS_EBADHANDLE => DfsError::BadHandle,
            _              => panic!("invalid DFS error {}", val),
        }
    }
}

/// Initialize the filesystem.
///
/// Rust-specific: instead of using `DFS_DEFAULT_LOCATION`, pass `None` for `base_fs_loc`.
///
/// See [`dfs_init`](libdragon_sys::dfs_init) for details.
pub fn init(base_fs_loc: Option<u32>) -> Result<()> {
    let s = unsafe {
        libdragon_sys::dfs_init(base_fs_loc.unwrap_or(libdragon_sys::DFS_DEFAULT_LOCATION))
    };

    match s {
        DFS_ESUCCESS => Ok(()),
        e @ _ => Err(LibDragonError::DfsError { error: e.into() }),
    }
}

/// Change directories to the specified path.
///
/// See [`dfs_chdir`](libdragon_sys::dfs_chdir) for details.
pub fn chdir(path: &PathBuf) -> Result<()> {
    let path_bytes: &[u8] = path.as_bytes();
    let cpath = CString::new(path_bytes).unwrap();
    let err = unsafe {
        libdragon_sys::dfs_chdir(cpath.as_ptr())
    };
    if err < 0 {
        Err(LibDragonError::DfsError { error: err.into() })
    } else {
        Ok(())
    }
}

/// Wrapper for results when calling [dir_findfirst] and [dir_findnext].
pub struct DirEntry {
    /// Type of the current entry, which may be [EntryType::Eof]
    pub entry_type: EntryType,
    /// File name (not full path) of the current entry, which may be empty in the case of Eof.
    pub name: PathBuf,
}

impl From<i32> for EntryType {
    fn from(val: i32) -> EntryType {
        match val {
            FLAGS_FILE => EntryType::File,
            FLAGS_DIR  => EntryType::Directory,
            FLAGS_EOF  => EntryType::Eof,
            _          => panic!("invalid value for EntryType {}", val),
        }
    }
}

/// Find the first file or directory in a directory listing.
///
/// See [`dfs_dir_findfirst`](libdragon_sys::dfs_dir_findfirst) for details.
pub fn dir_findfirst(path: &PathBuf) -> Result<DirEntry> {
    let path_bytes: &[u8] = path.as_bytes();
    let cpath = CString::new(path_bytes).unwrap();
    let mut namebuf = [0u8; MAX_FILENAME_LEN];
    let ret = unsafe {
        libdragon_sys::dfs_dir_findfirst(cpath.as_ptr(), namebuf.as_mut_ptr() as *mut _)
    };
    if ret < 0 {
        Err(LibDragonError::DfsError { error: ret.into() })
    } else {
        let c_str = unsafe { CStr::from_ptr(namebuf.as_ptr() as *const i8) };
        Ok(DirEntry {
            entry_type: ret.into(),
            name: PathBuf::from(c_str.to_str()?),
        })
    }
}

/// Find the next file or directory in a directory listing.
///
/// See [`dfs_dir_findnext`](libdragon_sys::dfs_dir_findnext) for details.
pub fn dir_findnext() -> Result<DirEntry> {
    let mut namebuf = [0u8; MAX_FILENAME_LEN];
    let ret = unsafe {
        libdragon_sys::dfs_dir_findnext(namebuf.as_mut_ptr() as *mut _)
    };
    if ret < 0 {
        Err(LibDragonError::DfsError { error: ret.into() })
    } else {
        let c_str = unsafe { CStr::from_ptr(namebuf.as_ptr() as *const i8) };
        Ok(DirEntry {
            entry_type: ret.into(),
            name: PathBuf::from(c_str.to_str()?),
        })
    }
}

/// Wrapper around file handles to be used exclusively with dfs_* calls
///
/// An important distinction between [DfsFileHandle] and [File] is that 
/// [DfsFileHandle] calls LibDragon's `dfs_*` functions directly
/// rather than interfacing with the C standard library file functions.
///
/// [DfsFileHandle] also implements [Read] and [Seek].
pub struct DfsFileHandle(Option<u32>);

/// Open a file given a path
///
/// See [`dfs_open`](libdragon_sys::dfs_open) for details.
pub fn open<T: AsRef<Path>>(path: T) -> Result<DfsFileHandle> {
    let path_bytes: &[u8] = path.as_ref().as_bytes();
    let cpath = CString::new(path_bytes).unwrap();
    let ret = unsafe {
        libdragon_sys::dfs_open(cpath.as_ptr())
    };
    if ret < 0 {
        Err(LibDragonError::DfsError { error: ret.into() })
    } else {
        Ok(DfsFileHandle(Some(ret as u32)))
    }
}

impl DfsFileHandle {
    /// Close an already open file handle
    ///
    /// See [`dfs_close`](libdragon_sys::dfs_close)
    pub fn close(&mut self) {
        if let Some(fp) = core::mem::replace(&mut self.0, None) {
            unsafe { 
                libdragon_sys::dfs_close(fp);
            }
        }
    }

    /// Return the current offset into a file
    ///
    /// See [`dfs_tell`](libdragon_sys::dfs_tell)
    pub fn tell(&self) -> Result<u32> {
        if let Some(ref fp) = self.0 {
            let r = unsafe { libdragon_sys::dfs_tell(*fp) };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::BadHandle })
            } else {
                Ok(r as u32)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
        }
    }

    /// Return whether the end of the file has been reached
    ///
    /// See [`dfs_eof`](libdragon_sys::dfs_eof)
    pub fn eof(&self) -> Result<bool> {
        if let Some(ref fp) = self.0 {
            let r = unsafe {
                libdragon_sys::dfs_eof(*fp)
            };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::BadHandle })
            } else {
                Ok(r != 0)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
        }
    }

    /// Return the file size of an open file
    ///
    /// See [`dfs_size`](libdragon_sys::dfs_size)
    pub fn size(&self) -> Result<usize> {
        if let Some(ref fp) = self.0 {
            let r = unsafe {
                libdragon_sys::dfs_size(*fp)
            };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::BadHandle })
            } else {
                Ok(r as usize)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
        }
    }
}

// Because we implement embedded_io::Read and Seek, we need to specify the errortype this object uses
impl embedded_io::ErrorType for DfsFileHandle {
    type Error = DfsError;
}

impl embedded_io::Read for DfsFileHandle {
    /// Read data from a file
    ///
    /// Rust-specific: the maximum amount of data to read is given by the length of the slice. On success,
    /// the amount of valid data read is returned.
    ///
    /// See [`dfs_read`](libdragon_sys::dfs_read) for details.
    fn read(&mut self, buf: &mut [u8]) -> core::result::Result<usize, Self::Error> {
        if let Some(ref fp) = self.0 {
            let max_size = buf.len() as i32;
            let read_size = unsafe { 
                let buf_ptr = buf.as_mut_ptr() as *mut ::core::ffi::c_void;
                libdragon_sys::dfs_read(buf_ptr, 1, max_size, *fp)
            };
            if read_size < 0 {
                Err(DfsError::BadHandle)
            } else {
                Ok(read_size as usize)
            }
        } else {
            Err(DfsError::InvalidInput)
        }
    }
}

impl embedded_io::Seek for DfsFileHandle {
    /// Seek to an offset in the file
    ///
    /// See [`dfs_seek`](libdragon_sys::dfs_seek) for details.
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> core::result::Result<u64, Self::Error> {
        if let Some(ref fp) = self.0 {
            let (seek_pos, offset) = match pos {
                embedded_io::SeekFrom::Start(n)   => (libdragon_sys::SEEK_SET, n as i64),
                embedded_io::SeekFrom::End(n)     => (libdragon_sys::SEEK_END, n),
                embedded_io::SeekFrom::Current(n) => (libdragon_sys::SEEK_CUR, n),
            };
            let r = unsafe {
                libdragon_sys::dfs_seek(*fp, offset as ::core::ffi::c_long, seek_pos as ::core::ffi::c_int)
            };
            if r < 0 {
                Err(DfsError::BadHandle)
            } else {
                Ok(r as u64)
            }
        } else {
            Err(DfsError::InvalidInput)
        }
    }
}

/// Return the physical address of a file (in ROM space)
///
/// See [`dfs_rom_addr`](libdragon_sys::dfs_rom_addr) for details.
pub fn rom_addr(path: &str) -> u32 {
    let cpath = CString::new(path).unwrap();
    unsafe {
        libdragon_sys::dfs_rom_addr(cpath.as_ptr())
    }
}

/// File is a wrapper over the C standard library [FILE](libdragon_sys::FILE) object
///
/// Not all [FILE][libdragon_sys::FILE] functionality is provided, but [File] implements
/// [Read] and [Seek] traits for useability.
#[derive(Debug)]
pub struct File {
    pub(crate) fp: Option<*mut libdragon_sys::FILE>,
}

impl File {
    /// Opens a [FILE](libdragon_sys::FILE) using [fopen](libdragon_sys::fopen) and returns a [File] wrapper object.
    pub fn open<T: AsRef<Path>>(path: T, mode: &str) -> Result<File> {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = CString::new(path_bytes).unwrap();
        let cmode = CString::new(mode).unwrap();
        let fp = unsafe {
            libdragon_sys::fopen(cpath.as_ptr(), cmode.as_ptr())
        };
    
        if fp == core::ptr::null_mut() {
            Err(LibDragonError::DfsError { error: DfsError::NotFound })
        } else {
            Ok(File { fp: Some(fp) })
        }
    }

    /// Closes a [FILE](libdragon_sys::FILE) using [fclose](libdragon_sys::fclose) and invalidates this [File] wrapper object.
    pub fn close(&mut self) {
        if let Some(fp) = core::mem::replace(&mut self.fp, None) {
            unsafe { 
                libdragon_sys::fclose(fp);
            }
        }
    }

    /// Returns the file position using [ftell](libdragon_sys::ftell)
    pub fn tell(&self) -> Result<u64> {
        if let Some(ref fp) = self.fp {
            let r = unsafe { libdragon_sys::ftell(*fp) };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::BadHandle })
            } else {
                Ok(r as u64)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
        }
    }

    /// Check if the [FILE](libdragon_sys::FILE) has reached EOF using [feof](libdragon_sys::feof).
    pub fn eof(&self) -> Result<bool> {
        if let Some(ref fp) = self.fp {
            let r = unsafe {
                libdragon_sys::feof(*fp)
            };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::BadHandle })
            } else {
                Ok(r != 0)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
        }
    }

    /// Return the file size of the [FILE](libdragon_sys::FILE) using [fstat](libdragon_sys::fstat)
    pub fn size(&self) -> Result<usize> {
        if let Some(ref fp) = self.fp {
            let mut stat_data: core::mem::MaybeUninit<libdragon_sys::stat> = core::mem::MaybeUninit::uninit();
            let r = unsafe {
                libdragon_sys::fstat(libdragon_sys::fileno(*fp), stat_data.as_mut_ptr())
            };
            if r < 0 {
                Err(LibDragonError::DfsError { error: DfsError::BadHandle })
            } else {
                Ok(unsafe { stat_data.assume_init() }.st_size as usize)
            }
        } else {
            Err(LibDragonError::DfsError { error: DfsError::InvalidInput })
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        self.close();
    }
}

// Because we implement embedded_io::Read and Seek, we need to specify the errortype this object uses
impl embedded_io::ErrorType for File {
    type Error = DfsError;
}

impl embedded_io::Read for File {
    /// Read data from a [FILE](libdragon_sys::FILE) using [fread](libdragon_sys::fread).
    ///
    /// The maximum amount of data to read is given by the length of the slice. On success,
    /// the amount of valid data read is returned.
    fn read(&mut self, buf: &mut [u8]) -> core::result::Result<usize, Self::Error> {
        if let Some(ref fp) = self.fp {
            let max_size = buf.len() as u32;
            let read_size = unsafe { 
                let buf_ptr = buf.as_mut_ptr() as *mut ::core::ffi::c_void;
                libdragon_sys::fread(buf_ptr, 1, max_size, *fp)
            };
            // fread returns unsigned int, never errors, so we must use ferror
            let error = unsafe { libdragon_sys::ferror(*fp) };
            if error != 0 {
                Err(DfsError::BadHandle)
            } else {
                Ok(read_size as usize)
            }
        } else {
            Err(DfsError::InvalidInput)
        }
    }
}

impl embedded_io::Seek for File {
    /// Seek to a position within a [FILE](libdragon_sys::FILE) using
    /// [fseek](libdragon_sys::fseek).
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
                Err(DfsError::BadHandle)
            } else {
                Ok(r as u64)
            }
        } else {
            Err(DfsError::InvalidInput)
        }
    }
}

/// When using dir_findfirst (both in dfs and stdlib), indicates the current entry type.
///
/// Mapping from the various C stdlib DT_* types into an enum
/// Mapping from the various dfs FILES_* flags into an enum
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntryType {
    /// The current type is a File
    File,
    /// The current type is a Directory
    Directory,
    /// The current type an end-of-list marker. This is not used with C stdlib `dir_findfirst`/`next`.
    Eof
}

/// Dir is a wrapper over the C standard library [dir_t](libdragon_sys::dir_t) object
pub struct Dir<'a> {
    path: &'a str,
    dir: libdragon_sys::dir_t,
}

impl<'a> Dir<'a> {
    /// Create a new [Dir] object by calling [dir_findfirst](libdragon_sys::dir_findfirst).
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

    /// Move onto the next record in the directory by calling [dir_findnext](libdragon_sys::dir_findnext).
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

    /// Return a [PathBuf] object with the file name set to the current entry.
    /// See [dir_t](libdragon_sys::dir_t) for more information.
    pub fn d_name(&self) -> PathBuf {
        let slice = unsafe { CStr::from_ptr(self.dir.d_name.as_ptr()) };
        let mut pb = PathBuf::new();
        pb.set_file_name(slice.to_bytes());
        pb
    }

    /// Returns the [EntryType] of the current entry.
    pub fn d_type(&self) -> Result<EntryType> {
        match self.dir.d_type {
            DT_REG => Ok(EntryType::File),
            DT_DIR => Ok(EntryType::Directory),
            _ => Err(LibDragonError::DfsError { error: DfsError::NotFound }),
        }
    }
}
