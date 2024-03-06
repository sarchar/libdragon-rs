use core::sync::atomic::{AtomicU32, Ordering};

use crate::*;
use audio::samplebuffer;

/// See [`MIXER_MAX_CHANNELS`](libdragon_sys::MIXER_MAX_CHANNELS) for details.
pub const MIXER_MAX_CHANNELS: usize = libdragon_sys::MIXER_MAX_CHANNELS as usize;

/// See [`MIXER_LOOP_OVERREAD`](libdragon_sys::MIXER_LOOP_OVERREAD) for details.
pub const MIXER_LOOP_OVERREAD: usize = libdragon_sys::MIXER_LOOP_OVERREAD as usize;

/// The mixer system must be initialized after the audio system.
///
/// See [`mixer_init`](libdragon_sys::mixer_init) for details.
pub fn init(num_channels: i32) { unsafe { libdragon_sys::mixer_init(num_channels); } }

/// See [`mixer_close`](libdragon_sys::mixer_close) for details.
pub fn close() { unsafe { libdragon_sys::mixer_close(); } }

/// Set master volume.
///
/// See [`mixer_set_vol`](libdragon_sys::mixer_set_vol) for details.
pub fn set_vol(vol: f32) { unsafe { libdragon_sys::mixer_set_vol(vol); } }

/// Set channel volume (as left/right).
///
/// See [`mixer_ch_set_vol`](libdragon_sys::mixer_ch_set_vol) for details.
pub fn ch_set_vol(ch: i32, lvol: f32, rvol: f32) { unsafe { libdragon_sys::mixer_ch_set_vol(ch, lvol, rvol); } }

/// Set channel volume (as volume and panning).
///
/// See [`mixer_ch_set_vol_pan`](libdragon_sys::mixer_ch_set_vol_pan) for details.
pub fn ch_set_vol_pan(ch: i32, lvol: f32, rvol: f32) { unsafe { libdragon_sys::mixer_ch_set_vol_pan(ch, lvol, rvol); } }

/// Set channel volume with Dolby Pro Logic II encoding.
///
/// See [`mixer_ch_set_vol_dolby`](libdragon_sys::mixer_ch_set_vol_dolby) for details.
pub fn ch_set_vol_dolby(ch: i32, fl: f32, fr: f32, c: f32, sl: f32, sr: f32) { unsafe { libdragon_sys::mixer_ch_set_vol_dolby(ch, fl, fr, c, sl, sr); } }

/// Change the frequency for the specified channel.
///
/// See [`mixer_ch_set_freq`](libdragon_sys::mixer_ch_set_freq) for details.
pub fn ch_set_freq(ch: i32, frequency: f32) { unsafe { libdragon_sys::mixer_ch_set_freq(ch, frequency); } }

/// Change the current playback position within a waveform.
///
/// See [`mixer_ch_set_pos`](libdragon_sys::mixer_ch_set_pos) for details.
pub fn ch_set_pos(ch: i32, pos: f32) { unsafe { libdragon_sys::mixer_ch_set_pos(ch, pos); } }

/// Read the current playback position of the waveform in the channel.
///
/// See [`mixer_ch_get_pos`](libdragon_sys::mixer_ch_get_pos) for details.
pub fn ch_get_pos(ch: i32) -> f32 { unsafe { libdragon_sys::mixer_ch_get_pos(ch) } }

/// Stop playing samplems on the specified channel.
///
/// See [`mixer_ch_stop`](libdragon_sys::mixer_ch_stop) for details.
pub fn ch_stop(ch: i32) { unsafe { libdragon_sys::mixer_ch_stop(ch); } }

/// Return true if the channel is currently playing samples.
///
/// See [`mixer_ch_playing`](libdragon_sys::mixer_ch_playing) for details.
pub fn ch_playing(ch: i32) -> bool { unsafe { libdragon_sys::mixer_ch_playing(ch) } }

/// Configure the limits of a channel with respect to the sample bit size, and frequency.
///
/// See [`mixer_ch_set_limits`](libdragon_sys::mixer_ch_set_limits) for details.
pub fn ch_set_limits(ch: i32, max_bits: i32, max_frequency: f32, max_buf_sz: usize) { 
    unsafe { libdragon_sys::mixer_ch_set_limits(ch, max_bits, max_frequency, max_buf_sz as i32); } 
}

/// Throttle the mixer by specifying the maximum number of samples it can generate.
///
/// See [`mixer_throttle`](libdragon_sys::mixer_throttle) for details.
pub fn throttle(num_samples: f32) { unsafe { libdragon_sys::mixer_throttle(num_samples); } }

/// Unthrottle the mixer
///
/// See [`mixer_unthrottle`](libdragon_sys::mixer_unthrottle) for details.
pub fn unthrottle() { unsafe { libdragon_sys::mixer_unthrottle(); } }

/// Run the mixer to produce output samples.
///
/// See [`mixer_poll`](libdragon_sys::mixer_poll) for details.
pub fn poll(out: &mut [i16]) { 
    let nsamples = out.len() / 2;
    unsafe {
        libdragon_sys::mixer_poll(out.as_mut_ptr(), nsamples as i32);
    }
}

/// Request the mixer to try and write audio samples to be played, if possible.
///
/// See [`mixer_try_play`](libdragon_sys::mixer_try_play) for details.
pub fn try_play() { unsafe { libdragon_sys::mixer_try_play(); } }

/// Callback invoked by mixer_poll at a specified time.
///
/// Rust specific: context is not returned to the caller. You should capture what you need in the
/// closure.
///
/// See [`MixerEvent`](libdragon_sys::MixerEvent) for details.
pub type MixerEventCallback = Box<dyn FnMut() -> i32 + 'static + Sync + Send>;

/// Internal structure used to keep track of MixerEventCallback
struct MixerEventInternal {
    user_callback: MixerEventCallback,

    /// We need a reference count on this object to prevent double-free
    rc: AtomicU32,
}

/// Internal wrapper for C-to-Rust callbacks
extern "C" fn event_callback(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    // obtain the MixerEventInternal struct
    let mut cb = unsafe {
        let ctx: *mut MixerEventInternal = ctx as *mut MixerEventInternal;
        Box::from_raw(ctx)
    };

    // call user code
    let res = (cb.user_callback)();

    // if the return code from the callback is 0, the callback memory can be freed
    // note that this may only decrement the resource counter and not free the memory
    if res != 0 { let _ = Box::leak(cb); }

    // non-zero values re-schedule the callback
    res
}

/// Register a time-based event into the mixer.
///
/// Rust-specific: the returned `MixerEvent` allows for removing the event.  If 
/// it is dropped without being removed, you must return 0 from the callback at some point,
/// otherwise the memory will leak.
///
/// See [`mixer_add_event`](libdragon_sys::mixer_add_event) for details.
pub fn add_event(delay: i64, cb: MixerEventCallback) -> MixerEvent {
    let cb = Box::new(MixerEventInternal { user_callback: cb, rc: AtomicU32::new(2) });
    let ctx = unsafe {
        let ctx: *mut MixerEventInternal = Box::leak(cb); // Leak the function callback to prevent
                                                          // memory from being freed
        libdragon_sys::mixer_add_event(delay, Some(event_callback), ctx as *mut ::core::ffi::c_void);
        ctx
    };

    MixerEvent(Some(ctx))
}

/// Wrapper around mixer_remove_event
pub struct MixerEvent(Option<*mut MixerEventInternal>);

impl MixerEvent {
    /// Deregister a time-based event from the mixer.
    ///
    /// See [`mixer_remove_event`](libdragon_sys::mixer_remove_event) for details.
    pub fn remove(&mut self) {
        if let Some(ctx) = self.0 {
            // Unregister with libdragon
            unsafe {
                libdragon_sys::mixer_remove_event(Some(event_callback), ctx as *mut ::core::ffi::c_void);
            }

            // At this point, event_callback will never be called, and self is still alive, so RC must be 1.
            // Create the MixerEventInternal
            let cb = unsafe { Box::from_raw(ctx) };
            cb.rc.store(1, Ordering::SeqCst);

            // Leak the object for free in drop()
            let _ = Box::leak(cb);
        }
    }
}

/// Implement drop for MixerEvent. We use a resource count of 2 (one for this object and one for
/// the leaked pointer to the mixer callback).  Once both objects have been dropped (or removed +
/// dropped), the memory is freed.
impl Drop for MixerEvent {
    fn drop(&mut self) {
        if let Some(ctx) = self.0 {
            // creating a Box and then dropping it will free the MixerEventInternal memory
            let cb = unsafe { Box::from_raw(ctx) };

            // decrement RC
            let res = cb.rc.fetch_sub(1, Ordering::SeqCst);

            // if RC is now 0 (res == 1), let cb be dropped and the memory freed. Otherwise, leak it.
            if res != 1 {
                let _ = Box::leak(cb);
            }
        }
    }
}

/// Waveform callback function invoked by the mixer to read/generate samples.
///
/// Rust specific: context is not returned to the caller. You should capture what you need in the
/// closure.
///
/// See [`WaveformRead`](libdragon_sys::WaveformRead) for details.
pub type WaveformReadCallback = Box<dyn FnMut(&mut samplebuffer::SampleBuffer, usize, usize, bool) + 'static + Sync + Send>;

/// Maximum number of samples in a waveform
pub const WAVEFORM_MAX_LEN: usize = libdragon_sys::WAVEFORM_MAX_LEN as usize;

/// Specify that the waveform length is unknown
pub const WAVEFORM_UNKNOWN_LEN: usize = libdragon_sys::WAVEFORM_UNKNOWN_LEN as usize;

/// A waveform that can be played back through the mixer.
///
/// See [`struct waveform_s`](libdragon_sys::waveform_s) for details.
pub struct Waveform {
    ptr: *mut libdragon_sys::waveform_s,
}

impl Waveform {
    pub(crate) fn from_ptr(ptr: *mut libdragon_sys::waveform_s) -> Self { Self { ptr: ptr } }

    /// Start playing the waveform on the specified channel.
    ///
    /// See [`mixer_ch_play`](libdragon_sys::mixer_ch_play) for details.
    pub fn play(&self, ch: i32) { unsafe { libdragon_sys::mixer_ch_play(ch, self.ptr); } }

    /// Get the (debug) name of the `Waveform`
    ///
    /// See [`waveform_s.name`](libdragon_sys::waveform_s) for details.
    pub fn name<'a>(&'a self) -> Result<&'a str> {
        let name_str = unsafe {
            CStr::from_ptr((*self.ptr).name as *const i8)
        };
        Ok(name_str.to_str()?)
    }

    /// Width of a sample of this waveform, in bits
    ///
    /// See [`waveform_s.bits`](libdragon_sys::waveform_s) for details.
    pub fn bits(&self) -> u8 {
        unsafe {
            (*self.ptr).bits as u8
        }
    }

    /// Number of interleaved audio channels in this waveform.
    ///
    /// See [`waveform_s.channels`](libdragon_sys::waveform_s) for details.
    pub fn channels(&self) -> u8 {
        unsafe {
            (*self.ptr).channels as u8
        }
    }

    /// Desired playback frequency (in samples per second, aka Hz)
    ///
    /// See [`waveform_s.frequency`](libdragon_sys::waveform_s) for details.
    pub fn frequency(&self) -> f32 {
        unsafe {
            (*self.ptr).frequency
        }
    }

    /// Length of the waveform, in number of samples.
    ///
    /// See [`waveform_s.len`](libdragon_sys::waveform_s) for details.
    pub fn len(&self) -> usize {
        unsafe {
            (*self.ptr).len as usize
        }
    }

    /// Length of the loop of the waveform (from the end).
    ///
    /// See [`waveform_s.loop_len`](libdragon_sys::waveform_s) for details.
    pub fn loop_len(&self) -> usize {
        unsafe {
            (*self.ptr).loop_len as usize
        }
    }
}


