use libdragon::gl;
use libdragon::glu;

pub struct Camera {
    pub distance: f32,
    pub rotation: f32
}

impl Camera {
    pub fn transform(&self) {
        gl::LoadIdentity();
        glu::LookAt(0.0, -self.distance, -self.distance,
                    0.0,            0.0,            0.0,
                    0.0,            1.0,            0.0);
        gl::Rotatef(self.rotation, 0.0, 1.0, 0.0);
    }
}

