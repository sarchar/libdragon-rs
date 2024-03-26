use crate::*;

/// RSP DMDM: 4K of data memory
pub const SP_DMEM     : Register<u32, { 0x1000 >> 2 }> = Register { address: 0xA400_0000 as *mut _ };
/// RSP IMDM: 4K of instruction memory
pub const SP_IMEM     : Register<u32, { 0x1000 >> 2 }> = Register { address: 0xA400_1000 as *mut _ };
/// Current SP program counter
pub const SP_PC       : Register<u32> = Register { address: 0xA408_0000 as *mut _ };
/// SP status register
pub const SP_STATUS   : Register<u32> = Register { address: 0xA404_0010 as *mut _ };
/// SP DMA full register
pub const SP_DMA_FULL : Register<u32> = Register { address: 0xA404_0014 as *mut _ };
/// SP DMA busy register
pub const SP_DMA_BUSY : Register<u32> = Register { address: 0xA404_0018 as *mut _ };
/// SP semaphore register
pub const SP_SEMAPHORE: Register<u32> = Register { address: 0xA404_001C as *mut _ };

/// SP halted
pub const SP_STATUS_HALTED            : u32 = libdragon_sys::SP_STATUS_HALTED             as u32;
/// SP executed a break instruction
pub const SP_STATUS_BROKE             : u32 = libdragon_sys::SP_STATUS_BROKE              as u32;
/// SP DMA busy
pub const SP_STATUS_DMA_BUSY          : u32 = libdragon_sys::SP_STATUS_DMA_BUSY           as u32;
/// SP DMA full
pub const SP_STATUS_DMA_FULL          : u32 = libdragon_sys::SP_STATUS_DMA_FULL           as u32;
/// SP IO busy
pub const SP_STATUS_IO_BUSY           : u32 = libdragon_sys::SP_STATUS_IO_BUSY            as u32;
/// SP is in a single step mode
pub const SP_STATUS_SSTEP             : u32 = libdragon_sys::SP_STATUS_SSTEP              as u32;
/// SP generate interrupt when hit a break instruction
pub const SP_STATUS_INTERRUPT_ON_BREAK: u32 = libdragon_sys::SP_STATUS_INTERRUPT_ON_BREAK as u32;
/// SP signal 0 is set
pub const SP_STATUS_SIG0              : u32 = libdragon_sys::SP_STATUS_SIG0               as u32;
/// SP signal 1 is set
pub const SP_STATUS_SIG1              : u32 = libdragon_sys::SP_STATUS_SIG1               as u32;
/// SP signal 2 is set
pub const SP_STATUS_SIG2              : u32 = libdragon_sys::SP_STATUS_SIG2               as u32;
/// SP signal 3 is set
pub const SP_STATUS_SIG3              : u32 = libdragon_sys::SP_STATUS_SIG3               as u32;
/// SP signal 4 is set
pub const SP_STATUS_SIG4              : u32 = libdragon_sys::SP_STATUS_SIG4               as u32;
/// SP signal 5 is set
pub const SP_STATUS_SIG5              : u32 = libdragon_sys::SP_STATUS_SIG5               as u32;
/// SP signal 6 is set
pub const SP_STATUS_SIG6              : u32 = libdragon_sys::SP_STATUS_SIG6               as u32;
/// SP signal 7 is set
pub const SP_STATUS_SIG7              : u32 = libdragon_sys::SP_STATUS_SIG7               as u32;

/// SP_STATUS write mask: clear #SP_STATUS_HALTED bit
pub const SP_WSTATUS_CLEAR_HALT       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_HALT       as u32;
/// SP_STATUS write mask: set #SP_STATUS_HALTED bit
pub const SP_WSTATUS_SET_HALT         : u32 = libdragon_sys::SP_WSTATUS_SET_HALT         as u32;
/// SP_STATUS write mask: clear BROKE bit
pub const SP_WSTATUS_CLEAR_BROKE      : u32 = libdragon_sys::SP_WSTATUS_CLEAR_BROKE      as u32;
/// SP_STATUS write mask: clear INTR bit
pub const SP_WSTATUS_CLEAR_INTR       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_INTR       as u32;
/// SP_STATUS write mask: set HALT bit
pub const SP_WSTATUS_SET_INTR         : u32 = libdragon_sys::SP_WSTATUS_SET_INTR         as u32;
/// SP_STATUS write mask: clear SSTEP bit
pub const SP_WSTATUS_CLEAR_SSTEP      : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SSTEP      as u32;
/// SP_STATUS write mask: set SSTEP bit
pub const SP_WSTATUS_SET_SSTEP        : u32 = libdragon_sys::SP_WSTATUS_SET_SSTEP        as u32;
/// SP_STATUS write mask: clear #SP_STATUS_INTERRUPT_ON_BREAK bit
pub const SP_WSTATUS_CLEAR_INTR_BREAK : u32 = libdragon_sys::SP_WSTATUS_CLEAR_INTR_BREAK as u32;
/// SP_STATUS write mask: set SSTEP bit
pub const SP_WSTATUS_SET_INTR_BREAK   : u32 = libdragon_sys::SP_WSTATUS_SET_INTR_BREAK   as u32;
/// SP_STATUS write mask: clear SIG0 bit
pub const SP_WSTATUS_CLEAR_SIG0       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG0       as u32;
/// SP_STATUS write mask: set SIG0 bit
pub const SP_WSTATUS_SET_SIG0         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG0         as u32;
/// SP_STATUS write mask: clear SIG1 bit
pub const SP_WSTATUS_CLEAR_SIG1       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG1       as u32;
/// SP_STATUS write mask: set SIG1 bit
pub const SP_WSTATUS_SET_SIG1         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG1         as u32;
/// SP_STATUS write mask: clear SIG2 bit
pub const SP_WSTATUS_CLEAR_SIG2       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG2       as u32;
/// SP_STATUS write mask: set SIG2 bit
pub const SP_WSTATUS_SET_SIG2         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG2         as u32;
/// SP_STATUS write mask: clear SIG3 bit
pub const SP_WSTATUS_CLEAR_SIG3       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG3       as u32;
/// SP_STATUS write mask: set SIG3 bit
pub const SP_WSTATUS_SET_SIG3         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG3         as u32;
/// SP_STATUS write mask: clear SIG4 bit
pub const SP_WSTATUS_CLEAR_SIG4       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG4       as u32;
/// SP_STATUS write mask: set SIG4 bit
pub const SP_WSTATUS_SET_SIG4         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG4         as u32;
/// SP_STATUS write mask: clear SIG5 bit
pub const SP_WSTATUS_CLEAR_SIG5       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG5       as u32;
/// SP_STATUS write mask: set SIG5 bit
pub const SP_WSTATUS_SET_SIG5         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG5         as u32;
/// SP_STATUS write mask: clear SIG6 bit
pub const SP_WSTATUS_CLEAR_SIG6       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG6       as u32;
/// SP_STATUS write mask: set SIG6 bit
pub const SP_WSTATUS_SET_SIG6         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG6         as u32;
/// SP_STATUS write mask: clear SIG7 bit
pub const SP_WSTATUS_CLEAR_SIG7       : u32 = libdragon_sys::SP_WSTATUS_CLEAR_SIG7       as u32;
/// SP_STATUS write mask: set SIG7 bit
pub const SP_WSTATUS_SET_SIG7         : u32 = libdragon_sys::SP_WSTATUS_SET_SIG7         as u32;

/// Snapshot of the registers status of the RSP.
///
/// We can use LibDragon's `rsp_snapshot_t` directly
///
/// See [`rsp_snapshot_t`](libdragon_sys::rsp_snapshot_t) for details.
pub type Snapshot = libdragon_sys::rsp_snapshot_t;

/// Wrapper for LibDragon's RSP ucode definition
///
/// See [`rsp_ucode_t`](libdragon_sys::rsp_ucode_t) for details.
#[derive(Debug,Clone)]
pub struct RspUcode {
    /// Don't access this data directly
    #[doc(hidden)]
    pub rsp_ucode : core::pin::Pin<Box<RspUcodeT>>,

    /// If this ucode has been registered, the overlay ID of the registered ucode
    pub overlay_id: Option<u32>,
}

pub type RspUcodeT = libdragon_sys::rsp_ucode_t;

/// Define one RSP ucode compiled via libdragon-rs's build system
/// [libdragon-build](libdragon-build).
///
/// Rust-specific: this macro takes a single ident argument which defines a local
/// variable of the given name.  See the `rspqdemo` example program for usage.
///
/// See `DEFINE_RSP_UCODE` in `rsp.h` of LibDragon source.
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

            let mut $sname: $crate::rsp::RspUcode = $crate::rsp::RspUcode {
                rsp_ucode: Box::pin($crate::rsp::RspUcodeT {
                    code: unsafe { &mut [<$sname _text_start>] as *mut _ },
                    code_end: unsafe { &mut [<$sname _text_end>] as *mut _ },
                    data: unsafe { &mut [<$sname _data_start>] as *mut _ },
                    data_end: unsafe { &mut [<$sname _data_end>] as *mut _ },

                    name: 0 as *const _, // TODO { stringify!($sname) },

                    start_pc: 0,
                    crash_handler: None,
                    assert_handler: None,
                }),
                overlay_id: None,
            };
        }
    }
}

impl RspUcode {
    /// Load a RSP ucode
    ///
    /// See [`rsp_load`](libdragon_sys::rsp_load) for details.
    pub fn load(&mut self) { unsafe { libdragon_sys::rsp_load(self.rsp_ucode.as_mut().get_mut()); } }

    /// (unsafe) Access `rsp_ucode_t.code`
    pub unsafe fn code(&mut self) -> *mut u8 { self.rsp_ucode.code }
    /// (unsafe) Access `rsp_ucode_t.code_end`
    pub unsafe fn code_end(&mut self) -> *mut ::core::ffi::c_void { self.rsp_ucode.code_end }
    /// (unsafe) Access `rsp_ucode_t.data`
    pub unsafe fn data(&mut self) -> *mut u8 { self.rsp_ucode.data }
    /// (unsafe) Access `rsp_ucode_t.data_end`
    pub unsafe fn data_end(&mut self) -> *mut ::core::ffi::c_void { self.rsp_ucode.data_end }
    /// Access `rsp_ucode_t.name`
    pub fn name(&self) -> &str { 
        let c_str = unsafe { CStr::from_ptr(self.rsp_ucode.name) };
        c_str.to_str().unwrap()
    }
    /// Access `rsp_ucode_t.start_pc`
    pub fn start_pc(&self) -> u32 { self.rsp_ucode.start_pc }
}

/// Initialize the RSP subsystem
///
/// See [`rsp_init`](libdragon_sys::rsp_init) for details.
pub fn init() { unsafe { libdragon_sys::rsp_init(); } }

/// Run RSP ucode
///
/// See [`rsp_run`](libdragon_sys::rsp_run) for details.
pub fn run() { unsafe { libdragon_sys::rsp_run(); } }

/// Run RSP async
///
/// See [`rsp_run`](libdragon_sys::rsp_run) for details.
#[inline]
pub fn run_async() { 
    extern "C" {
        fn __rsp_run_async(status_flags: u32);
    }
    unsafe { 
        __rsp_run_async(SP_WSTATUS_SET_INTR_BREAK); 
    } 
}

/// Wait until RSP has finished processing.
///
/// See [`rsp_wait`](libdragon_sys::rsp_wait)
pub fn wait() { unsafe { libdragon_sys::rsp_wait(); } }

/// Do a DMA transfer to load a piece of code into RSP IMEM.
///
/// See [`rsp_load_code`](libdragon_sys::rsp_load_code) for details.
pub fn load_code<T>(code: &mut [T], imem_offset: usize) {
    let sz = code.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::rsp_load_code(code.as_mut_ptr() as *mut ::core::ffi::c_void, sz as u32, imem_offset as u32);
    }
}

/// Do a DMA transfer to load a piece of data into RSP DMEM.
///
/// See [`rsp_load_data`](libdragon_sys::rsp_load_data) for details.
pub fn load_data<T>(data: &mut [T], dmem_offset: usize) {
    let sz = data.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::rsp_load_data(data.as_mut_ptr() as *mut ::core::ffi::c_void, sz as u32, dmem_offset as u32);
    }
}

/// Do a DMA transfer to load a piece of code from RSP IMEM to RDRAM.
///
/// Rust-specific: the amount of bytes transfered is the count requested
/// times the size of an element.
///
/// See [`rsp_read_code`](libdragon_sys::rsp_read_code) for details.
pub fn read_code<T>(count: usize, imem_offset: usize) -> Vec<T> {
    let sz = count * ::core::mem::size_of::<T>();
    let mut res = Vec::with_capacity(sz);
    unsafe {
        libdragon_sys::rsp_read_code(res.as_mut_ptr() as *mut ::core::ffi::c_void, sz as u32, imem_offset as u32);
    }
    res
}

/// Do a DMA transfer to load a piece of data from RSP DMEM to RDRAM.
///
/// Rust-specific: the amount of bytes transfered is the count requested
/// times the size of an element.
///
/// See [`rsp_read_data`](libdragon_sys::rsp_read_data) for details.
pub fn read_data<T>(count: usize, dmem_offset: usize) -> Vec<T> {
    let sz = count * ::core::mem::size_of::<T>();
    let mut res = Vec::with_capacity(sz);
    unsafe {
        libdragon_sys::rsp_read_data(res.as_mut_ptr() as *mut ::core::ffi::c_void, sz as u32, dmem_offset as u32);
    }
    res
}

/// Abort the program showing a RSP crash screen with a symptom message.
///
/// Rust-specific: to use this macro, tag your function its used in with #[named]
///
/// See [`rsp_crash`](libdragon_sys::rsp_crash) for details.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! rsp_crash {
    () => ($crate::rsp::__crash(file!(), line!(), function_name!(), ""));
    ($($arg:tt)*) => (
        $crate::rsp::__crash(file!(), line!(), function_name!(), &format!("{}\n", format_args!($($arg)*)));
    );
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! rsp_crash {
    () => ();
    ($($arg:tt)*) => ();
}

/// Create a loop that waits for some condition that is related to RSP,
/// aborting with a RSP crash after a timeout.
///
/// See the [RSP_WAIT_LOOP] macro in LibDragon's rsp.h header.
///
/// ```rust
/// // This example shows a loop that waits for the RSP to set signal
/// // 2 in the status register. It is just an example on how to use the macro.
///
/// wait_loop!(150) {
///     if (rsp::SP_STATUS.read() & rsp::SP_STATUS_SIG2) != 0 { break; }
/// }
/// ```
///
/// TODO this macro is not yet implemented.
#[macro_export]
macro_rules! wait_loop {
    ($timeout_ms:literal, $($tt:tt)+) => {
        todo!("need n64sys module");
    }
}

/// Internal functions used by [rsp_crash!] and [wait_loop!].
///
/// This is the lower-layer crash function. It's preferred to use the [rsp_crash!] 
/// and [wait_loop!] macros instead, with a named function, to better describe 
/// the location of the crash.
///
/// See [`__rsp_crash`](libdragon_sys::__rsp_crash) for details.
#[cfg(debug_assertions)]
pub fn __crash(file: &str, line: u32, func: &str, msg: &str) -> ! {
    let c_file  = CString::new(file).unwrap();
    let c_func  = CString::new(func).unwrap();
    let c_msg   = CString::new(msg).unwrap();
    let fmt_str = CString::new("%s").unwrap();
    unsafe {
        libdragon_sys::__rsp_crash(c_file.as_ptr(), line as i32, c_func.as_ptr(), fmt_str.as_ptr(), c_msg.as_ptr())
    }
}

/// See [__crash]
#[cfg(debug_assertions)]
pub fn __check_assert(file: &str, line: u32, func: &str) {
    let c_file  = CString::new(file).unwrap();
    let c_func  = CString::new(func).unwrap();
    unsafe {
        libdragon_sys::__rsp_check_assert(c_file.as_ptr(), line as i32, c_func.as_ptr());
    }
}

/// See [__crash]
#[cfg(not(debug_assertions))]
pub fn __check_assert(_file: &str, _line: u32, _func: &str) {}
