/// Initialize the throttling engine
///
/// See [`throttle_init`](libdragon_sys::throttle_init)
pub fn init(fps: f32, can_framestip: i32, frames_advance: i32) {
    unsafe { libdragon_sys::throttle_init(fps, can_framestip, frames_advance); }
}

/// Throttle the CPU (spin-wait)
///
/// See [`throttle_wait`](libdragon_sys::throttle_wait)
pub fn wait() -> i32 { unsafe { libdragon_sys::throttle_wait() } }

/// Return the approximate length of a frame
///
/// See [`throttle_frame_length`](libdragon_sys::throttle_frame_length)
pub fn frame_length() -> u32 { unsafe { libdragon_sys::throttle_frame_length() } }

/// Return the amount of time left before the end of the current frame.
///
/// See [`throttle_frame_time_left`](libdragon_sys::throttle_frame_time_left)
pub fn frame_time_left() -> i32 { unsafe { libdragon_sys::throttle_frame_time_left() } }

