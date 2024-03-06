use crate::*;

/// See [`timer_init`](libdragon-sys::timer_init)
pub fn init() {
    unsafe {
        libdragon_sys::timer_init();
    }
}

/// See [`timer_close`](libdragon-sys::timer_close)
pub fn close() {
    unsafe {
        libdragon_sys::timer_close();
    }
}

/// See [`timer_ticks`](libdragon-sys::timer_ticks)
pub fn ticks() -> u64 {
    unsafe {
        libdragon_sys::timer_ticks() as u64
    }
}

/// Calculate microseconds based on timer ticks
pub fn micros(ticks: u64) -> u64 {
    (ticks * 1000000) / ticks_per_second()
}

/// Timer mode.
///
/// `OneShot` - Timer should fire only once
/// `Continuous` - Timer should fire at a regular interval
/// `*Disabled` - Timer is enabled or not. Can be used to get a new timer that's not started
pub enum Mode {
    OneShot,
    Continuous,
    OneShotDisabled,
    ContinuousDisabled,
}

impl Into<::core::ffi::c_int> for Mode {
    fn into(self) -> ::core::ffi::c_int {
        match self {
            Mode::OneShot    => libdragon_sys::TF_ONE_SHOT   as ::core::ffi::c_int,
            Mode::Continuous => libdragon_sys::TF_CONTINUOUS as ::core::ffi::c_int,
            Mode::OneShotDisabled    => 
                (libdragon_sys::TF_ONE_SHOT | libdragon_sys::TF_DISABLED) as ::core::ffi::c_int,
            Mode::ContinuousDisabled => 
                (libdragon_sys::TF_CONTINUOUS | libdragon_sys::TF_DISABLED) as ::core::ffi::c_int,
        }
    }
}

#[inline(always)]
pub fn ticks_per_second() -> u64 {
    cpu_frequency() / 2
}

#[inline(always)]
pub fn make_ticks(us: u64) -> i32 {
    ((us * ticks_per_second()) / 1000000) as i32
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

pub struct Timer {
    ptr: *mut libdragon_sys::timer_link_t,
    ctx: Option<*mut TimerCallback>,
}

impl Timer {
    /// Create a new timer.  See [`timer_init`](libdragon_sys::new_timer) for details.
    ///
    /// Dropping the returned Timer will call [`delete_timer`](libdragon_sys::delete_timer). 
    pub fn new(ticks: i32, mode: Mode, callback: Box<dyn Fn(i32) + 'static + Sync + Send>) -> Timer {
        let cb = Box::new(TimerCallback { user_callback: callback });
    
        let (timer_link, ctx) = unsafe {
            let ctx: *mut TimerCallback = Box::leak(cb);
            (libdragon_sys::new_timer_context(ticks, mode.into(), Some(timer_callback), 
                                              ctx as *mut ::core::ffi::c_void), ctx)
        };

        Self {
            ptr: timer_link,
            ctx: Some(ctx),
        }
    }

    fn free_context(&mut self) {
        if let Some(ptr) = core::mem::replace(&mut self.ctx, None) {
            // creating a Box and then dropping it will free the TimerCallback memory
            let _ = unsafe {
                Box::from_raw(ptr as *mut TimerCallback)
            };
        }
    }

    /// See [`start_timer`](libdragon-sys::start_timer)
    pub fn start(&mut self, ticks: i32, mode: Mode, callback: Box<dyn Fn(i32) + 'static + Sync + Send>) {
        // stop timer and free context
        self.stop();
        self.free_context();

        let cb = Box::new(TimerCallback {
            user_callback: callback
        });

        let ctx = unsafe {
            let ctx: *mut TimerCallback = Box::leak(cb);
            libdragon_sys::start_timer_context(self.ptr, ticks, mode.into(), Some(timer_callback), 
                                             ctx as *mut ::core::ffi::c_void);
            ctx
        };

        self.ctx = Some(ctx);
    }

    /// See [`restart_timer`](libdragon-sys::restart_timer)
    pub fn restart(&mut self) {
        unsafe {
            libdragon_sys::restart_timer(self.ptr);
        }
    }

    /// See [`stop_timer`](libdragon-sys::stop_timer)
    pub fn stop(&mut self) {
        unsafe {
            libdragon_sys::stop_timer(self.ptr);
        }
    }
}

impl Drop for Timer {
    /// See [`delete_timer`](libdragon-sys::delete_timer)
    fn drop(&mut self) {
        unsafe {
            libdragon_sys::delete_timer(self.ptr);
        }
        self.free_context();
    }
}
