use crate::*;

/// Use if you're doing USB operations without the PI Manager (libultra only)
pub const USE_OSRAW: u32 = libdragon_sys::USE_OSRAW as u32;
/// Max size of USB I/O. The bigger this value, the more ROM you lose!
pub const DEBUG_ADDRESS_SIZE: u32 = libdragon_sys::DEBUG_ADDRESS_SIZE as u32;
/// Stops the USB library from working if it detects an emulator to prevent problems
pub const CHECK_EMULATOR: u32 = libdragon_sys::CHECK_EMULATOR as u32;

/// Cart definitions
pub enum Cart {
    None,
    _64Drive,
    EverDrive,
    Sc64
}

impl From<u32> for Cart {
    fn from(val: u32) -> Cart {
        match val {
            libdragon_sys::CART_NONE      => Cart::None,
            libdragon_sys::CART_64DRIVE   => Cart::_64Drive,
            libdragon_sys::CART_EVERDRIVE => Cart::EverDrive,
            libdragon_sys::CART_SC64      => Cart::Sc64,
            _ => panic!("invalid value"),
        }
    }
}

/// Data types defintions
pub enum DataType {
    Text,
    RawBinary,
    Header,
    Screenshot,
    Heartbeat,
    RdbPacket,
}

impl From<u32> for DataType {
    fn from(val: u32) -> DataType {
        match val {
            libdragon_sys::DATATYPE_TEXT        => DataType::Text,
            libdragon_sys::DATATYPE_RAWBINARY   => DataType::RawBinary,
            libdragon_sys::DATATYPE_HEADER      => DataType::Header,
            libdragon_sys::DATATYPE_SCREENSHOT  => DataType::Screenshot,
            libdragon_sys::DATATYPE_HEARTBEAT   => DataType::Heartbeat,
            libdragon_sys::DATATYPE_RDBPACKET   => DataType::RdbPacket,
            _ => panic!("invalid value"),
        }
    }
}

impl From<DataType> for u32 {
    fn from(v: DataType) -> Self {
        match v {
            DataType::Text          => libdragon_sys::DATATYPE_TEXT,
            DataType::RawBinary     => libdragon_sys::DATATYPE_RAWBINARY,
            DataType::Header        => libdragon_sys::DATATYPE_HEADER,
            DataType::Screenshot    => libdragon_sys::DATATYPE_SCREENSHOT,
            DataType::Heartbeat     => libdragon_sys::DATATYPE_HEARTBEAT,
            DataType::RdbPacket     => libdragon_sys::DATATYPE_RDBPACKET,
        }
    }
}

/// Initializes the USB buffers and pointers
///
/// See [`usb_initialize`](libdragon_sys::usb_initialize) for details.
#[inline]
pub fn initialize() -> Result<()> { 
    let v = unsafe { libdragon_sys::usb_initialize() };
    if v != 1 { return Err(LibDragonError::UsbError { code: v }) }
    Ok(())
}

/// Returns which flashcart is currently connected
///
/// See [`usb_getcart`](libdragon_sys::usb_getcart) for details.
#[inline]
pub fn getcart() -> Cart { unsafe { libdragon_sys::usb_getcart() as u32 }.into() }

/// Writes data to the USB
///
/// See [`usb_write`](libdragon_sys::usb_write) for details.
#[inline]
pub fn write<T>(datatype: DataType, t: &T) { 
    let size = ::core::mem::size_of::<T>();
    let datatype: u32 = datatype.into();
    unsafe { 
        libdragon_sys::usb_write(datatype as i32, t as *const T as *const _, size as i32);
    }
}

/// Returns the header of data being received via USB
///
/// See [`usb_poll`](libdragon_sys::usb_poll) for details.
#[inline]
pub fn poll() -> u32 { unsafe { libdragon_sys::usb_poll() } }

/// Reads bytes from USB into the provided buffer
///
/// See [`usb_read`](libdragon_sys::usb_read) for details.
#[inline]
pub fn read<T>(buffer: &mut [T]) { 
    let size = buffer.len() * ::core::mem::size_of::<T>();
    unsafe { 
        libdragon_sys::usb_read(buffer.as_mut_ptr() as *mut _, size as i32);
    }
}

/// Skips a USB read by the specified amount of bytes
///
/// See [`usb_skip`](libdragon_sys::usb_skip) for details.
#[inline]
pub fn skip(nbytes: usize) { unsafe { libdragon_sys::usb_skip(nbytes as i32); } }

/// Rewinds a USB read by the specified amount of bytes
///
/// See [`usb_rewind`](libdragon_sys::usb_rewind) for details.
#[inline]
pub fn rewind(nbytes: usize) { unsafe { libdragon_sys::usb_rewind(nbytes as i32); } }

/// Purges the incoming USB data
///
/// See [`usb_purge`](libdragon_sys::usb_purge) for details.
#[inline]
pub fn purge() { unsafe { libdragon_sys::usb_purge() } }

/// Checks if the USB timed out recently
///
/// See [`usb_timedout`](libdragon_sys::usb_timedout) for details.
#[inline]
pub fn timedout() -> bool { unsafe { libdragon_sys::usb_timedout() != 0 } }

/// Sends a heartbeat packet to the PC
///
/// See [`usb_sendheartbeat`](libdragon_sys::usb_sendheartbeat) for details.
#[inline]
pub fn sendheartbeat() { unsafe { libdragon_sys::usb_sendheartbeat(); } }

