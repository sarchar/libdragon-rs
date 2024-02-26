
use libdragon::*;

use crate::Vertex;

use ::core::ptr::addr_of;

pub struct Cube;

static CUBE_SIZE: f32 = 3.0;

static VERTICES: [Vertex; 24] = [
    Vertex { position: [ CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], texcoord: [0.0, 0.0], normal: [ 1.0,  0.0,  0.0], color: 0xFF0000FF },
    Vertex { position: [ CUBE_SIZE,  CUBE_SIZE, -CUBE_SIZE], texcoord: [1.0, 0.0], normal: [ 1.0,  0.0,  0.0], color: 0xFF0000FF },
    Vertex { position: [ CUBE_SIZE,  CUBE_SIZE,  CUBE_SIZE], texcoord: [1.0, 1.0], normal: [ 1.0,  0.0,  0.0], color: 0xFF0000FF },
    Vertex { position: [ CUBE_SIZE, -CUBE_SIZE,  CUBE_SIZE], texcoord: [0.0, 1.0], normal: [ 1.0,  0.0,  0.0], color: 0xFF0000FF },

    // -X
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], texcoord: [0.0, 0.0], normal: [-1.0,  0.0,  0.0], color: 0x00FFFFFF },
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE,  CUBE_SIZE], texcoord: [0.0, 1.0], normal: [-1.0,  0.0,  0.0], color: 0x00FFFFFF },
    Vertex { position: [-CUBE_SIZE,  CUBE_SIZE,  CUBE_SIZE], texcoord: [1.0, 1.0], normal: [-1.0,  0.0,  0.0], color: 0x00FFFFFF },
    Vertex { position: [-CUBE_SIZE,  CUBE_SIZE, -CUBE_SIZE], texcoord: [1.0, 0.0], normal: [-1.0,  0.0,  0.0], color: 0x00FFFFFF },

    // +Y
    Vertex { position: [-CUBE_SIZE,  CUBE_SIZE, -CUBE_SIZE], texcoord: [0.0, 0.0], normal: [ 0.0,  1.0,  0.0], color: 0x00FF00FF },
    Vertex { position: [-CUBE_SIZE,  CUBE_SIZE,  CUBE_SIZE], texcoord: [0.0, 1.0], normal: [ 0.0,  1.0,  0.0], color: 0x00FF00FF },
    Vertex { position: [ CUBE_SIZE,  CUBE_SIZE,  CUBE_SIZE], texcoord: [1.0, 1.0], normal: [ 0.0,  1.0,  0.0], color: 0x00FF00FF },
    Vertex { position: [ CUBE_SIZE,  CUBE_SIZE, -CUBE_SIZE], texcoord: [1.0, 0.0], normal: [ 0.0,  1.0,  0.0], color: 0x00FF00FF },

    // -Y
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], texcoord: [0.0, 0.0], normal: [ 0.0, -1.0,  0.0], color: 0xFF00FFFF },
    Vertex { position: [ CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], texcoord: [1.0, 0.0], normal: [ 0.0, -1.0,  0.0], color: 0xFF00FFFF },
    Vertex { position: [ CUBE_SIZE, -CUBE_SIZE,  CUBE_SIZE], texcoord: [1.0, 1.0], normal: [ 0.0, -1.0,  0.0], color: 0xFF00FFFF },
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE,  CUBE_SIZE], texcoord: [0.0, 1.0], normal: [ 0.0, -1.0,  0.0], color: 0xFF00FFFF },

    // +Z
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE,  CUBE_SIZE], texcoord: [0.0, 0.0], normal: [ 0.0,  0.0,  1.0], color: 0x0000FFFF },
    Vertex { position: [ CUBE_SIZE, -CUBE_SIZE,  CUBE_SIZE], texcoord: [1.0, 0.0], normal: [ 0.0,  0.0,  1.0], color: 0x0000FFFF },
    Vertex { position: [ CUBE_SIZE,  CUBE_SIZE,  CUBE_SIZE], texcoord: [1.0, 1.0], normal: [ 0.0,  0.0,  1.0], color: 0x0000FFFF },
    Vertex { position: [-CUBE_SIZE,  CUBE_SIZE,  CUBE_SIZE], texcoord: [0.0, 1.0], normal: [ 0.0,  0.0,  1.0], color: 0x0000FFFF },

    // -Z
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], texcoord: [0.0, 0.0], normal: [ 0.0,  0.0, -1.0], color: 0xFFFF00FF },
    Vertex { position: [-CUBE_SIZE,  CUBE_SIZE, -CUBE_SIZE], texcoord: [0.0, 1.0], normal: [ 0.0,  0.0, -1.0], color: 0xFFFF00FF },
    Vertex { position: [ CUBE_SIZE,  CUBE_SIZE, -CUBE_SIZE], texcoord: [1.0, 1.0], normal: [ 0.0,  0.0, -1.0], color: 0xFFFF00FF },
    Vertex { position: [ CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], texcoord: [1.0, 0.0], normal: [ 0.0,  0.0, -1.0], color: 0xFFFF00FF },
];

static INDICES: [u16; 36] = [
     0,  1,  2,  0,  2,  3,
     4,  5,  6,  4,  6,  7,
     8,  9, 10,  8, 10, 11,
    12, 13, 14, 12, 14, 15,
    16, 17, 18, 16, 18, 19,
    20, 21, 22, 20, 22, 23,
];

impl Cube {
    pub fn new() -> Self {
        Self {
        }
    }

    fn draw(&self) {
        gl::EnableClientState(gl::VERTEX_ARRAY);
        gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
        gl::EnableClientState(gl::NORMAL_ARRAY);
        gl::EnableClientState(gl::COLOR_ARRAY);

        let stride = core::mem::size_of::<Vertex>();
        gl::VertexPointer(3, gl::FLOAT, stride, addr_of!(VERTICES[0].position));
        gl::TexCoordPointer(2, gl::FLOAT, stride, addr_of!(VERTICES[0].texcoord));
        gl::NormalPointer(gl::FLOAT, stride, addr_of!(VERTICES[0].normal));
        gl::ColorPointer(4, gl::UNSIGNED_BYTE, stride, addr_of!(VERTICES[0].color));
        gl::DrawElements(gl::TRIANGLES, INDICES.len(), gl::UNSIGNED_SHORT, addr_of!(INDICES[0]));

        gl::DisableClientState(gl::VERTEX_ARRAY);
        gl::DisableClientState(gl::TEXTURE_COORD_ARRAY);
        gl::DisableClientState(gl::NORMAL_ARRAY);
        gl::DisableClientState(gl::COLOR_ARRAY);
    }

    pub fn render(&self) {
        rdpq::debug_log_msg("Cube");
        gl::PushMatrix();
        gl::Translatef(0.0 ,-1.0, 0.0);

        // Apply vertex color as material color.
        // Because the cube has colors set per vertex, we can color each face seperately
        gl::Enable(gl::COLOR_MATERIAL);

        // Apply to ambient and diffuse material properties
        gl::ColorMaterial(gl::FRONT_AND_BACK, gl::AMBIENT_AND_DIFFUSE);

        self.draw();
        
        gl::Disable(gl::COLOR_MATERIAL);

        gl::PopMatrix();
    }
}


