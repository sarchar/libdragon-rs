#![no_std]
#![no_main]

const NUM_VECTOR_SLOTS: usize = 16;
const NUM_VECTORS: usize = NUM_VECTOR_SLOTS * 2;
const NUM_MATRICES: usize = 4;
const MTX_SLOT: usize = 30;

use core_maths::*;
use libdragon::*;

mod vec;
use vec::{VecSlot, VecMtx};

fn print_vectors(arr: &[Vec4]) {
    for v in arr {
        println!("{:11.4}  {:11.4}  {:11.4}  {:11.4}", v.v[0], v.v[1], v.v[2], v.v[3]);
    }
}

fn print_output(header: &str, dest: &mut [Vec4], source: &[VecSlot]) {
    rspq::wait();
    println!("{}", header);
    vec::vectors_to_floats(dest, source);
    print_vectors(dest);
}

#[no_mangle]
extern "C" fn main() -> ! {
    console::init();
    console::set_debug(true);
    debug::init_features(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    let vec = vec::Vec::new();

    // initialized to zero, but still in cache, so re-initialize
    let mut input_vectors_cached = [VecSlot::default(); NUM_VECTOR_SLOTS];
    let mut output_vectors_cached = [VecSlot::default(); NUM_VECTOR_SLOTS];
    let mut matrices_cached = [VecMtx::default(); NUM_MATRICES];

    input_vectors_cached.cache_hit_invalidate(true);
    let input_vectors = input_vectors_cached.uncached_mut();

    output_vectors_cached.cache_hit_invalidate(true);
    let output_vectors = output_vectors_cached.uncached_mut();

    matrices_cached.cache_hit_invalidate(true);
    let matrices = matrices_cached.uncached_mut();

    // Initialize input vectors
    let mut vectors = vec![];
    for z in 0..2 {
        for y in 0..4 {
            for x in 0..4 {
                vectors.push(Vec4 {
                    v: [x as f32, y as f32, z as f32, 1.0],
                });
            }
        }
    }

    assert!(vectors.len() == NUM_VECTORS);

    // Convert to fixed point format required by the overlay
    vec::floats_to_vectors(input_vectors, &vectors);

    // Initialize matrices
    let identity = Mat4x4::identity();
    let scale = Mat4x4::scale(0.5, 2.0, 1.1);
    let rotation = Mat4x4::rotate_y(4.0);
    let translation = Mat4x4::translate(0.0, -3.1, 8.0);

    vec::floats_to_vectors(&mut matrices[0].c, &identity.m);
    vec::floats_to_vectors(&mut matrices[1].c, &scale.m);
    vec::floats_to_vectors(&mut matrices[2].c, &rotation.m);
    vec::floats_to_vectors(&mut matrices[3].c, &translation.m);

    // This block defines a reusable sequence of commands that could
    // be understood as a "function" that will transform the vectors
    // in slots 0-15 with the matrix in slots 30-31.
    // It is repeatedly called further down to transform an array of vectors with
    // different matrices.
    rspq::block_begin();
    vec.load(0, input_vectors);
    for i in 0..NUM_VECTOR_SLOTS {
        vec.transform(i, MTX_SLOT, i);
    }
    vec.store(output_vectors, 0);
    let transform_vectors_block = rspq::block_end(); 

    // Print inputs first for reference
    println!("Input vectors:");
    print_vectors(&vectors);

    // Scale
    vec.load(MTX_SLOT, &matrices[1].c);
    transform_vectors_block.run();
    print_output("Scaled:", &mut vectors, &output_vectors);

    // Rotate
    vec.load(MTX_SLOT, &matrices[2].c);
    transform_vectors_block.run();
    print_output("Rotated:", &mut vectors, &output_vectors);

    // Translate
    vec.load(MTX_SLOT, &matrices[3].c);
    transform_vectors_block.run();
    print_output("Translated:", &mut vectors, &output_vectors);

    // Typical affine matrix: first scale, then rotate, then translate
    // Load 3 matrices starting at slot 16
    vec.load(16, &matrices[1].c);
    vec.load(18, &matrices[2].c);
    vec.load(20, &matrices[3].c);

    // Perform matrix composition by multiplying them together (transforming column vectors)
    // The resulting matrix is written to MTX_SLOT
    vec.transform(22, 18, 16);         // Rotation * scale (first two columns)
    vec.transform(23, 18, 17);         // Rotation * scale (last two columns)
    vec.transform(MTX_SLOT+0, 20, 22); // Translation * rotation * scale (first two columns)
    vec.transform(MTX_SLOT+1, 20, 23); // Translation * rotation * scale (last two columns)
    transform_vectors_block.run();
    print_output("Combined:", &mut vectors, &output_vectors);

    drop(transform_vectors_block);
    drop(vec);

    loop {};
}

#[derive(Debug)]
pub struct Vec4 {
    v: [f32; 4],
}

#[derive(Debug)]
pub struct Mat4x4 {
    m: [Vec4; 4],
}

impl Mat4x4 {
    fn identity() -> Self {
        Self {
            m: [
                Vec4 { v: [1.0, 0.0, 0.0, 0.0] },
                Vec4 { v: [0.0, 1.0, 0.0, 0.0] },
                Vec4 { v: [0.0, 0.0, 1.0, 0.0] },
                Vec4 { v: [0.0, 0.0, 0.0, 1.0] },
            ],
        }
    }

    fn scale(xs: f32, ys: f32, zs: f32) -> Self {
        Self {
            m: [
                Vec4 { v: [xs , 0.0, 0.0, 0.0] },
                Vec4 { v: [0.0, ys , 0.0, 0.0] },
                Vec4 { v: [0.0, 0.0, zs , 0.0] },
                Vec4 { v: [0.0, 0.0, 0.0, 1.0] },
            ],
        }
    }

    fn translate(xt: f32, yt: f32, zt: f32) -> Self {
        Self {
            m: [
                Vec4 { v: [1.0, 0.0, 0.0, 0.0] },
                Vec4 { v: [0.0, 1.0, 0.0, 0.0] },
                Vec4 { v: [0.0, 0.0, 1.0, 0.0] },
                Vec4 { v: [xt , yt , zt , 1.0] },
            ],
        }
    }

    fn _rotate_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            m: [
                Vec4 { v: [1.0, 0.0, 0.0, 0.0] },
                Vec4 { v: [0.0,  c ,  s , 0.0] },
                Vec4 { v: [0.0, -s ,  c , 0.0] },
                Vec4 { v: [0.0, 0.0, 0.0, 1.0] },
            ],
        }
    }

    fn rotate_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            m: [
                Vec4 { v: [ c , 0.0, -s , 0.0] },
                Vec4 { v: [0.0, 1.0, 0.0, 0.0] },
                Vec4 { v: [ s , 0.0,  c , 0.0] },
                Vec4 { v: [0.0, 0.0, 0.0, 1.0] },
            ],
        }
    }

    fn _rotate_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            m: [
                Vec4 { v: [ c ,  s , 0.0, 0.0] },
                Vec4 { v: [-s ,  c , 0.0, 0.0] },
                Vec4 { v: [0.0, 0.0, 1.0, 0.0] },
                Vec4 { v: [0.0, 0.0, 0.0, 1.0] },
            ],
        }
    }
}

