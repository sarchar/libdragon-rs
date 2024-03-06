use crate::*;

use audio::mixer::{WaveformReadCallback};

/// Internal structure used to keep track of WaveformReadCallback
struct WaveformReadInternal {
    user_callback: WaveformReadCallback,
}

/// `SampleBuffer` is a wrapper around LibDragon's `samplebuffer_s`. 
///
/// See [`struct samplebuffer_s`](libdragon_sys::samplebuffer_s) for details.
pub struct SampleBuffer {
    ptr: *mut libdragon_sys::samplebuffer_s,
    backing_instance: Option<core::pin::Pin<Box<libdragon_sys::samplebuffer_s>>>,
    owned_memory: Option<*mut u8>,
    waveform_read: Option<*mut WaveformReadInternal>,
}

impl SampleBuffer {
    /// Create a new, uninitialized SampleBuffer. This buffer has no
    /// backing memory and cannot be used until [SampleBuffer::init] is called.
    pub fn new() -> Self {
        let sbuf = core::mem::MaybeUninit::<libdragon_sys::samplebuffer_s>::zeroed();
        let mut backing_instance = Box::pin(unsafe { sbuf.assume_init() });
        Self {
            ptr: backing_instance.as_mut().get_mut(),
            backing_instance: Some(backing_instance),
            owned_memory: None,
            waveform_read: None,
        }
    }

    /// Create a new sample buffer with a new buffer allocated of size `size`
    ///
    /// This function has no equivalent in LibDragon. Here, memory is allocated
    /// of size `size` in bytes and the sample buffer is initialized. If [SampleBuffer::close] is
    /// called on this object, then it can still be used after a new call to [SampleBuffer::init].
    pub fn new_with_capacity(size: usize) -> Self {
        let buf: &mut [u8] = unsafe {
            let mem = libdragon_sys::malloc_uncached_aligned(8, size) as *mut _;
            assert!(mem != core::ptr::null_mut());
            core::slice::from_raw_parts_mut(mem, size)
        };

        let mut sbuf: core::mem::MaybeUninit<libdragon_sys::samplebuffer_s> = core::mem::MaybeUninit::uninit();
        unsafe {
            libdragon_sys::samplebuffer_init(sbuf.as_mut_ptr(), buf.as_mut_ptr(), size as i32);
        }

        let mut backing_instance = Box::pin(unsafe { sbuf.assume_init() } );
        Self {
            ptr: backing_instance.as_mut().get_mut(),
            backing_instance: Some(backing_instance),
            owned_memory: Some(buf.as_mut_ptr() as *mut _),
            waveform_read: None,
        }
    }

    /// Replace or initialize the underlying `samplebuffer_s`'s memory storage with the given memory.
    /// The memory must be 8-byte aligned and in the uncached segment. The caller is responsible
    /// for managing the memory, and it must persist as long as self.
    ///
    /// See [`samplebuffer_init`](libdragon_sys::samplebuffer_init) for details.
    pub fn init<'a>(&'a mut self, uncached_mem: &'a mut [u8]) {
        // free backed memory
        if let Some(ptr) = core::mem::replace(&mut self.owned_memory, None) {
            unsafe {
                libdragon_sys::free_uncached(ptr as *mut ::core::ffi::c_void);
            }
        }

        unsafe {
            libdragon_sys::samplebuffer_init(self.ptr, uncached_mem.as_mut_ptr(), uncached_mem.len() as i32);
        }
    }

    /// Configure the bit width of the samples stored in the buffer.
    ///
    /// See [`samplebuffer_set_bps`](libdragon_sys::samplebuffer_set_bps) for details.
    pub fn set_bps(&mut self, bps: i32) {
        unsafe {
            libdragon_sys::samplebuffer_set_bps(self.ptr, bps);
        }
    }

    /// Internal wrapper for C-to-Rust callbacks
    extern "C" fn waveform_read_callback(ctx: *mut ::core::ffi::c_void, sbufptr: *mut libdragon_sys::samplebuffer_t, 
                                         wpos: i32, wlen: i32, seeking: bool) {
        // create an ephemeral instance of SampleBuffer
        // without concrete instances, dropping this doesn't free any memory
        let mut sbuf = SampleBuffer {
            ptr: sbufptr,
            backing_instance: None,
            owned_memory: None,
            waveform_read: None,
        };

        // obtain the WaveformReadInternal struct
        let mut cb = unsafe {
            let ctx: *mut WaveformReadInternal = ctx as *mut WaveformReadInternal;
            Box::from_raw(ctx)
        };

        // call user code
        (cb.user_callback)(&mut sbuf, wpos as usize, wlen as usize, seeking);

        // leak the callback context
        let _ = Box::leak(cb);
    }

    /// Connect a waveform reader callback to this sample buffer.
    ///
    /// See [`samplebuffer_set_waveform`](libdragon_sys::samplebuffer_set_waveform) for details.
    pub fn set_waveform(&mut self, cb: WaveformReadCallback) {
        assert!(self.waveform_read.is_none(), "cannot set waveform more than once");

        let cb = Box::new(WaveformReadInternal { user_callback: cb });
        let ctx = unsafe {
            let ctx: *mut WaveformReadInternal = Box::leak(cb); // Leak the function callback to prevent
                                                                // memory from being freed
            libdragon_sys::samplebuffer_set_waveform(self.ptr, Some(Self::waveform_read_callback), 
                                                     ctx as *mut ::core::ffi::c_void);
            ctx
        };

        self.waveform_read = Some(ctx);
    }

    /// Get a pointer to specific set of samples in the buffer (zero-copy)
    ///
    /// Rust-only: you must tell the compiler the size of a sample by providing
    /// the type of the slice elements. `wlen` is not mutable, as the length 
    /// of the returned slice provides that information.
    ///
    /// See [`samplebuffer_get`](libdragon_sys::samplebuffer_get) for details.
    pub fn get<'a,T>(&'a self, wpos: usize, wlen: usize) -> &'a [T] {
        unsafe {
            let mut i: i32 = wlen as i32;
            let ptr = libdragon_sys::samplebuffer_get(self.ptr, wpos as i32, &mut i as *mut _);
            core::slice::from_raw_parts_mut(ptr as *mut _, i as usize)
        }
    }

    /// Append samples into the buffer (zero-copy).
    ///
    /// See [`samplebuffer_append`](libdragon_sys::samplebuffer_append) for details.
    pub fn append<'a,T>(&'a mut self, wlen: usize) -> &'a mut [T] {
        unsafe {
            let ptr = libdragon_sys::samplebuffer_append(self.ptr, wlen as i32);
            core::slice::from_raw_parts_mut(ptr as *mut _, wlen)
        }
    }

    /// Remove the specified number of samples from the tail of the buffer.
    ///
    /// See [`samplebuffer_undo`](libdragon_sys::samplebuffer_undo) for details.
    pub fn undo(&mut self, wlen: usize) {
        unsafe {
            libdragon_sys::samplebuffer_undo(self.ptr, wlen as i32);
        }
    }

    /// Discard all samples from the buffer that come before a specified
    /// absolute waveform position.
    ///
    /// See [`samplebuffer_discard`](libdragon_sys::samplebuffer_discard) for details.
    pub fn discard(&mut self, wpos: usize) {
        unsafe {
            libdragon_sys::samplebuffer_discard(self.ptr, wpos as i32);
        }
    }
    
    /// Flush (reset) the sample buffer to empty status, discarding all samples.
    ///
    /// See [`samplebuffer_flush`](libdragon_sys::samplebuffer_flush) for details.
    pub fn flush(&mut self) {
        unsafe {
            libdragon_sys::samplebuffer_flush(self.ptr);
        }
    }

    /// Close the sample buffer.
    ///
    /// After calling close, the sample buffer must be initialized again before using it.
    ///
    /// See [`samplebuffer_close`](libdragon_sys::samplebuffer_close) for details.
    pub fn close(&mut self) {
        unsafe {
            libdragon_sys::samplebuffer_close(self.ptr);
            self.backing_instance = None;
        }
    }

    /// See [`struct samplebuffer_s`](libdragon_sys::samplebuffer_s) for details.
    pub fn size (&self) -> usize { unsafe { (*self.ptr).size  as usize } }
    pub fn wpos (&self) -> usize { unsafe { (*self.ptr).wpos  as usize } }
    pub fn widx (&self) -> usize { unsafe { (*self.ptr).widx  as usize } }
    pub fn ridx (&self) -> usize { unsafe { (*self.ptr).ridx  as usize } }
    pub fn wnext(&self) -> isize { unsafe { (*self.ptr).wnext as isize } }
}

impl Drop for SampleBuffer {
    fn drop(&mut self) {
        if self.backing_instance.is_some() {
            self.close();
        }

        // free backed memory
        if let Some(ptr) = self.owned_memory {
            unsafe {
                libdragon_sys::free_uncached(ptr as *mut ::core::ffi::c_void);
            }
        }

        // free callback memory
        if let Some(ctx) = self.waveform_read {
            let _ = unsafe { Box::from_raw(ctx) };
        }
    }
}
