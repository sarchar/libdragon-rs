
use libdragon::*;

use crate::Vec4;

pub struct Vec {
    rsp_vec: rsp::RspUcode,
}

impl Vec {
    const CMD_LOAD : u32 = 0x00;
    const CMD_STORE: u32 = 0x01;
    const CMD_TRANS: u32 = 0x02;

    pub fn new() -> Self {
        define_rsp_ucode!(rsp_vec);

        eprintln!("rsp_vec_code = {:08p}", rsp_vec.rsp_ucode.code);
        eprintln!("rsp_vec_code_end = {:08p}", rsp_vec.rsp_ucode.code_end);

        eprintln!("rsp_vec_data = {:08p}", rsp_vec.rsp_ucode.data);
        eprintln!("rsp_vec_data_end = {:08p}", rsp_vec.rsp_ucode.data_end);

        rspq::init();
    
        rsp_vec.get_state_mut::<u8>(0x400).uncached_mut().fill(0);
    
        rsp_vec.register().expect("failed to register overlay");

        eprintln!("rsp_vec overlay ID = 0x{:08X}", rsp_vec.id().unwrap());

        Self {
            rsp_vec: rsp_vec,
        }
    }

    pub fn load(&self, slot: usize, src: &[VecSlot]) {
        let phys_addr = (src.as_ptr() as *const _ as u32) & 0x00FF_FFFF;
        let arg = (((((src.len() * core::mem::size_of::<VecSlot>()) - 1) & 0xFFF) as u32) << 16)
                  | (((slot * core::mem::size_of::<VecSlot>()) & 0xFF0) as u32);

        let mut w = rspq::Writer::begin(self.rsp_vec.id().unwrap(), Vec::CMD_LOAD, 2);
        w.arg(phys_addr);
        w.arg(arg);
        w.end();
    }

    pub fn store(&self, dest: &[VecSlot], slot: usize) {
        let phys_addr = (dest.as_ptr() as *const _ as u32) & 0x00FF_FFFF;
        let arg = (((((dest.len() * core::mem::size_of::<VecSlot>()) - 1) & 0xFFF) as u32) << 16)
                  | (((slot * core::mem::size_of::<VecSlot>()) & 0xFF0) as u32);

        let mut w = rspq::Writer::begin(self.rsp_vec.id().unwrap(), Vec::CMD_STORE, 2);
        w.arg(phys_addr);
        w.arg(arg);
        w.end();
    }

    pub fn transform(&self, dest: usize, mtx: usize, vec: usize) {
        let dest = ((dest * core::mem::size_of::<VecSlot>()) & 0xFF0) as u32;
        let arg = ((((mtx * core::mem::size_of::<VecSlot>()) & 0xFF0) as u32) << 16)
                   | (((vec * core::mem::size_of::<VecSlot>()) & 0xFF0) as u32);
        
        let mut w = rspq::Writer::begin(self.rsp_vec.id().unwrap(), Vec::CMD_TRANS, 2);
        w.arg(dest);
        w.arg(arg);
        w.end();
    }
}

impl Drop for Vec {
    fn drop(&mut self) {
        self.rsp_vec.unregister();
        rspq::close();
    }
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct VecSlot {
    pub i: [i16; 8],
    pub f: [u16; 8],
}

impl Default for VecSlot {
    fn default() -> Self {
        Self {
            i: [0; 8],
            f: [0; 8],
        }
    }
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct VecMtx {
    pub c: [VecSlot; 2],
}

impl Default for VecMtx {
    fn default() -> Self {
        Self {
            c: [VecSlot::default(); 2],
        }
    }
}

pub fn floats_to_vectors(dest: &mut [VecSlot], source: &[Vec4]) {
    for i in 0..source.len() {
        for j in 0..4 {
            let fixed = (source[i].v[j] * 65536.0) as i32;
            // two Vec4's per one VecSlot
            let dest_index = i / 2;
            let lane_index = 4*(i % 2) + j;
            dest[dest_index].i[lane_index] = (((fixed as u32) & 0xFFFF_0000) >> 16) as i16;
            dest[dest_index].f[lane_index] =  ((fixed as u32) & 0x0000_FFFF)        as u16;
        }
    }
}

pub fn vectors_to_floats(dest: &mut [Vec4], source: &[VecSlot]) {
    for i in 0..dest.len() {
        for j in 0..4 {
            let src_index = i / 2;
            let lane_index = 4*(i % 2) + j;
            let integer = source[src_index].i[lane_index];
            let fraction = source[src_index].f[lane_index];
            let fixed = (((integer as u32) << 16) | (fraction as u32)) as i32;
            dest[i].v[j] = (fixed as f32) / 65536.0;
        }
    }
}

