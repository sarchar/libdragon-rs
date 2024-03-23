use crate::*;

pub const EEPFS_ESUCCESS: i32 = libdragon_sys::EEPFS_ESUCCESS as i32;

/// EEPROM filesystem error codes
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EepfsError {
    /// Input parameters invalid
    BadInput,
    /// File does not exist
    NoFile,
    /// Bad filesystem
    BadFS,
    /// No memory for operation
    NoMem,
    /// Invalid file handle
    BadHandle,
    /// Filesystem already initialized
    Conflict,
}

impl From<::core::ffi::c_int> for EepfsError {
    fn from(val: ::core::ffi::c_int) -> EepfsError {
        match val {
            libdragon_sys::EEPFS_EBADINPUT => EepfsError::BadInput,
            libdragon_sys::EEPFS_ENOFILE => EepfsError::NoFile,
            libdragon_sys::EEPFS_EBADFS => EepfsError::BadFS,
            libdragon_sys::EEPFS_ENOMEM => EepfsError::NoMem,
            libdragon_sys::EEPFS_EBADHANDLE => EepfsError::BadHandle,
            libdragon_sys::EEPFS_ECONFLICT => EepfsError::Conflict,
            _ => panic!("invalid value"),
        }
    }
}

/// Manager of the eepfs filesystem.  Use this object to access the EEPROMFS.
pub struct Eepfs {
    names: Vec<core::pin::Pin<Box<CString>>>,
    e: Vec<libdragon_sys::eepfs_entry_t>,
}

impl Eepfs {
    /// Create a new [Eepfs] object.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            e: Vec::new(),
        }
    }

    /// Add a new Entry to the filesystem configuration table. This wraps
    /// the creation of a [`eepfs_entry_t`](libdragon_sys::eepfs_entry_t).
    pub fn add<T: AsRef<dfs::Path>>(&mut self, path: T, size: usize) -> &mut Self {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = Box::pin(CString::new(path_bytes).unwrap());
        let ptr = cpath.as_ptr();

        self.names.push(cpath);
        self.e.push(libdragon_sys::eepfs_entry_t { path: ptr, size: size });

        self
    }

    /// Initializes the EEPROM filesystem.
    ///
    /// Example:
    /// ```rust
    ///     let fs = Eepfs::new()
    ///                    .add(dfs::PathBuf::from("/high_scores.dat"), ::core::mem::size_of::<HighScores>())
    ///                    .add(dfs::PathBuf::from("/settings.dat"), ::core::mem::size_of::<Settings>())
    ///                    .add(dfs::PathBuf::from("/player.sav"), ::core::mem::size_of::<SaveData>())
    ///                    .init().expect("Could not initialize EEPROMFS");
    /// ```
    ///
    /// See [`eepfs_init`](libdragon_sys::eepfs_init) for details.
    pub fn init(&mut self) -> Result<()> {
        let v = unsafe { libdragon_sys::eepfs_init(self.e.as_ptr(), self.e.len()) };
        if v != EEPFS_ESUCCESS { return Err(LibDragonError::EepfsError { error: v.into() }) }
        Ok(())
    }

    /// Reads an entire file from the EEPROM filesystem (up to `size` bytes).
    /// 
    /// `size` bytes are allocated in a [Vec] before the call to `eepfs_read`. If you
    /// prefer to avoid the allocation, use [read_into] instead.
    ///
    /// See [`eepfs_read`](libdragon_sys::eepfs_read) for details.
    pub fn read<T: AsRef<dfs::Path>>(&self, path: T, size: usize) -> Result<Vec<u8>> {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = Box::pin(CString::new(path_bytes).unwrap());
        let mut res = Vec::with_capacity(size);
        let v = unsafe { libdragon_sys::eepfs_read(cpath.as_ptr(), res.as_mut_ptr() as *mut _, size) };
        if v != EEPFS_ESUCCESS { return Err(LibDragonError::EepfsError { error: v.into() }) }
        Ok(res)
    }

    /// Reads an entire file from the EEPROM filesystem (up to `size` bytes).
    ///
    /// Rust: the data from the read is stored in `dest`
    /// 
    /// See [`eepfs_read`](libdragon_sys::eepfs_read) for details.
    pub fn read_into<S, T: AsRef<dfs::Path>>(&self, path: T, dest: &mut [S]) -> Result<()> {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = Box::pin(CString::new(path_bytes).unwrap());
        let size = dest.len() * ::core::mem::size_of::<S>();
        let v = unsafe { libdragon_sys::eepfs_read(cpath.as_ptr(), dest.as_ptr() as *mut _, size) };
        if v != EEPFS_ESUCCESS { return Err(LibDragonError::EepfsError { error: v.into() }) }
        Ok(())
    }

    /// Writes an entire file to the EEPROM filesystem
    /// 
    /// See [`eepfs_write`](libdragon_sys::eepfs_write) for details.
    pub fn write<S, T: AsRef<dfs::Path>>(&mut self, path: T, src: &[S]) -> Result<()> {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = Box::pin(CString::new(path_bytes).unwrap());
        let size = src.len() * ::core::mem::size_of::<S>();
        let v = unsafe { libdragon_sys::eepfs_write(cpath.as_ptr(), src.as_ptr() as *const _, size) };
        if v != EEPFS_ESUCCESS { return Err(LibDragonError::EepfsError { error: v.into() }) }
        Ok(())
    }

    /// Erases a file in the EEPROM filesystem
    /// 
    /// See [`eepfs_erase`](libdragon_sys::eepfs_erase) for details.
    pub fn erase<T: AsRef<dfs::Path>>(&mut self, path: T) -> Result<()> {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = Box::pin(CString::new(path_bytes).unwrap());
        let v = unsafe { libdragon_sys::eepfs_erase(cpath.as_ptr()) };
        if v != EEPFS_ESUCCESS { return Err(LibDragonError::EepfsError { error: v.into() }) }
        Ok(())
    }

    /// Validates the first block of EEPROM
    /// 
    /// See [`eepfs_erase`](libdragon_sys::eepfs_erase) for details.
    #[inline] pub fn verify_signature(&self) -> bool { unsafe { libdragon_sys::eepfs_verify_signature() } }

    /// Erases all blocks in EEPROM and sets a new signature.
    ///
    /// Be advised: this is a destructive operation that cannot be undone!
    /// 
    /// See [`eepfs_wipe`](libdragon_sys::eepfs_wipe) for details.
    #[inline] pub fn wipe(&mut self) { unsafe { libdragon_sys::eepfs_wipe(); } }
}

impl Drop for Eepfs {
    /// De-initializes the EEPROM filesystem.
    ///
    /// See [`eepfs_close`](libdragon_sys::eepfs_close) for details.
    fn drop(&mut self) {
        // We can ignore EBADFS in cases where the fs was never initialized
        let _ = unsafe { libdragon_sys::eepfs_close() };
    }
}
