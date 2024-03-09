use crate::*;

use sprite::Sprite;

/// RDP registers

/// DP start register
pub const DP_START    : Register<u32> = Register { address: 0xA410_0000 as *mut _ };
/// DP end register
pub const DP_END      : Register<u32> = Register { address: 0xA410_0004 as *mut _ };
/// DP current register
pub const DP_CURRENT  : Register<u32> = Register { address: 0xA410_0008 as *mut _ };
/// DP status register
pub const DP_STATUS   : Register<u32> = Register { address: 0xA410_000C as *mut _ };
/// DP clock counter
pub const DP_CLOCK    : Register<u32> = Register { address: 0xA410_0010 as *mut _ };
/// DP command buffer busy
pub const DP_BUSY     : Register<u32> = Register { address: 0xA410_0014 as *mut _ };
/// DP pipe busy
pub const DP_PIPE_BUSY: Register<u32> = Register { address: 0xA410_0018 as *mut _ };
/// DP tmem busy
pub const DP_TMEM_BUSY: Register<u32> = Register { address: 0xA410_001C as *mut _ };

/// Status bits (reading DP_STATUS)

/// DP is using DMEM DMA
pub const DP_STATUS_DMEM_DMA    : u32 = libdragon_sys::DP_STATUS_DMEM_DMA     as u32;
/// DP is frozen
pub const DP_STATUS_FREEZE      : u32 = libdragon_sys::DP_STATUS_FREEZE       as u32;
/// DP is flushed
pub const DP_STATUS_FLUSH       : u32 = libdragon_sys::DP_STATUS_FLUSH        as u32;
/// DP GCLK is busy
pub const DP_STATUS_GCLK_ALIVE  : u32 = libdragon_sys::DP_STATUS_GCLK_ALIVE   as u32;
/// DP TMEM is busy
pub const DP_STATUS_TMEM_BUSY   : u32 = libdragon_sys::DP_STATUS_TMEM_BUSY    as u32;
/// DP pipeline is busy
pub const DP_STATUS_PIPE_BUSY   : u32 = libdragon_sys::DP_STATUS_PIPE_BUSY    as u32;
/// DP command unit is busy
pub const DP_STATUS_BUSY        : u32 = libdragon_sys::DP_STATUS_BUSY         as u32;
/// DP command buffer is ready
pub const DP_STATUS_BUFFER_READY: u32 = libdragon_sys::DP_STATUS_BUFFER_READY as u32;
/// DP DMA is busy
pub const DP_STATUS_DMA_BUSY    : u32 = libdragon_sys::DP_STATUS_DMA_BUSY     as u32;
/// DP command end register is valid
pub const DP_STATUS_END_VALID   : u32 = libdragon_sys::DP_STATUS_END_VALID    as u32;
/// DP command start register is valid
pub const DP_STATUS_START_VALID : u32 = libdragon_sys::DP_STATUS_START_VALID  as u32;

/// Write status bits (writing DP_STATUS)

/// clear [DP_STATUS_DMEM_DMA] bit
pub const DP_WSTATUS_RESET_XBUS_DMEM_DMA: u32 = libdragon_sys::DP_WSTATUS_RESET_XBUS_DMEM_DMA;
/// set [DP_STATUS_DMEM_DMA] bit
pub const DP_WSTATUS_SET_XBUS_DMEM_DMA  : u32 = libdragon_sys::DP_WSTATUS_SET_XBUS_DMEM_DMA;
/// clear [DP_STATUS_FREEZE] bit
pub const DP_WSTATUS_RESET_FREEZE       : u32 = libdragon_sys::DP_WSTATUS_RESET_FREEZE;
/// set [DP_STATUS_FREEZE] bit
pub const DP_WSTATUS_SET_FREEZE         : u32 = libdragon_sys::DP_WSTATUS_SET_FREEZE;
/// clear [DP_STATUS_FLUSH] bit
pub const DP_WSTATUS_RESET_FLUSH        : u32 = libdragon_sys::DP_WSTATUS_RESET_FLUSH;
/// set [DP_STATUS_FLUSH] bit
pub const DP_WSTATUS_SET_FLUSH          : u32 = libdragon_sys::DP_WSTATUS_SET_FLUSH;
/// clear TMEM counter
pub const DP_WSTATUS_RESET_TMEM_COUNTER : u32 = libdragon_sys::DP_WSTATUS_RESET_TMEM_COUNTER;
/// clear PIPE counter
pub const DP_WSTATUS_RESET_PIPE_COUNTER : u32 = libdragon_sys::DP_WSTATUS_RESET_PIPE_COUNTER;
/// clear CMD counter
pub const DP_WSTATUS_RESET_CMD_COUNTER  : u32 = libdragon_sys::DP_WSTATUS_RESET_CMD_COUNTER;
/// clear CLOCK counter
pub const DP_WSTATUS_RESET_CLOCK_COUNTER: u32 = libdragon_sys::DP_WSTATUS_RESET_CLOCK_COUNTER;

/// Mirror settings for textures
#[derive(Debug, Clone, Copy)]
pub enum Mirror {
    /// Disabled texture mirroring
    Disabled,
    /// Enable texture mirroring on x axis
    X,
    /// Enable texture mirroring on y axis
    Y,
    /// Enable texture mirroring on both x & y axis
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

/// Caching strategy for loaded textures
#[derive(Debug, Clone, Copy)]
pub enum Flush {
    /// Textures are assumed to be pre-flushed
    None,
    /// Cache will be flushed on all incoming textures
    Automatic
}

impl Into<libdragon_sys::flush_t> for Flush {
    fn into(self) -> libdragon_sys::flush_t {
        match self {
            Flush::None => libdragon_sys::flush_t_FLUSH_STRATEGY_NONE,
            Flush::Automatic => libdragon_sys::flush_t_FLUSH_STRATEGY_AUTOMATIC,
        }
    }
}

/// Enable display of 2D filled (untextured) triangles, with possible alpha blending.
///
/// See [`rdp_enable_blend_fill`](libdragon_sys::rdp_enable_blend_fill]
pub fn enable_blend_fill() { unsafe { libdragon_sys::rdp_enable_blend_fill(); } }

/// Load a sprite into RDP TMEM
///
/// See [`rdp_load_texture`](libdragon_sys::rdp_load_texture)
pub fn load_texture(texslot: u32, texloc: u32, mirror: Mirror, sprite: &Sprite) -> u32 {
    unsafe {
        libdragon_sys::rdp_load_texture(texslot, texloc, mirror.into(), sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_t) as u32
    }
}

/// Load part of a sprite into RDP TMEM
///
/// See [`rdp_load_texture_stride`](libdragon_sys::rdp_load_texture_stride)
pub fn load_texture_stride(texslot: u32, texloc: u32, mirror: Mirror, sprite: &Sprite, offset: i32) -> u32 {
    unsafe {
        libdragon_sys::rdp_load_texture_stride(texslot, texloc, mirror.into(), sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_t, offset) as u32
    }
}

/// Draw a textured rectangle
///
/// See [`rdp_draw_textured_rectangle`](libdragon_sys::rdp_draw_textured_rectangle)
pub fn draw_textured_rectangle(texslot: u32, tx: i32, ty: i32, bx: i32, by: i32, mirror: Mirror) {
    unsafe {
        libdragon_sys::rdp_draw_textured_rectangle(texslot, tx, ty, bx, by, mirror.into());
    }
}

/// Draw a textured rectangle with a scaled texture
///
/// See [`rdp_draw_textured_rectangle_scaled`](libdragon_sys::rdp_draw_textured_rectangle_scaled)
pub fn draw_textured_rectangle_scaled(texslot: u32, tx: i32, ty: i32, bx: i32, by: i32, x_scale: f64, y_scale: f64, mirror: Mirror) {
    unsafe {
        libdragon_sys::rdp_draw_textured_rectangle_scaled(texslot, tx, ty, bx, by, x_scale, y_scale, mirror.into());
    }
}

/// Draw a texture to the screen as a sprite
///
/// See [`rdp_draw_sprite`](libdragon_sys::rdp_draw_sprite)
pub fn draw_sprite(texslot: u32, x: i32, y: i32, mirror: Mirror) {
    unsafe {
        libdragon_sys::rdp_draw_sprite(texslot, x, y, mirror.into());
    }
}

/// Draw a texture to the screen as a scaled sprite
///
/// See [`rdp_draw_sprite`](libdragon_sys::rdp_draw_sprite)
pub fn draw_sprite_scaled(texslot: u32, x: i32, y: i32, x_scale: f64, y_scale: f64, mirror: Mirror) {
    unsafe {
        libdragon_sys::rdp_draw_sprite_scaled(texslot, x, y, x_scale, y_scale, mirror.into());
    }
}

/// Set the blend draw color for subsequent filled primitive operations
///
/// See [`rdp_set_blend_color`](libdragon_sys::rdp_set_blend_color)
pub fn set_blend_color(color: u32) { unsafe { libdragon_sys::rdp_set_blend_color(color); } }

/// Draw a filled triangle
///
/// See [`rdp_draw_filled_triangle`](libdragon_sys::rdp_draw_filled_triangle)
pub fn draw_filled_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
    unsafe {
        libdragon_sys::rdp_draw_filled_triangle(x1, y1, x2, y2, x3, y3);
    }
}

/// Set the flush strategy for texture loads
///
/// See [`rdp_set_texture_flush`](libdragon_sys::rdp_set_texture_flush)
pub fn set_texture_flush(flush: Flush) { unsafe { libdragon_sys::rdp_set_texture_flush(flush.into()); } }
