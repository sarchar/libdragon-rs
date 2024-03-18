
/// Structure for storing RTC time data.
///
/// Wrapper around [`rtc_time_t`](libdragon_sys::rtc_time_t).
///
/// This struct has the same data layout as the C struct.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Time {
    /// Year. [1900-20XX]
    year: u16,
    /// Month. [0-11]
    month: u8,
    /// Day of month. [1-31]
    day: u8,
    /// Hours. [0-23]
    hour: u8,
    /// Minutes. [0-59]
    min: u8,
    /// Seconds. [0-59]
    sec: u8,
    /// Day of week. [0-6] (Sun-Sat)
    week_day: u8,
}

impl Time {
    /// High-level convenience helper to get the RTC date/time.
    ///
    /// See [`rtc_Get`](libdragon_sys::rtc_Get) for details.
    pub fn get() -> Option<Time> {
        let mut t: core::mem::MaybeUninit<libdragon_sys::rtc_time_t> = core::mem::MaybeUninit::uninit();
        let r = unsafe {
            libdragon_sys::rtc_get(t.as_mut_ptr())
        };
        if !r { return None; }
        Some(unsafe {
            core::mem::transmute(t.assume_init())
        })
    }

    /// High-level convenience helper to set the RTC date/time.
    ///
    /// See [`rtc_set`](libdragon_sys::rtc_set) for details.
    #[inline]
    pub fn set(&mut self) -> Option<()> {
        let r = unsafe {
            libdragon_sys::rtc_set(core::mem::transmute(self as *mut _))
        };
        if !r { return None; }
        Some(())
    }

    /// Calculate sane values for arbitrary time inputs.
    ///
    /// See [`rtc_normalize_time`](libdragon_sys::rtc_normalize_time) for details.
    #[inline]
    pub fn normalize(&mut self) { unsafe { libdragon_sys::rtc_normalize_time(core::mem::transmute(self as *mut _)) } }
}

/// High-level convenience helper to initialize the RTC subsystem.
///
/// See [`rtc_init`](libdragon_sys::rtc_init) for details.
#[inline] pub fn init() -> bool { unsafe { libdragon_sys::rtc_init() } }

/// Close the RTC Subsystem, disabling system hooks.
///
/// See [`rtc_close`](libdragon_sys::rtc_close) for details.
#[inline] pub fn close() { unsafe { libdragon_sys::rtc_close(); } }

/// Determine whether the RTC supports writing the time.
///
/// See [`rtc_is_writable`](libdragon_sys::rtc_is_writable) for details.
#[inline] pub fn is_writable() -> bool { unsafe { libdragon_sys::rtc_is_writable() } }

