use crate::*;

/// Exceptions codes
///
/// See [`exception_code_t`](libdragon_sys::exception_code_t) for details
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExceptionCode {
    Interrupt,
    TlbModification,
    TlbLoadIMiss,
    TlbStoreMiss,
    LoadIAddressError,
    StoreAddressError,
    IBusError,
    DBusError,
    SysCall,
    Breakpoint,
    ReservedInstruction,
    CoprocessorUnusable,
    ArithmeticOverflow,
    Trap,
    FloatingPoint,
    Watch
}

impl From<libdragon_sys::exception_code_t> for ExceptionCode {
    fn from(val: libdragon_sys::exception_code_t) -> ExceptionCode {
        match val {
            libdragon_sys::exception_code_t_EXCEPTION_CODE_INTERRUPT => ExceptionCode::Interrupt,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_TLB_MODIFICATION => ExceptionCode::TlbModification,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_TLB_LOAD_I_MISS => ExceptionCode::TlbLoadIMiss,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_TLB_STORE_MISS => ExceptionCode::TlbStoreMiss,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_LOAD_I_ADDRESS_ERROR => ExceptionCode::LoadIAddressError,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_STORE_ADDRESS_ERROR => ExceptionCode::StoreAddressError,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_I_BUS_ERROR => ExceptionCode::IBusError,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_D_BUS_ERROR => ExceptionCode::DBusError,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_SYS_CALL => ExceptionCode::SysCall,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_BREAKPOINT => ExceptionCode::Breakpoint,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_RESERVED_INSTRUCTION => ExceptionCode::ReservedInstruction,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_COPROCESSOR_UNUSABLE => ExceptionCode::CoprocessorUnusable,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_ARITHMETIC_OVERFLOW => ExceptionCode::ArithmeticOverflow,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_TRAP => ExceptionCode::Trap,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_FLOATING_POINT => ExceptionCode::FloatingPoint,
            libdragon_sys::exception_code_t_EXCEPTION_CODE_WATCH => ExceptionCode::Watch,
            _ => panic!("invalid value"),
        }
    }
}

/// Wrapper around [`reg_block_t`](libdragon_sys::reg_block_t).
///
/// Internally, a pointer to the `reg_block_t` in the exception is stored and used, and changes to registers
/// can be made here during an exception or system call.
#[derive(Copy, Clone)]
pub struct RegBlock<'a> {
    ptr: *mut libdragon_sys::reg_block_t,
    phantom: core::marker::PhantomData<&'a u8>,
}

impl<'a> RegBlock<'a> {
    /// General purpose registers 1-32
    #[inline(always)] pub fn gpr    (&    self, i: usize)         ->      u64 { unsafe {      (*self.ptr).gpr[i]      } }
    #[inline(always)] pub fn gpr_mut(&mut self, i: usize)         -> &mut u64 { unsafe { &mut (*self.ptr).gpr[i]      } }
    #[inline(always)] pub fn set_gpr(&mut self, i: usize, v: u64)             { unsafe {      (*self.ptr).gpr[i] = v; } }

    /// HI
    #[inline(always)] pub fn hi    (&    self)         ->      u64 { unsafe {      (*self.ptr).hi      } }
    #[inline(always)] pub fn hi_mut(&mut self)         -> &mut u64 { unsafe { &mut (*self.ptr).hi      } }
    #[inline(always)] pub fn set_hi(&mut self, v: u64)             { unsafe {      (*self.ptr).hi = v; } }

    /// LO
    #[inline(always)] pub fn lo    (&    self)         ->      u64 { unsafe {      (*self.ptr).lo      } }
    #[inline(always)] pub fn lo_mut(&mut self)         -> &mut u64 { unsafe { &mut (*self.ptr).lo      } }
    #[inline(always)] pub fn set_lo(&mut self, v: u64)             { unsafe {      (*self.ptr).lo = v; } }

    /// SR
    #[inline(always)] pub fn sr    (&    self)         ->      u32 { unsafe {      (*self.ptr).sr      } }
    #[inline(always)] pub fn sr_mut(&mut self)         -> &mut u32 { unsafe { &mut (*self.ptr).sr      } }
    #[inline(always)] pub fn set_sr(&mut self, v: u32)             { unsafe {      (*self.ptr).sr = v; } }

    /// CR (NOTE: can't modify this from an exception handler)
    #[inline(always)] pub fn cr    (&    self)         ->      u32 { unsafe {      (*self.ptr).cr      } }

    /// represents EPC - COP0 register $14. See [`reg_block_t.epc`](libdragon_sys::reg_block_t::epc) for details.
    #[inline(always)] pub fn epc    (&    self)         ->      u32 { unsafe {      (*self.ptr).epc      } }
    #[inline(always)] pub fn epc_mut(&mut self)         -> &mut u32 { unsafe { &mut (*self.ptr).epc      } }
    #[inline(always)] pub fn set_epc(&mut self, v: u32)             { unsafe {      (*self.ptr).epc = v; } }

    /// FC31
    #[inline(always)] pub fn fc31    (&    self)         ->      u32 { unsafe {      (*self.ptr).fc31      } }
    #[inline(always)] pub fn fc31_mut(&mut self)         -> &mut u32 { unsafe { &mut (*self.ptr).fc31      } }
    #[inline(always)] pub fn set_fc31(&mut self, v: u32)             { unsafe {      (*self.ptr).fc31 = v; } }

    /// Floating point registers 1-32
    #[inline(always)] pub fn fpr    (&    self, i: usize)         ->      u64 { unsafe {      (*self.ptr).fpr[i]      } }
    #[inline(always)] pub fn fpr_mut(&mut self, i: usize)         -> &mut u64 { unsafe { &mut (*self.ptr).fpr[i]      } }
    #[inline(always)] pub fn set_fpr(&mut self, i: usize, v: u64)             { unsafe {      (*self.ptr).fpr[i] = v; } }
}

/// Wrapper around [`exception_t`](libdragon::exception_t).
#[derive(Copy, Clone)]
pub struct Exception {
    ptr: *mut libdragon_sys::exception_t,
}

impl Exception {
    /// Access [`exception_t.type`](libdragon_sys::exception_t::type).
    #[inline(always)] pub fn type_(&self) -> i32 { unsafe { (*self.ptr).type_ } }
    /// Access [`exception_t.code`](libdragon_sys::exception_t::code).
    #[inline(always)] pub fn code(&self) -> ExceptionCode { unsafe { (*self.ptr).code }.into() }
    /// Access [`exception_t.info`](libdragon_sys::exception_t::info).
    #[inline(always)] pub fn info(&self) -> Result<&str> { 
        let c_str = unsafe { CStr::from_ptr((*self.ptr).info) };
        Ok(c_str.to_str()?)
    }
    /// Access [`exception_t.regs`](libdragon_sys::exception_t::regs).
    #[inline] pub fn regs<'a>(&'a self) -> RegBlock<'a> { 
        RegBlock { 
            ptr: unsafe { (*self.ptr).regs },
            phantom: core::marker::PhantomData 
        } 
    } 
}

type ExceptionHandlerCallback = Box<dyn Fn(Exception) -> bool + 'static + Sync + Send>;
struct ExceptionHandlerInternal {
    user_callback: ExceptionHandlerCallback,
    chain: Option<Box<ExceptionHandlerInternal>>,
}

static mut EXCEPTION_HANDLER: Option<Box<ExceptionHandlerInternal>> = None;

/// Register an exception handler to handle exceptions
///
/// Rust: the registered callback should be a `Box`ed `Fn(Exception) -> bool`. The bool
/// return signal indicates whether we should call the next exception handler in the chain (true
/// -> call chain, false -> break from calling the next exception handler).
///
/// See [`register_exception_handler`](libdragon_sys::register_exception_handler) for details.
pub fn register_exception_handler(cb: ExceptionHandlerCallback) {
    let cb = Box::new(ExceptionHandlerInternal { user_callback: cb, chain: None });
    
    interrupts::disable();
    unsafe {
        let old_eh = core::mem::replace(&mut EXCEPTION_HANDLER, Some(cb));
        let old = libdragon_sys::register_exception_handler(Some(exception_handler));
        if old_eh.is_some() {
            // Old handler is a Rust handler, so copy over EXCEPTION_HANDLER instead of 'old'
            (EXCEPTION_HANDLER.as_mut().unwrap()).chain = old_eh;
        } else if old.is_some() {
            // Old handler was a C function, call it
            (EXCEPTION_HANDLER.as_mut().unwrap()).chain = Some(
                Box::new(ExceptionHandlerInternal { 
                    user_callback: Box::new(move |e| { old.unwrap()(e.ptr); true }), 
                    chain: None 
                }
            ));
        }
    }
    interrupts::enable();
}

extern "C" fn exception_handler(exc: *mut libdragon_sys::exception_t) {
    let cb = unsafe { EXCEPTION_HANDLER.as_ref().unwrap() };
    let mut call_chain = (cb.user_callback)(Exception { ptr: exc });

    let mut chain_option = unsafe { EXCEPTION_HANDLER.as_ref().unwrap().chain.as_ref() };
    while let Some(chain) = chain_option {
        if !call_chain { break; }
        call_chain = (chain.user_callback)(Exception { ptr: exc });
        chain_option = chain.chain.as_ref();
    }
}

type SysCallHandlerCallback = Box<dyn Fn(Exception, u32) + 'static + Sync + Send>;
struct SysCallHandlerInternal {
    user_callback: SysCallHandlerCallback,
}

static mut SYSCALL_HANDLER: Option<Box<SysCallHandlerInternal>> = None;

/// Register a handler that will be called when a syscall exception occurs
///
/// Rust: the registered callback should be a `Box`ed `Fn(Exception, u32) -> ()`. The second
/// argument is the syscall code.
///
/// See [`register_syscall_handler`](libdragon_sys::register_syscall_handler) for details.
pub fn register_syscall_handler(cb: SysCallHandlerCallback, first_code: u32, last_code: u32) {
    let cb = Box::new(SysCallHandlerInternal { user_callback: cb });
    
    unsafe {
        SYSCALL_HANDLER = Some(cb);
        libdragon_sys::register_syscall_handler(Some(syscall_handler), first_code, last_code);
    }
}

extern "C" fn syscall_handler(exc: *mut libdragon_sys::exception_t, code: ::core::ffi::c_uint) {
    let cb = unsafe { SYSCALL_HANDLER.as_ref().unwrap() };
    (cb.user_callback)(Exception { ptr: exc }, code);
}


