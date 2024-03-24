use crate::*;

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        /// Flag for compatibility
        const LAZY = libdragon_sys::RTLD_LAZY;
        /// Flag for compatibility
        const NOW = libdragon_sys::RTLD_NOW;
        /// Export symbols to other dynamic libraries
        const GLOBAL = libdragon_sys::RTLD_GLOBAL;
        /// Don't export symbols to other dynamic libraries
        const LOCAL = libdragon_sys::RTLD_LOCAL;
        /// Never unload dynamic library from memory
        const NODELETE = libdragon_sys::RTLD_NODELETE;
        /// Don't load dynamic library to memory if not loaded
        const NOLOAD = libdragon_sys::RTLD_NOLOAD;
    }
}

/// Wrapper around dl* function error returns
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// An error occurred in dlopen()
    DlOpenError,
    /// An error occurred in dlsym()
    DlSymError,
    /// An error occurred in dladdr()
    DlAddrError { code: i32 },
    /// An error occurred in dlclose()
    DlCloseError { code: i32 },
}

/// Wrapper around dl* functions and the handle it returns.
pub struct DyLib {
    handle: *mut ::core::ffi::c_void,
}

impl DyLib {
    /// Open dynamic library
    ///
    /// See [`dlopen`](libdragon_sys::dlopen) for details.
    pub fn open<T: AsRef<dfs::Path>>(path: T, flags: Flags) -> Result<Self> {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let handle = unsafe {
            libdragon_sys::dlopen(cpath.as_ptr(), flags.bits() as i32)
        };

        if handle == ::core::ptr::null_mut() {
            return Err(LibDragonError::DlError { error: Error::DlOpenError });
        }

        Ok(Self { handle: handle })
    }

    /// Grab symbol from loaded dynamic library
    ///
    /// See [`dlsym`](libdragon_sys::dlsym) for details.
    pub unsafe fn sym(&mut self, name: &str) -> Result<*mut ::core::ffi::c_void> {
        let c_name = CString::new(name).unwrap();

        let ptr = unsafe { libdragon_sys::dlsym(self.handle, c_name.as_ptr()) };

        if ptr == ::core::ptr::null_mut() {
            return Err(LibDragonError::DlError { error: Error::DlSymError });
        }

        Ok(ptr)
    }

}

impl Drop for DyLib {
    /// Close loaded dynamic library
    ///
    /// See [`dlclose`](libdragon_sys::dlclose) for details.
    fn drop(&mut self) {
        let _ = unsafe {
            libdragon_sys::dlclose(self.handle)
        };
    }
}

/// Wrapper around [Dl_info](libdragon_sys::Dl_info).
///
/// Obtained via [SymbolInfo::try_from] with a pointer from [DyLib].
pub struct SymbolInfo {
    info: libdragon_sys::Dl_info,
}

impl TryFrom<*mut ::core::ffi::c_void> for SymbolInfo {
    type Error = LibDragonError;

    /// Convert address to symbol
    ///
    /// See [`dladdr`](libdragon_sys::dladdr) for details.
    fn try_from(addr: *mut ::core::ffi::c_void) -> core::result::Result<Self, Self::Error> {
        let mut info: core::mem::MaybeUninit<libdragon_sys::Dl_info> = core::mem::MaybeUninit::uninit();
        let r = unsafe {
            libdragon_sys::dladdr(addr, info.as_mut_ptr())
        };
        if r != 0 { 
            return Err(LibDragonError::DlError { error: Error::DlAddrError { code: r } });
        }
        Ok(Self {
            info: unsafe { info.assume_init() },
        })
    }
}

impl SymbolInfo {
    /// Access [`Dl_info.dli_fname`](libdragon_sys::Dl_info::dli_fname).
    pub fn fname(&self) -> Result<&str> {
        let c_str = unsafe { CStr::from_ptr(self.info.dli_fname) };
        Ok(c_str.to_str()?)
    }

    /// Access [`Dl_info.dli_fbase`](libdragon_sys::Dl_info::dli_fbase).
    pub unsafe fn fbase(&self) -> *mut ::core::ffi::c_void {
        self.info.dli_fbase
    }

    /// Access [`Dl_info.dli_sname`](libdragon_sys::Dl_info::dli_sname).
    pub fn sname(&self) -> Result<&str> {
        let c_str = unsafe { CStr::from_ptr(self.info.dli_sname) };
        Ok(c_str.to_str()?)
    }

    /// Access [`Dl_info.dli_saddr`](libdragon_sys::Dl_info::dli_saddr).
    pub unsafe fn saddr(&self) -> *mut ::core::ffi::c_void {
        self.info.dli_saddr
    }
}

/// Return last error that occurred in dynamic linker
///
/// See [`dlerror`](libdragon_sys::dlerror) for details
pub fn error() -> Result<&'static str> {
    let c_str = unsafe { CStr::from_ptr(libdragon_sys::dlerror()) };
    Ok(c_str.to_str()?)
}

