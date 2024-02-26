
use libdragon::*;

pub struct Decal;

impl Decal {
    pub fn new() -> Self {
        Self {
        }
    }

    fn draw_quad(&self) {
        gl::Begin(gl::TRIANGLE_STRIP);
            gl::Normal3f(0.0, 1.0, 0.0);
            gl::TexCoord2f(0.0, 0.0);
            gl::Vertex3f(-0.5, 0.0, -0.5);
            gl::TexCoord2f(0.0, 1.0);
            gl::Vertex3f(-0.5, 0.0, 0.5);
            gl::TexCoord2f(1.0, 0.0);
            gl::Vertex3f(0.5, 0.0, -0.5);
            gl::TexCoord2f(1.0, 1.0);
            gl::Vertex3f(0.5, 0.0, 0.5);
        gl::End();
    }

    pub fn render(&self) {
        rdpq::debug_log_msg("Decal");

        gl::PushMatrix();
        gl::Translatef(0.0, 0.0, 6.0);

        gl::Rotatef(35.0, 0.0, 1.0, 0.0);
        gl::Scalef(3.0, 3.0, 3.0);

        // Decals are drawn with the depth func set to gl::EQUAL. Note that gl::PolygonOffset is not supported on N64.
        gl::DepthFunc(gl::EQUAL);

        // Disable writing to depth buffer, because the depth value will be the same anyway
        gl::DepthMask(gl::FALSE);

        // Apply vertex color as material color.
        // This time, we set one vertex color for the entire model.
        gl::Enable(gl::COLOR_MATERIAL);
        gl::Color4f(1.0, 0.4, 0.2, 0.5);

        self.draw_quad();

        gl::Disable(gl::COLOR_MATERIAL);
        gl::DepthMask(gl::TRUE);
        gl::DepthFunc(gl::LESS);
        
        gl::PopMatrix();
    }
}


