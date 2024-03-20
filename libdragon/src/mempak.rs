//!
//! This module extends [Port](joypad::Port) with Controller Pak Filesystem access functionality.
use crate::*;

/// Size in bytes of a Controller Pak block
pub const BLOCK_SIZE: usize = libdragon_sys::MEMPAK_BLOCK_SIZE as usize;

/// Create a [MemPak] object from a given [Port](joypad::Port)
pub trait MemPakGetter: joypad::BasePort {
    #[inline]
    fn mempak(&mut self) -> MemPak { MemPak { port: self.port() } }
}

impl MemPakGetter for joypad::Port {}

/// Structure containing the mempak functionality. This structure knows the controller port
/// being operated on, simplifying the interface.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MemPak {
    port: usize,
}

impl MemPak {
    #[inline]
    /// Read a sector from a Controller Pak
    ///
    /// Rust: this version returns a [Vec].
    ///
    /// See [`read_mempak_sector`](libdragon_sys::read_mempak_sector) for details.
    pub fn read_sector(&self, sector: usize) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(BLOCK_SIZE);
        let v = unsafe { libdragon_sys::read_mempak_sector(self.port as i32, sector as i32, res.as_mut_ptr()) };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(res)
    }

    /// Read a sector from a Controller Pak
    ///
    /// Rust: this version requires a mutable slice of at least [BLOCK_SIZE] bytes.
    ///
    /// See [`read_mempak_sector`](libdragon_sys::read_mempak_sector) for details.
    #[inline]
    pub fn read_sector_into<T>(&self, sector: usize, dest: &mut [T]) -> Result<()> {
        let size = dest.len() * ::core::mem::size_of::<T>();
        assert!(size >= BLOCK_SIZE, "dest slice must be at least BLOCK_SIZE bytes");
        let v = unsafe { libdragon_sys::read_mempak_sector(self.port as i32, sector as i32, dest.as_ptr() as *mut _) };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(())
    }
    
    /// Write a sector to a Controller Pak
    ///
    /// See [`write_mempak_sector`](libdragon_sys::write_mempak_sector) for details.
    #[inline]
    pub fn write_sector<T>(&mut self, sector: usize, src: &mut [T]) -> Result<()> {
        let v = unsafe { libdragon_sys::write_mempak_sector(self.port as i32, sector as i32, src.as_ptr() as *mut _) };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(())
    }

    /// Return whether a Controller Pak is valid
    ///
    /// See [`validate_mempak`](libdragon_sys::validate_mempak) for details.
    #[inline]
    pub fn validate(&self) -> Result<()> {
        let v = unsafe { libdragon_sys::validate_mempak(self.port as i32) };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(())
    }

    /// Return the number of free blocks on a Controller Pak
    ///
    /// See [`get_mempak_free_space`](libdragon_sys::get_mempak_free_space) for details.
    #[inline]
    pub fn get_free_space(&self) -> Result<usize> {
        let v = unsafe { libdragon_sys::get_mempak_free_space(self.port as i32) };
        if v < 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(v as usize)
    }

    /// Read an entry on a Controller Pak
    ///
    /// See [`get_mempak_entry`](libdragon_sys::get_mempak_entry) for details.
    pub fn get_entry(&self, entry: usize) -> Result<Entry> {
        let mut tmp: core::mem::MaybeUninit<libdragon_sys::entry_structure_t> = core::mem::MaybeUninit::uninit();
        let v = unsafe {
            libdragon_sys::get_mempak_entry(self.port as i32, entry as i32, tmp.as_mut_ptr())
        };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(Entry { 
            port: self.port, 
            e: unsafe { tmp.assume_init() } 
        })
    }

    /// Format a Controller Pak
    ///
    /// See [`format_mempak`](libdragon_sys::format_mempak) for details.
    #[inline]
    pub fn format(&self) -> Result<()> {
        let v = unsafe { libdragon_sys::format_mempak(self.port as i32) };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(())
    }

}

/// Wrapper around [`entry_structure_t`](libdragon_sys::entry_structure_t).
pub struct Entry {
    port: usize,
    e: libdragon_sys::entry_structure_t,
}

impl Entry {
    /// Read the data associated with an entry on a Controller Pak
    ///
    /// See [`read_mempak_entry_data`](libdragon_sys::read_mempak_entry_data) for details.
    pub fn read_data(&mut self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(self.blocks() * BLOCK_SIZE);
        let v = unsafe {
            libdragon_sys::read_mempak_entry_data(self.port as i32, &mut self.e as *mut _, res.as_mut_ptr())
        };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(res)
    }

    /// Write associated data to a Controller Pak entry
    ///
    /// See [`write_mempak_entry_data`](libdragon_sys::write_mempak_entry_data) for details.
    pub fn write_data<T>(&mut self, src: &mut [T]) -> Result<()> {
        let required_size = self.blocks() * BLOCK_SIZE;
        let size = src.len() * ::core::mem::size_of::<T>();
        assert!(size >= required_size, "src data must be at least (blocks() * BLOCK_SIZE) bytes");
        let v = unsafe {
            libdragon_sys::write_mempak_entry_data(self.port as i32, &mut self.e as *mut _, src.as_mut_ptr() as *mut _)
        };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(())
    }

    /// Delete a Controller Pak entry and associated data
    ///
    /// See [`write_mempak_entry_data`](libdragon_sys::write_mempak_entry_data) for details.
    pub fn delete(&mut self) -> Result<()> {
        let v = unsafe {
            libdragon_sys::delete_mempak_entry(self.port as i32, &mut self.e as *mut _)
        };
        if v != 0 { return Err(LibDragonError::MemPakError { code: v }); }
        Ok(())
    }

    /// Access [`entry_structure_t.vendor`](libdragon_sys::entry_structure_t::vendor).
    #[inline(always)] pub fn vendor(&self) -> u32 { self.e.vendor }
    /// Access [`entry_structure_t.game_id`](libdragon_sys::entry_structure_t::game_id).
    #[inline(always)] pub fn game_id(&self) -> u16 { self.e.game_id }
    /// Access [`entry_structure_t.inode`](libdragon_sys::entry_structure_t::inode).
    #[inline(always)] pub fn inode(&self) -> u16 { self.e.inode }
    /// Access [`entry_structure_t.region`](libdragon_sys::entry_structure_t::region).
    #[inline(always)] pub fn region(&self) -> u8 { self.e.region }
    /// Access [`entry_structure_t.blocks`](libdragon_sys::entry_structure_t::blocks).
    #[inline(always)] pub fn blocks(&self) -> usize { self.e.blocks as usize }
    /// Access [`entry_structure_t.valid`](libdragon_sys::entry_structure_t::valid).
    #[inline(always)] pub fn valid(&self) -> bool { self.e.valid != 0 }
    /// Access [`entry_structure_t.entry_id`](libdragon_sys::entry_structure_t::entry_id).
    #[inline(always)] pub fn entry_id(&self) -> u8 { self.e.entry_id }
    /// Access [`entry_structure_t.name`](libdragon_sys::entry_structure_t::name).
    #[inline(always)] pub fn name(&self) -> Result<&str> { 
        let c_str = unsafe { CStr::from_ptr(self.e.name.as_ptr() as *const _) };
        Ok(c_str.to_str()?)
    }
}
