use crate::*;

// rspq.h
pub fn init() {
    unsafe {
        libdragon_sys::rspq_init();
    }
}

pub fn close() {
    unsafe {
        libdragon_sys::rspq_close();
    }
}

extern "C" {
    fn rspq_write_begin_r(w: *mut libdragon_sys::rspq_write_t, ovl_id: ::core::ffi::c_uint, cmd_id: ::core::ffi::c_uint, size: ::core::ffi::c_int);
    fn rspq_write_arg_r(w: *mut libdragon_sys::rspq_write_t, value: ::core::ffi::c_uint);
    fn rspq_write_end_r(w: *mut libdragon_sys::rspq_write_t);
}

#[derive(Debug)]
pub struct Writer {
    w: Option<libdragon_sys::rspq_write_t>,
}

impl Writer {
    pub fn begin(ovl_id: u32, cmd_id: u32, size: usize) -> Self {
        let mut data: core::mem::MaybeUninit<libdragon_sys::rspq_write_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            rspq_write_begin_r(data.as_mut_ptr(), ovl_id, cmd_id, size as ::core::ffi::c_int);
        }
        Self {
            w: Some(unsafe { data.assume_init() }),
        }
    }

    pub fn arg(&mut self, value: u32) -> &Self {
        unsafe {
            rspq_write_arg_r(self.w.as_mut().unwrap() as *mut _, value);
        }
        self
    }

    pub fn end(&mut self) {
        unsafe {
            rspq_write_end_r(self.w.as_mut().unwrap() as *mut _);
        }
        self.w = None;
    }
}

impl Drop for Writer {
	fn drop(&mut self) {
        assert!(self.w.is_none(), "forgot to call Writer::end()?");
	}
}

/// rspq overlays

/// Extend RspUcode with rspq support
impl rsp::RspUcode {
    pub fn register(&mut self) -> Result<()> {
        let id = unsafe {
            libdragon_sys::rspq_overlay_register(self.rsp_ucode.as_mut().get_mut() as *mut libdragon_sys::rsp_ucode_t)
        };

        self.overlay_id = Some(id);
        Ok(())
    }

    pub fn register_static(&mut self, id: u32) -> Result<()> {
        unsafe {
            libdragon_sys::rspq_overlay_register_static(self.rsp_ucode.as_mut().get_mut() as *mut libdragon_sys::rsp_ucode_t, id as ::core::ffi::c_uint);
        }

        self.overlay_id = Some(id);
        Ok(())
    }

    pub fn unregister(&mut self) {
        unsafe {
            libdragon_sys::rspq_overlay_unregister(self.overlay_id.unwrap() as ::core::ffi::c_uint);
        }
        self.overlay_id = None;
    }

    #[inline(always)]
    pub fn is_registered(&self) -> bool { self.overlay_id.is_some() }

    #[inline(always)]
    pub fn id(&self) -> Option<u32> { self.overlay_id }

    /// The caller must know the size of the state block
    /// The slice is returned in the cached rdram segment, so if you need uncached access
    /// call \[T\].uncached_mut().
    #[inline(always)]
    pub fn get_state_mut<'a, T>(&'a mut self, count: usize) -> &'a mut [T] {
        unsafe {
            let ptr = libdragon_sys::rspq_overlay_get_state(self.rsp_ucode.as_mut().get_mut() as *mut libdragon_sys::rsp_ucode_t);
            ::core::slice::from_raw_parts_mut(ptr as *mut _, count)
        }
    }
}

#[derive(Debug)]
pub struct Block {
    ptr: *mut libdragon_sys::rspq_block_t,
}

impl Block {
    pub fn run(&self) {
        unsafe {
            libdragon_sys::rspq_block_run(self.ptr);
        }
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe {
            libdragon_sys::rspq_block_free(self.ptr);
        }
    }
}

pub fn block_begin() {
    unsafe {
        libdragon_sys::rspq_block_begin();
    }
}

pub fn block_end() -> Block {
    let block_ptr = unsafe {
        libdragon_sys::rspq_block_end()
    };

    Block {
        ptr: block_ptr,
    }
}

pub fn wait() {
    unsafe {
        libdragon_sys::rspq_wait();
    }
}

// rspq_constants.h
const MAX_OVERLAY_COUNT: usize = 8;
const PROFILE_SLOT_COUNT: usize = MAX_OVERLAY_COUNT + 6;

// rspq_profile.h

/// Profiling data of a single slot (for example an overlay)
///
/// See [`rspq_profile_slot_t`](libdragon_sys::rspq_profile_slot_t) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProfileSlot {
    /// The total number of rcp ticks that were spent running in this slot
    /// See [`rspq_profile_slot_t.total_ticks`](libdragon_sys::rspq_profile_slot_t::total_ticks)
    pub total_ticks : u64,
    /// The number of individual samples that were recorded
    /// See [`rspq_profile_slot_t.sample_count`](libdragon_sys::rspq_profile_slot_t::sample_count)
    pub sample_count: u64,
    #[doc(hidden)]
    name            : *const ::core::ffi::c_char,
}

impl Default for ProfileSlot {
    fn default() -> Self {
        Self {
            total_ticks : 0,
            sample_count: 0,
            name        : ::core::ptr::null(),
        }
    }
}

impl ProfileSlot {
    /// The name of this slot, if it is used.
    /// See [`rspq_profile_slot_t.name`](libdragon_sys::rspq_profile_slot_t::name)
    pub fn name(&self) -> &str {
        let c_str = unsafe { CStr::from_ptr(self.name) };
        c_str.to_str().unwrap()
    }
}

/// RSPQ Profiling data
///
/// See [`rspq_profile_data_t`](libdragon_sys::rspq_profile_data_t) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ProfileData {
    /// The list of slots
    /// See [`rspq_profile_data_t.slots`](libdragon_sys::rspq_profile_data_t::slots)
    pub slots         : [ProfileSlot; PROFILE_SLOT_COUNT],
    /// The total elapsed rcp ticks since the last reset
    /// See [`rspq_profile_data_t.total_ticks`](libdragon_sys::rspq_profile_data_t::total_ticks)
    pub total_ticks   : u64,
    /// The accumulated ticks sampled from DP_BUSY
    /// See [`rspq_profile_data_t.rdp_busy_ticks`](libdragon_sys::rspq_profile_data_t::rdp_busy_ticks)
    pub rdp_busy_ticks: u64,
    /// The number of recorded frames since the last reset
    /// See [`rspq_profile_data_t.frame_count`](libdragon_sys::rspq_profile_data_t::frame_count)
    pub frame_count   : u64,
}

/// Start the rspq profiler. See [`rspq_profile_start`](libdragon_sys::rspq_profile_start).
pub fn profile_start() { unsafe { libdragon_sys::rspq_profile_start(); } }

/// Stop the rspq profiler. See [`rspq_profile_stop`](libdragon_sys::rspq_profile_stop).
pub fn profile_stop() { unsafe { libdragon_sys::rspq_profile_stop(); } }

/// Reset the rspq profiler and discard any recorded samples. See [`rspq_profile_reset`](libdragon_sys::rspq_profile_reset).
pub fn profile_reset() { unsafe { libdragon_sys::rspq_profile_reset(); } }

/// Mark the start of the next frame to the rspq profiler. See [`rspq_profile_next_frame`](libdragon_sys::rspq_profile_next_frame).
pub fn profile_next_frame() { unsafe { libdragon_sys::rspq_profile_next_frame(); } }

/// Dump the recorded data to the console. See [`rspq_profile_dump`](libdragon_sys::rspq_profile_dump).
pub fn profile_dump() { unsafe { libdragon_sys::rspq_profile_dump(); } }

/// Copy the recorded data. See [`rspq_profile_get_data`](libdragon_sys::rspq_profile_get_data).
pub fn profile_get_data() -> ProfileData {
    let mut data = Box::new(ProfileData::default());
    unsafe {
        libdragon_sys::rspq_profile_get_data(
            core::mem::transmute::<_, *mut libdragon_sys::rspq_profile_data_t>(data.as_mut() as *mut _)
        );
    }
    *data.as_ref()
}
