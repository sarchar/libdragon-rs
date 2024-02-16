use std::alloc::{GlobalAlloc, Layout};

pub use libdragon_sys::*;

pub struct LibdragonAllocator;

#[global_allocator]
pub static ALLOCATOR: LibdragonAllocator = LibdragonAllocator {};

use core::ffi::c_void;

unsafe impl GlobalAlloc for LibdragonAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut size = layout.size();

        // alignment is a minimum of 4 (should it be 8?)
        // It seems that something either in Rust or in C is not happy with non-4b-aligned
        // pointers to text strings.  This only deals with half the problem, since static
        // strings in Rust are still not aligned
        let alignment = std::cmp::max(4, layout.align());

        // size has to be a multiple of alignment
        size = (size + (alignment - 1)) & !(alignment - 1);

        // aligned_alloc requries size to be a multiple of alignment
        // return null on allocation error and let Rust libraries handle the error
        aligned_alloc(alignment as usize, size as usize) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }
}

