
use libdragon::*;

pub struct PrimTest;

impl PrimTest {
    pub fn new() -> Self {
        Self {
        }
    }

    fn points(&self) {
        gl::Begin(gl::POINTS);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(1.0, -1.0);
            gl::Vertex2f(1.0, 1.0);
            gl::Vertex2f(-1.0, 1.0);
        gl::End();
    }
    
    fn lines(&self) {
        gl::Begin(gl::LINES);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(1.0, -1.0);
            gl::Vertex2f(-1.0, 0.0);
            gl::Vertex2f(1.0, 0.0);
            gl::Vertex2f(-1.0, 1.0);
            gl::Vertex2f(1.0, 1.0);
        gl::End();
    }
    
    fn line_strip(&self) {
        gl::Begin(gl::LINE_STRIP);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(1.0, -1.0);
            gl::Vertex2f(1.0, 1.0);
            gl::Vertex2f(-1.0, 1.0);
        gl::End();
    }
    
    fn line_loop(&self) {
        gl::Begin(gl::LINE_LOOP);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(1.0, -1.0);
            gl::Vertex2f(1.0, 1.0);
            gl::Vertex2f(-1.0, 1.0);
        gl::End();
    }
    
    fn triangles(&self) {
        gl::Begin(gl::TRIANGLES);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(0.0, -1.0);
            gl::Vertex2f(-1.0, 0.0);
    
            gl::Vertex2f(1.0, 1.0);
            gl::Vertex2f(1.0, 0.0);
            gl::Vertex2f(0.0, 1.0);
        gl::End();
    }
    
    fn triangle_strip(&self) {
        gl::Begin(gl::TRIANGLE_STRIP);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(1.0, -1.0);
            gl::Vertex2f(-1.0, 1.0);
            gl::Vertex2f(1.0, 1.0);
        gl::End();
    }
    
    fn triangle_fan(&self) {
        gl::Begin(gl::TRIANGLE_FAN);
            gl::Vertex2f(0.0, 0.0);
            gl::Vertex2f(-1.0, 0.0);
            gl::Vertex2f(0.0, -1.0);
            gl::Vertex2f(1.0, 0.0);
            gl::Vertex2f(0.0, 1.0);
            gl::Vertex2f(-1.0, 0.0);
        gl::End();
    }
    
    fn quads(&self) {
        gl::Begin(gl::QUADS);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(0.0, -1.0);
            gl::Vertex2f(0.0, 0.0);
            gl::Vertex2f(-1.0, 0.0);
    
            gl::Vertex2f(1.0, 1.0);
            gl::Vertex2f(0.0, 1.0);
            gl::Vertex2f(0.0, 0.0);
            gl::Vertex2f(1.0, 0.0);
        gl::End();
    }
    
    fn quad_strip(&self) {
        gl::Begin(gl::QUAD_STRIP);
            gl::Vertex2f(-1.0, -1.0);
            gl::Vertex2f(1.0, -1.0);
            gl::Vertex2f(-0.50, 0.0);
            gl::Vertex2f(0.50, 0.0);
            gl::Vertex2f(-1.0, 1.0);
            gl::Vertex2f(1.0, 1.0);
        gl::End();
    }
    
    fn polygon(&self) {
        gl::Begin(gl::POLYGON);
            gl::Vertex2f(-1.0, 0.0);
            gl::Vertex2f(-0.750, -0.750);
            gl::Vertex2f(0.0, -1.0);
            gl::Vertex2f(0.750, -0.750);
            gl::Vertex2f(1.0, 0.0);
            gl::Vertex2f(0.750, 0.750);
            gl::Vertex2f(0.0, 1.0);
            gl::Vertex2f(-0.750, 0.750);
        gl::End();
    }


    fn prim_test(&self) {
        gl::PushMatrix();
        gl::Translatef(-6.0, 1.5, 0.0);
        self.points();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(-3.0, 1.5, 0.0);
        self.lines();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(0.0, 1.5, 0.0);
        self.line_strip();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(3.0, 1.5, 0.0);
        self.line_loop();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(6.0, 1.5, 0.0);
        self.triangles();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(-6.0, -1.5, 0.0);
        self.triangle_strip();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(-3.0, -1.5, 0.0);
        self.triangle_fan();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(0.0, -1.5, 0.0);
        self.quads();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(3.0, -1.5, 0.0);
        self.quad_strip();
        gl::PopMatrix();
    
        gl::PushMatrix();
        gl::Translatef(6.0, -1.5, 0.0);
        self.polygon();
        gl::PopMatrix();
    }

    pub fn render(&self, rotation: f32) {
        rdpq::debug_log_msg("PrimTest");
        gl::PushMatrix();

        gl::Translatef(0.0, 6.0, 0.0);
        gl::Rotatef(-rotation*2.46, 0.0, 1.0, 0.0);

        // Configure alpha blending (transparency)
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        // Set a constant alpha for all vertices
        gl::Color4f(1.0, 1.0, 1.0, 0.4);

        // We want to see back faces as well
        gl::Disable(gl::CULL_FACE);

        // Transparent polygons should not write to the depth buffer
        gl::DepthMask(gl::FALSE);

        self.prim_test();

        gl::DepthMask(gl::TRUE);
        gl::Enable(gl::CULL_FACE);
        gl::Disable(gl::BLEND);

        gl::PopMatrix();
    }
}


