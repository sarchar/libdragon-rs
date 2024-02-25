use crate::*;

use sprite::Sprite;

#[derive(Debug, Clone, Copy)]
pub enum Mirror {
    Disabled,
    X,
    Y,
    XY,
}

impl Into<libdragon_sys::mirror_t> for Mirror {
    fn into(self) -> libdragon_sys::mirror_t {
        match self {
            Mirror::Disabled => libdragon_sys::mirror_t_MIRROR_DISABLED,
            Mirror::X => libdragon_sys::mirror_t_MIRROR_X,
            Mirror::Y => libdragon_sys::mirror_t_MIRROR_Y,
            Mirror::XY => libdragon_sys::mirror_t_MIRROR_XY,
        }
    }
}

pub fn load_texture(texslot: u32, texloc: u32, mirror: Mirror, sprite: &Sprite) -> u32 {
    unsafe {
        libdragon_sys::rdp_load_texture(texslot, texloc, mirror.into(), sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_t) as u32
    }
}

pub fn load_texture_stride(texslot: u32, texloc: u32, mirror: Mirror, sprite: &Sprite, offset: i32) -> u32 {
    unsafe {
        libdragon_sys::rdp_load_texture_stride(texslot, texloc, mirror.into(), sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_t, offset) as u32
    }
}

pub fn draw_sprite(texslot: u32, x: i32, y: i32, mirror: Mirror) {
    unsafe {
        libdragon_sys::rdp_draw_sprite(texslot, x, y, mirror.into());
    }
}
