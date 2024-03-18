use crate::*;

/// PI DMA: DRAM address register
pub const PI_DRAM_ADDR: Register<u32> = Register { address: 0xA460_0000 as *mut _ };
/// PI DMA: cartridge address register
pub const PI_CART_ADDR: Register<u32> = Register { address: 0xA460_0004 as *mut _ };
/// PI DMA: reard length register
pub const PI_RD_LEN   : Register<u32> = Register { address: 0xA460_0008 as *mut _ };
/// PI DMA: write length register
pub const PI_WR_LEN   : Register<u32> = Register { address: 0xA460_000C as *mut _ };
/// PI DMA: status register
pub const PI_STATUS   : Register<u32> = Register { address: 0xA460_0010 as *mut _ };

/// Start writing data to a peripheral through PI DMA (low-level)
///
/// Rust: Length of the transfer is the slice length times the element size
///
/// Unsafe: this function is unsafe because the memory referenced by `ram_address` will be written
/// to after this function returns.
///
/// See [`dma_write_raw_async`](libdragon_sys::dma_write_raw_async) for details.
#[inline] pub unsafe fn write_raw_async<T>(ram_address: &[T], pi_address: u32) {
    let len = ram_address.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::dma_write_raw_async(ram_address.as_ptr() as *const _, pi_address, len as u32);
    }
}

/// Write to a peripheral
///
/// Rust: Length of the transfer is the slice length times the element size
///
/// See [`dma_write`](libdragon_sys::dma_write) for details.
#[inline] pub fn write<T>(ram_address: &[T], pi_address: u32) {
    let len = ram_address.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::dma_write(ram_address.as_ptr() as *const _, pi_address, len as u32);
    }
}

/// Start reading data from a peripheral through PI DMA (low-level)
///
/// Rust: Length of the transfer is the slice length times the element size
///
/// Unsafe: this function is unsafe because the memory referenced by `ram_address` will be read
/// from after this function returns.
///
/// See [`dma_read_raw_async`](libdragon_sys::dma_read_raw_async) for details.
#[inline] pub unsafe fn read_raw_async<T>(ram_address: &mut [T], pi_address: u32) {
    let len = ram_address.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::dma_read_raw_async(ram_address.as_mut_ptr() as *mut _, pi_address, len as u32);
    }
}

/// Start reading data from a peripheral through PI DMA (low-level)
///
/// Rust: Length of the transfer is the slice length times the element size
///
/// Unsafe: this function is unsafe because the memory referenced by `ram_address` will be read
/// from after this function returns.
///
/// See [`dma_read_async`](libdragon_sys::dma_read_async) for details.
#[inline] pub unsafe fn read_async<T>(ram_address: &mut [T], pi_address: u32) {
    let len = ram_address.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::dma_read_async(ram_address.as_mut_ptr() as *mut _, pi_address, len as u32);
    }
}

/// Read data from a peripheral through PI DMA, waiting for completion
///
/// Rust: Length of the transfer is the slice length times the element size
///
/// See [`dma_read`](libdragon_sys::dma_read) for details.
#[inline] pub fn read<T>(ram_address: &mut [T], pi_address: u32) {
    let len = ram_address.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::dma_read(ram_address.as_ptr() as *mut _, pi_address, len as u32);
    }
}

/// Wait until an async DMA or I/O transfer is finished.
///
/// See [`dma_wait`](libdragon_sys::dma_wait) for details.
#[inline] pub fn wait() { unsafe { libdragon_sys::dma_wait(); } }

/// Read a 32-bit integer from a peripheral using the CPU.
///
/// See [`io_read`](libdragon_sys::io_read) for details
#[inline] pub fn io_read(pi_address: u32) -> u32 { unsafe { libdragon_sys::io_read(pi_address) } }

/// Write a 32-bit integer to a peripheral using the CPU.
///
/// See [`io_write`](libdragon_sys::io_write) for details
#[inline] pub fn io_write(pi_address: u32, data: u32) { unsafe { libdragon_sys::io_write(pi_address, data); } }

/// Check whether the specified PI address can be accessed doing I/O from CPU
///
/// See [`io_accessible`](libdragon_sys::io_accessible) for details.
#[inline] pub fn io_accessible(pi_address: u32) -> bool { unsafe { libdragon_sys::io_accessible(pi_address) } }
