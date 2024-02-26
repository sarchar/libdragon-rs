//use crate::*;

#[derive(Debug, Copy, Clone)]
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

// rspq_profile.h
const PROFILE_SLOT_COUNT: usize = MAX_OVERLAY_COUNT + 6;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProfileSlot {
    pub total_ticks : u64,
    pub sample_count: u64,
    name            : *const ::core::ffi::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProfileData {
    pub slots         : [ProfileSlot; PROFILE_SLOT_COUNT],
    pub total_ticks   : u64,
    pub rdp_busy_ticks: u64,
    pub frame_count   : u64,
}

pub fn profile_start() {
    unsafe {
        libdragon_sys::rspq_profile_start();
    }
}

pub fn profile_stop() {
    unsafe {
        libdragon_sys::rspq_profile_stop();
    }
}

pub fn profile_reset() {
    unsafe {
        libdragon_sys::rspq_profile_reset();
    }
}

pub fn profile_next_frame() {
    unsafe {
        libdragon_sys::rspq_profile_next_frame();
    }
}

pub fn profile_dump() {
    unsafe {
        libdragon_sys::rspq_profile_dump();
    }
}

pub fn profile_get_data() -> ProfileData {
    let mut data: core::mem::MaybeUninit<libdragon_sys::rspq_profile_data_s> = core::mem::MaybeUninit::uninit();
    unsafe {
        libdragon_sys::rspq_profile_get_data(data.as_mut_ptr());
        core::mem::transmute(data)
    }
}
