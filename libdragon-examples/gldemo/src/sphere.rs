
use libdragon::*;

use crate::Vertex;

use ::core::mem::offset_of;
use core::f32::consts::PI;
use core_maths::*;

const SPHERE_RADIUS: f32 = 20.0;
const SPHERE_MIN_RINGS: usize = 4;
const SPHERE_MAX_RINGS: usize = 64;
const SPHERE_MIN_SEGMENTS: usize = 4;
const SPHERE_MAX_SEGMENTS: usize = 64;

pub struct Sphere {
    sphere_buffers: [u32; 2],
    sphere_array: u32,
    sphere_rings: usize,
    sphere_segments: usize,
    sphere_index_count: usize,
    sphere_list: u32,
}

impl Sphere {
    pub fn new() -> Self {
        let mut sphere_buffers = [0u32; 2];
        gl::GenBuffersARB(&mut sphere_buffers);
        let sphere_rings = 8;
        let sphere_segments = 8;

        let mut sphere_array = [0u32; 1];
        gl::GenVertexArrays(&mut sphere_array);
        gl::BindVertexArray(sphere_array[0]);

        gl::EnableClientState(gl::VERTEX_ARRAY);
        gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
        gl::EnableClientState(gl::NORMAL_ARRAY);
        
        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, sphere_buffers[0]);

        let stride = core::mem::size_of::<Vertex>();
        gl::VertexPointer(3, gl::FLOAT, stride, offset_of!(Vertex, position) as *const f32);
        gl::TexCoordPointer(2, gl::FLOAT, stride, offset_of!(Vertex, texcoord) as *const f32);
        gl::NormalPointer(gl::FLOAT, stride, offset_of!(Vertex, normal) as *const f32);
        
        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, 0);

        gl::BindVertexArray(0);

        let sphere_list = gl::GenLists(1);

        Self {
            sphere_buffers: sphere_buffers,
            sphere_array: sphere_array[0],
            sphere_rings: sphere_rings,
            sphere_segments: sphere_segments,
            sphere_index_count: 0,
            sphere_list: sphere_list,
        }
    }

    fn make_sphere_vertex(&self, ring: usize, segment: usize) -> Vertex {
        let r = SPHERE_RADIUS;
        let phi = (2.0 * PI * (segment as f32)) / (self.sphere_segments as f32);
        let theta = (PI * (ring as f32)) / ((self.sphere_rings + 1) as f32);

        let sintheta = theta.sin();

        let x = r * phi.cos() * sintheta;
        let y = r * phi.sin() * sintheta;
        let z = r * theta.cos();

        let position = [x, y, z];

        let mag2 = x*x + y*y + z*z;
        let mag = mag2.sqrt();
        let inv_m = 1.0 / mag;

        let normal = [-x * inv_m, -y * inv_m, -z * inv_m];

        Vertex {
            position: position,
            normal: normal,
            texcoord: [
                if (segment & 1) != 0 { 1.0 } else { 0.0 },
                if (ring & 1) != 0 { 1.0 } else { 0.0 },
            ],
            color: 0,
        }
    }

    pub fn make_sphere_mesh(&mut self) {
        let sphere_vertex_count = self.sphere_rings * self.sphere_segments + 2;

        let vtsize = core::mem::size_of::<Vertex>();
        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, self.sphere_buffers[0]);
        gl::BufferDataARB(gl::ARRAY_BUFFER_ARB, sphere_vertex_count * vtsize, ::core::ptr::null::<u8>(), gl::STATIC_DRAW_ARB);

        let vertex_data = gl::MapBufferARB(gl::ARRAY_BUFFER_ARB, gl::WRITE_ONLY_ARB);
        let vertices: &mut [Vertex] = unsafe { ::core::slice::from_raw_parts_mut(vertex_data, sphere_vertex_count) };

        vertices[0] = self.make_sphere_vertex(0, 0);
        
        for r in 0..self.sphere_rings {
            for s in 0..self.sphere_segments {
                vertices[r * self.sphere_segments + s + 1] = self.make_sphere_vertex(r + 1, s);
            }
        }

        vertices[sphere_vertex_count - 1] = self.make_sphere_vertex(self.sphere_rings + 1, 0);
        
        gl::UnmapBufferARB(gl::ARRAY_BUFFER_ARB);
        gl::BindBufferARB(gl::ARRAY_BUFFER_ARB, 0);

        let fan_index_count = self.sphere_segments + 2;
        let ring_index_count = self.sphere_segments * 6;

        self.sphere_index_count = fan_index_count * 2 + ring_index_count * (self.sphere_rings - 1);

        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, self.sphere_buffers[1]);
        gl::BufferDataARB(gl::ELEMENT_ARRAY_BUFFER_ARB, self.sphere_index_count * ::core::mem::size_of::<u16>(), 
                          ::core::ptr::null::<u8>(), gl::STATIC_DRAW_ARB);

        let index_data = gl::MapBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, gl::WRITE_ONLY_ARB);
        let indices: &mut [u16] = unsafe { ::core::slice::from_raw_parts_mut(index_data, self.sphere_index_count) };

        // Ends
        for i in 0..(fan_index_count - 1) {
            indices[i] = i as u16;
            indices[fan_index_count + i] = (sphere_vertex_count - i - 1) as u16;
        }

        indices[self.sphere_segments + 1] = 1;
        indices[fan_index_count + self.sphere_segments + 1] = (sphere_vertex_count - 2) as u16;

        let rings_index_offset = fan_index_count * 2;

        // Rings
        for r in 0..(self.sphere_rings - 1) {
            let ring_indices = &mut indices[rings_index_offset + r * ring_index_count..];
            let first_ring_index = 1 + r * self.sphere_segments;
            let second_ring_index = 1 + (r + 1) * self.sphere_segments;

            for s in 0..self.sphere_segments {
                let next_segment = (s + 1) % self.sphere_segments;
                ring_indices[s * 6 + 0] = (first_ring_index + s) as u16;
                ring_indices[s * 6 + 1] = (second_ring_index + s) as u16;
                ring_indices[s * 6 + 2] = (first_ring_index + next_segment) as u16;
                ring_indices[s * 6 + 3] = (second_ring_index + s) as u16;
                ring_indices[s * 6 + 4] = (second_ring_index + next_segment) as u16;
                ring_indices[s * 6 + 5] = (first_ring_index + next_segment) as u16;
            }
        }

        gl::UnmapBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB);
        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, 0);

        gl::NewList(self.sphere_list, gl::COMPILE);
        self.draw_sphere_internal();
        gl::EndList();
    }

    fn draw_sphere_internal(&self) {
        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, self.sphere_buffers[1]);
        gl::BindVertexArray(self.sphere_array);

        gl::DrawElements(gl::TRIANGLE_FAN, self.sphere_segments + 2, gl::UNSIGNED_SHORT, 0 as *const u8);
        gl::DrawElements(gl::TRIANGLE_FAN, self.sphere_segments + 2, gl::UNSIGNED_SHORT, 
                         ((self.sphere_segments + 2) * ::core::mem::size_of::<u16>()) as *const u8);
        gl::DrawElements(gl::TRIANGLES, (self.sphere_rings - 1) * (self.sphere_segments * 6), gl::UNSIGNED_SHORT, 
                         ((self.sphere_segments + 2) * 2 * ::core::mem::size_of::<u16>()) as *const u8);

        gl::BindVertexArray(0);
        gl::BindBufferARB(gl::ELEMENT_ARRAY_BUFFER_ARB, 0);
    }

    fn draw(&self) {
        gl::CallList(self.sphere_list);
    }

    pub fn render(&self, rotation: f32) {
        rdpq::debug_log_msg("Sphere");

        gl::PushMatrix();

        gl::Rotatef(rotation*0.23, 1.0, 0.0, 0.0);
        gl::Rotatef(rotation*0.98, 0.0, 0.0, 1.0);
        gl::Rotatef(rotation*1.71, 0.0, 1.0, 0.0);

        // We want to see back faces instead of front faces, because the camera will be inside the sphere
        gl::CullFace(gl::FRONT);
        self.draw();
        gl::CullFace(gl::BACK);

        gl::PopMatrix();
    }

    pub fn increment_rings(&mut self) {
        if self.sphere_rings < SPHERE_MAX_RINGS {
            self.sphere_rings += 1;
        }
    }

    pub fn increment_segments(&mut self) {
        if self.sphere_segments < SPHERE_MAX_SEGMENTS {
            self.sphere_segments += 1;
        }
    }

    pub fn decrement_rings(&mut self) {
        if self.sphere_rings > SPHERE_MIN_RINGS {
            self.sphere_rings -= 1;
        }
    }

    pub fn decrement_segments(&mut self) {
        if self.sphere_segments > SPHERE_MIN_SEGMENTS {
            self.sphere_segments -= 1;
        }
    }
}


