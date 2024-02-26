#![allow(non_snake_case)]

// GLU
#[inline(always)]
pub fn LookAt(eyex: f32, eyey: f32, eyez: f32,
              centerx: f32, centery: f32, centerz: f32,
              upx: f32, upy: f32, upz: f32) {
    unsafe {
        libdragon_sys::gluLookAt(eyex, eyey, eyez,
                                 centerx, centery, centerz,
                                 upx, upy, upz);
    }
}
