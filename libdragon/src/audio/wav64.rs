use crate::*;

/// WAV64 structure
///
/// See [`struct wav64_t`](libdragon_sys::wav64_t) for details.
pub struct Wav64 {
    ptr: *mut libdragon_sys::wav64_t,
    backing_instance: Option<core::pin::Pin<Box<libdragon_sys::wav64_t>>>,
}

impl Wav64 {
    /// Create a Wav64 object for playback
    ///
    /// See [`wav64_open`](libdragon_sys::wav64_open) for details.
    pub fn open(path: dfs::DfsPathBuf) -> Result<Self> {
        let path_bytes: &[u8] = path.as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let mut obj: core::mem::MaybeUninit<libdragon_sys::wav64_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            libdragon_sys::wav64_open(obj.as_mut_ptr(), cpath.as_ptr());
        }

        let mut backing_instance = Box::pin(unsafe { obj.assume_init() });
        Ok(Self {
            ptr: backing_instance.as_mut().get_mut(),
            backing_instance: Some(backing_instance),
        })
    }

    /// Configure a Wav64 for looping playback
    ///
    /// See [`wav64_set_loop`](libdragon_sys::wav64_set_loop) for details.
    pub fn set_loop(&mut self, loop_: bool) {
        unsafe {
            libdragon_sys::wav64_set_loop(self.ptr, loop_);
        }
    }

    /// Start playing a Wav64
    ///
    /// See [`wav64_play`](libdragon_sys::wav64_play) for details.
    pub fn play(&mut self, ch: i32) {
        unsafe {
            libdragon_sys::wav64_play(self.ptr, ch);
        }
    }

    /// Get the (possibly compressed) bitrate of the Wav64 file
    ///
    /// See [`wav64_get_bitrate`](libdragon_sys::wav64_get_bitrate) for details.
    pub fn get_bitrate(&mut self) -> i32 {
        unsafe {
            libdragon_sys::wav64_get_bitrate(self.ptr)
        }
    }

    /// Access the [Waveform](crate::audio::mixer::Waveform) for this Wav64.
    ///
    /// See [`struct wav64_t`](libdragon_sys::wav64_t::wave) for details.
    pub fn wave(&mut self) -> audio::mixer::Waveform {
        audio::mixer::Waveform::from_ptr(unsafe { &mut (*self.ptr).wave })
    }

    pub fn rom_addr(&self) -> u32 { unsafe { (*self.ptr).rom_addr } }
}

impl Drop for Wav64 {
    /// Uses [`wav64_close`](libdragon_sys::wav64_close).
    fn drop(&mut self) {
        if let Some(_) = core::mem::replace(&mut self.backing_instance, None) {
            unsafe {
                libdragon_sys::wav64_close(self.ptr);
                self.ptr = core::ptr::null_mut();
            }
        }
    }
}
