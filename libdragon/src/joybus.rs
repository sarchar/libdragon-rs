use crate::*;

/// Size of a Joybus input/output block in bytes
///
/// See [`JOYBUS_BLOCK_SIZE`](libdragon_sys::JOYBUS_BLOCK_SIZE) for details.
pub const BLOCK_SIZE: u32 = libdragon_sys::JOYBUS_BLOCK_SIZE;
/// Size of a Joybux input/output block in double-words
pub const BLOCK_DWORDS: u32 = BLOCK_SIZE / (::core::mem::size_of::<u64>() as u32);
/// Size of a Joybus N64 accessory read/write payload in bytes.
///
/// See [`JOYBUS_ACCESSORY_DATA_SIZE`](libdragon_sys::JOYBUS_ACCESSORY_DATA_SIZE) for details.
pub const ACCESSORY_DATA_SIZE: u32 = libdragon_sys::JOYBUS_ACCESSORY_DATA_SIZE;
/// Count of Joybus ports.
///
/// See [`JOYBUS_PORT_COUNT`](libdragon_sys::JOYBUS_PORT_COUNT) for details.
pub const PORT_COUNT: u32 = libdragon_sys::JOYBUS_PORT_COUNT;
/// Joypad Identifier type
pub type JoybusIdentifier = u16;
/// Joybus identifier for an unknown or malfunctioning device.
///
/// See [`JOYBUS_IDENTIFIER_UNKNOWN`](libdragon_sys::JOYBUS_IDENTIFIER_UNKNOWN) for details.
pub const IDENTIFIER_UNKNOWN: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_UNKNOWN as JoybusIdentifier;
/// Joybus identifier for a port with no device connected.
///
/// See [`JOYBUS_IDENTIFIER_NONE`](libdragon_sys::JOYBUS_IDENTIFIER_NONE) for details.
pub const IDENTIFIER_NONE: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_NONE as JoybusIdentifier;
/// Joybus identifier for the Nintendo 64 voice recognition peripheral (NUS-020).
///
/// See [`JOYBUS_IDENTIFIER_N64_VOICE_RECOGNITION`](libdragon_sys::JOYBUS_IDENTIFIER_N64_VOICE_RECOGNITION) for details.
pub const IDENTIFIER_N64_VOICE_RECOGNITION: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_N64_VOICE_RECOGNITION as JoybusIdentifier;
/// Joybus identifier for the Nintendo 64 Randnet keyboard peripheral (RND-001)
///
/// See [`JOYBUS_IDENTIFIER_N64_RANDNET_KEYBOARD`](libdragon_sys::JOYBUS_IDENTIFIER_N64_RANDNET_KEYBOARD) for details.
pub const IDENTIFIER_N64_RANDNET_KEYBOARD: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_N64_RANDNET_KEYBOARD as JoybusIdentifier;
/// Joybus identifier for the unrealized 64GB Link Cable
///
/// See [`JOYBUS_IDENTIFIER_64GB_LINK_CABLE`](libdragon_sys::JOYBUS_IDENTIFIER_64GB_LINK_CABLE) for details.
pub const IDENTIFIER_64GB_LINK_CABLE: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_64GB_LINK_CABLE as JoybusIdentifier;
/// Joybus identifier for Game Boy Advance Link Cable (DOL-011).
///
/// See [`JOYBUS_IDENTIFIER_GBA_LINK_CABLE`](libdragon_sys::JOYBUS_IDENTIFIER_GBA_LINK_CABLE) for details.
pub const IDENTIFIER_GBA_LINK_CABLE: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_GBA_LINK_CABLE as JoybusIdentifier;
/// Joybus identifier for cartridge-based real-time clock.
///
/// See [`JOYBUS_IDENTIFIER_CART_RTC`](libdragon_sys::JOYBUS_IDENTIFIER_CART_RTC) for details.
pub const IDENTIFIER_CART_RTC: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_CART_RTC as JoybusIdentifier;
/// Joybus identifier for cartridge-based 4Kbit EEPROM save type.
///
/// See [`JOYBUS_IDENTIFIER_CART_EEPROM_4KBIT`](libdragon_sys::JOYBUS_IDENTIFIER_CART_EEPROM_4KBIT) for details.
pub const IDENTIFIER_CART_EEPROM_4KBIT: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_CART_EEPROM_4KBIT as JoybusIdentifier;
/// Joybus identifier for cartridge-based 16Kbit EEPROM save type.
///
/// See [`JOYBUS_IDENTIFIER_CART_EEPROM_16KBIT`](libdragon_sys::JOYBUS_IDENTIFIER_CART_EEPROM_16KBIT) for details.
pub const IDENTIFIER_CART_EEPROM_16KBIT: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_CART_EEPROM_16KBIT as JoybusIdentifier;
/// Joybus identifier for a standard Nintendo 64 controller (NUS-005).
///
/// See [`JOYBUS_IDENTIFIER_N64_CONTROLLER`](libdragon_sys::JOYBUS_IDENTIFIER_N64_CONTROLLER) for details.
pub const IDENTIFIER_N64_CONTROLLER: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_N64_CONTROLLER as JoybusIdentifier;
/// Joybus identifier for the Nintendo 64 mouse peripheral (NUS-017).
///
/// See [`JOYBUS_IDENTIFIER_N64_MOUSE`](libdragon_sys::JOYBUS_IDENTIFIER_N64_MOUSE) for details.
pub const IDENTIFIER_N64_MOUSE: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_N64_MOUSE as JoybusIdentifier;
/// Joybus identifier platform bitfield mask
///
/// See [`JOYBUS_IDENTIFIER_MASK_PLATFORM`](libdragon_sys::JOYBUS_IDENTIFIER_MASK_PLATFORM) for details.
pub const IDENTIFIER_MASK_PLATFORM: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_MASK_PLATFORM as JoybusIdentifier;
/// GameCube Joybus identifier playform value
///
/// See [`JOYBUS_IDENTIFIER_PLATFORM_GCN`](libdragon_sys::JOYBUS_IDENTIFIER_PLATFORM_GCN) for details.
pub const IDENTIFIER_PLATFORM_GCN: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_PLATFORM_GCN as JoybusIdentifier;
/// Joybus identifier GameCube standard controller flag.
///
/// See [`JOYBUS_IDENTIFIER_MASK_GCN_CONTROLLER`](libdragon_sys::JOYBUS_IDENTIFIER_MASK_GCN_CONTROLLER) for details.
pub const IDENTIFIER_MASK_GCN_CONTROLLER: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_MASK_GCN_CONTROLLER as JoybusIdentifier;
/// Joybus identifier GameCube rumble support flag.
///
/// See [`JOYBUS_IDENTIFIER_MASK_GCN_NORUMBLE`](libdragon_sys::JOYBUS_IDENTIFIER_MASK_GCN_NORUMBLE) for details.
pub const IDENTIFIER_MASK_GCN_NORUMBLE: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_MASK_GCN_NORUMBLE as JoybusIdentifier;
/// Joybus identifier GameCube wireless flag.
///
/// See [`JOYBUS_IDENTIFIER_MASK_GCN_WIRELESS`](libdragon_sys::JOYBUS_IDENTIFIER_MASK_GCN_WIRELESS) for details.
pub const IDENTIFIER_MASK_GCN_WIRELESS: JoybusIdentifier = libdragon_sys::JOYBUS_IDENTIFIER_MASK_GCN_WIRELESS as JoybusIdentifier;

/// Joybus identify status byte mask for N64 accessory presence values.
///
/// See [`JOYBUS_IDENTIFY_STATUS_ACCESSORY_MASK`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_MASK) for details.
pub const IDENTIFY_STATUS_ACCESSORY_MASK: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_MASK;
/// Joybus identify status for an N64 controller that does not support accessories.
///
/// See [`JOYBUS_IDENTIFY_STATUS_ACCESSORY_UNSUPPORTED`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_UNSUPPORTED) for details.
pub const IDENTIFY_STATUS_ACCESSORY_UNSUPPORTED: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_UNSUPPORTED;
/// Joybus identify status for an N64 controller with an accessory present.
///
/// See [`JOYBUS_IDENTIFY_STATUS_ACCESSORY_PRESENT`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_PRESENT) for details.
pub const IDENTIFY_STATUS_ACCESSORY_PRESENT: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_PRESENT;
/// Joybus identify status for an N64 controller with no accessory present.
///
/// See [`JOYBUS_IDENTIFY_STATUS_ACCESSORY_ABSENT`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_ABSENT) for details.
pub const IDENTIFY_STATUS_ACCESSORY_ABSENT: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_ABSENT;
/// Joybus identify status for an N64 controller with an accessory present that as changed since it
/// was last identified.
///
/// See [`JOYBUS_IDENTIFY_STATUS_ACCESSORY_CHANGED`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_CHANGED) for details.
pub const IDENTIFY_STATUS_ACCESSORY_CHANGED: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_ACCESSORY_CHANGED;
/// Joybus identify status bit for a VRU/VRS that is initialized and ready.
///
/// See [`JOYBUS_IDENTIFY_STATUS_VOICE_RECOGNITON_READY`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_VOICE_RECOGNITON_READY) for details.
pub const IDENTIFY_STATUS_VOICE_RECOGNITON_READY: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_VOICE_RECOGNITON_READY;
/// Joybus identify status bit that signifies the previous command had a checksum error.
pub const IDENTIFY_STATUS_COMMAND_CHECKSUM_ERROR: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_COMMAND_CHECKSUM_ERROR;
/// Joybus identify status for GameCube controllers that indicates whether the rumble motor is
/// currently active.
///
/// See [`JOYBUS_IDENTIFY_STATUS_GCN_RUMBLE_ACTIVE`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_GCN_RUMBLE_ACTIVE) for details.
pub const IDENTIFY_STATUS_GCN_RUMBLE_ACTIVE: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_GCN_RUMBLE_ACTIVE;
/// Joybus identify status bit for EEPROM devices that indicates a write is in-progress.
///
/// See [`JOYBUS_IDENTIFY_STATUS_EEPROM_BUSY`](libdragon_sys::JOYBUS_IDENTIFY_STATUS_EEPROM_BUSY) for details.
pub const IDENTIFY_STATUS_EEPROM_BUSY: u32 = libdragon_sys::JOYBUS_IDENTIFY_STATUS_EEPROM_BUSY;

/// Since Joybus commands are applied to a specific port, extend Joypad Ports with the joybus functionality:
impl joypad::Port {
    /// Execute a Joybus command synchronously on the given port.
    ///
    /// See [`joybus_exec_cmd`](libdragon_sys::joybus_exec_cmd) for details.
    #[inline]
    pub fn exec_cmd<T,U>(&mut self, send_data: &[T], recv_data: &mut [T]) {
        extern "C" {
            fn joybus_exec_cmd(port: ::core::ffi::c_int, send_len: ::core::ffi::c_uint, recv_len: ::core::ffi::c_uint, 
                               send_data: *const ::core::ffi::c_void, recv_data: *mut ::core::ffi::c_void);
        }
        let send_len = send_data.len() * ::core::mem::size_of::<T>();
        let recv_len = recv_data.len() * ::core::mem::size_of::<U>();
        unsafe {
            joybus_exec_cmd(self.port as i32, send_len as u32, recv_len as u32, 
                            send_data.as_ptr() as *const _, recv_data.as_mut_ptr() as *mut _);
        }
    }
}

/// Write a 64-byte block of data to the PIF and read the 64-byte result.
///
/// Experimental
///
/// See [`joybus_exec`](libdragon_sys::joybus_exec) for details.
#[inline]
pub unsafe fn exec<T,U>(inblock: &[T], outblock: &mut [U]) {
    libdragon_sys::joybus_exec(inblock.as_ptr() as *const _, outblock.as_mut_ptr() as *mut _);
}

/// Joybus accessory read/write status values
///
/// See [`joybus_accessory_io_status_t`](libdragon_sys::joybus_accessory_io_status_t) for details.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessoryIoStatus {
    Ok, NoDevice, NoPak, BadCrc
}

impl From<libdragon_sys::joybus_accessory_io_status_t> for AccessoryIoStatus {
    fn from(val: libdragon_sys::joybus_accessory_io_status_t) -> AccessoryIoStatus {
        match val { 
            libdragon_sys::joybus_accessory_io_status_t_JOYBUS_ACCESSORY_IO_STATUS_OK => AccessoryIoStatus::Ok,
            libdragon_sys::joybus_accessory_io_status_t_JOYBUS_ACCESSORY_IO_STATUS_NO_DEVICE => AccessoryIoStatus::NoDevice,
            libdragon_sys::joybus_accessory_io_status_t_JOYBUS_ACCESSORY_IO_STATUS_NO_PAK => AccessoryIoStatus::NoPak,
            libdragon_sys::joybus_accessory_io_status_t_JOYBUS_ACCESSORY_IO_STATUS_BAD_CRC => AccessoryIoStatus::BadCrc,
            _ => panic!("invalid value"),
        }
    }
}

/// Extend [joypad::Port] with Joybus accessory functions
impl joypad::Port {
    /// Synchronously perform a Joybus N64 accessory read command.
    ///
    /// See [`joybus_accessory_read`](libdragon_sys::joybus_accessory_read) for details.
    #[inline]
    pub fn accessory_read(&mut self, addr: u16) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(32);
        let r: AccessoryIoStatus = unsafe {
            libdragon_sys::joybus_accessory_read(self.port as i32, addr, res.as_mut_ptr() as *mut _).into()
        };
        match r {
            AccessoryIoStatus::Ok => Ok(res),
            e @ _ => Err(LibDragonError::AccessoryIoError{ error: e }),
        }
    }

    /// Synchronously perform a Joybus N64 accessory write command.
    ///
    /// See [`joybus_accessory_write`](libdragon_sys::joybus_accessory_write) for details.
    #[inline]
    pub fn accessory_write(&mut self, addr: u16, data: &[u8]) -> Result<()> {
        assert!(data.len() >= 32, "data block must be 32 bytes of data");
        let r: AccessoryIoStatus = unsafe {
            libdragon_sys::joybus_accessory_write(self.port as i32, addr, data.as_ptr() as *const _).into()
        };
        match r {
            AccessoryIoStatus::Ok => Ok(()),
            e @ _ => Err(LibDragonError::AccessoryIoError{ error: e }),
        }
    }
}
