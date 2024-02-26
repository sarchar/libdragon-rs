
use libdragon::*;

use ::core::ptr::addr_of;
use core_maths::*;

use crate::Camera;

pub struct Skinned;

#[repr(C)]
struct SkinnedVertex {
    position: [f32; 3],
    texcoord: [f32; 2],
    normal  : [f32; 3],
    mtx_index: u8,
}

static VERTICES: [SkinnedVertex; 8] = [
    SkinnedVertex { position: [-2.0,  0.0, -1.0], texcoord: [0.0, 0.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 0 },
    SkinnedVertex { position: [-2.0,  0.0,  1.0], texcoord: [1.0, 0.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 0 },
    SkinnedVertex { position: [-1.0,  0.0, -1.0], texcoord: [0.0, 1.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 0 },
    SkinnedVertex { position: [-1.0,  0.0,  1.0], texcoord: [1.0, 1.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 0 },
    SkinnedVertex { position: [ 1.0,  0.0, -1.0], texcoord: [0.0, 2.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 1 },
    SkinnedVertex { position: [ 1.0,  0.0,  1.0], texcoord: [1.0, 2.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 1 },
    SkinnedVertex { position: [ 2.0,  0.0, -1.0], texcoord: [0.0, 3.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 1 },
    SkinnedVertex { position: [ 2.0,  0.0,  1.0], texcoord: [1.0, 3.0], normal: [ 0.0,  1.0,  0.0], mtx_index: 1 },
];

impl Skinned {
    pub fn new() -> Self {
        Self {
        }
    }

    fn draw(&self) {
        gl::Enable(gl::MATRIX_PALETTE_ARB);

        gl::EnableClientState(gl::VERTEX_ARRAY);
        gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
        gl::EnableClientState(gl::NORMAL_ARRAY);
        gl::EnableClientState(gl::MATRIX_INDEX_ARRAY_ARB);

        let stride = core::mem::size_of::<SkinnedVertex>();
        gl::VertexPointer(        3,  gl::FLOAT,           stride,   addr_of!(VERTICES[0].position));
        gl::TexCoordPointer(      2,  gl::FLOAT,           stride,   addr_of!(VERTICES[0].texcoord));
        gl::NormalPointer(            gl::FLOAT,           stride,   addr_of!(VERTICES[0].normal));
        gl::MatrixIndexPointerARB(1,  gl::UNSIGNED_BYTE,   stride,   addr_of!(VERTICES[0].mtx_index));

        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, VERTICES.len());

        gl::DisableClientState(gl::VERTEX_ARRAY);
        gl::DisableClientState(gl::TEXTURE_COORD_ARRAY);
        gl::DisableClientState(gl::NORMAL_ARRAY);
        gl::DisableClientState(gl::MATRIX_INDEX_ARRAY_ARB);

        gl::Disable(gl::MATRIX_PALETTE_ARB);
    }

    pub fn render(&self, _camera: &Camera, animation: f32) {
        rdpq::debug_log_msg("Skinned");
        gl::PushMatrix();
        
        gl::Translatef(0.0, 3.0, -6.0);
        gl::Scalef(2.0, 2.0, 2.0);

        // Set bone transforms. Note that because there is no matrix stack in palette mode, we need
        // to apply the camera transform and model transform as well for each bone.
        gl::MatrixMode(gl::MATRIX_PALETTE_ARB);

        // Set transform of first bone
        gl::CurrentPaletteMatrixARB(0);
        gl::CopyMatrixN64(gl::MODELVIEW);
        gl::Rotatef((animation*0.1).sin()*45.0, 0.0, 0.0, 1.0);

        // Set transform of second bone
        gl::CurrentPaletteMatrixARB(1);
        gl::CopyMatrixN64(gl::MODELVIEW);
        gl::Rotatef(-(animation*0.1).sin()*45.0, 0.0, 0.0, 1.0);

        gl::MatrixMode(gl::MODELVIEW);

        gl::Disable(gl::CULL_FACE);
        self.draw();
        gl::Enable(gl::CULL_FACE);

        gl::PopMatrix();
    }
}


