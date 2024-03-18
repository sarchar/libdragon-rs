use crate::*;

/// COP1 Control/Status bits definition. Please refer to MIPS R4300 manual.
/// Flag recording inexact operation
pub const FLAG_INEXACT_OP: u32 = libdragon_sys::C1_FLAG_INEXACT_OP;          
/// Flag recording underflow
pub const FLAG_UNDERFLOW: u32 = libdragon_sys::C1_FLAG_UNDERFLOW;           
/// Flag recording overflow
pub const FLAG_OVERFLOW: u32 = libdragon_sys::C1_FLAG_OVERFLOW;            
/// Flag recording division by zero
pub const FLAG_DIV_BY_0: u32 = libdragon_sys::C1_FLAG_DIV_BY_0;            
/// Flag recording invalid operation
pub const FLAG_INVALID_OP: u32 = libdragon_sys::C1_FLAG_INVALID_OP;          

/// Enable inexact operation exception
pub const ENABLE_INEXACT_OP: u32 = libdragon_sys::C1_ENABLE_INEXACT_OP;        
/// Enable underflow exception
pub const ENABLE_UNDERFLOW: u32 = libdragon_sys::C1_ENABLE_UNDERFLOW;         
/// Enable overflow exception
pub const ENABLE_OVERFLOW: u32 = libdragon_sys::C1_ENABLE_OVERFLOW;          
/// Enable division by zero exception
pub const ENABLE_DIV_BY_0: u32 = libdragon_sys::C1_ENABLE_DIV_BY_0;          
/// Enable invalid operation exception
pub const ENABLE_INVALID_OP: u32 = libdragon_sys::C1_ENABLE_INVALID_OP;        
/// Mask for all enable bits
pub const ENABLE_MASK: u32 = libdragon_sys::C1_ENABLE_MASK;              

/// Triggered inexact operation exception
pub const CAUSE_INEXACT_OP: u32 = libdragon_sys::C1_CAUSE_INEXACT_OP;         
/// Triggered underflow exception
pub const CAUSE_UNDERFLOW: u32 = libdragon_sys::C1_CAUSE_UNDERFLOW;          
/// Triggered overflow exception
pub const CAUSE_OVERFLOW: u32 = libdragon_sys::C1_CAUSE_OVERFLOW;           
/// Triggered division by zero exception
pub const CAUSE_DIV_BY_0: u32 = libdragon_sys::C1_CAUSE_DIV_BY_0;           
/// Triggered invalid operation exception
pub const CAUSE_INVALID_OP: u32 = libdragon_sys::C1_CAUSE_INVALID_OP;         
/// Triggered not implemented exception
pub const CAUSE_NOT_IMPLEMENTED: u32 = libdragon_sys::C1_CAUSE_NOT_IMPLEMENTED;    
/// Mask for all cause bits
pub const CAUSE_MASK: u32 = libdragon_sys::C1_CAUSE_MASK;               

/// Flush denormals to zero/min
pub const FCR31_FS: u32 = libdragon_sys::C1_FCR31_FS;

/// Read the COP1 FCR31 register (floating-point control register 31)
#[inline(always)] pub fn fcr31() -> u32 { let x; unsafe { asm!("cfc {value},$f31", value = out(reg) x); } x }
/// Write to the COP1 FCR31 register
#[inline(always)] pub fn write_fcr31(x: u32) { unsafe { asm!("ctc1 {value},$f31", value = in(reg) x); } }

