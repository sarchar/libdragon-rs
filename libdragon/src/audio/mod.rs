
use crate::*;

/// Waveform mixer
pub mod mixer;
/// SampleBuffers for encoding waveform data
pub mod samplebuffer;
/// Play WAV files
pub mod wav64;
/// Play XM files
pub mod xm64;
/// Play YM files
pub mod ym64;

/// Initialize the audio subsystem
/// 
/// See [`audio_init`](libdragon_sys::audio_init) for details.
pub fn init(freq: i32, numbuffers: usize) {
    unsafe {
        libdragon_sys::audio_init(freq, numbuffers as i32);
    }
}

/// Close the audio subsystem
///
/// See [`audio_close`](libdragon_sys::audio_close) for details.
pub fn close() {
    unsafe {
        libdragon_sys::audio_close();
    }
}

// Need an unsafe global for our Rust callback, unfortunately
type FillBufferCallback = Box<dyn FnMut(&mut [i16]) + 'static + Sync + Send>;
static mut FILL_BUFFER_CALLBACK: Option<FillBufferCallback> = None;

extern "C" fn fill_buffer_callback_rust(buffer: *mut ::core::ffi::c_short, numsamples: usize) {
    let buf: &mut [i16] = unsafe {
        ::core::slice::from_raw_parts_mut(buffer, 2 * numsamples) // two channels * numsamples
    };

    unsafe {
        match FILL_BUFFER_CALLBACK.as_mut() {
            Some(cb) => {
                cb(buf);
            },
            None => {
                buf.fill(0)
            }
        }
    }
}

/// Install an audio callback to fill the audio buffer when required.
/// 
/// See [`audio_set_buffer_callback`](libdragon_sys::audio_set_buffer_callback) for details.
pub fn set_buffer_callback(cb: FillBufferCallback) {
    unsafe {
        FILL_BUFFER_CALLBACK = Some(cb);
        libdragon_sys::audio_set_buffer_callback(Some(fill_buffer_callback_rust));
    }
}

/// Pause or resume audio playback
///
/// See [`audio_pause`](libdragon_sys::audio_pause) for details.
pub fn pause(pause: bool) {
    unsafe {
        libdragon_sys::audio_pause(pause);
    }
}

/// Return whether there is an empty buffer to write to
///
/// See [`audio_can_write`](libdragon_sys::audio_can_write) for details.
pub fn audio_can_write() -> bool {
    unsafe {
        libdragon_sys::audio_can_write() != 0
    }
}

/// Write a chunk of silence
///
/// See [`audio_write_silence`](libdragon_sys::audio_write_silence) for details.
pub fn write_silence() {
    unsafe {
        libdragon_sys::audio_write_silence();
    }
}

/// Return actual frequency of audio playback
///
/// See [`audio_get_frequency`](libdragon_sys::audio_get_frequency) for details.
pub fn get_frequency() -> i32 {
    unsafe {
        libdragon_sys::audio_get_frequency()
    }
}

/// Get the number of stereo samples that fit into an allocated buffer
///
/// See [`audio_get_buffer_length`](libdragon_sys::audio_get_buffer_length) for details.
pub fn get_buffer_length() -> usize {
    unsafe {
        libdragon_sys::audio_get_buffer_length() as usize
    }
}

/// Write to the first free internal buffer.
///
/// Rust-specific: this function is a wrapper around 
/// [`audio_write_begin`](libdragon_sys::audio_write_begin)
/// and [`audio_write_end`](libdragon_sys::audio_write_end).  
///
/// This function will call `audio_write_begin`, which may block,
/// and then call the provided `cb` callback with a buffer slice to write to. The size 
/// of the provided slice is the number of *samples* to write - one per channel.  A slice of,
/// for example length 16, is for 8 stereo samples.
///
/// See [`audio_write_begin`](libdragon_sys::audio_write_begin) and 
/// [`audio_write_end`](libdragon_sys::audio_write_end) for details.
pub fn write<F: FnMut(&mut [i16]) -> ()>(mut cb: F) {
    let size = get_buffer_length() * 2;
    let buf = unsafe {
        let ptr = libdragon_sys::audio_write_begin(); // blocking call
        ::core::slice::from_raw_parts_mut(ptr, size)
    };
    cb(buf);
    unsafe {
        libdragon_sys::audio_write_end();
    }
}

/// Push a chunk of audio data (high-level function)
///
/// See [`audio_push`](libdragon_sys::audio_push) for details.
pub fn push(buffer: &[i16], nsamples: i32, blocking: bool) -> i32 {
    unsafe {
        libdragon_sys::audio_push(buffer.as_ptr(), nsamples, blocking)
    }
}
