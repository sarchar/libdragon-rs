//!
//! All the functions in this module are implemented as macros in the C header file `cop0.h` from
//! LibDragon. Refer to that file for detailed information.
use crate::*;

/// Read the COP0 Count register (see also [ticks::read]).
#[inline(always)] pub fn count() -> u32 { let x; unsafe { asm!("mfc0 {x}, $9", x = out(reg) x); } x }

/// Write the COP0 Count register
#[inline(always)] pub fn write_count(x: u32) { unsafe { asm!("mtc0 {x}, $9", x = in(reg) x); } }

/// Read the COP0 Compare register (see also [ticks::read]).
#[inline(always)] pub fn compare() -> u32 { let x; unsafe { asm!("mfc0 {x}, $11", x = out(reg) x); } x }

/// Write the COP0 Compare register
#[inline(always)] pub fn write_compare(x: u32) { unsafe { asm!("mtc0 {x}, $11", x = in(reg) x); } }

/// Read the COP0 Status register (see also [ticks::read]).
#[inline(always)] pub fn status() -> u32 { let x; unsafe { asm!("mfc0 {x}, $12", x = out(reg) x); } x }

/// Write the COP0 Status register
#[inline(always)] pub fn write_status(x: u32) { unsafe { asm!("mtc0 {x}, $12", x = in(reg) x); } }

/// Read the COP0 Cause register (see also [ticks::read]).
#[inline(always)] pub fn cause() -> u32 { let x; unsafe { asm!("mfc0 {x}, $13", x = out(reg) x); } x }
/// Alternative naming
#[inline(always)] pub fn cr() -> u32 { cause() }

/// Write the COP0 Cause register
#[inline(always)] pub fn write_cause(x: u32) { unsafe { asm!("mtc0 {x}, $13", x = in(reg) x); } }
/// Alternative naming
#[inline(always)] pub fn write_cr(x: u32) { write_cause(x) }

/// Returns the COP0 register $8 (BadVAddr)
#[inline]
pub fn badvaddr() -> u64 {
    let w0: u32;
    let w1: u32;
    unsafe {
        asm!("dmfc0 {w0},$8", 
             "dsrl32 {w1}, {w0}, 0",
             w0 = out(reg) w0,
             w1 = out(reg) w1);
    }
    (w0 as u64) | ((w1 as u64) << 32)
}

/// Read the COP0 register $14 (EPC)
#[inline(always)] pub fn epc() -> u32 { let x; unsafe { asm!("mfc0 {x}, $14", x = out(reg) x); } x }

/// Read the COP0 Index register (see also [ticks::read]).
#[inline(always)] pub fn index() -> u32 { let x; unsafe { asm!("mfc0 {x}, $0", x = out(reg) x); } x }

/// Write the COP0 Index register
#[inline(always)] pub fn write_index(x: u32) { unsafe { asm!("mtc0 {x}, $0", "nop", "nop", x = in(reg) x); } }

/// Read the COP0 ENTRYHI register
#[inline(always)] pub fn entryhi() -> u32 { let x; unsafe { asm!("mfc0 {x}, $10", x = out(reg) x); } x }

/// Write the COP0 ENTRYHI register
#[inline(always)] pub fn write_entryhi(x: u32) { unsafe { asm!("mtc0 {x}, $10", "nop", "nop", x = in(reg) x); } }

/// Read the COP0 ENTRYLO0 register
#[inline(always)] pub fn entrylo0() -> u32 { let x; unsafe { asm!("mfc0 {x}, $2", x = out(reg) x); } x }

/// Write the COP0 ENTRYLO0 register
#[inline(always)] pub fn write_entrylo0(x: u32) { unsafe { asm!("mtc0 {x}, $2", "nop", "nop", x = in(reg) x); } }

/// Read the COP0 ENTRYLO1 register
#[inline(always)] pub fn entrylo1() -> u32 { let x; unsafe { asm!("mfc0 {x}, $3", x = out(reg) x); } x }

/// Write the COP0 ENTRYLO1 register
#[inline(always)] pub fn write_entrylo1(x: u32) { unsafe { asm!("mtc0 {x}, $3", "nop", "nop", x = in(reg) x); } }

/// Read the COP0 PAGEMASK register
#[inline(always)] pub fn pagemask() -> u32 { let x; unsafe { asm!("mfc0 {x}, $5", x = out(reg) x); } x }

/// Write the COP0 PAGEMASK register
#[inline(always)] pub fn write_pagemask(x: u32) { unsafe { asm!("mtc0 {x}, $5", "nop", "nop", x = in(reg) x); } }

/// Read the COP0 WIRED register
#[inline(always)] pub fn wired() -> u32 { let x; unsafe { asm!("mfc0 {x}, $6", x = out(reg) x); } x }

/// Write the COP0 WIRED register
#[inline(always)] pub fn write_wired(x: u32) { unsafe { asm!("mtc0 {x}, $6", "nop", "nop", x = in(reg) x); } }

/// Read the COP0 WATCHLO register
#[inline(always)] pub fn watchlo() -> u32 { let x; unsafe { asm!("mfc0 {x}, $18", x = out(reg) x); } x }

/// Write the COP0 WATCHLO register
#[inline(always)] pub fn write_watchlo(x: u32) { unsafe { asm!("mtc0 {x}, $18", x = in(reg) x); } }

/// COP0 Status bits definition. Please refer to the MIPS R4300 manual.
/// 
/// Status: interrupt enable
pub const STATUS_IE: u32 = libdragon_sys::C0_STATUS_IE;
/// Status: within exception
pub const STATUS_EXL: u32 = libdragon_sys::C0_STATUS_EXL;
/// Status: within error
pub const STATUS_ERL: u32 = libdragon_sys::C0_STATUS_ERL;

/// COP0 Cause bits definition. Please refer to MIPS R4300 manual.
///
/// Cause: exception triggered in delay slot
pub const CAUSE_BD: u32 = libdragon_sys::C0_CAUSE_BD;
/// Cause: coprocessor exception
pub const CAUSE_CE: u32 = libdragon_sys::C0_CAUSE_CE;
/// Cause: exception code
pub const CAUSE_EXC_CODE: u32 = libdragon_sys::C0_CAUSE_EXC_CODE;

/// COP0 interrupt bits definition. These are compatible bothwith mask and pending bits.
/// Status/Cause: SW interrupt 0
pub const INTERRUPT_0: u32 = libdragon_sys::C0_INTERRUPT_0;
/// Status/Cause: SW interrupt 1
pub const INTERRUPT_1: u32 = libdragon_sys::C0_INTERRUPT_1;      
/// Status/Cause: HW interrupt 2 (RCP)
pub const INTERRUPT_2: u32 = libdragon_sys::C0_INTERRUPT_2;      
/// Status/Cause: HW interrupt 3 (CART)
pub const INTERRUPT_3: u32 = libdragon_sys::C0_INTERRUPT_3;      
/// Status/Cause: HW interrupt 4 (PRENMI)
pub const INTERRUPT_4: u32 = libdragon_sys::C0_INTERRUPT_4;      
/// Status/Cause: HW interrupt 5
pub const INTERRUPT_5: u32 = libdragon_sys::C0_INTERRUPT_5;      
/// Status/Cause: HW interrupt 6
pub const INTERRUPT_6: u32 = libdragon_sys::C0_INTERRUPT_6;      
/// Status/Cause: HW interrupt 7 (Timer)
pub const INTERRUPT_7: u32 = libdragon_sys::C0_INTERRUPT_7;      

/// Status/Cause: HW interrupt 2 (RCP)
pub const INTERRUPT_RCP: u32 = libdragon_sys::C0_INTERRUPT_RCP;    
/// Status/Cause: HW interrupt 3 (CART)
pub const INTERRUPT_CART: u32 = libdragon_sys::C0_INTERRUPT_CART;   
/// Status/Cause: HW interrupt 4 (PRENMI)
pub const INTERRUPT_PRENMI: u32 = libdragon_sys::C0_INTERRUPT_PRENMI; 
/// Status/Cause: HW interrupt 7 (Timer)
pub const INTERRUPT_TIMER: u32 = libdragon_sys::C0_INTERRUPT_TIMER;  

/// Get the CE value from the COP0 status register
#[inline(always)] pub fn get_cause_ce(cr: u32) -> u32 { (cr & CAUSE_CE) >> 28 }
/// Get the exception code value from the COP0 status register value
#[inline(always)] pub fn get_cause_exc_code(sr: u32) -> u32 { (sr & CAUSE_EXC_CODE) >> 2 }

/// Flag bits for COP ENTRYLO0/ENTERYLO1 registers
/// ENTRYLO: mapping is global (all ASIDs)
pub const ENTRYLO_GLOBAL: u32 = libdragon_sys::C0_ENTRYLO_GLOBAL;
/// ENTRYLO: mapping is active (not disabled)
pub const ENTRYLO_VALID: u32 = libdragon_sys::C0_ENTRYLO_VALID;
/// ENTRYLO: mapping is writable
pub const ENTRYLO_DIRTY: u32 = libdragon_sys::C0_ENTRYLO_DIRTY;

/// Flag bits valid for COP0 INDEX register
/// INDEX: set when a TLBP probe failed to find a match
pub const INDEX_PROBE_FAILED: u32 = libdragon_sys::C0_INDEX_PROBE_FAILED;

/// COP0 TLBWI opcode
#[inline(always)] pub fn tlbwi() { unsafe { asm!("tlbwi", "nop", "nop", "nop", "nop"); } }
/// COP0 TLBWR opcode
#[inline(always)] pub fn tlbwr() { unsafe { asm!("tlbwr", "nop", "nop", "nop", "nop"); } }
/// COP0 TLBR opcode
#[inline(always)] pub fn tlbr() { unsafe { asm!("tlbr", "nop", "nop", "nop", "nop"); } }
/// COP0 TLBP opcode
#[inline(always)] pub fn tlbp() { unsafe { asm!("tlbp", "nop", "nop", "nop", "nop"); } }


