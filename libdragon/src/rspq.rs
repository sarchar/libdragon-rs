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
