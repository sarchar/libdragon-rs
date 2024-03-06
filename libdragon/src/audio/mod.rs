
use crate::*;

pub mod mixer;
pub mod samplebuffer;
pub mod wav64;

pub fn init(freq: i32, numbuffers: usize) {
    unsafe {
        libdragon_sys::audio_init(freq, numbuffers as i32);
    }
}

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

pub fn set_buffer_callback(cb: FillBufferCallback) {
    unsafe {
        FILL_BUFFER_CALLBACK = Some(cb);
        libdragon_sys::audio_set_buffer_callback(Some(fill_buffer_callback_rust));
    }
}

pub fn pause(pause: bool) {
    unsafe {
        libdragon_sys::audio_pause(pause);
    }
}

pub fn can_write() -> bool {
    unsafe {
        libdragon_sys::audio_can_write() != 0
    }
}

pub fn write_silence() {
    unsafe {
        libdragon_sys::audio_write_silence();
    }
}

pub fn get_frequency() -> i32 {
    unsafe {
        libdragon_sys::audio_get_frequency()
    }
}

pub fn get_buffer_length() -> usize {
    unsafe {
        libdragon_sys::audio_get_buffer_length() as usize
    }
}

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

pub fn push(buffer: &[i16], nsamples: i32, blocking: bool) -> i32 {
    unsafe {
        libdragon_sys::audio_push(buffer.as_ptr(), nsamples, blocking)
    }
}
