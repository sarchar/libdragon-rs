use crate::*;

/// A custom effect callback to allow music synchronization
///
/// See [`xm64player_set_effect_callback`](libdragon_sys::xm64player_set_effect_callback) for details.
pub type EffectCallback = Box<dyn FnMut(&mut Xm64, u8, u8, u8) + 'static + Sync + Send>;

struct EffectInternal {
    user_callback: EffectCallback,
    ptr: *mut libdragon_sys::xm64player_t,
}

/// Xm64 structure. This is a wrapper around LibDragon's `xm64player_t`.
///
/// See [`struct xm64player_t`](libdragon_sys::xm64player_t) for details.
pub struct Xm64 {
    ptr: *mut libdragon_sys::xm64player_t,
    backing_instance: Option<core::pin::Pin<Box<libdragon_sys::xm64player_t>>>,
    effect_callback: Option<*mut EffectInternal>,
}

impl Xm64 {
    /// Create a XM64 module file and prepare for playback
    ///
    /// See [`xm64player_open`](libdragon_sys::xm64player_open) for details.
    pub fn open<T: AsRef<dfs::Path>>(path: T) -> Result<Self> {
        let path_bytes: &[u8] = path.as_ref().as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let mut backing_instance = Box::pin(unsafe {
            core::mem::MaybeUninit::<libdragon_sys::xm64player_t>::zeroed().assume_init()
        });

        unsafe {
            libdragon_sys::xm64player_open(backing_instance.as_mut().get_mut(), cpath.as_ptr());
        }

        Ok(Self {
            ptr: backing_instance.as_mut().get_mut(),
            backing_instance: Some(backing_instance),
            effect_callback: None,
        })
    }

    /// Get the number of channels in the XM64 file
    ///
    /// See [`xm64player_num_channels`](libdragon_sys::xm64player_num_channels) for details.
    pub fn num_channels(&self) -> i32 {
        unsafe {
            libdragon_sys::xm64player_num_channels(self.ptr)
        }
    }

    /// Configure a Xm64 file for looping.
    ///
    /// See [`xm64player_set_loop`](libdragon_sys::xm64player_set_loop) for details.
    pub fn set_loop(&mut self, loop_: bool) {
        unsafe {
            libdragon_sys::xm64player_set_loop(self.ptr, loop_);
        }
    }

    /// Start playing the XM64 module
    ///
    /// See [`xm64player_play`](libdragon_sys::xm64player_play) for details.
    pub fn play(&mut self, first_ch: i32) {
        unsafe {
            libdragon_sys::xm64player_play(self.ptr, first_ch);
        }
    }

    /// Stop XM playback
    ///
    /// See [`xm64player_stop`](libdragon_sys::xm64player_stop) for details.
    pub fn stop(&mut self) {
        unsafe {
            libdragon_sys::xm64player_stop(self.ptr);
        }
    }

    /// Read the current position of the XM module
    ///
    /// See [`xm64player_tell`](libdragon_sys::xm64player_tell) for details.
    pub fn tell(&self, patidx: &mut i32, row: &mut i32, secs: &mut f32) {
        unsafe {
            libdragon_sys::xm64player_tell(self.ptr, patidx as *mut _, row as *mut _, secs as *mut _);
        }
    }

    /// Seek to a specific position of the XM module
    ///
    /// See [`xm64player_seek`](libdragon_sys::xm64player_seek) for details.
    pub fn seek(&mut self, patidx: i32, row: i32, tick: i32) {
        unsafe {
            libdragon_sys::xm64player_seek(self.ptr, patidx, row, tick);
        }
    }

    /// Seek to a specific position of the XM module
    ///
    /// See [`xm64player_set_vol`](libdragon_sys::xm64player_set_vol) for details.
    pub fn set_vol(&mut self, volume: f32) {
        unsafe {
            libdragon_sys::xm64player_set_vol(self.ptr, volume);
        }
    }

    /// Internal wrapper for C-to-Rust callbacks
    extern "C" fn effect_callback(ctx: *mut ::core::ffi::c_void, patidx: u8, row: u8, tick: u8) {
        // obtain the EffectInternal struct
        let mut cb = unsafe {
            let ctx: *mut EffectInternal = ctx as *mut EffectInternal;
            Box::from_raw(ctx)
        };

        // create an ephemeral instance of Xm64
        // without concrete instances, dropping this doesn't free any memory
        let mut xm64 = Self {
            ptr: cb.ptr,
            backing_instance: None,
            effect_callback: None,
        };

        // call user code
        (cb.user_callback)(&mut xm64, patidx, row, tick);

        // leak the callback context
        let _ = Box::leak(cb);
    }

    /// A custom effect callback to allow music synchronization
    ///
    /// See [`xm64player_set_effect_callback`](libdragon_sys::xm64player_set_effect_callback) for details.
    pub fn set_effect_callback(&mut self, cb: EffectCallback) {
        if self.effect_callback.is_some() {
            unimplemented!("cannot set effect callback than once");
        }

        if self.backing_instance.is_none() {
            panic!("cannot set effect callback on ephemeral instance");
        }

        let cb = Box::new(EffectInternal { user_callback: cb, ptr: self.ptr });
        let ctx = unsafe {
            let ctx: *mut EffectInternal = Box::leak(cb); // Leak the function callback to prevent
                                                          // memory from being freed
            libdragon_sys::xm64player_set_effect_callback(self.ptr, Some(Self::effect_callback), 
                                                          ctx as *mut ::core::ffi::c_void);
            ctx
        };

        self.effect_callback = Some(ctx);
    }

    /// Access the libxm context
    ///
    /// See [`struct xm64player_t`](libdragon_sys::xm64player_t) for details.
    pub fn ctx(&mut self) -> *mut libdragon_sys::xm_context_t {
        todo!("need an XmContext wrapper")
    }

    /// Access the [Waveform](crate::audio::mixer::Waveform) for this Xm64.
    ///
    /// See [`struct xm64player_t`](libdragon_sys::xm64player_t::waves) for details.
    pub fn wave(&mut self, idx: usize) -> audio::mixer::Waveform {
        let waves: &mut [libdragon_sys::waveform_t] = unsafe {
            core::slice::from_raw_parts_mut((*self.ptr).waves, self.nwaves())
        };

        audio::mixer::Waveform::from_ptr(&mut waves[idx] as *mut _)
    }

    /// See [`struct xm64player_t`](libdragon_sys::xm64player_t) for details.
    pub fn nwaves(&self) -> usize { unsafe { (*self.ptr).nwaves as usize } }
    pub fn first_ch(&self) -> i32 { unsafe { (*self.ptr).first_ch } }
    pub fn playing(&self) -> bool { unsafe { (*self.ptr).playing } }
    pub fn looping(&self) -> bool { unsafe { (*self.ptr).looping } }
}

impl Drop for Xm64 {
    /// Uses [`xm64player_close`](libdragon_sys::xm64player_close).
    fn drop(&mut self) {
        if let Some(_) = core::mem::replace(&mut self.backing_instance, None) {
            // free callback memory
            if let Some(ctx) = self.effect_callback {
                let _ = unsafe { Box::from_raw(ctx) };
            }

            unsafe {
                libdragon_sys::xm64player_close(self.ptr);
                self.ptr = core::ptr::null_mut();
            }
        }
    }
}
