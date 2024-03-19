use crate::*;

// rspq.h

/// Maximum size of a command (in 32-bit words).
pub const MAX_COMMAND_SIZE: usize = libdragon_sys::RSPQ_MAX_COMMAND_SIZE as usize;

/// Maximum size of a command that it is writable with [Writer]
pub const MAX_SHORT_COMMAND_SIZE: usize = libdragon_sys::RSPQ_MAX_SHORT_COMMAND_SIZE as usize;

/// Initialize the RSPQ library.
///
/// See [`rspq_init`](libdragon_sys::rspq_init) for details.
#[inline] pub fn init() { unsafe { libdragon_sys::rspq_init(); } }

/// Shut down the RSPQ library.
///
/// See [`rspq_close`](libdragon_sys::rspq_close) for details.
#[inline] pub fn close() { unsafe { libdragon_sys::rspq_close(); } }

// rspq overlays

/// Extend RspUcode with rspq support
impl rsp::RspUcode {
    /// Register a rspq overlay into the RSP queue engine
    ///
    /// See [`rspq_overlay_register`](libdragon_sys::rspq_overlay_register) for details.
    pub fn register(&mut self) -> Result<()> {
        let id = unsafe {
            libdragon_sys::rspq_overlay_register(self.rsp_ucode.as_mut().get_mut() as *mut libdragon_sys::rsp_ucode_t)
        };

        self.overlay_id = Some(id);
        Ok(())
    }

    /// Register an overlay into the RSP queue engine assigning a static ID to it
    ///
    /// See [`rspq_overlay_register_static`](libdragon_sys::rspq_overlay_register_static) for
    /// details.
    pub fn register_static(&mut self, id: u32) -> Result<()> {
        unsafe {
            libdragon_sys::rspq_overlay_register_static(self.rsp_ucode.as_mut().get_mut() as *mut libdragon_sys::rsp_ucode_t, id as ::core::ffi::c_uint);
        }

        self.overlay_id = Some(id);
        Ok(())
    }

    /// Unregister a ucode overlay from the RSP queue engine
    ///
    /// See [`rspq_overlay_unregister`](libdragon_sys::rspq_overlay_unregister) for details.
    pub fn unregister(&mut self) {
        unsafe {
            libdragon_sys::rspq_overlay_unregister(self.overlay_id.unwrap() as ::core::ffi::c_uint);
        }
        self.overlay_id = None;
    }

    /// Return a slice to the overlay state (in RDRAM)
    ///
    /// Rust: The caller must know the size of the state block
    /// The slice is returned in the cached rdram segment, so if you need uncached access
    /// call \[T\].uncached_mut().
    ///
    /// See [`rspq_overlay_get_state`](libdragon_sys::rspq_overlay_get_state) for details.
    #[inline(always)]
    pub fn get_state_mut<'a, T>(&'a mut self, count: usize) -> &'a mut [T] {
        unsafe {
            let ptr = libdragon_sys::rspq_overlay_get_state(self.rsp_ucode.as_mut().get_mut() as *mut libdragon_sys::rsp_ucode_t);
            ::core::slice::from_raw_parts_mut(ptr as *mut _, count)
        }
    }

    /// Returns true if this ucode is registered
    #[inline(always)]
    pub fn is_registered(&self) -> bool { self.id().is_some() }

    /// Returns the overlay ID of the overlay, if registered
    #[inline(always)]
    pub fn id(&self) -> Option<u32> { self.overlay_id }

}


/// This Writer struct provides functionality to write commands into the RSP code.
///
/// Begin by calling [Writer::begin].  Call [Writer::arg] for each argument (including the 0th),
/// and when finished call [Writer::end].  [arg](Writer::arg) calls can be chained.
/// 
/// See the example program `rspqdemo` for an example usage.
#[derive(Debug)]
pub struct Writer {
    w: Option<libdragon_sys::rspq_write_t>,
}

impl Writer {
    /// Begin writing a new command into the RSP queue.
    ///
    /// See [`rspq_write_begin`](libdragon_sys::rspq_write_begin) for details.
    #[inline]
    pub fn begin(ovl_id: u32, cmd_id: u32, size: usize) -> Self {
        extern "C" {
            fn rspq_write_begin_r(w: *mut libdragon_sys::rspq_write_t, ovl_id: ::core::ffi::c_uint, cmd_id: ::core::ffi::c_uint, size: ::core::ffi::c_int);
        }
        let mut data: core::mem::MaybeUninit<libdragon_sys::rspq_write_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            rspq_write_begin_r(data.as_mut_ptr(), ovl_id, cmd_id, size as ::core::ffi::c_int);
        }
        Self {
            w: Some(unsafe { data.assume_init() }),
        }
    }

    /// Add one argument to the command being enqueued.
    ///
    /// See [`rspq_write_arg`](libdragon_sys::rspq_write_arg) for details.
    #[inline]
    pub fn arg(&mut self, value: u32) -> &Self {
        extern "C" {
            fn rspq_write_arg_r(w: *mut libdragon_sys::rspq_write_t, value: ::core::ffi::c_uint);
        }
        unsafe {
            rspq_write_arg_r(self.w.as_mut().unwrap() as *mut _, value);
        }
        self
    }

    /// Finish enqueuing a command into the queue.
    ///
    /// See [`rspq_write_end`](libdragon_sys::rspq_write_end) for details.
    #[inline]
    pub fn end(&mut self) {
        extern "C" {
            fn rspq_write_end_r(w: *mut libdragon_sys::rspq_write_t);
        }
        unsafe {
            rspq_write_end_r(self.w.as_mut().unwrap() as *mut _);
        }
        self.w = None;
    }
}

impl Drop for Writer {
    /// Verifies that [Writer::end] has been called.
	fn drop(&mut self) {
        assert!(self.w.is_none(), "forgot to call Writer::end()?");
	}
}

/// Make sure that RSP starts executing up to the last written command.
///
/// See [`rspq_flush`](libdragon_sys::rspq_flush) for details.
#[inline] pub fn flush() { unsafe { libdragon_sys::rspq_flush(); } }

/// Wait until all commands in the queue have been executed by RSP.
///
/// See [`rspq_wait`](libdragon_sys::rspq_wait) for details.
#[inline] pub fn wait() { unsafe { libdragon_sys::rspq_wait(); } }

/// A syncpoint in the queue. A wrapper around
/// [`rspq_syncpoint_t`](libdragon_sys::rspq_syncpoint_t).
pub struct SyncPoint(libdragon_sys::rspq_syncpoint_t);

impl SyncPoint {
    /// Create a syncpoint in the queue.
    ///
    /// See [`rspq_syncpoint_new`](libdragon_sys::rspq_syncpoint_new) for details.
    #[inline] pub fn new() -> Self {
        let v = unsafe { libdragon_sys::rspq_syncpoint_new() };
        Self(v)
    }

    /// Create a syncpoint in the queue that triggers a callback on the CPU.
    ///
    /// See [`rspq_syncpoint_new_cb`](libdragon_sys::rspq_syncpoint_new_cb) for details.
    #[inline] pub fn new_cb(cb: SyncpointCallback) -> Self {
        let cb = Box::new(SyncpointCallbackInternal { user_callback: cb });
        let v = unsafe {
            let ctx: *mut SyncpointCallbackInternal = Box::leak(cb); // Leak the function callback to prevent
                                                                     // memory from being freed
            libdragon_sys::rspq_syncpoint_new_cb(Some(SyncPoint::syncpoint_callback), ctx as *mut _)
        };
        Self(v)
    }

    extern "C" fn syncpoint_callback(ctx: *mut ::core::ffi::c_void) {
        // in this function, ctx must be valid
        let cb = unsafe {
            let ctx: *mut SyncpointCallbackInternal = ctx as *mut _;
            Box::from_raw(ctx)
        };

        // call user code
        (cb.user_callback)();

        // let the box drop and free memory
    }

    /// Check whether a syncpoint was reached by RSP or not.
    /// 
    /// See [`rspq_syncpoint_check`](libdragon_sys::rspq_syncpoint_check) for details.
    #[inline] pub fn check(&self) -> bool { unsafe { libdragon_sys::rspq_syncpoint_check(self.0) } }

    /// Wait until a syncpoint is reached by RSP.
    ///
    /// See [`rspq_syncpoint_wait`](libdragon_sys::rspq_syncpoint_wait) for details.
    #[inline] pub fn wait(&self) { unsafe { libdragon_sys::rspq_syncpoint_wait(self.0) } }

    /// Enqueue a callback to be called by the CPU
    ///
    /// See [`rspq_call_deferred`](libdragon_sys::rspq_call_deferred) for details.
    #[inline] pub fn call_deferred(cb: SyncpointCallback) {
        let _ = Self::new_cb(cb);
        flush();
    }
}

type SyncpointCallback = Box<dyn Fn() + 'static + Sync + Send>;
struct SyncpointCallbackInternal {
    user_callback: SyncpointCallback,
}

/// A preconstructed block of commands
///
/// See [`rspq_block_t](libdragon_sys::rspq_block_t) for details.
#[derive(Debug)]
pub struct Block {
    ptr: *mut libdragon_sys::rspq_block_t,
}

impl Block {
    /// Begin creating a new block
    ///
    /// See [`rspq_block_begin`](libdragon_sys::rspq_block_begin) for details.
    pub fn begin() { unsafe { libdragon_sys::rspq_block_begin(); } }
    
    /// Finish creating a block
    ///
    /// Rust: this function returns a [Block] struct that frees the underlying RSPQ block
    /// when the struct is dropped.
    ///
    /// See [`rspq_block_end`](libdragon_sys::rspq_block_end) for details.
    pub fn end() -> Self {
        let block_ptr = unsafe { libdragon_sys::rspq_block_end() };
        Self { ptr: block_ptr }
    }

    /// Add to the RSP queue a command that runs a block
    ///
    /// See [`rspq_block_run`](libdragon_sys::rspq_block_run) for details.
    pub fn run(&self) { unsafe { libdragon_sys::rspq_block_run(self.ptr); } }
}

impl Drop for Block {
    /// Free a block that is not needed any more.
    ///
    /// See [`rspq_block_free`](libdragon_sys::rspq_block_free) for details.
    fn drop(&mut self) {
        unsafe {
            libdragon_sys::rspq_block_free(self.ptr);
        }
    }
}

/// Start building a high-priority queue.
///
/// Rust: this function requires calling [highpri_end] when finished. It's recommended to use the
/// [highpri!] macro instead.
///
/// See [`rspq_highpri_begin`](libdragon_sys::rspq_highpri_begin) for details.
#[inline] pub fn highpri_begin() { unsafe { libdragon_sys::rspq_highpri_begin(); } }

/// Finish building a high-priority queue and close it.
///
/// See [`rspq_highpri_end`](libdragon_sys::rspq_highpri_end) for details.
#[inline] pub fn highpri_end() { unsafe { libdragon_sys::rspq_highpri_end(); } }

/// Wait for the RSP to finish processing all the high-priority queues.
///
/// See [`rspq_highpri_sync`](libdragon_sys::rspq_highpri_sync) for details.
#[inline] pub fn highpri_sync() { unsafe { libdragon_sys::rspq_highpri_sync(); } }

/// Execute rsqp functions in a high-priority queue, the Rust way. Usage:
///
/// ```rust
///     use libdragon::rspq;
///     ...
///     let block = rspq::Block::end();
///     ...
///     rspq::highpri! {
///         block.run();
///     };
/// ```
#[macro_export]
macro_rules! rspq_highpri {
    ($($s:stmt;)*) => {
        $crate::rspq::highpri_begin();
        $($s;)*
        $crate::rspq::highpri_end();
    };
}

/// Enqueue a no-op command in the queue.
///
/// See [`rspq_noop`](libdragon_sys::rspq_noop) for details.
#[inline] pub fn noop() { unsafe { libdragon_sys::rspq_noop(); } }

/// Enqueue a command that sets a signal in SP status
///
/// See [`rspq_signal`](libdragon_sys::rspq_signal) for details.
#[inline] pub fn signal(i: u32) { unsafe { libdragon_sys::rspq_signal(i as u32); } }

/// Enqueue a command to do a DMA transfer from DMEM to RDRAM
///
/// Rust: the number of bytes transfered is the slice length times the element size.
///
/// See [`rspq_dma_to_rdram`](libdragon_sys::rspq_dma_to_rdram) for details.
#[inline] pub fn dma_to_rdram<T>(rdram_addr: &mut [T], dmem_addr: u32, is_async: bool) { 
    let len = rdram_addr.len() * ::core::mem::size_of::<T>();
    assert!((len & 7) == 0, "size transfered must be a multiple of 8");
    unsafe { 
        libdragon_sys::rspq_dma_to_rdram(rdram_addr.as_mut_ptr() as *mut _, dmem_addr, len as u32, is_async);
    } 
}

/// Enqueue a command to do a DMA transfer from RDRAM to DMEM
///
/// Rust: the number of bytes transfered is the slice length times the element size.
///
/// See [`rspq_dma_to_dmem`](libdragon_sys::rspq_dma_to_dmem) for details.
#[inline] pub fn dma_to_dmem<T>(dmem_addr: u32, rdram_addr: &mut [T], is_async: bool) { 
    let len = rdram_addr.len() * ::core::mem::size_of::<T>();
    assert!((len & 7) == 0, "size transfered must be a multiple of 8");
    unsafe { 
        libdragon_sys::rspq_dma_to_dmem(dmem_addr, rdram_addr.as_mut_ptr() as *mut _, len as u32, is_async);
    } 
}

// rspq_constants.h
pub mod consts {
    pub const DEBUG: bool = libdragon_sys::RSPQ_DEBUG != 0;
    pub const PROFILE: bool = libdragon_sys::RSPQ_PROFILE != 0;

    /// Size of each RSPQ RDRAM buffer for lowpri queue (in 32-bit words)
    pub const DRAM_LOWPRI_BUFFER_SIZE: usize = libdragon_sys::RSPQ_DRAM_LOWPRI_BUFFER_SIZE as usize;
    /// Size of each RSPQ RDRAM buffer for highpri queue (in 32-bit words)
    pub const DRAM_HIGHPRI_BUFFER_SIZE: usize = libdragon_sys::RSPQ_DRAM_HIGHPRI_BUFFER_SIZE as usize;

    /// Size of the RSPQ DMEM buffer (in bytes)
    pub const DMEM_BUFFER_SIZE: usize = libdragon_sys::RSPQ_DMEM_BUFFER_SIZE as usize;

    /// Number of overlay ID bits (0-F)
    pub const RSPQ_OVERLAY_ID_BITS: usize = libdragon_sys::RSPQ_OVERLAY_ID_BITS as usize;
    /// Number of overlay command bits
    pub const OVERLAY_CMD_BITS: usize = libdragon_sys::RSPQ_OVERLAY_CMD_BITS as usize;
    /// Maximum number of overlays that can be registered
    pub const MAX_OVERLAYS: usize = libdragon_sys::RSPQ_MAX_OVERLAYS as usize;

    pub const MAX_OVERLAY_COMMAND_COUNT: usize = libdragon_sys::RSPQ_MAX_OVERLAY_COMMAND_COUNT as usize;

    /// Internal overlay header size in bytes
    pub const OVERLAY_HEADER_SIZE: usize = libdragon_sys::RSPQ_OVERLAY_HEADER_SIZE as usize;

    /// Minimum / maximum size of a block's chunk (contiguous memory buffer)
    pub const BLOCK_MIN_SIZE: usize = libdragon_sys::RSPQ_BLOCK_MIN_SIZE as usize;
    pub const BLOCK_MAX_SIZE: usize = libdragon_sys::RSPQ_BLOCK_MAX_SIZE as usize;

    /// Maximum number of nested block calls
    pub const MAX_BLOCK_NESTING_LEVEL: usize = libdragon_sys::RSPQ_MAX_BLOCK_NESTING_LEVEL as usize;
    /// Special slot used to store the current lowpri pointer
    pub const LOWPRI_CALL_SLOT: usize = libdragon_sys::RSPQ_LOWPRI_CALL_SLOT as usize;
    /// Special slot used to store the current highpri pointer
    pub const HIGHPRI_CALL_SLOT: usize = libdragon_sys::RSPQ_HIGHPRI_CALL_SLOT as usize;

    /// Signal used by RDP SYNC_FULL command to notify that an interrupt is pending
    pub const SP_STATUS_SIG_RDPSYNCFULL: u32 = libdragon_sys::SP_STATUS_SIG_RDPSYNCFULL;
    pub const SP_WSTATUS_SET_SIG_RDPSYNCFULL: u32 = libdragon_sys::SP_WSTATUS_SET_SIG_RDPSYNCFULL;
    pub const SP_WSTATUS_CLEAR_SIG_RDPSYNCFULL: u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG_RDPSYNCFULL;

    /// Signal used by RSP to notify that a syncpoint was reached
    pub const SP_STATUS_SIG_SYNCPOINT: u32 = libdragon_sys::SP_STATUS_SIG_SYNCPOINT;
    pub const SP_WSTATUS_SET_SIG_SYNCPOINT: u32 = libdragon_sys::SP_WSTATUS_SET_SIG_SYNCPOINT;
    pub const SP_WSTATUS_CLEAR_SIG_SYNCPOINT: u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG_SYNCPOINT;

    /// Signal used to notify that RSP is executing the highpri queue
    pub const SP_STATUS_SIG_HIGHPRI_RUNNING: u32 = libdragon_sys::SP_STATUS_SIG_HIGHPRI_RUNNING;
    pub const SP_WSTATUS_SET_SIG_HIGHPRI_RUNNING: u32 = libdragon_sys::SP_WSTATUS_SET_SIG_HIGHPRI_RUNNING;
    pub const SP_WSTATUS_CLEAR_SIG_HIGHPRI_RUNNING: u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG_HIGHPRI_RUNNING;

    /// Signal used to notify that the CPU has requested that the RSP switches to the highpri queue
    pub const SP_STATUS_SIG_HIGHPRI_REQUESTED: u32 = libdragon_sys::SP_STATUS_SIG_HIGHPRI_REQUESTED;
    pub const SP_WSTATUS_SET_SIG_HIGHPRI_REQUESTED: u32 = libdragon_sys::SP_WSTATUS_SET_SIG_HIGHPRI_REQUESTED;
    pub const SP_WSTATUS_CLEAR_SIG_HIGHPRI_REQUESTED: u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG_HIGHPRI_REQUESTED;

    /// Signal used by RSP to notify that has finished one of the two buffers of the highpri queue
    pub const SP_STATUS_SIG_BUFDONE_HIGH: u32 = libdragon_sys::SP_STATUS_SIG_BUFDONE_HIGH;
    pub const SP_WSTATUS_SET_SIG_BUFDONE_HIGH: u32 = libdragon_sys::SP_WSTATUS_SET_SIG_BUFDONE_HIGH;
    pub const SP_WSTATUS_CLEAR_SIG_BUFDONE_HIGH: u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG_BUFDONE_HIGH;

    /// Signal used by RSP to notify that has finished one of the two buffers of the lowpri queue
    pub const SP_STATUS_SIG_BUFDONE_LOW: u32 = libdragon_sys::SP_STATUS_SIG_BUFDONE_LOW;
    pub const SP_WSTATUS_SET_SIG_BUFDONE_LOW: u32 = libdragon_sys::SP_WSTATUS_SET_SIG_BUFDONE_LOW;
    pub const SP_WSTATUS_CLEAR_SIG_BUFDONE_LOW: u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG_BUFDONE_LOW;

    /// Signal used by the CPU to notify the RSP that more data has been written in the current queue
    pub const SP_STATUS_SIG_MORE: u32 = libdragon_sys::SP_STATUS_SIG_MORE;
    pub const SP_WSTATUS_SET_SIG_MORE: u32 = libdragon_sys::SP_WSTATUS_SET_SIG_MORE;
    pub const SP_WSTATUS_CLEAR_SIG_MORE: u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG_MORE;

    /// RSP assert codes (for assers generated by rsp_queue.S)
    /// A command is referencing an overlay that is not registered
    pub const ASSERT_INVALID_OVERLAY: u32 = libdragon_sys::ASSERT_INVALID_OVERLAY;
    /// The requested command is not defined in the overlay
    pub const ASSERT_INVALID_COMMAND: u32 = libdragon_sys::ASSERT_INVALID_COMMAND;

    pub const PROFILE_OVERLAY_COUNT: usize = libdragon_sys::RSPQ_PROFILE_OVERLAY_COUNT as usize;

    pub const PROFILE_SLOT_SIZE: usize = libdragon_sys::RSPQ_PROFILE_SLOT_SIZE as usize;
    pub const PROFILE_SLOT_COUNT: usize = libdragon_sys::RSPQ_PROFILE_SLOT_COUNT as usize;
    pub const PROFILE_BUILTIN_SLOT: usize = libdragon_sys::RSPQ_PROFILE_BUILTIN_SLOT as usize;
    pub const PROFILE_IDLE_SLOT: usize = libdragon_sys::RSPQ_PROFILE_IDLE_SLOT as usize;
    pub const PROFILE_IDLE_RDP_SLOT: usize = libdragon_sys::RSPQ_PROFILE_IDLE_RDP_SLOT as usize;
    pub const PROFILE_IDLE_SYNC_SLOT: usize = libdragon_sys::RSPQ_PROFILE_IDLE_SYNC_SLOT as usize;
    pub const PROFILE_RDPQ_SYNC_SLOT: usize = libdragon_sys::RSPQ_PROFILE_RDPQ_SYNC_SLOT as usize;
    pub const PROFILE_OVL_SWITCH_SLOT: usize = libdragon_sys::RSPQ_PROFILE_OVL_SWITCH_SLOT as usize;
}

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
    pub slots         : [ProfileSlot; consts::PROFILE_SLOT_COUNT],
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
