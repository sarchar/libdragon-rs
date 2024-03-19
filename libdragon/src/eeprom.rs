use crate::*;

pub const BLOCK_SIZE: u32 = libdragon_sys::EEPROM_BLOCK_SIZE;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EepromType {
    None, _4K, _16K,
}

impl From<libdragon_sys::eeprom_type_t> for EepromType {
    fn from(val: libdragon_sys::eeprom_type_t) -> EepromType {
        match val {
            libdragon_sys::eeprom_type_t_EEPROM_NONE => EepromType::None,
            libdragon_sys::eeprom_type_t_EEPROM_4K => EepromType::_4K,
            libdragon_sys::eeprom_type_t_EEPROM_16K => EepromType::_16K,
            _ => panic!("invalid value"),
        }
    }
}

/// Probe the EEPROM interface on the cartridge
///
/// See [`eeprom_present`](libdragon_sys::eeprom_present) for details.
#[inline] pub fn present() -> EepromType { unsafe { libdragon_sys::eeprom_present().into() } }

/// Determine how many blocks of EEPROM exist on the cartridge
///
/// See [`eeprom_total_blocks`](libdragon_sys::eeprom_total_blocks) for details
#[inline] pub fn total_blocks() -> usize { unsafe { libdragon_sys::eeprom_total_blocks() as usize } }

/// Read a block from EEPROM
///
/// See [`eeprom_read`](libdragon_sys::eeprom_read) for details
#[inline] pub fn read(block: usize) -> Vec<u8> { 
    let mut ret = Vec::with_capacity(8);
    unsafe { 
        libdragon_sys::eeprom_read(block as u8, ret.as_mut_ptr());
    } 
    ret
}

/// Read a block from EEPROM
///
/// Rust: read the data into the given slice.
///
/// See [`eeprom_read`](libdragon_sys::eeprom_read) for details
#[inline] pub fn read_into(block: usize, dest: &mut [u8]) { 
    unsafe { 
        libdragon_sys::eeprom_read(block as u8, dest.as_mut_ptr());
    } 
}

/// Write a block from EEPROM
///
/// See [`eeprom_write`](libdragon_sys::eeprom_write) for details
#[inline] pub fn write(block: usize, src: &[u8]) -> u8 { unsafe { libdragon_sys::eeprom_write(block as u8, src.as_ptr()) } }

/// Read a buffer of bytes from EEPROM.
///
/// See [`eeprom_read_bytes`](libdragon_sys::eeprom_read_bytes) for details
#[inline] pub fn read_bytes(start: usize, len: usize) -> Vec<u8> {
    let mut ret = Vec::with_capacity(len);
    unsafe {
        libdragon_sys::eeprom_read_bytes(ret.as_mut_ptr(), start, len);
    }
    ret
}

/// Read a buffer of bytes from EEPROM.
///
/// Rust: read the data into the given slice. The length of the data read is the length of the
/// provided slice.
///
/// See [`eeprom_read_bytes`](libdragon_sys::eeprom_read_bytes) for details
#[inline] pub fn read_bytes_into(dest: &mut [u8], start: usize) {
    unsafe {
        libdragon_sys::eeprom_read_bytes(dest.as_mut_ptr(), start, dest.len());
    }
}

/// Write a buffer of bytes to EEPROM.
///
/// See [`eeprom_write_bytes`](libdragon_sys::eeprom_write_bytes) for details
#[inline] pub fn write_bytes(src: &[u8], start: usize) {
    unsafe {
        libdragon_sys::eeprom_write_bytes(src.as_ptr(), start, src.len());
    }
}


