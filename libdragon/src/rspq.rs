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
pub struct RspqWrite {
    w: Option<libdragon_sys::rspq_write_t>,
}

impl RspqWrite {
    pub fn begin(ovl_id: u32, cmd_id: u32, size: usize) -> Self {
        let mut data: core::mem::MaybeUninit<libdragon_sys::rspq_write_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            rspq_write_begin_r(data.as_mut_ptr(), ovl_id, cmd_id, size as ::core::ffi::c_int);
        }
        Self {
            w: Some(unsafe { data.assume_init() }),
        }
    }

    pub fn arg(&mut self, value: u32) {
        unsafe {
            rspq_write_arg_r(self.w.as_mut().unwrap() as *mut _, value);
        }
    }

    pub fn end(&mut self) {
        unsafe {
            rspq_write_end_r(self.w.as_mut().unwrap() as *mut _);
        }
        self.w = None;
    }
}

impl Drop for RspqWrite {
	fn drop(&mut self) {
        assert!(self.w.is_none(), "forgot to call RspqWrite::end()?");
	}
}

#[derive(Debug,Clone)]
pub struct RspqUcode {
    pub rsp_ucode : core::pin::Pin<Box<RspUcodeT>>,
    pub overlay_id: Option<u32>,
}

pub type RspUcodeT = libdragon_sys::rsp_ucode_t;

#[macro_export]
macro_rules! define_rsp_ucode {
    ($sname:ident) => {
        crate::paste! {
            extern "C" {
                static mut [<$sname _text_start>]: ::core::ffi::c_uchar;
                static mut [<$sname _text_end>]: ::core::ffi::c_void;
                static mut [<$sname _data_start>]: ::core::ffi::c_uchar;
                static mut [<$sname _data_end>]: ::core::ffi::c_void;
            }

            let mut $sname: rspq::RspqUcode = rspq::RspqUcode {
                rsp_ucode: Box::pin(rspq::RspUcodeT {
                    code: unsafe { &mut [<$sname _text_start>] as *mut _ },
                    code_end: unsafe { &mut [<$sname _text_end>] as *mut _ },
                    data: unsafe { &mut [<$sname _data_start>] as *mut _ },
                    data_end: unsafe { &mut [<$sname _data_end>] as *mut _ },

                    name: 0 as *const _, //{ stringify!($sname) },

                    start_pc: 0,
                    crash_handler: None,
                    assert_handler: None,
                }),
                overlay_id: None,
            };

        }
    }
}

impl RspqUcode {
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
    /// call [T].uncached_mut().
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
