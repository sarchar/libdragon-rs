
use crate::*;

/// Only level two requires initialization
///
/// See `asset.h` in the LibDragon source
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompressionLevel {
    One,
    Two,
    Three
}

/// Enable a non-default compression level
///
/// See [`asset_init_compression`](libdragon_sys::asset_init_compression)
pub fn init_compression(level: CompressionLevel) {
    match level {
        CompressionLevel::One => {},
        CompressionLevel::Two => unsafe { libdragon_sys::__asset_init_compression_lvl2(); },
        CompressionLevel::Three => { unimplemented!("__asset_init_compression_lvl3();") },
    }
}

/// Load an asset file (possibly uncompressing it)
///
/// Memory will be freed when the Box is dropped.
///
/// See [`asset_load`](libdragon_sys::asset_load)
pub fn load<S, T: AsRef<dfs::Path>>(path: T) -> Option<Box<[S]>> {
    let path_bytes: &[u8] = path.as_ref().as_bytes();
    let cpath = CString::new(path_bytes).unwrap();

    unsafe {
        let mut sz: i32 = 0;
        let ptr = libdragon_sys::asset_load(cpath.as_ptr(), &mut sz as *mut _);
        if ptr == core::ptr::null_mut() { return None; }

        // Create a slice
        let slice: &mut [S] = core::slice::from_raw_parts_mut(ptr as *mut _, (sz as usize) / core::mem::size_of::<S>());

        // asset_load returns a pointer that should be freed with free().  Conveniently,
        // that's what our global allocator uses. I just hope Rust doesn't do anything to the
        // pointer to the slice.
        Some(Box::from_raw(slice))
    }
}

/// Open an asset file for reading (with transparent decompression)
///
/// Rust-specific: the object returned is a [dfs::File]
///
/// See [`asset_fopen`](libdragon_sys::asset_fopen)
pub fn fopen<T: AsRef<dfs::Path>>(path: T) -> Result<dfs::File> {
    let path_bytes: &[u8] = path.as_ref().as_bytes();
    let cpath = CString::new(path_bytes).unwrap();

    let fp = unsafe {
        let mut sz: i32 = 0;
        libdragon_sys::asset_fopen(cpath.as_ptr(), &mut sz as *mut _)
    };

    if fp == core::ptr::null_mut() {
        Err(LibDragonError::DfsError { error: dfs::DfsError::NotFound })
    } else {
        Ok(dfs::File { fp: Some(fp) })
    }
}
