
use libdragon::*;

use crate::Vertex;

use ::core::mem::offset_of;

const PLANE_SIZE: f32 = 20.0;
const PLANE_SEGMENTS: usize = 16;

pub struct Plane {
    plane_buffers: [u32; 2],
    plane_array: u32,
    plane_index_count: usize,
}

impl Plane {
    pub fn new() -> Self {
        let mut plane_buffers = [0u32; 2];
        gl::GenBuffersARB(&mut plane_buffers);

        let mut plane_array = [0u32; 1];
        gl::GenVertexArrays(&mut plane_array);
        gl::BindVertexArray(plane_array[0]);

        gl::EnableClientState(gl::VERTEX_ARRAY);
        gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
        gl::EnableClientState(gl::NORMAL_ARRAY);

        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, plane_buffers[0]);

        let stride = core::mem::size_of::<Vertex>();
        gl::VertexPointer(3, gl::FLOAT, stride, offset_of!(Vertex, position) as *const f32);
        gl::TexCoordPointer(2, gl::FLOAT, stride, offset_of!(Vertex, texcoord) as *const f32);
        gl::NormalPointer(gl::FLOAT, stride, offset_of!(Vertex, normal) as *const f32);

        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, 0);
        gl::BindVertexArray(0);

        Self {
            plane_buffers: plane_buffers,
            plane_array: plane_array[0],
            plane_index_count: 0,
        }
    }

    pub fn make_plane_mesh(&mut self) {
        let plane_vertex_count = (PLANE_SEGMENTS + 1) * (PLANE_SEGMENTS + 1);

        let vtsize = core::mem::size_of::<Vertex>();
        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, self.plane_buffers[0]);
        gl::BufferDataARB(gl::ARRAY_BUFFER_ARB, plane_vertex_count * vtsize, ::core::ptr::null::<u8>(), gl::STATIC_DRAW_ARB);

        let p0 = - PLANE_SIZE / 2.0;
        let incr = PLANE_SIZE / (PLANE_SEGMENTS as f32);

        let vertex_data = gl::MapBufferARB::<Vertex>(gl::ARRAY_BUFFER_ARB, gl::WRITE_ONLY_ARB);
        let vertices: &mut [Vertex] = unsafe { ::core::slice::from_raw_parts_mut(vertex_data, plane_vertex_count) };

        for y in 0..(PLANE_SEGMENTS + 1) {
            for x in 0..(PLANE_SEGMENTS + 1) { 
                let i = y * (PLANE_SEGMENTS + 1) + x;
                let v = &mut vertices[i];

                v.position[0] = p0 + incr * (x as f32);
                v.position[1] = 0.0;
                v.position[2] = p0 + incr * (y as f32);

                v.normal[0] = 0.0;
                v.normal[1] = 1.0;
                v.normal[2] = 0.0;

                v.texcoord[0] = x as f32;
                v.texcoord[1] = y as f32;
            }
        }
    
        gl::UnmapBufferARB(gl::ARRAY_BUFFER_ARB);
        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, 0);

        self.plane_index_count = PLANE_SEGMENTS * PLANE_SEGMENTS * 6;

        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, self.plane_buffers[1]);
        gl::BufferDataARB(gl::ELEMENT_ARRAY_BUFFER_ARB, self.plane_index_count * ::core::mem::size_of::<u16>(), 
                          ::core::ptr::null::<u8>(), gl::STATIC_DRAW_ARB);

        let index_data = gl::MapBufferARB::<u16>(gl::ELEMENT_ARRAY_BUFFER_ARB, gl::WRITE_ONLY_ARB);
        let indices: &mut [u16] = unsafe { ::core::slice::from_raw_parts_mut(index_data, self.plane_index_count) };

        for y in 0..PLANE_SEGMENTS {
            for x in 0..PLANE_SEGMENTS { 
                let i = (y * PLANE_SEGMENTS + x) * 6;

                let row_start = y * (PLANE_SEGMENTS + 1);
                let next_row_start = (y + 1) * (PLANE_SEGMENTS + 1);

                indices[i + 0] = (x + row_start) as u16;
                indices[i + 1] = (x + next_row_start) as u16;
                indices[i + 2] = (x + row_start + 1) as u16;
                indices[i + 3] = (x + next_row_start) as u16;
                indices[i + 4] = (x + next_row_start + 1) as u16;
                indices[i + 5] = (x + row_start + 1) as u16;
            }
        }

        gl::UnmapBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB);
        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, 0);
    }

    fn draw(&self) {
        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, self.plane_buffers[1]);
        gl::BindVertexArray(self.plane_array);

        gl::DrawElements(gl::TRIANGLES, self.plane_index_count, gl::UNSIGNED_SHORT, ::core::ptr::null::<u8>());

        gl::BindVertexArray(0);
        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, 0);
    }

    pub fn render(&self) {
        rdpq::debug_log_msg("Plane");

        self.draw();
    }
}


