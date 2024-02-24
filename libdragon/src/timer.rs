use crate::*;

pub fn init() {
    unsafe {
        libdragon_sys::timer_init();
    }
}

pub enum Mode {
    OneShot,
    Continuous,
    Disabled
}

impl Into<::core::ffi::c_int> for Mode {
    fn into(self) -> ::core::ffi::c_int {
        match self {
            Mode::OneShot    => libdragon_sys::TF_ONE_SHOT   as ::core::ffi::c_int,
            Mode::Continuous => libdragon_sys::TF_CONTINUOUS as ::core::ffi::c_int,
            Mode::Disabled   => libdragon_sys::TF_DISABLED   as ::core::ffi::c_int,
        }
    }
}

struct TimerCallback {
    user_callback: Box<dyn Fn(i32) + 'static + Sync + Send>,
}

extern "C" fn timer_callback(ovfl: ::core::ffi::c_int, ctx: *mut ::core::ffi::c_void) {
    // obtain the TimerCallback struct
    let cb = unsafe {
        let ctx: *mut TimerCallback = ctx as *mut TimerCallback;
        Box::from_raw(ctx)
    };

    // call user code
    (cb.user_callback)(ovfl as i32);

    // leak the pointer again for the next call
    let _ = Box::leak(cb);
}

pub fn new(ticks: i32, mode: Mode, callback: Box<dyn Fn(i32) + 'static + Sync + Send>) {
    let cb = Box::new(TimerCallback {
        user_callback: callback
    });

    let _ = unsafe {
        let ctx: *mut TimerCallback = Box::leak(cb);
        libdragon_sys::new_timer_context(ticks, mode.into(), Some(timer_callback), 
                                         ctx as *mut ::core::ffi::c_void)
    };
}

#[inline(always)]
pub fn ticks_per_second() -> u64 {
    cpu_frequency() / 2
}

#[inline(always)]
pub fn ticks(us: u64) -> i32 {
    ((us * ticks_per_second()) / 1000000) as i32
}
