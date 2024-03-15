use crate::*;

use surface::Surface;
use sprite::Sprite;

// rdpq.h
pub const OVL_ID: u32 = libdragon_sys::RDPQ_OVL_ID;

pub const CMD_NOOP: u32 = libdragon_sys::RDPQ_CMD_NOOP;
pub const CMD_SET_LOOKUP_ADDRESS: u32 = libdragon_sys::RDPQ_CMD_SET_LOOKUP_ADDRESS;
pub const CMD_FILL_RECTANGLE_EX: u32 = libdragon_sys::RDPQ_CMD_FILL_RECTANGLE_EX;
pub const CMD_RESET_RENDER_MODE: u32 = libdragon_sys::RDPQ_CMD_RESET_RENDER_MODE;
pub const CMD_SET_COMBINE_MODE_2PASS: u32 = libdragon_sys::RDPQ_CMD_SET_COMBINE_MODE_2PASS;
pub const CMD_PUSH_RENDER_MODE: u32 = libdragon_sys::RDPQ_CMD_PUSH_RENDER_MODE;
pub const CMD_POP_RENDER_MODE: u32 = libdragon_sys::RDPQ_CMD_POP_RENDER_MODE;
pub const CMD_TRI: u32 = libdragon_sys::RDPQ_CMD_TRI;
pub const CMD_TRI_ZBUF: u32 = libdragon_sys::RDPQ_CMD_TRI_ZBUF;
pub const CMD_TRI_TEX: u32 = libdragon_sys::RDPQ_CMD_TRI_TEX;
pub const CMD_TRI_TEX_ZBUF: u32 = libdragon_sys::RDPQ_CMD_TRI_TEX_ZBUF;
pub const CMD_TRI_SHADE: u32 = libdragon_sys::RDPQ_CMD_TRI_SHADE;
pub const CMD_TRI_SHADE_ZBUF: u32 = libdragon_sys::RDPQ_CMD_TRI_SHADE_ZBUF;
pub const CMD_TRI_SHADE_TEX: u32 = libdragon_sys::RDPQ_CMD_TRI_SHADE_TEX;
pub const CMD_TRI_SHADE_TEX_ZBUF: u32 = libdragon_sys::RDPQ_CMD_TRI_SHADE_TEX_ZBUF;
pub const CMD_TEXTURE_RECTANGLE_EX: u32 = libdragon_sys::RDPQ_CMD_TEXTURE_RECTANGLE_EX;
pub const CMD_SET_DEBUG_MODE: u32 = libdragon_sys::RDPQ_CMD_SET_DEBUG_MODE;
pub const CMD_SET_SCISSOR_EX: u32 = libdragon_sys::RDPQ_CMD_SET_SCISSOR_EX;
pub const CMD_SET_PRIM_COLOR_COMPONENT: u32 = libdragon_sys::RDPQ_CMD_SET_PRIM_COLOR_COMPONENT;
pub const CMD_MODIFY_OTHER_MODES: u32 = libdragon_sys::RDPQ_CMD_MODIFY_OTHER_MODES;
pub const CMD_SET_FILL_COLOR_32: u32 = libdragon_sys::RDPQ_CMD_SET_FILL_COLOR_32;
pub const CMD_SET_BLENDING_MODE: u32 = libdragon_sys::RDPQ_CMD_SET_BLENDING_MODE;
pub const CMD_SET_FOG_MODE: u32 = libdragon_sys::RDPQ_CMD_SET_FOG_MODE;
pub const CMD_SET_COMBINE_MODE_1PASS: u32 = libdragon_sys::RDPQ_CMD_SET_COMBINE_MODE_1PASS;
pub const CMD_AUTOTMEM_SET_ADDR: u32 = libdragon_sys::RDPQ_CMD_AUTOTMEM_SET_ADDR;
pub const CMD_AUTOTMEM_SET_TILE: u32 = libdragon_sys::RDPQ_CMD_AUTOTMEM_SET_TILE;
pub const CMD_TRIANGLE: u32 = libdragon_sys::RDPQ_CMD_TRIANGLE;
pub const CMD_TRIANGLE_DATA: u32 = libdragon_sys::RDPQ_CMD_TRIANGLE_DATA;
pub const CMD_TEXTURE_RECTANGLE: u32 = libdragon_sys::RDPQ_CMD_TEXTURE_RECTANGLE;
pub const CMD_TEXTURE_RECTANGLE_FLIP: u32 = libdragon_sys::RDPQ_CMD_TEXTURE_RECTANGLE_FLIP;
pub const CMD_SYNC_LOAD: u32 = libdragon_sys::RDPQ_CMD_SYNC_LOAD;
pub const CMD_SYNC_PIPE: u32 = libdragon_sys::RDPQ_CMD_SYNC_PIPE;
pub const CMD_SYNC_TILE: u32 = libdragon_sys::RDPQ_CMD_SYNC_TILE;
pub const CMD_SYNC_FULL: u32 = libdragon_sys::RDPQ_CMD_SYNC_FULL;
pub const CMD_SET_KEY_GB: u32 = libdragon_sys::RDPQ_CMD_SET_KEY_GB;
pub const CMD_SET_KEY_R: u32 = libdragon_sys::RDPQ_CMD_SET_KEY_R;
pub const CMD_SET_CONVERT: u32 = libdragon_sys::RDPQ_CMD_SET_CONVERT;
pub const CMD_SET_SCISSOR: u32 = libdragon_sys::RDPQ_CMD_SET_SCISSOR;
pub const CMD_SET_PRIM_DEPTH: u32 = libdragon_sys::RDPQ_CMD_SET_PRIM_DEPTH;
pub const CMD_SET_OTHER_MODES: u32 = libdragon_sys::RDPQ_CMD_SET_OTHER_MODES;
pub const CMD_LOAD_TLUT: u32 = libdragon_sys::RDPQ_CMD_LOAD_TLUT;
pub const CMD_DEBUG: u32 = libdragon_sys::RDPQ_CMD_DEBUG;
pub const CMD_SET_TILE_SIZE: u32 = libdragon_sys::RDPQ_CMD_SET_TILE_SIZE;
pub const CMD_LOAD_BLOCK: u32 = libdragon_sys::RDPQ_CMD_LOAD_BLOCK;
pub const CMD_LOAD_TILE: u32 = libdragon_sys::RDPQ_CMD_LOAD_TILE;
pub const CMD_SET_TILE: u32 = libdragon_sys::RDPQ_CMD_SET_TILE;
pub const CMD_FILL_RECTANGLE: u32 = libdragon_sys::RDPQ_CMD_FILL_RECTANGLE;
pub const CMD_SET_FILL_COLOR: u32 = libdragon_sys::RDPQ_CMD_SET_FILL_COLOR;
pub const CMD_SET_FOG_COLOR: u32 = libdragon_sys::RDPQ_CMD_SET_FOG_COLOR;
pub const CMD_SET_BLEND_COLOR: u32 = libdragon_sys::RDPQ_CMD_SET_BLEND_COLOR;
pub const CMD_SET_PRIM_COLOR: u32 = libdragon_sys::RDPQ_CMD_SET_PRIM_COLOR;
pub const CMD_SET_ENV_COLOR: u32 = libdragon_sys::RDPQ_CMD_SET_ENV_COLOR;
pub const CMD_SET_COMBINE_MODE_RAW: u32 = libdragon_sys::RDPQ_CMD_SET_COMBINE_MODE_RAW;
pub const CMD_SET_TEXTURE_IMAGE: u32 = libdragon_sys::RDPQ_CMD_SET_TEXTURE_IMAGE;
pub const CMD_SET_Z_IMAGE: u32 = libdragon_sys::RDPQ_CMD_SET_Z_IMAGE;
pub const CMD_SET_COLOR_IMAGE: u32 = libdragon_sys::RDPQ_CMD_SET_COLOR_IMAGE;

/// Configuration flag: enable automatic generation of SYNC_PIPE commands
pub const CFG_AUTOSYNCPIPE: u32 = libdragon_sys::RDPQ_CFG_AUTOSYNCPIPE;
/// Configuration flag: enable automatic generation of SYNC_LOAD commands
pub const CFG_AUTOSYNCLOAD: u32 = libdragon_sys::RDPQ_CFG_AUTOSYNCLOAD;
/// Configuration flag: enable automatic generation of SYNC_TILE commands
pub const CFG_AUTOSYNCTILE: u32 = libdragon_sys::RDPQ_CFG_AUTOSYNCTILE;
/// Configuration flag: enable automatic generation of SET_SCISSOR commands on render target change
pub const CFG_AUTOSCISSOR: u32 = libdragon_sys::RDPQ_CFG_AUTOSCISSOR;
/// Configuration flag: default configuration
pub const CFG_DEFAULT: u32 = libdragon_sys::RDPQ_CFG_DEFAULT;

// Used in inline functions as part of the autosync engine. Not part of public API.
#[inline(always)] fn _autosync_tile(n: u32) -> u32 { 1    << (0+n) }     // Autosync state: Bit used for tile N
#[inline(always)] fn _autosync_tiles()      -> u32 { 0xFF << 0     }     // Autosync state: Mask for all bits regarding tile
#[inline(always)] fn _autosync_tmem(n: u32) -> u32 { 1    << (8+n) }     // Autosync state: Bit used for tmem portion N
#[inline(always)] fn _autosync_tmems()      -> u32 { 0xFF << 8     }     // Autosync state: Mask for all bits regarding TMEM
#[inline(always)] fn _autosync_pipe()       -> u32 { 1    << 16    }     // Autosync state: Bit used for pipe

// Used internally for bit-packing RDP commands. Not part of public API
#[inline(always)] fn _carg(value: u32, mask: u32, shift: u32) -> u32 { (((value as u32) & mask)) << shift }

// Used for manually translated inline functions
extern "C" {
    fn __rdpq_write8(cmd_id: u32, arg0: u32, arg1: u32);
    fn __rdpq_fixup_mode(cmd_id: u32, w0: u32, w1: u32);
    fn __rdpq_fixup_mode3(cmd_id: u32, w0: u32, w1: u32, w2: u32);
    fn __rdpq_fixup_mode4(cmd_id: u32, w0: u32, w1: u32, w2: u32, w3: u32);
    fn __rdpq_write8_syncchange(cmd_id: u32, arg0: u32, arg1: u32, autosync: u32);
    fn __rdpq_fixup_write8_syncchange(cmd_id: u32, arg0: u32, arg1: u32, autosync: u32);
}

/// Tile descriptors
///
/// The only valid tiles are 0-7
///
/// See [`rdpq_tile_t`](libdragon_sys::rdpq_tile_t`) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Tile(pub u32);

/// Tile parameters for [set_tile]. 
///
/// This is a wrapper around `rdpq_tileparms_t`.
///
/// See [`rdpq_tileparms_t`](libdragon_sys::rdpq_tileparms_t) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TileParms {
    palette: u8,
    s      : TileSTParms,
    t      : TileSTParms,
}

impl Into<libdragon_sys::rdpq_tileparms_t> for TileParms {
    fn into(self) -> libdragon_sys::rdpq_tileparms_t {
        assert!(::core::mem::size_of::<Self>() == ::core::mem::size_of::<libdragon_sys::rdpq_tileparms_t>());
        unsafe {
            *::core::mem::transmute::<&Self, *const libdragon_sys::rdpq_tileparms_t>(&self)
        }
    }
}

/// Additional mapping parameters; Leave them as default() if not required;
///
/// See [`rdpq_tileparms_t`](libdragon_sys::rdpq_tileparms_t) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TileSTParms {
    clamp : bool,
    mirror: bool,
    mask  : u8,
    shift : i8,
}

/// Tile descriptor internally used by some RDPQ functions. Avoid using if possible
pub const TILE_INTERNAL: Tile = Tile(7);

/// Initialize the RDPQ library.
///
/// See [`rdpq_init`](libdragon_sys::rdpq_init) for details.
pub fn init() { unsafe { libdragon_sys::rdpq_init(); } }

/// Shutdown the RDPQ library.
///
/// See [`rdpq_close`](libdragon_sys::rdpq_close) for details.
pub fn close() { unsafe { libdragon_sys::rdpq_close(); } }

/// Set the configuration of the RDPQ module.
///
/// See [`rdpq_config_set`](libdragon_sys::rdpq_config_set) for details.
pub fn config_set(cfg: u32) -> u32 { unsafe { libdragon_sys::rdpq_config_set(cfg) } }

/// Enable a specific set of configuration flags
///
/// See [`rdpq_config_enable`](libdragon_sys::rdpq_config_enable) for details.
pub fn config_enable(cfg_enable_bits: u32) -> u32 { unsafe { libdragon_sys::rdpq_config_enable(cfg_enable_bits) } }

/// Disable a specific set of configuration flags
///
/// See [`rdpq_config_disable`](libdragon_sys::rdpq_config_disable) for details.
pub fn config_disable(cfg_disable_bits: u32) -> u32 { unsafe { libdragon_sys::rdpq_config_disable(cfg_disable_bits) } }

/// Low level function set the green and blue components of the chroma key
///
/// See [`rdpq_set_chromakey_parms`](libdragon_sys::rdpq_set_chromakey_parms) for details.
pub fn set_chromakey_parms(color: graphics::Color, edge_r: i32, edge_g: i32, edge_b: i32, width_r: i32, width_g: i32, width_b: i32) {
    extern "C" {
        fn rdpq_set_chromakey_parms(color: libdragon_sys::color_t,
                                    edge_r: ::core::ffi::c_int, edge_g: ::core::ffi::c_int, edge_b: ::core::ffi::c_int,
                                    width_r: ::core::ffi::c_int, width_g: ::core::ffi::c_int, width_b: ::core::ffi::c_int);
    }
    unsafe {
        rdpq_set_chromakey_parms(color.c, edge_r, edge_g, edge_b, width_r, width_g, width_b);
    }
}

/// Low level functions to set the matrix coefficients for texture format conversion
///
/// See [`rdpq_set_yuv_parms`](libdragon_sys::rdpq_set_yuv_parms) for details.
pub fn set_yuv_parms(k0: u16, k1: u16, k2: u16, k3: u16, k4: u16, k5: u16) {
    extern "C" {
        fn rdpq_set_yuv_parms(k0: ::core::ffi::c_ushort, k1: ::core::ffi::c_ushort, k2: ::core::ffi::c_ushort,
                              k3: ::core::ffi::c_ushort, k4: ::core::ffi::c_ushort, k5: ::core::ffi::c_ushort);
    }
    unsafe {
        rdpq_set_yuv_parms(k0, k1, k2, k3, k4, k5);
    }
}

/// Configure a scissoring rectangle in screen coordinates (RDP command: SET_SCISSOR)
///
/// See [`rdpq_set_scissor`](libdragon_sys::rdpq_set_scissor) for details.
#[inline]
pub fn set_scissor(x0: i32, y0: i32, x1: i32, y1: i32) {
    let x0fx = x0*4;
    let y0fx = y0*4;
    let x1fx = x1*4;
    let y1fx = y1*4;
    assert!(x0fx <= x1fx, "x1 must be greater or equal to x0");
    assert!(y0fx <= y1fx, "y1 must be greater or equal to y0");
    assert!(x0fx >= 0, "x0 must be positive");
    assert!(y0fx >= 0, "y0 must be positive");
    unsafe {
        libdragon_sys::__rdpq_set_scissor( 
            _carg(x0fx as u32, 0xFFF, 12) | _carg(y0fx as u32, 0xFFF, 0), 
            _carg(x1fx as u32, 0xFFF, 12) | _carg(y1fx as u32, 0xFFF, 0)
        ); 
    }
}

/// Set a fixed Z value to be used instead of a per-pixel value (RDP command; SET_PRIM_DEPTH)
///
/// See [`rdpq_set_prim_depth_raw`](libdragon_sys::rdpq_set_prim_depth_raw) for details.
#[inline]
pub fn set_prim_depth_raw(prim_z: u16, prim_dz: i16) {
    extern "C" {
        fn rdpq_set_prim_depth_raw(prim_z: ::core::ffi::c_ushort, prim_dz: ::core::ffi::c_short);
    }
    unsafe {
        rdpq_set_prim_depth_raw(prim_z, prim_dz);
    }
}

/// Load a portion of a texture into TMEM (RDP command: LOAD_TILE)
///
/// See [`rdpq_load_tile`](libdragon_sys::rdpq_load_tile) for details.
#[inline]
pub fn load_tile(tile: Tile, s0: u16, t0: u16, s1: u16, t1: u16) {
    assert!(s0 < 1024 && t0 < 1024 && s1 < 1024 && t1 < 1024, "texture coordinates must be smaller than 1024");
    load_tile_fx(tile, s0*4, t0*4, s1*4, t1*4);
}

/// Load a portion of a texture into TMEM -- fixed point version (RDP command:: LOAD_TILE)
///
/// See [`rdpq_load_tile_fx`](libdragon_sys::rdpq_load_tile_fx) for details.
#[inline]
pub fn load_tile_fx(tile: Tile, s0: u16, t0: u16, s1: u16, t1: u16) {
    extern "C" {
        fn rdpq_load_tile_fx(tile: libdragon_sys::rdpq_tile_t,
                             s0: ::core::ffi::c_ushort, t0: ::core::ffi::c_ushort,
                             s1: ::core::ffi::c_ushort, t1: ::core::ffi::c_ushort);
    }
    unsafe {
        rdpq_load_tile_fx(tile.0 as libdragon_sys::rdpq_tile_t, s0, t0, s1, t1);
    }
}

/// Load a palette of colors into TMEM (RDP command: LOAD_TLUT)
///
/// See [`rdpq_load_tlut_raw`](libdragon_sys::rdpq_load_tlut_raw) for details.
#[inline]
pub fn load_tlut_raw(tile: Tile, color_idx: u8, num_colors: u8) {
    extern "C" {
        fn rdpq_load_tlut_raw(tile: libdragon_sys::rdpq_tile_t, color_idx: ::core::ffi::c_uchar, num_colors: ::core::ffi::c_uchar);
    }
    unsafe {
        rdpq_load_tlut_raw(tile.0 as libdragon_sys::rdpq_tile_t, color_idx, num_colors);
    }
}

/// Configure the extents of a tile descriptor (RDP command: SET_TILE_SIZE)
///
/// See [`rdpq_set_tile_size`](libdragon_sys::rdpq_set_tile_size) for details.
#[inline]
pub fn set_tile_size(tile: Tile, s0: u16, t0: u16, s1: u16, t1: u16) {
    set_tile_size_fx(tile, s0*4, t0*4, s1*4, t1*4);
}

/// Configure the extents of a tile descriptor -- fixed point version (RDP command: SET_TILE_SIZE)
///
/// See [`rdpq_set_tile_size_fx`](libdragon_sys::rdpq_set_tile_size_fx) for details.
#[inline]
pub fn set_tile_size_fx(tile: Tile, s0: u16, t0: u16, s1: u16, t1: u16) {
    extern "C" {
        fn rdpq_set_tile_size_fx(tile: libdragon_sys::rdpq_tile_t,
                                 s0: ::core::ffi::c_ushort, t0: ::core::ffi::c_ushort,
                                 s1: ::core::ffi::c_ushort, t1: ::core::ffi::c_ushort);
    }
    unsafe {
        rdpq_set_tile_size_fx(tile.0 as libdragon_sys::rdpq_tile_t, s0, t0, s1, t1);
    }
}

/// Low level function to load a texture image into TMEM in a single memory transfer
///
/// See [`rdpq_load_block_fx`](libdragon_sys::rdpq_load_block_fx) for details.
#[inline]
pub fn load_block_fx(tile: Tile, s0: u16, t0: u16, num_texels: u16, dxt: u16) {
    extern "C" {
        fn rdpq_load_block_fx(tile: libdragon_sys::rdpq_tile_t, 
                              s0: ::core::ffi::c_ushort, t0: ::core::ffi::c_ushort,
                              num_texels: ::core::ffi::c_ushort, dxt: ::core::ffi::c_ushort);
    }
    unsafe {
        rdpq_load_block_fx(tile.0 as libdragon_sys::rdpq_tile_t, s0, t0, num_texels, dxt);
    }
}

/// Load a texture image into TMEM with a single contiguous memory transfer (RDP command: LOAD_BLOCK)
///
/// See [`rdpq_load_block`](libdragon_sys::rdpq_load_block) for details.
#[inline]
pub fn load_block(tile: Tile, s0: u16, t0: u16, num_texels: u16, tmem_pitch: u16) {
    extern "C" {
        fn rdpq_load_block(tile: libdragon_sys::rdpq_tile_t,
                           s0: ::core::ffi::c_ushort, t0: ::core::ffi::c_ushort,
                           num_texels: ::core::ffi::c_ushort, tmem_pitch: ::core::ffi::c_ushort);
    }
    unsafe {
        rdpq_load_block(tile.0 as libdragon_sys::rdpq_tile_t, s0, t0, num_texels, tmem_pitch);
    }
}

/// Special TMEM address to pass to [set_tile] to use automatic TMEM allocation
pub const AUTOTMEM: i16 = libdragon_sys::RDPQ_AUTOTMEM as i16;
/// Special TMEM address to pass to [set_tile] to configure a tile with the same address of previous tile
#[allow(non_snake_case)]
#[inline(always)]
pub fn AUTOTMEM_REUSE(offset: i16) -> i16 { 0x4000 | (offset / 8) }

/// Enqueue a RDP SET_TILE command (full version)
///
/// See [`rdpq_set_tile`](libdragon_sys::rdpq_set_tile) for details.
#[inline]
pub fn set_tile(tile: Tile, format: surface::TexFormat, tmem_addr: i32, tmem_pitch: u16, parms: &TileParms) {
    extern "C" {
        fn rdpq_set_tile(tile: libdragon_sys::rdpq_tile_t, format: libdragon_sys::tex_format_t,
                         tmem_addr: ::core::ffi::c_int, tmem_pitch: ::core::ffi::c_ushort,
                         parms: *const libdragon_sys::rdpq_tileparms_t);
    }
    let parms: libdragon_sys::rdpq_tileparms_t = (*parms).into();
    unsafe {
        rdpq_set_tile(tile.0 as libdragon_sys::rdpq_tile_t, format.into(), tmem_addr, tmem_pitch, &parms);
    }
}

/// Configure the auto-TMEM feature of [set_tile]
///
/// See [`rdpq_set_tile_autotmem`](libdragon_sys::rdpq_set_tile_autotmem) for details.
#[inline]
pub fn set_tile_autotmem(tmem_bytes: i16) { unsafe { libdragon_sys::rdpq_set_tile_autotmem(tmem_bytes); } }

/// Enqueue a SET_FILL_COLOR RDP command.
///
/// See [`rdpq_set_fill_color`](libdragon_sys::rdpq_set_fill_color) for details.
#[inline]
pub fn set_fill_color(color: graphics::Color) {
    extern "C" {
        fn __rdpq_set_fill_color(c: ::core::ffi::c_uint);
    }
    let c = color.to_packed32();
    unsafe {
        __rdpq_set_fill_color(c);
    }
}

/// Enqueue a SET_FILL_COLOR RDP command to draw a striped pattern.
///
/// See [`rdpq_set_fill_color_stripes`](libdragon_sys::rdpq_set_fill_color_stripes) for details.
#[inline]
pub fn set_fill_color_stripes(color1: graphics::Color, color2: graphics::Color) {
    extern "C" {
        fn rdpq_set_fill_color_stripes(color1: libdragon_sys::color_t, color2: libdragon_sys::color_t);
    }
    unsafe {
        // TODO this may not work right, might want to check the registers as 
        // there might be ABI issues since we're passing structs by value.
        rdpq_set_fill_color_stripes(color1.c, color2.c);
    }
}

/// Set the RDP FOG blender register
///
/// See [`rdpq_set_fog_color`](libdragon_sys::rdpq_set_fog_color) for details.
#[inline]
pub fn set_fog_color(color: graphics::Color) {
    extern "C" {
        fn rdpq_set_fog_color(color: libdragon_sys::color_t);
    }
    unsafe {
        rdpq_set_fog_color(color.c);
    }
}

/// Set the RDP BLEND blender register
///
/// See [`rdpq_set_blend_color`](libdragon_sys::rdpq_set_blend_color) for details.
#[inline]
pub fn set_blend_color(color: graphics::Color) {
    unsafe {
        __rdpq_write8_syncchange(CMD_SET_BLEND_COLOR, 0, color.to_packed32(), libdragon_sys::AUTOSYNC_PIPE);
    }
}

/// Set the RDM PRIM combiner register (color only) (RDP command: SET_PRIM_COLOR)
///
/// See [`rdpq_set_prim_color`](libdragon_sys::rdpq_set_prim_color) for details.
#[inline]
pub fn set_prim_color(color: graphics::Color) {
    unsafe {
        __rdpq_fixup_write8_syncchange(CMD_SET_PRIM_COLOR_COMPONENT, 0<<16, color.to_packed32(), 0);
    }
}

/// Set the detail/sharpet blending factor (RDP command: SET_PRIM_COLOR (partial))
///
/// See [`rdpq_set_detail_factor`](libdragon_sys::rdpq_set_detail_factor) for details.
#[inline]
pub fn set_detail_factor(value: f32) {
    // TODO should we check 0 <= value <= 1?
    let conv: u32 = (((1.0 - value) * 31.0) as i8) as u32;
    unsafe {
        __rdpq_fixup_write8_syncchange(CMD_SET_PRIM_COLOR_COMPONENT, (((conv & 0x1F) << 8) | (2<<16)) as u32, 0, 0);
    }
}

/// Set the RDP PRIM LOD FRAC combiner register (RDP command: SET_PRIM_COLOR (partial))
/// 
/// See [`rdpq_set_prim_lod_frac`](libdragon_sys::rdpq_set_prim_lod_frac) for details.
#[inline]
pub fn set_prim_load_frac(value: u8) {
    unsafe {
        __rdpq_fixup_write8_syncchange(CMD_SET_PRIM_COLOR_COMPONENT, (value as u32) | (1<<16), 0, 0);
    }
}

/// Set the RDP PRIM combiner register (Raw version) (RDP command: SET_PRIM_COLOR)
///
/// See [`rdpq_set_prim_register_raw`](libdragon_sys::rdpq_set_prim_register_raw) for details.
#[inline]
pub fn set_prim_register_raw(color: graphics::Color, minlod: u8, primlod: u8) {
    unsafe {
        __rdpq_write8(CMD_SET_PRIM_COLOR, (((minlod as u32) & 0x1F) << 8) | (primlod as u32), color.to_packed32());
    }
}

/// Set the RDM ENV combiner register (RDP command: SET_ENV_COLOR)
///
/// See [`rdpq_set_env_color`](libdragon_sys::rdpq_set_env_color) for details.
#[inline]
pub fn set_env_color(color: graphics::Color) {
    unsafe {
        __rdpq_write8_syncchange(CMD_SET_ENV_COLOR, 0, color.to_packed32(), libdragon_sys::AUTOSYNC_PIPE);
    }
}

/// Configure the framebuffer to render to (RDP command: SET_COLOR_IMAGE)
///
/// See [`rdpq_set_color_image`](libdragon_sys::rdpq_set_color_image) for details.
#[inline]
pub fn set_color_image(surface: &surface::Surface) {
    unsafe {
        libdragon_sys::rdpq_set_color_image(surface.ptr as *const _);
    }
}

/// Configure the Z-buffer to use (RDP command: SET_Z_IMAGE)
///
/// See [`rdpq_set_z_image`](libdragon_sys::rdpq_set_z_image) for details.
#[inline]
pub fn set_z_image(surface: &surface::Surface) {
    unsafe {
        libdragon_sys::rdpq_set_z_image(surface.ptr as *const _);
    }
}

/// Configure the texture to use (RDP command: SET_TEX_IMAGE)
///
/// See [`rdpq_set_texture_image`](libdragon_sys::rdpq_set_texture_image) for details.
#[inline]
pub fn set_texture_image(surface: &surface::Surface) {
    unsafe {
        libdragon_sys::rdpq_set_texture_image(surface.ptr as *const _);
    }
}

/// Low-level version of [set_color_image], with address lookup capability
///
/// See [`rdpq_set_color_image_raw`](libdragon_sys::rdpq_set_color_image_raw) for details.
#[inline]
pub fn set_color_image_raw(index: u8, offset: u32, format: surface::TexFormat, width: u32, height: u32, stride: u32) {
    extern "C" {
        fn rdpq_set_color_image_raw(index: ::core::ffi::c_uchar, offset: ::core::ffi::c_uint,
                                   format: libdragon_sys::tex_format_t, width: ::core::ffi::c_uint,
                                   height: ::core::ffi::c_uint, stride: ::core::ffi::c_uint);
    }
    unsafe {
        rdpq_set_color_image_raw(index, offset, format.into(), width, height, stride);
    }
}

/// Low-level version of [set_z_image], with address lookup capability
///
/// See [`rdpq_set_z_image_raw`](libdragon_sys::rdpq_set_z_image_raw) for details.
#[inline]
pub fn set_z_image_raw(index: u8, offset: u32) {
    extern "C" {
        fn rdpq_set_z_image_raw(index: ::core::ffi::c_uchar, offset: ::core::ffi::c_uint);
    }
    unsafe {
        rdpq_set_z_image_raw(index, offset);
    }
}

/// Low-level version of [set_texture_image], with address lookup capability
///
/// See [`rdpq_set_texture_image_raw`](libdragon_sys::rdpq_set_texture_image_raw) for details.
#[inline]
pub fn set_texture_image_raw(index: u8, offset: u32, format: surface::TexFormat, width: u16, height: u16) {
    extern "C" {
        fn rdpq_set_texture_image_raw(index: ::core::ffi::c_uchar, offset: ::core::ffi::c_uint,
                                     format: libdragon_sys::tex_format_t, width: ::core::ffi::c_ushort,
                                     height: ::core::ffi::c_ushort);
    }
    unsafe {
        rdpq_set_texture_image_raw(index, offset, format.into(), width, height);
    }
}


/// Store an address into the rdpq lookup table
/// 
/// See [`rdpq_set_lookup_address`](libdragon_sys::rdpq_set_lookup_address) for details.
#[inline]
pub fn set_lookup_address<T>(index: u8, rdram_addr: &[T]) {
    assert!(index > 0 && index <= 15, "Lookup address index out of range [1,15]: {}", index);
    unsafe {
        __rdpq_write8(CMD_SET_LOOKUP_ADDRESS, (index as u32) << 2, rdram_addr.physical_ref().as_ptr() as _);
    }
}

/// Schedule a RDP SYNC_PIPE command.
///
/// See [`rdpq_sync_pipe`](libdragon_sys::rdpq_sync_pipe) for details.
#[inline] pub fn sync_pipe() { unsafe { libdragon_sys::rdpq_sync_pipe(); } }

/// Schedule a RDP SYNC_TILE command.
///
/// See [`rdpq_sync_tile`](libdragon_sys::rdpq_sync_tile) for details.
#[inline] pub fn sync_tile() { unsafe { libdragon_sys::rdpq_sync_tile(); } }

/// Schedule a RDP SYNC_LOAD command.
///
/// See [`rdpq_sync_load`](libdragon_sys::rdpq_sync_load) for details.
#[inline] pub fn sync_load() { unsafe { libdragon_sys::rdpq_sync_load(); } }

/// Schedule a RDP SYNC_FULL command and register a callback when it is done.
///
/// See [`rdpq_sync_full`](libdragon_sys::rdpq_sync_full) for details.
pub fn sync_full(cb: Option<RdpqSimpleCallback>) {
    if let Some(user_callback) = cb {
        let cb = Box::new(RdpqCallbackInternal { user_callback: user_callback });
        unsafe {
            let ctx: *mut RdpqCallbackInternal = Box::leak(cb); // Leak the function callback to prevent
                                                            // memory from being freed
            libdragon_sys::rdpq_sync_full(Some(rdpq_simple_callback), ctx as *mut ::core::ffi::c_void);
        }
    } else {
        unsafe {
            libdragon_sys::rdpq_sync_full(None, ::core::ptr::null_mut());
        }
    }
}

type RdpqSimpleCallback = Box<dyn FnOnce() + 'static + Sync + Send>;
struct RdpqCallbackInternal {
    user_callback: RdpqSimpleCallback,
}

// This function is also used by [call_deferred]
extern "C" fn rdpq_simple_callback(ctx: *mut ::core::ffi::c_void) {
    // in this function, ctx must be valid
    let cb = unsafe {
        let ctx: *mut RdpqCallbackInternal = ctx as *mut RdpqCallbackInternal;
        Box::from_raw(ctx)
    };

    // call user code
    (cb.user_callback)();

    // let the Box be dropped
}

/// Low-level function to set the rendering mode register.
///
/// See [`rdpq_set_other_modes_raw`](libdragon_sys::rdpq_set_other_modes_raw) for details.
#[inline]
pub fn set_other_modes_raw(mode: u64) {
    extern "C" {
        fn __rdpq_set_other_modes(a: ::core::ffi::c_uint, b: ::core::ffi::c_uint);
    }
    unsafe {
        __rdpq_set_other_modes(((mode >> 32) & 0x00FFFFFF) as u32, (mode & 0xFFFFFFFF) as u32);
    }
}

/// Low-level function to partly change the rendering mode register.
///
/// See [`rdpq_change_other_modes_raw`](libdragon_sys::rdpq_change_other_modes_raw) for details.
#[inline]
pub fn change_other_modes_raw(mask: u64, val: u64) {
    extern "C" { 
        fn __rdpq_change_other_modes(a: u32, b: u32, c: u32);
    }
    unsafe {
        if (mask >> 32) != 0 {
            __rdpq_change_other_modes(0, !(mask >> 32) as u32, (val >> 32) as u32);
        }
        if (mask as u32) != 0 {
            __rdpq_change_other_modes(4, !(mask as u32), val as u32);
        }
    }
}

/// Read the current render mode register.
///
/// See [`rdpq_get_other_modes_raw`](libdragon_sys::rdpq_get_other_modes_raw) for details.
#[inline] pub fn get_other_modes_raw() -> u64 { unsafe { libdragon_sys::rdpq_get_other_modes_raw() } }

/// Load-level function to change the RDP combiner.
///
/// See [`rdpq_set_combiner_raw`](libdragon_sys::rdpq_set_combiner_raw) for details.
#[inline]
pub fn set_combiner_raw(comb: u64) {
    unsafe {
        __rdpq_write8_syncchange(CMD_SET_COMBINE_MODE_RAW,
                                 ((comb >> 32) & 0x00FFFFFF) as u32,
                                 comb as u32,
                                 libdragon_sys::AUTOSYNC_PIPE);
    }
}

/// Add a fench to synchronize RSP with RDP commands.
///
/// See [`rdpq_fence`](libdragon_sys::rdpq_fence) for details.
#[inline] pub fn fence() { unsafe { libdragon_sys::rdpq_fence(); } }

/// Send to the RDP a buffer of RDP commands from RDRAM
///
///
/// See [`rdpq_exec`](libdragon_sys::rdpq_exec) for details.
#[inline]
pub fn exec<T>(buffer: &mut [T]) {
    let size = buffer.len() * ::core::mem::size_of::<T>();
    unsafe {
        libdragon_sys::rdpq_exec(buffer.as_mut_ptr() as *mut _, size as i32);
    }
}

/// Enqueue a callback that will be called after the RSP and the RDP have
/// finished processing all commands enqueued until now.
///
/// See [`rdpq_call_deferred`](libdragon_sys::rdpq_call_deferred) for details.
pub fn call_deferred(cb: RdpqSimpleCallback) {
    let cb = Box::new(RdpqCallbackInternal { user_callback: cb });
    unsafe {
        let ctx: *mut RdpqCallbackInternal = Box::leak(cb); // Leak the function callback to prevent
                                                        // memory from being freed
        libdragon_sys::rdpq_call_deferred(Some(rdpq_simple_callback), ctx as *mut ::core::ffi::c_void);
    }
}

/// Enqueue a RSP command that also generates RDP commands.
///
/// Rust-specific: the C version `rdpq_write` asks that you pass -1 when the number
/// of generated RDP commands is large or unknown.  For Rust, pass None for that case.  
///
/// This function calls into [rspq::Writer].
///
/// See [`rdpq_write`](libdragon_sys::rdpq_write) for details.
#[inline]
pub fn write(num_rdp_commands: Option<usize>, ovl_id: u32, cmd_id: u32, args: Vec<u32>) {
    extern "C" {
        static rspq_block: *mut libdragon_sys::rspq_block_t;
        fn __rdpq_block_reserve(sz: ::core::ffi::c_int);
    }
    if let Some(count) = num_rdp_commands {
        unsafe {
            if rspq_block != ::core::ptr::null_mut() {
                __rdpq_block_reserve(count as i32);
            }
        }
    }
    let mut writer = rspq::Writer::begin(ovl_id, cmd_id, args.len());
    for v in args {
        writer.arg(v);
    }
    writer.end();
}

// rdpq_attach.h

/// Attach the RDP to a color surface (and optionally a Z buffer)
///
/// See [`rdpq_attach`][(libdragon_sys::rdpq_attach) for details.
pub fn attach(surf_color: &Surface, surf_depth: Option<&Surface>) {
    let depth_null_surface = Surface::from_ptr(::core::ptr::null_mut());
    unsafe {
        libdragon_sys::rdpq_attach_clear(surf_color.ptr, surf_depth.unwrap_or(&depth_null_surface).ptr);
    }
}

/// Attach the RDP to a surface and clear it
///
/// See [`rdpq_attach_clear`](libdragon_sys::rdpq_attach_clear) for details.
pub fn attach_clear(surf_color: &Surface, surf_depth: Option<&Surface>) {
    let depth_null_surface = Surface::from_ptr(::core::ptr::null_mut());
    unsafe {
        libdragon_sys::rdpq_attach_clear(surf_color.ptr, 
                                         surf_depth.unwrap_or(&depth_null_surface).ptr);
    }
}

/// Clear the current render target with the specified color.
///
/// See [`rdpq_clear`](libdragon_sys::rdpq_clear) for details.
#[inline]
pub fn clear(color: graphics::Color) {
    extern "C" { fn __rdpq_clear(color: *const libdragon_sys::color_t); }
    unsafe { __rdpq_clear(&color.c); }
}

/// Clear the current Z buffer to the given value.
///
/// See [`rdpq_clear_z`](libdragon_sys::rdpq_clear_z) for details.
#[inline]
pub fn clear_z(z: u16) {
    extern "C" { fn __rdpq_clear_z(z: *const ::core::ffi::c_ushort); }
    unsafe { __rdpq_clear_z(&z); }
}

/// Detach the RDP from the current surface, and restore the previous one
///
/// See [`rdpq_detach`](libdragon_sys::rdpq_detach) for details.
#[inline]
pub fn detach() { unsafe { libdragon_sys::rdpq_detach_cb(None, ::core::ptr::null_mut()); } }

/// Check if the RDP is currently attached to a surface
///
/// See [`rdpq_is_attached`](libdragon_sys::rdpq_is_attached) for details.
pub fn is_attached() -> bool { unsafe { libdragon_sys::rdpq_is_attached() } }

/// Detach the RDP from the current framebuffer, and show it on screen
///
/// See [`rdpq_detach_show`](libdragon_sys::rdpq_detach_show) for details.
pub fn detach_show() { unsafe { libdragon_sys::rdpq_detach_show(); } }

/// Detach the RDP from the current surface, waiting for RDP to finish drawing.
///
/// See [`rdpq_detach_wait`](libdragon_sys::rdpq_detach_wait) for details.
#[inline]
pub fn detach_wait() {
    detach();
    rspq::wait();
}

/// Detach the RDP from the current surface, and call a callback when
/// the RDP has finished drawing to it.
///
/// See [`rdpq_detach_cb`](libdragon_sys::rdpq_detach_cb) for details.
pub fn detach_cb(cb: RdpqSimpleCallback) {
    let cb = Box::new(RdpqCallbackInternal { user_callback: cb });
    unsafe {
        let ctx: *mut RdpqCallbackInternal = Box::leak(cb); // Leak the function callback to prevent
                                                        // memory from being freed
        libdragon_sys::rdpq_detach_cb(Some(rdpq_simple_callback), ctx as *mut ::core::ffi::c_void);
    }
}

/// Get the surface that is currently attached to the RDP
///
/// See [`rdpq_get_attached`](libdragon_sys::rdpq_get_attached) for details.
pub fn get_attached<'a>() -> Surface<'a> {
    let ptr = unsafe {
        libdragon_sys::rdpq_get_attached()
    };
    surface::Surface::from_const_ptr(ptr)
}

// rdpq_constants.h

pub const ADDRESS_TABLE_SIZE: u32 = libdragon_sys::RDPQ_ADDRESS_TABLE_SIZE as u32;

pub const DYNAMIC_BUFFER_SIZE: u32 = libdragon_sys::RDPQ_DYNAMIC_BUFFER_SIZE as u32;

/// Asserted if `rdpq_mode_blender` was called in fill/copy mode
pub const ASSERT_FILLCOPY_BLENDING: u32 = libdragon_sys::RDPQ_ASSERT_FILLCOPY_BLENDING as u32;
/// Asserted if a 2-pass combiner is set with `rdpq_mode_combiner] while mipmap is enabled.
pub const ASSERT_MIPMAP_COMB2: u32 = libdragon_sys::RDPQ_ASSERT_MIPMAP_COMB2 as u32;
/// Asserted if RDPQCmd_Triangle is called with `RDPQ_TRIANGLE_REFERENCE == 0`
pub const ASSERT_INVALID_CMD_TRI: u32 = libdragon_sys::RDPQ_ASSERT_INVALID_CMD_TRI as u32;
/// Asserted if RDPQ_Send is called with invalid parameters (begin > end)
pub const ASSERT_SEND_INVALID_SIZE: u32 = libdragon_sys::RDPQ_ASSERT_SEND_INVALID_SIZE as u32;
/// Asserted if the TMEM is full during an auto-TMEM operation
pub const ASSERT_AUTOTMEM_FULL: u32 = libdragon_sys::RDPQ_ASSERT_AUTOTMEM_FULL as u32;
/// Asserted if the TMEM is full during an auto-TMEM operation
pub const ASSERT_AUTOTMEM_UNPAIRED: u32 = libdragon_sys::RDPQ_ASSERT_AUTOTMEM_UNPAIRED as u32;

pub const MAX_COMMAND_SIZE: u32 = libdragon_sys::RDPQ_MAX_COMMAND_SIZE as u32;
/// RDPQ block minimum size (in 32-bit words)
pub const BLOCK_MIN_SIZE: u32 = libdragon_sys::RDPQ_BLOCK_MIN_SIZE as u32;
/// RDPQ block minimum size (in 32-bit words)
pub const BLOCK_MAX_SIZE: u32 = libdragon_sys::RDPQ_BLOCK_MAX_SIZE as u32;

/// Whether or not the reference implementation is enabled
pub const TRIANGLE_REFERENCE: u32 = libdragon_sys::RDPQ_TRIANGLE_REFERENCE as u32;

// rdpq_debug.h

/// Initialize the RDPQ debugging engine.
///
/// See [`rdpq_debug_start`](libdragon_sys::rdpq_debug_start) for details.
pub fn debug_start() { unsafe { libdragon_sys::rdpq_debug_start(); } }

/// Stop the rdpq debugging engine.
pub fn debug_stop() { unsafe { libdragon_sys::rdpq_debug_stop(); } }

/// Show a full log of all the RDP commands
///
/// See [`rdpq_debug_log`](libdragon_sys::rdpq_debug_log) for details.
pub fn debug_log(show_log: bool) { unsafe { libdragon_sys::rdpq_debug_log(show_log); } }

/// Add a custom message in the RDP logging
///
/// See [`rdpq_debug_log_msg`](libdragon_sys::rdpq_debug_log_msg) for details.
pub fn debug_log_msg(msg: &str) {
    let cmsg = CString::new(msg).unwrap();
    unsafe {
        libdragon_sys::rdpq_debug_log_msg(cmsg.as_ptr());
    }
}

/// Acquiare a dump of the current contents of TMEM
///
/// See [`rdpq_debug_get_tmem`](libdragon_sys::rdpq_debug_get_tmem) for details.
pub fn debug_get_tmem<'a>() -> surface::Surface<'a> {
    // initialize surface_t from libdragon
    let mut surface: core::mem::MaybeUninit<libdragon_sys::surface_t> = core::mem::MaybeUninit::uninit();
    extern "C" {
        fn rdpq_debug_get_tmem_r(s: *mut libdragon_sys::surface_t);
    }
    unsafe {
        rdpq_debug_get_tmem_r(surface.as_mut_ptr());
    }

    // pin the structure in place before getting memory address
    let mut backing_instance = Box::pin(unsafe { surface.assume_init() });

    // create a backed surface that will be freed
    surface::Surface {
        ptr: backing_instance.as_mut().get_mut(),
        _backing_instance: Some(backing_instance),
        needs_free: true,
        is_const: false,
        phantom: core::marker::PhantomData,
    }
}

/// Install a custom hook that will be called every time a RDP command is processed.
///
/// See [`rdpq_debug_install_hook`](libdragon_sys::rdpq_debug_install_hook) for details.
pub fn debug_install_hook(cb: RdpqCommandHookCallback) {
    let cb = Box::new(RdpqCommandHookInternal { user_callback: cb });
    unsafe {
        let ctx: *mut RdpqCommandHookInternal = Box::leak(cb);
        libdragon_sys::rdpq_debug_install_hook(Some(debug_hook_callback), ctx as *mut _);
    }
}

type RdpqCommandHookCallback = Box<dyn FnMut(&[u64]) + 'static + Sync + Send>;
struct RdpqCommandHookInternal {
    user_callback: RdpqCommandHookCallback,
}

extern "C" fn debug_hook_callback(ctx: *mut ::core::ffi::c_void, cmd: *mut ::core::ffi::c_ulonglong, cmd_size: ::core::ffi::c_int) {
    let slice: &[u64] = unsafe {
        // TODO is cmd_size in u64's or bytes?
        core::slice::from_raw_parts(cmd, cmd_size as usize)
    };

    let mut cb = unsafe {
        let ctx: *mut RdpqCommandHookInternal = ctx as *mut RdpqCommandHookInternal;
        Box::from_raw(ctx)
    };

    (cb.user_callback)(slice);

    // leak the pointer again for future calls
    let _ = Box::leak(cb);
}

/// Disassemble a RDP command
///
/// See [`rdpq_debug_disasm`](libdragon_sys::rdpq_debug_disasm) for details.
pub fn debug_disasm(buf: &mut [u64], out: &mut dfs::File) -> bool {
    unsafe {
        libdragon_sys::rdpq_debug_disasm(buf.as_mut_ptr(), out.fp.unwrap())
    }
}

/// Return the size of the next RDP commands
///
/// See [`rdpq_debug_disasm_size`](libdragon_sys::rdpq_debug_disasm_size) for details.
pub fn debug_disasm_size(buf: &mut [u64]) -> usize {
    unsafe {
        libdragon_sys::rdpq_debug_disasm_size(buf.as_mut_ptr()) as usize
    }
}

// rdpq_font.h

/// Wrapper around [`rdpq_font_t`](libdragon_sys::rdpq_font_s)
pub struct Font<'a> {
    pub(crate) ptr: *mut libdragon_sys::rdpq_font_t,
    pub(crate) phantom: core::marker::PhantomData<&'a u8>,
}

impl<'a> Font<'a> {
    /// Load a font from a file (.font64 format).
    ///
    /// Rust-specific: the LibDragon [`rdpq_font_t`](libdragon-sys::rdpq_font_t) is freed when this object is dropped
    ///
    /// See [`rdpq_font_load`](libdragon-sys::rdpq_font_load) for details.
    pub fn load(filename: &str) -> Self {
        let cfilename = CString::new(filename).unwrap();
        let ptr = unsafe {
            libdragon_sys::rdpq_font_load(cfilename.as_ptr())
        };
        Self {
            ptr: ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Load a font from a buffer in memory (.font64 format)
    ///
    /// The buffer provided must outlive the returned [Font]
    ///
    /// See [`rdpq_font_load_buf`](libdragon_sys::rdpq_font_load_buf)
    pub fn load_buf<'b, T>(buf: &'b mut [T]) -> Font<'b> {
        let sz = buf.len() * ::core::mem::size_of::<T>();
        let ptr = unsafe {
            libdragon_sys::rdpq_font_load_buf(buf.as_mut_ptr() as *mut _, sz as i32)
        };
        Font {
            ptr: ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Create a style for a font
    ///
    /// See [`rdpq_font_style`](libdragon_sys::rdpq_font_style) for details.
    pub fn style(&mut self, id: u8, style: FontStyle) {
        unsafe {
            libdragon_sys::rdpq_font_style(self.ptr, id, &Into::<libdragon_sys::rdpq_fontstyle_t>::into(style) as *const libdragon_sys::rdpq_fontstyle_t)
        }
    }

    /// Render a certain number of chars from a paragraph.
    ///
    /// Crash warning: you must terminate the `chars` array with a [ParagraphChar] that
    /// has a different `font_id` than the previous element.
    ///
    /// See [`rdpq_font_render_paragraph`](libdragon_sys::rdpq_font_render_paragraph) for details.
    pub fn render_paragraph(&self, chars: &[ParagraphChar], x0: f32, y0: f32) -> usize {
        // must have at least two elements
        if chars.len() < 2 { return 0; }

        // TODO in debug builds we could scan the elements to check the font_ids.

        unsafe {
            let chars_ptr = chars.as_ptr() as *const _;
            libdragon_sys::rdpq_font_render_paragraph(self.ptr as *const _, chars_ptr, x0, y0) as usize
        }
    }
}

impl<'a> Drop for Font<'a> {
    /// Free a font
    ///
    /// Rust-specific: this call happens for you, don't worry about it.
    ///
    /// See [`rdpq_font_free`](libdragon_sys::rdpq_font_free) for details.
    fn drop(&mut self) {
        unsafe {
            libdragon_sys::rdpq_font_free(self.ptr);
        }
    }
}

/// Wrapper around [`rdpq_fontstyle_t`](libdragon_sys::rdpq_fontstyle_t).
///
/// Initialize the structure manually.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FontStyle {
    pub color: graphics::Color,
}

impl Into<libdragon_sys::rdpq_fontstyle_t> for FontStyle {
    fn into(self) -> libdragon_sys::rdpq_fontstyle_t {
        unsafe {
            *core::mem::transmute::<&Self, *const libdragon_sys::rdpq_fontstyle_t>(&self)
        }
    }
}

// rdpq_macros.h
pub mod consts {
    //! module contains many of the defines from rdpq_macros.h and other various files.
    use crate::combiner1;
    use paste::paste;
    use crate::rdpq::{Blender, Combiner};

    /// SET_OTHER_MODES bit constants. See rdpq_macros.h in LibDragon.
    ///
    /// Flag to mark the combiner as required two passes
    pub const COMBINER_2PASS: u64 = 1u64 <<63;
    /// Combiner: mask to isolate settings related to cycle 0
    pub const COMB0_MASK: u64 = (0xFu64<<52)|(0x1Fu64<<47)|(0x7u64<<44)|(0x7u64<<41)|(0xFu64<<28)|(0x7u64<<15)|(0x7u64<<12)|(0x7u64<<9);
    /// Combiner: mask to isolate settings related to cycle 1
    pub const COMB1_MASK: u64 = !COMB0_MASK & 0x00FFFFFFFFFFFFFFu64;

    /// Some standard color combiners
    ///
    /// Draw a flat color.
    pub const COMBINER_FLAT: Combiner = crate::combiner1!((0 - 0) * 0 + PRIM, (0 - 0) * 0 + PRIM);
    /// Draw an interpolated color.
    pub const COMBINER_SHADE: Combiner = crate::combiner1!((0 - 0) * 0 + SHADE, (0 - 0) * 0 + SHADE);
    /// Draw with a texture.
    pub const COMBINER_TEX: Combiner = crate::combiner1!((0 - 0) * 0 + TEX0, (0 - 0) * 0 + TEX0);
    /// Draw with a texture modulated with a flat color.
    pub const COMBINER_TEX_FLAT: Combiner = crate::combiner1!((TEX0 - 0) * PRIM + 0, (TEX0 - 0) * PRIM + 0);
    /// Draw with a texture modulated with an interpolated color.
    pub const COMBINER_TEX_SHADE: Combiner = crate::combiner1!((TEX0 - 0) * SHADE + 0, (TEX0 - 0) * SHADE + 0);

    /// Rdpq extension: number of LODs
    pub const SOMX_NUMLODS_MASK: u64 = 7u64 << 59;            
    /// Rdpq extension: number of LODs shift
    pub const SOMX_NUMLODS_SHIFT: u64 = 59;
    
    /// Atomic: serialize command execution 
    pub const SOM_ATOMIC_PRIM: u64 = 1u64 << 55;            
    
    /// Set cycle-type: 1cyc
    pub const SOM_CYCLE_1: u64 = 0u64 << 52;            
    /// Set cycle-type: 2cyc
    pub const SOM_CYCLE_2: u64 = 1u64 << 52;            
    /// Set cycle-type: copy
    pub const SOM_CYCLE_COPY: u64 = 2u64 << 52;            
    /// Set cycle-type: fill
    pub const SOM_CYCLE_FILL: u64 = 3u64 << 52;            
    /// Cycle-type mask
    pub const SOM_CYCLE_MASK: u64 = 3u64 << 52;            
    /// Cycle-type shift
    pub const SOM_CYCLE_SHIFT: u64 = 52;
    
    /// Texture: enable perspective correction
    pub const SOM_TEXTURE_PERSP: u64 = 1u64 << 51;              
    /// Texture: enable "detail"
    pub const SOM_TEXTURE_DETAIL: u64 = 1u64 << 50;              
    /// Texture: enable "sharpen"
    pub const SOM_TEXTURE_SHARPEN: u64 = 1u64 << 49;              
    /// Texture: enable LODs.
    pub const SOM_TEXTURE_LOD: u64 = 1u64 << 48;              
    /// Texture: LODs shift
    pub const SOM_TEXTURE_LOD_SHIFT: u64 = 48;
    
    /// TLUT: no palettes
    pub const SOM_TLUT_NONE: u64 = 0u64 << 46;              
    /// TLUT: draw with palettes in format RGB16
    pub const SOM_TLUT_RGBA16: u64 = 2u64 << 46;              
    /// TLUT: draw with palettes in format IA16
    pub const SOM_TLUT_IA16: u64 = 3u64 << 46;              
    /// TLUT mask
    pub const SOM_TLUT_MASK: u64 = 3u64 << 46;              
    /// TLUT mask shift
    pub const SOM_TLUT_SHIFT: u64 = 46;
    
    /// Texture sampling: point sampling (1x1)
    pub const SOM_SAMPLE_POINT: u64 = 0u64 << 44;              
    /// Texture sampling: bilinear interpolation (2x2)
    pub const SOM_SAMPLE_BILINEAR: u64 = 2u64 << 44;              
    /// Texture sampling: mid-texel average (2x2)
    pub const SOM_SAMPLE_MEDIAN: u64 = 3u64 << 44;              
    /// Texture sampling mask
    pub const SOM_SAMPLE_MASK: u64 = 3u64 << 44;              
    /// Texture sampling mask shift
    pub const SOM_SAMPLE_SHIFT: u64 = 44;
    
    /// Texture Filter, cycle 0 (TEX0): standard fetching (for RGB)
    pub const SOM_TF0_RGB: u64 = 1u64 << 43;               
    /// Texture Filter, cycle 0 (TEX0): fetch nearest and do first step of color conversion (for YUV)
    pub const SOM_TF0_YUV: u64 = 0u64 << 43;               
    /// Texture Filter, cycle 1 (TEX1): standard fetching (for RGB)
    pub const SOM_TF1_RGB: u64 = 2u64 << 41;               
    /// Texture Filter, cycle 1 (TEX1): fetch nearest and do first step of color conversion (for YUV)
    pub const SOM_TF1_YUV: u64 = 0u64 << 41;               
    /// Texture Filter, cycle 1 (TEX1): don't fetch, and instead do color conversion on TEX0 (allows YUV with bilinear filtering)
    pub const SOM_TF1_YUVTEX0: u64 = 1u64 << 41;               
    /// Texture Filter mask
    pub const SOM_TF_MASK: u64 = 7u64 << 41;               
    /// Texture filter mask shift
    pub const SOM_TF_SHIFT: u64 = 41;
    
    /// RGB Dithering: square filter
    pub const SOM_RGBDITHER_SQUARE: u64 = 0u64 << 38;            
    /// RGB Dithering: bayer filter
    pub const SOM_RGBDITHER_BAYER: u64 = 1u64 << 38;            
    /// RGB Dithering: noise
    pub const SOM_RGBDITHER_NOISE: u64 = 2u64 << 38;            
    /// RGB Dithering: none
    pub const SOM_RGBDITHER_NONE: u64 = 3u64 << 38;            
    /// RGB Dithering mask
    pub const SOM_RGBDITHER_MASK: u64 = 3u64 << 38;            
    /// RGB Dithering mask shift
    pub const SOM_RGBDITHER_SHIFT: u64 = 38;
    
    /// Alpha Dithering: same as RGB
    pub const SOM_ALPHADITHER_SAME: u64 = 0u64 << 36;            
    /// Alpha Dithering: invert pattern compared to RG
    pub const SOM_ALPHADITHER_INVERT: u64 = 1u64 << 36;            
    /// Alpha Dithering: noise
    pub const SOM_ALPHADITHER_NOISE: u64 = 2u64 << 36;            
    /// Alpha Dithering: none
    pub const SOM_ALPHADITHER_NONE: u64 = 3u64 << 36;            
    /// Alpha Dithering mask
    pub const SOM_ALPHADITHER_MASK: u64 = 3u64 << 36;            
    /// Alpha Dithering mask shift
    pub const SOM_ALPHADITHER_SHIFT: u64 = 36;
    
    /// RDPQ special state: fogging is enabled
    pub const SOMX_FOG: u64 = 1u64 << 32;            
    /// RDPQ special state: render mode update is frozen (see #rdpq_mode_begin)
    pub const SOMX_UPDATE_FREEZE: u64 = 1u64 << 33;            
    /// RDPQ special state: reduced antialiasing is enabled
    pub const SOMX_AA_REDUCED: u64 = 1u64 << 34;            
    /// RDPQ special state: mimap interpolation (aka trilinear) requested
    pub const SOMX_LOD_INTERPOLATE: u64 = 1u64 << 35;            
    
    /// Blender: mask of settings related to pass 0
    pub const SOM_BLEND0_MASK: u64 = 0xCCCC0000u64 | SOM_BLENDING | SOM_READ_ENABLE | SOMX_BLEND_2PASS;
    /// Blender: mask of settings related to pass 1
    pub const SOM_BLEND1_MASK: u64 = 0x33330000u64 | SOM_BLENDING | SOM_READ_ENABLE | SOMX_BLEND_2PASS;
    /// Blender: mask of all settings
    pub const SOM_BLEND_MASK: u64 = SOM_BLEND0_MASK | SOM_BLEND1_MASK;
    
    /// RDPQ special state: record that the blender is made of 2 passes
    pub const SOMX_BLEND_2PASS: u64 = 1u64 << 15;            
    
    /// Activate blending for all pixels
    pub const SOM_BLENDING: u64 = 1u64 << 14;            
    
    /// Blender IN_ALPHA is the output of the combiner output (default)
    pub const SOM_BLALPHA_CC: u64 = 0u64 << 12;          
    /// Blender IN_ALPHA is the coverage of the current pixel
    pub const SOM_BLALPHA_CVG: u64 = 2u64 << 12;          
    /// Blender IN_ALPHA is the product of the combiner output and the coverage
    pub const SOM_BLALPHA_CVG_TIMES_CC: u64 = 3u64 << 12;          
    /// Blender alpha configuration mask
    pub const SOM_BLALPHA_MASK: u64 = 3u64 << 12;          
    /// Blender alpha configuration shift
    pub const SOM_BLALPHA_SHIFT: u64 = 12;
    
    /// Z-mode: opaque surface
    pub const SOM_ZMODE_OPAQUE: u64 = 0u64 << 10;        
    /// Z-mode: interprenating surfaces
    pub const SOM_ZMODE_INTERPENETRATING: u64 = 1u64 << 10;        
    /// Z-mode: transparent surface
    pub const SOM_ZMODE_TRANSPARENT: u64 = 2u64 << 10;        
    /// Z-mode: decal surface
    pub const SOM_ZMODE_DECAL: u64 = 3u64 << 10;        
    /// Z-mode mask
    pub const SOM_ZMODE_MASK: u64 = 3u64 << 10;        
    /// Z-mode mask shift
    pub const SOM_ZMODE_SHIFT: u64 = 10;
    
    /// Activate Z-buffer write
    pub const SOM_Z_WRITE: u64 = 1u64 << 5;             
    /// Z-buffer write bit shift
    pub const SOM_Z_WRITE_SHIFT: u64 = 5;
    
    /// Activate Z-buffer compare
    pub const SOM_Z_COMPARE: u64 = 1u64 << 4;             
    /// Z-buffer compare bit shift
    pub const SOM_Z_COMPARE_SHIFT: u64 = 4;
    
    /// Z-source: per-pixel Z
    pub const SOM_ZSOURCE_PIXEL: u64 = 0u64 << 2;             
    /// Z-source: fixed value
    pub const SOM_ZSOURCE_PRIM: u64 = 1u64 << 2;             
    /// Z-source mask
    pub const SOM_ZSOURCE_MASK: u64 = 1u64 << 2;             
    /// Z-source mask shift
    pub const SOM_ZSOURCE_SHIFT: u64 = 2;
    
    /// Alpha Compare: disable
    pub const SOM_ALPHACOMPARE_NONE: u64 = 0u64 << 0;        
    /// Alpha Compare: use blend alpha as threshold
    pub const SOM_ALPHACOMPARE_THRESHOLD: u64 = 1u64 << 0;        
    /// Alpha Compare: use noise as threshold
    pub const SOM_ALPHACOMPARE_NOISE: u64 = 3u64 << 0;        
    /// Alpha Compare mask
    pub const SOM_ALPHACOMPARE_MASK: u64 = 3u64 << 0;        
    /// Alpha Compare mask shift
    pub const SOM_ALPHACOMPARE_SHIFT: u64 = 0;
    
    /// Enable reads from framebuffer
    pub const SOM_READ_ENABLE: u64 = 1u64 << 6;  
    /// Enable anti-alias
    pub const SOM_AA_ENABLE: u64 = 1u64 << 3;  
    
    /// Coverage: add and clamp to 7 (full)
    pub const SOM_COVERAGE_DEST_CLAMP: u64 = 0u64 << 8;  
    /// Coverage: add and wrap from 0
    pub const SOM_COVERAGE_DEST_WRAP: u64 = 1u64 << 8;  
    /// Coverage: force 7 (full)
    pub const SOM_COVERAGE_DEST_ZAP: u64 = 2u64 << 8;  
    /// Coverage: save (don't write)
    pub const SOM_COVERAGE_DEST_SAVE: u64 = 3u64 << 8;  
    /// Coverage mask
    pub const SOM_COVERAGE_DEST_MASK: u64 = 3u64 << 8;  
    /// Coverage mask shift
    pub const SOM_COVERAGE_DEST_SHIFT: u64 = 8;
    
    /// Update color buffer only on coverage overflow
    pub const SOM_COLOR_ON_CVG_OVERFLOW: u64 = 1u64 << 7;  

    /// Some standard blend modes
    ///
    /// Blending mode: multiplicative alpha
    pub const BLENDER_MULTIPLY: Blender = crate::blender!(IN_RGB * IN_ALPHA + MEMORY_RGB * INV_MUX_ALPHA);
    /// Blending mode: multiplicative alpha with a constant value
    pub const BLENDER_MULTIPLY_CONST: Blender = crate::blender!(IN_RGB * FOG_ALPHA + MEMORY_RGB * INV_MUX_ALPHA);
    /// Blending mode: additive alpha
    pub const BLENDER_ADDTIVE: Blender = crate::blender!(IN_RGB * IN_ALPHA + MEMORY_RGB * ONE);
    /// Fogging mode: standard.
    pub const FOG_STANDARD: Blender = crate::blender!(IN_RGB * SHADE_ALPHA + FOG_RGB * INV_MUX_ALPHA);
    
    /// SOME_OTHER_MODES RDP Color Combiner configuration
    pub mod cc {
        //! Helper macros for [`combiner1`] and [`combiner2`] macros, which create
        //! [Combiner] states.
        //! 
        //! Generally, you don't need to access these values directly.
        pub const _COMB1_RGB_SUBA_TEX0: u64 = 1;
        pub const _COMB1_RGB_SUBA_PRIM: u64 = 3;
        pub const _COMB1_RGB_SUBA_SHADE: u64 = 4;
        pub const _COMB1_RGB_SUBA_ENV: u64 = 5;
        pub const _COMB1_RGB_SUBA_ONE: u64 = 6;
        pub const _COMB1_RGB_SUBA_1: u64 = 6;
        pub const _COMB1_RGB_SUBA_NOISE: u64 = 7;
        pub const _COMB1_RGB_SUBA_ZERO: u64 = 8;
        pub const _COMB1_RGB_SUBA_0: u64 = 8;
        
        pub const _COMB2A_RGB_SUBA_TEX0: u64 = 1;
        pub const _COMB2A_RGB_SUBA_TEX1: u64 = 2;
        pub const _COMB2A_RGB_SUBA_PRIM: u64 = 3;
        pub const _COMB2A_RGB_SUBA_SHADE: u64 = 4;
        pub const _COMB2A_RGB_SUBA_ENV: u64 = 5;
        pub const _COMB2A_RGB_SUBA_ONE: u64 = 6;
        pub const _COMB2A_RGB_SUBA_1: u64 = 6;
        pub const _COMB2A_RGB_SUBA_NOISE: u64 = 7;
        pub const _COMB2A_RGB_SUBA_ZERO: u64 = 8;
        pub const _COMB2A_RGB_SUBA_0: u64 = 8;
        
        pub const _COMB2B_RGB_SUBA_COMBINED: u64 = 0;
        /// TEX0 not available in 2nd cycle (pipelined)
        pub const _COMB2B_RGB_SUBA_TEX1: u64 = 1;
        pub const _COMB2B_RGB_SUBA_PRIM: u64 = 3;
        pub const _COMB2B_RGB_SUBA_SHADE: u64 = 4;
        pub const _COMB2B_RGB_SUBA_ENV: u64 = 5;
        pub const _COMB2B_RGB_SUBA_ONE: u64 = 6;
        pub const _COMB2B_RGB_SUBA_1: u64 = 6;
        pub const _COMB2B_RGB_SUBA_NOISE: u64 = 7;
        pub const _COMB2B_RGB_SUBA_ZERO: u64 = 8;
        pub const _COMB2B_RGB_SUBA_0: u64 = 8;
        
        pub const _COMB1_RGB_SUBB_TEX0: u64 = 1;
        pub const _COMB1_RGB_SUBB_PRIM: u64 = 3;
        pub const _COMB1_RGB_SUBB_SHADE: u64 = 4;
        pub const _COMB1_RGB_SUBB_ENV: u64 = 5;
        pub const _COMB1_RGB_SUBB_KEYCENTER: u64 = 6;
        pub const _COMB1_RGB_SUBB_K4: u64 = 7;
        pub const _COMB1_RGB_SUBB_ZERO: u64 = 8;
        pub const _COMB1_RGB_SUBB_0: u64 = 8;
        
        pub const _COMB2A_RGB_SUBB_TEX0: u64 = 1;
        pub const _COMB2A_RGB_SUBB_TEX1: u64 = 2;
        pub const _COMB2A_RGB_SUBB_PRIM: u64 = 3;
        pub const _COMB2A_RGB_SUBB_SHADE: u64 = 4;
        pub const _COMB2A_RGB_SUBB_ENV: u64 = 5;
        pub const _COMB2A_RGB_SUBB_KEYCENTER: u64 = 6;
        pub const _COMB2A_RGB_SUBB_K4: u64 = 7;
        pub const _COMB2A_RGB_SUBB_ZERO: u64 = 8;
        pub const _COMB2A_RGB_SUBB_0: u64 = 8;
        
        pub const _COMB2B_RGB_SUBB_COMBINED: u64 = 0;
        /// TEX0 not available in 2nd cycle (pipelined)
        pub const _COMB2B_RGB_SUBB_TEX1: u64 = 1;
        pub const _COMB2B_RGB_SUBB_PRIM: u64 = 3;
        pub const _COMB2B_RGB_SUBB_SHADE: u64 = 4;
        pub const _COMB2B_RGB_SUBB_ENV: u64 = 5;
        pub const _COMB2B_RGB_SUBB_KEYCENTER: u64 = 6;
        pub const _COMB2B_RGB_SUBB_K4: u64 = 7;
        pub const _COMB2B_RGB_SUBB_ZERO: u64 = 8;
        pub const _COMB2B_RGB_SUBB_0: u64 = 8;
        
        pub const _COMB1_RGB_MUL_TEX0: u64 = 1;
        pub const _COMB1_RGB_MUL_PRIM: u64 = 3;
        pub const _COMB1_RGB_MUL_SHADE: u64 = 4;
        pub const _COMB1_RGB_MUL_ENV: u64 = 5;
        pub const _COMB1_RGB_MUL_KEYSCALE: u64 = 6;
        pub const _COMB1_RGB_MUL_TEX0_ALPHA: u64 = 8;
        pub const _COMB1_RGB_MUL_PRIM_ALPHA: u64 = 10;
        pub const _COMB1_RGB_MUL_SHADE_ALPHA: u64 = 11;
        pub const _COMB1_RGB_MUL_ENV_ALPHA: u64 = 12;
        pub const _COMB1_RGB_MUL_LOD_FRAC: u64 = 13;
        pub const _COMB1_RGB_MUL_PRIM_LOD_FRAC: u64 = 14;
        pub const _COMB1_RGB_MUL_K5: u64 = 15;
        pub const _COMB1_RGB_MUL_ZERO: u64 = 16;
        pub const _COMB1_RGB_MUL_0: u64 = 16;
        
        pub const _COMB2A_RGB_MUL_TEX0: u64 = 1;
        pub const _COMB2A_RGB_MUL_TEX1: u64 = 2;
        pub const _COMB2A_RGB_MUL_PRIM: u64 = 3;
        pub const _COMB2A_RGB_MUL_SHADE: u64 = 4;
        pub const _COMB2A_RGB_MUL_ENV: u64 = 5;
        pub const _COMB2A_RGB_MUL_KEYSCALE: u64 = 6;
        pub const _COMB2A_RGB_MUL_TEX0_ALPHA: u64 = 8;
        pub const _COMB2A_RGB_MUL_TEX1_ALPHA: u64 = 9;
        pub const _COMB2A_RGB_MUL_PRIM_ALPHA: u64 = 10;
        pub const _COMB2A_RGB_MUL_SHADE_ALPHA: u64 = 11;
        pub const _COMB2A_RGB_MUL_ENV_ALPHA: u64 = 12;
        pub const _COMB2A_RGB_MUL_LOD_FRAC: u64 = 13;
        pub const _COMB2A_RGB_MUL_PRIM_LOD_FRAC: u64 = 14;
        pub const _COMB2A_RGB_MUL_K5: u64 = 15;
        pub const _COMB2A_RGB_MUL_ZERO: u64 = 16;
        pub const _COMB2A_RGB_MUL_0: u64 = 16;
        
        pub const _COMB2B_RGB_MUL_COMBINED: u64 = 0;
        /// TEX0 not available in 2nd cycle (pipelined)
        pub const _COMB2B_RGB_MUL_TEX1: u64 = 1;
        pub const _COMB2B_RGB_MUL_PRIM: u64 = 3;
        pub const _COMB2B_RGB_MUL_SHADE: u64 = 4;
        pub const _COMB2B_RGB_MUL_ENV: u64 = 5;
        pub const _COMB2B_RGB_MUL_KEYSCALE: u64 = 6;
        pub const _COMB2B_RGB_MUL_COMBINED_ALPHA: u64 = 7;
        /// TEX0_ALPHA not available in 2nd cycle (pipelined)
        pub const _COMB2B_RGB_MUL_TEX1_ALPHA: u64 = 8;
        pub const _COMB2B_RGB_MUL_PRIM_ALPHA: u64 = 10;
        pub const _COMB2B_RGB_MUL_SHADE_ALPHA: u64 = 11;
        pub const _COMB2B_RGB_MUL_ENV_ALPHA: u64 = 12;
        pub const _COMB2B_RGB_MUL_LOD_FRAC: u64 = 13;
        pub const _COMB2B_RGB_MUL_PRIM_LOD_FRAC: u64 = 14;
        pub const _COMB2B_RGB_MUL_K5: u64 = 15;
        pub const _COMB2B_RGB_MUL_ZERO: u64 = 16;
        pub const _COMB2B_RGB_MUL_0: u64 = 16;
        
        pub const _COMB1_RGB_ADD_TEX0: u64 = 1;
        pub const _COMB1_RGB_ADD_PRIM: u64 = 3;
        pub const _COMB1_RGB_ADD_SHADE: u64 = 4;
        pub const _COMB1_RGB_ADD_ENV: u64 = 5;
        pub const _COMB1_RGB_ADD_ONE: u64 = 6;
        pub const _COMB1_RGB_ADD_1: u64 = 6;
        pub const _COMB1_RGB_ADD_ZERO: u64 = 7;
        pub const _COMB1_RGB_ADD_0: u64 = 7;
        
        pub const _COMB2A_RGB_ADD_TEX0: u64 = 1;
        pub const _COMB2A_RGB_ADD_TEX1: u64 = 2;
        pub const _COMB2A_RGB_ADD_PRIM: u64 = 3;
        pub const _COMB2A_RGB_ADD_SHADE: u64 = 4;
        pub const _COMB2A_RGB_ADD_ENV: u64 = 5;
        pub const _COMB2A_RGB_ADD_ONE: u64 = 6;
        pub const _COMB2A_RGB_ADD_1: u64 = 6;
        pub const _COMB2A_RGB_ADD_ZERO: u64 = 7;
        pub const _COMB2A_RGB_ADD_0: u64 = 7;
        
        pub const _COMB2B_RGB_ADD_COMBINED: u64 = 0;
        /// TEX0 not available in 2nd cycle (pipelined)
        pub const _COMB2B_RGB_ADD_TEX1: u64 = 1;
        pub const _COMB2B_RGB_ADD_PRIM: u64 = 3;
        pub const _COMB2B_RGB_ADD_SHADE: u64 = 4;
        pub const _COMB2B_RGB_ADD_ENV: u64 = 5;
        pub const _COMB2B_RGB_ADD_ONE: u64 = 6;
        pub const _COMB2B_RGB_ADD_1: u64 = 6;
        pub const _COMB2B_RGB_ADD_ZERO: u64 = 7;
        pub const _COMB2B_RGB_ADD_0: u64 = 7;
        
        pub const _COMB1_ALPHA_ADDSUB_TEX0: u64 = 1;
        pub const _COMB1_ALPHA_ADDSUB_PRIM: u64 = 3;
        pub const _COMB1_ALPHA_ADDSUB_SHADE: u64 = 4;
        pub const _COMB1_ALPHA_ADDSUB_ENV: u64 = 5;
        pub const _COMB1_ALPHA_ADDSUB_ONE: u64 = 6;
        pub const _COMB1_ALPHA_ADDSUB_1: u64 = 6;
        pub const _COMB1_ALPHA_ADDSUB_ZERO: u64 = 7;
        pub const _COMB1_ALPHA_ADDSUB_0: u64 = 7;
        
        pub const _COMB2A_ALPHA_ADDSUB_TEX0: u64 = 1;
        pub const _COMB2A_ALPHA_ADDSUB_TEX1: u64 = 2;
        pub const _COMB2A_ALPHA_ADDSUB_PRIM: u64 = 3;
        pub const _COMB2A_ALPHA_ADDSUB_SHADE: u64 = 4;
        pub const _COMB2A_ALPHA_ADDSUB_ENV: u64 = 5;
        pub const _COMB2A_ALPHA_ADDSUB_ONE: u64 = 6;
        pub const _COMB2A_ALPHA_ADDSUB_1: u64 = 6;
        pub const _COMB2A_ALPHA_ADDSUB_ZERO: u64 = 7;
        pub const _COMB2A_ALPHA_ADDSUB_0: u64 = 7;
        
        pub const _COMB2B_ALPHA_ADDSUB_COMBINED: u64 = 0;
        /// TEX0 not available in 2nd cycle (pipelined)
        pub const _COMB2B_ALPHA_ADDSUB_TEX1: u64 = 1;
        pub const _COMB2B_ALPHA_ADDSUB_PRIM: u64 = 3;
        pub const _COMB2B_ALPHA_ADDSUB_SHADE: u64 = 4;
        pub const _COMB2B_ALPHA_ADDSUB_ENV: u64 = 5;
        pub const _COMB2B_ALPHA_ADDSUB_ONE: u64 = 6;
        pub const _COMB2B_ALPHA_ADDSUB_1: u64 = 6;
        pub const _COMB2B_ALPHA_ADDSUB_ZERO: u64 = 7;
        pub const _COMB2B_ALPHA_ADDSUB_0: u64 = 7;
        
        pub const _COMB1_ALPHA_MUL_LOD_FRAC: u64 = 0;
        pub const _COMB1_ALPHA_MUL_TEX0: u64 = 1;
        pub const _COMB1_ALPHA_MUL_PRIM: u64 = 3;
        pub const _COMB1_ALPHA_MUL_SHADE: u64 = 4;
        pub const _COMB1_ALPHA_MUL_ENV: u64 = 5;
        pub const _COMB1_ALPHA_MUL_PRIM_LOD_FRAC: u64 = 6;
        pub const _COMB1_ALPHA_MUL_ZERO: u64 = 7;
        pub const _COMB1_ALPHA_MUL_0: u64 = 7;
        
        pub const _COMB2A_ALPHA_MUL_LOD_FRAC: u64 = 0;
        pub const _COMB2A_ALPHA_MUL_TEX0: u64 = 1;
        pub const _COMB2A_ALPHA_MUL_TEX1: u64 = 2;
        pub const _COMB2A_ALPHA_MUL_PRIM: u64 = 3;
        pub const _COMB2A_ALPHA_MUL_SHADE: u64 = 4;
        pub const _COMB2A_ALPHA_MUL_ENV: u64 = 5;
        pub const _COMB2A_ALPHA_MUL_PRIM_LOD_FRAC: u64 = 6;
        pub const _COMB2A_ALPHA_MUL_ZERO: u64 = 7;
        pub const _COMB2A_ALPHA_MUL_0: u64 = 7;
        
        pub const _COMB2B_ALPHA_MUL_LOD_FRAC: u64 = 0;
        /// TEX0 not available in 2nd cycle (pipelined)
        pub const _COMB2B_ALPHA_MUL_TEX1: u64 = 1;
        pub const _COMB2B_ALPHA_MUL_PRIM: u64 = 3;
        pub const _COMB2B_ALPHA_MUL_SHADE: u64 = 4;
        pub const _COMB2B_ALPHA_MUL_ENV: u64 = 5;
        pub const _COMB2B_ALPHA_MUL_PRIM_LOD_FRAC: u64 = 6;
        pub const _COMB2B_ALPHA_MUL_ZERO: u64 = 7;
        pub const _COMB2B_ALPHA_MUL_0: u64 = 7;
    }

    /// SOME_OTHER_MODES RDP Color Blender configuration
    pub mod bl {
        //! Helper macros for [`blender`](crate::rdpq::blender) and [`blender2`](crate::rdpq::blender) macros, which create
        //! [Blender](crate::rdpq::Blender) states.
        //! 
        //! Generally, you don't need to access these values directly.
        pub const _SOM_BLEND1_A_IN_RGB: u32 = 0u32;
        pub const _SOM_BLEND1_A_MEMORY_RGB: u32 = 1u32;
        pub const _SOM_BLEND1_A_BLEND_RGB: u32 = 2u32;
        pub const _SOM_BLEND1_A_FOG_RGB: u32 = 3u32;
        
        pub const _SOM_BLEND1_B1_IN_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND1_B1_FOG_ALPHA: u32 = 1u32;
        pub const _SOM_BLEND1_B1_SHADE_ALPHA: u32 = 2u32;
        pub const _SOM_BLEND1_B1_ZERO: u32 = 3u32;
        pub const _SOM_BLEND1_B1_0: u32 = 3u32;
        
        pub const _SOM_BLEND1_B2_INV_MUX_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND1_B2_MEMORY_CVG: u32 = 1u32;
        pub const _SOM_BLEND1_B2_ONE: u32 = 2u32;
        pub const _SOM_BLEND1_B2_1: u32 = 2u32;
        pub const _SOM_BLEND1_B2_ZERO: u32 = 3u32;
        pub const _SOM_BLEND1_B2_0: u32 = 3u32;
        
        pub const _SOM_BLEND2A_A_IN_RGB: u32 = 0u32;
        pub const _SOM_BLEND2A_A_BLEND_RGB: u32 = 2u32;
        pub const _SOM_BLEND2A_A_FOG_RGB: u32 = 3u32;
        
        pub const _SOM_BLEND2A_B1_IN_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND2A_B1_FOG_ALPHA: u32 = 1u32;
        pub const _SOM_BLEND2A_B1_SHADE_ALPHA: u32 = 2u32;
        pub const _SOM_BLEND2A_B1_ZERO: u32 = 3u32;
        pub const _SOM_BLEND2A_B1_0: u32 = 3u32;
        
        pub const _SOM_BLEND2A_B2_INV_MUX_ALPHA: u32 = 0u32;    // only valid option is "1-b1" in the first pass
        
        pub const _SOM_BLEND2B_A_CYCLE1_RGB: u32 = 0u32;
        pub const _SOM_BLEND2B_A_MEMORY_RGB: u32 = 1u32;
        pub const _SOM_BLEND2B_A_BLEND_RGB: u32 = 2u32;
        pub const _SOM_BLEND2B_A_FOG_RGB: u32 = 3u32;
        
        pub const _SOM_BLEND2B_B1_IN_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND2B_B1_FOG_ALPHA: u32 = 1u32;
        pub const _SOM_BLEND2B_B1_SHADE_ALPHA: u32 = 2u32;
        pub const _SOM_BLEND2B_B1_ZERO: u32 = 3u32;
        pub const _SOM_BLEND2B_B1_0: u32 = 3u32;
        
        pub const _SOM_BLEND2B_B2_INV_MUX_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND2B_B2_MEMORY_CVG: u32 = 1u32;
        pub const _SOM_BLEND2B_B2_ONE: u32 = 2u32;
        pub const _SOM_BLEND2B_B2_1: u32 = 2u32;
        pub const _SOM_BLEND2B_B2_ZERO: u32 = 3u32;
        pub const _SOM_BLEND2B_B2_0: u32 = 3u32;
        
        pub const _SOM_BLEND_EXTRA_A_IN_RGB: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_A_CYCLE1_RGB: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_A_MEMORY_RGB: u32 = crate::rdpq::consts::SOM_READ_ENABLE as u32;
        pub const _SOM_BLEND_EXTRA_A_BLEND_RGB: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_A_FOG_RGB: u32 = 0u32;
        
        pub const _SOM_BLEND_EXTRA_B1_IN_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B1_FOG_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B1_SHADE_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B1_ZERO: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B1_0: u32 = 0u32;
        
        pub const _SOM_BLEND_EXTRA_B2_INV_MUX_ALPHA: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B2_MEMORY_CVG: u32 = crate::rdpq::consts::SOM_READ_ENABLE as u32;
        pub const _SOM_BLEND_EXTRA_B2_ONE: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B2_1: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B2_ZERO: u32 = 0u32;
        pub const _SOM_BLEND_EXTRA_B2_0: u32 = 0u32;
    }
}

// Implementation of __rdpq_1cyc_comb_rgb
#[doc(hidden)]
#[macro_export]
macro_rules! _comb_1cyc_rgb {
    ($suba:tt, $subb:tt, $mul:tt, $add:tt) => ({
        paste! {
            (  ($crate::rdpq::consts::cc::[<_COMB1_RGB_SUBA_ $suba>] << 52) 
             | ($crate::rdpq::consts::cc::[<_COMB1_RGB_SUBB_ $subb>] << 28) 
             | ($crate::rdpq::consts::cc::[<_COMB1_RGB_MUL_  $mul>]  << 47) 
             | ($crate::rdpq::consts::cc::[<_COMB1_RGB_ADD_  $add>]  << 15) 
             | ($crate::rdpq::consts::cc::[<_COMB1_RGB_SUBA_ $suba>] << 37) 
             | ($crate::rdpq::consts::cc::[<_COMB1_RGB_SUBB_ $subb>] << 24) 
             | ($crate::rdpq::consts::cc::[<_COMB1_RGB_MUL_  $mul>]  << 32) 
             | ($crate::rdpq::consts::cc::[<_COMB1_RGB_ADD_  $add>]   << 6)
            )
        }
    });
}

// Implementation of __rdpq_1cyc_comb_alpha
#[doc(hidden)]
#[macro_export]
macro_rules! _comb_1cyc_alpha {
    ($suba:tt, $subb:tt, $mul:tt, $add:tt) => ({
        paste! {
            (  ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_ADDSUB_ $suba>] << 44) 
             | ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_ADDSUB_ $subb>] << 12) 
             | ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_MUL_    $mul>]  << 41) 
             | ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_ADDSUB_ $add>]  <<  9) 
             | ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_ADDSUB_ $suba>] << 21) 
             | ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_ADDSUB_ $subb>] <<  3) 
             | ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_MUL_    $mul>]  << 18) 
             | ($crate::rdpq::consts::cc::[<_COMB1_ALPHA_ADDSUB_ $add>]   << 0)
            )
        }
    });
}

// Implementation of __rdpq_2cyc_comb2a_rgb
#[doc(hidden)]
#[macro_export]
macro_rules! _comb_2cyca_rgb {
    ($suba:tt, $subb:tt, $mul:tt, $add:tt) => ({
        paste! {
            (  ($crate::rdpq::consts::cc::[<_COMB2A_RGB_SUBA_ $suba>] << 52) 
             | ($crate::rdpq::consts::cc::[<_COMB2A_RGB_SUBB_ $subb>] << 28) 
             | ($crate::rdpq::consts::cc::[<_COMB2A_RGB_MUL_  $mul>]  << 47) 
             | ($crate::rdpq::consts::cc::[<_COMB2A_RGB_ADD_  $add>]  << 15) 
            )
        }
    });
}

// Implementation of __rdpq_2cyc_comb2a_alpha
#[doc(hidden)]
#[macro_export]
macro_rules! _comb_2cyca_alpha {
    ($suba:tt, $subb:tt, $mul:tt, $add:tt) => ({
        paste! {
            (  ($crate::rdpq::consts::cc::[<_COMB2A_ALPHA_ADDSUB_ $suba>] << 44) 
             | ($crate::rdpq::consts::cc::[<_COMB2A_ALPHA_ADDSUB_ $subb>] << 12) 
             | ($crate::rdpq::consts::cc::[<_COMB2A_ALPHA_MUL_    $mul>]  << 41) 
             | ($crate::rdpq::consts::cc::[<_COMB2A_ALPHA_ADDSUB_ $add>]  <<  9) 
            )
        }
    });
}

// Implementation of __rdpq_2cyc_comb2b_rgb
#[doc(hidden)]
#[macro_export]
macro_rules! _comb_2cycb_rgb {
    ($suba:tt, $subb:tt, $mul:tt, $add:tt) => ({
        paste! {
            (  ($crate::rdpq::consts::cc::[<_COMB2B_RGB_SUBA_ $suba>] << 37) 
             | ($crate::rdpq::consts::cc::[<_COMB2B_RGB_SUBB_ $subb>] << 24) 
             | ($crate::rdpq::consts::cc::[<_COMB2B_RGB_MUL_  $mul>]  << 32) 
             | ($crate::rdpq::consts::cc::[<_COMB2B_RGB_ADD_  $add>]  <<  6) 
            )
        }
    });
}

// Implementation of __rdpq_2cyc_comb2b_alpha
#[doc(hidden)]
#[macro_export]
macro_rules! _comb_2cycb_alpha {
    ($suba:tt, $subb:tt, $mul:tt, $add:tt) => ({
        paste! {
            (  ($crate::rdpq::consts::cc::[<_COMB2B_ALPHA_ADDSUB_ $suba>] << 21) 
             | ($crate::rdpq::consts::cc::[<_COMB2B_ALPHA_ADDSUB_ $subb>] <<  3) 
             | ($crate::rdpq::consts::cc::[<_COMB2B_ALPHA_MUL_    $mul>]  << 18) 
             | ($crate::rdpq::consts::cc::[<_COMB2B_ALPHA_ADDSUB_ $add>]  <<  0) 
            )
        }
    });
}

/// Build a 1-pass combiner formula
///
/// Rust: the Rust version has a slightly differently syntax than the C macro. Instead
/// of the double-parenthese and comma-list of arguments, define your blend modes using
/// the (A-B)*C+D syntax:
///
/// ```rust
///     let mode = combiner1!((TEX0 - 0) * SHADE + 0, (0 - 0) * 0 + TEX0);
/// ```
///
/// You may need to refer to the [`RDPQ_COMBINER1`] macro in `rdpq_macros.h` to
/// understand how the arguments to this macro work and which are valid.
#[macro_export]
macro_rules! combiner1 {
    (($suba_rgb:tt - $subb_rgb:tt) * $mul_rgb:tt + $add_rgb:tt,
     ($suba_al:tt - $subb_al:tt) * $mul_al:tt + $add_al:tt) => ({ 
        combiner1!($suba_rgb, $subb_rgb, $mul_rgb, $add_rgb,
                   $suba_al, $subb_al, $mul_al, $add_al) });

    ($suba_rgb:tt, $subb_rgb:tt, $mul_rgb:tt, $add_rgb:tt,
     $suba_al:tt, $subb_al:tt, $mul_al:tt, $add_al:tt) => ({
        let combrgb   = $crate::_comb_1cyc_rgb!($suba_rgb, $subb_rgb, $mul_rgb, $add_rgb);
        let combalpha = $crate::_comb_1cyc_alpha!($suba_al, $subb_al, $mul_al, $add_al);
        $crate::rdpq::Combiner::new_const(combrgb | combalpha)
    });
}

/// Build a 2-pass combiner formula
///
/// Rust: the Rust version has a slightly differently syntax than the C macro. Instead
/// of the double-parenthese and comma-list of arguments, define your blend modes using
/// the (A-B)*C+D syntax:
///
/// ```rust
///                           // RGB                      // A
///     let mode = combiner2!((TEX0     - 0) * SHADE + 0, (0 - 0) * 0 + TEX0,     // Cyc 0
///                           (COMBINED - 0) * ENV   + 0, (0 - 0) * 0 + COMBINED  // Cyc 1
///                          );
/// ```
///
/// You may need to refer to the [`RDPQ_COMBINER2`] macro in `rdpq_macros.h` to
/// understand how the arguments to this macro work and which are valid.
#[macro_export]
macro_rules! combiner2 {
    (($suba0_rgb:tt - $subb0_rgb:tt) * $mul0_rgb:tt + $add0_rgb:tt, ($suba0_al:tt - $subb0_al:tt) * $mul0_al:tt + $add0_al:tt,
     ($suba1_rgb:tt - $subb1_rgb:tt) * $mul1_rgb:tt + $add1_rgb:tt, ($suba1_al:tt - $subb1_al:tt) * $mul1_al:tt + $add1_al:tt) => ({
        combiner2!($suba0_rgb, $subb0_rgb, $mul0_rgb, $add0_rgb, $suba0_al, $subb0_al, $mul0_al, $add0_al,
                   $suba1_rgb, $subb1_rgb, $mul1_rgb, $add1_rgb, $suba1_al, $subb1_al, $mul1_al, $add1_al) });

    ($suba0_rgb:tt, $subb0_rgb:tt, $mul0_rgb:tt, $add0_rgb:tt, $suba0_al:tt, $subb0_al:tt, $mul0_al:tt, $add0_al:tt,
     $suba1_rgb:tt, $subb1_rgb:tt, $mul1_rgb:tt, $add1_rgb:tt, $suba1_al:tt, $subb1_al:tt, $mul1_al:tt, $add1_al:tt) => ({
        let combrgb0   = $crate::_comb_2cyca_rgb!($suba0_rgb, $subb0_rgb, $mul0_rgb, $add0_rgb);
        let combalpha0 = $crate::_comb_2cyca_alpha!($suba0_al, $subb0_al, $mul0_al, $add0_al);
        let combrgb1   = $crate::_comb_2cycb_rgb!($suba1_rgb, $subb1_rgb, $mul1_rgb, $add1_rgb);
        let combalpha1 = $crate::_comb_2cycb_alpha!($suba1_al, $subb1_al, $mul1_al, $add1_al);
        $crate::rdpq::Combiner::new_const(combrgb0 | combalpha0 | combrgb1 | combalpha1 | $crate::rdpq::consts::COMBINER_2PASS)
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! _blend {
    ($cyc:tt, $a1:tt, $b1:tt, $a2:tt, $b2:tt, $sa1:expr, $sb1:expr, $sa2:expr, $sb2:expr) => {
        paste! {
            (  ($crate::rdpq::consts::bl::[<_SOM_BLEND $cyc _A_  $a1>] << $sa1)
             | ($crate::rdpq::consts::bl::[<_SOM_BLEND $cyc _B1_ $b1>] << $sb1)
             | ($crate::rdpq::consts::bl::[<_SOM_BLEND $cyc _A_  $a2>] << $sa2)
             | ($crate::rdpq::consts::bl::[<_SOM_BLEND $cyc _B2_ $b2>] << $sb2)
             | ($crate::rdpq::consts::bl::[<_SOM_BLEND_EXTRA_A_  $a1>])
             | ($crate::rdpq::consts::bl::[<_SOM_BLEND_EXTRA_B1_ $b1>])
             | ($crate::rdpq::consts::bl::[<_SOM_BLEND_EXTRA_A_  $a2>])
             | ($crate::rdpq::consts::bl::[<_SOM_BLEND_EXTRA_B2_ $b2>])
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _blend_1cyc_0 {
    ($a1:tt, $b1:tt, $a2:tt, $b2:tt) => ({
        $crate::_blend!(1, $a1, $b1, $a2, $b2, 30, 26, 22, 18)
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! _blend_1cyc_1 {
    ($a1:tt, $b1:tt, $a2:tt, $b2:tt) => ({
        $crate::_blend!(1, $a1, $b1, $a2, $b2, 28, 24, 20, 16)
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! _blend_2cyc_0 {
    ($a1:tt, $b1:tt, $a2:tt, $b2:tt) => ({
        $crate::_blend!(2A, $a1, $b1, $a2, $b2, 30, 26, 22, 18)
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! _blend_2cyc_1 {
    ($a1:tt, $b1:tt, $a2:tt, $b2:tt) => ({
        $crate::_blend!(2B, $a1, $b1, $a2, $b2, 28, 24, 20, 16)
    });
}


/// Build a 1-pass blender formula
///
/// Rust: the Rust version has a slightly differently syntax than the C macro. Instead
/// of the double-parenthese and comma-list of arguments, define your blend modes using
/// the P*A+Q*B syntax:
///
/// ```rust
///     let mode = blender!(IN_RGB * IN_ALPHA + BLEND_RGB * INV_MUX_ALPHA);
/// ```
///
/// You may need to refer to the [`RDPQ_BLENDER`] macro in `rdpq_macros.h` to
/// understand how the arguments to this macro work and which are valid.
#[macro_export]
macro_rules! blender {
    ($a1:tt * $b1:tt + $a2:tt * $b2:tt) => ({ $crate::blender!($a1, $b1, $a2, $b2) });

    ($a1:tt, $b1:tt, $a2:tt, $b2:tt) => ({
        let onecycle0 = $crate::_blend_1cyc_0!($a1, $b1, $a2, $b2);
        let onecycle1 = $crate::_blend_1cyc_1!($a1, $b1, $a2, $b2);
        $crate::rdpq::Blender::new_const(onecycle0 | onecycle1)
    });
}

/// Build a 2-pass blender formula
///
/// Rust: this version uses the same syntax as above, but takes two formula:
///
/// ```rust
///    let mode = blender2!(IN_RGB     * IN_ALPHA + BLEND_RGB * INV_MUX_ALPHA,
///                         CYCLE1_RGB * IN_ALPHA + BLEND_RGB * INV_MUX_ALPHA);
/// ```
///
/// Not all arguments that are valid in 1-pass modes are valid in the second pass.
///
/// You may need to refer to the [`RDPQ_BLENDER2`] macro in `rdpq_macros.h` to
/// understand how the arguments to this macro work and which are valid.
#[macro_export]
macro_rules! blender2 {
    ($a1:tt * $b1:tt + $a2:tt * $b2:tt,
     $c1:tt * $d1:tt + $c2:tt * $d2:tt) => ({ blender2!($a1, $b1, $a2, $b2, $c1, $d1, $c2, $d2) });

    ($a1:tt, $b1:tt, $a2:tt, $b2:tt,
     $c1:tt, $d1:tt, $c2:tt, $d2:tt) => ({
        let twocycle0 = $crate::_blend_2cyc_0!($a1, $b1, $a2, $b2);
        let twocycle1 = $crate::_blend_2cyc_1!($c1, $d1, $c2, $d2);
        $crate::rdpq::Blender::new_const(twocycle0 | twocycle1 | ($crate::rdpq::consts::SOMX_BLEND_2PASS as u32))
    });
}

/// Wrapper around [`rdpq_combiner_t`](libdragon_sys::rdpq_combiner_t)
#[derive(Copy, Clone, PartialEq)]
pub struct Combiner(libdragon_sys::rdpq_combiner_t);

impl Combiner {
    pub const fn new_const(v: u64) -> Self {
        Self(v as libdragon_sys::rdpq_combiner_t)
    }
}

impl Into<u64> for Combiner {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl From<u64> for Combiner {
    fn from(v: u64) -> Self {
        Self(v as libdragon_sys::rdpq_combiner_t)
    }
}

/// Wrapper around [`rdpq_blender_t`](libdragon_sys::rdpq_blender_t)
#[derive(Copy, Clone, PartialEq)]
pub struct Blender(libdragon_sys::rdpq_blender_t);

impl Blender {
    pub const fn new_const(v: u32) -> Self {
        Self(v as libdragon_sys::rdpq_blender_t)
    }
}

impl Into<u32> for Blender {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

impl From<u32> for Blender {
    fn from(v: u32) -> Self {
        Self(v as libdragon_sys::rdpq_blender_t)
    }
}

// rdpq_mode.h

// Internal helper, not part of the public API
#[doc(hidden)]
#[inline]
fn __mode_change_som(mask: u64, val: u64) {
    if (mask >> 32) != 0 {
        unsafe {
            __rdpq_fixup_mode3(libdragon_sys::RDPQ_CMD_MODIFY_OTHER_MODES, 0 | (1 << 15), !((mask >> 32) as u32), (val >> 32) as u32);
        }
    }

    if (mask as u32) != 0 {
        unsafe {
            __rdpq_fixup_mode3(libdragon_sys::RDPQ_CMD_MODIFY_OTHER_MODES, 4 | (1 << 15), !(mask as u32), val as u32);
        }
    }
}

/// Push the current render mode into the stack
///
/// See [`rdpq_mode_push`](libdragon_sys::rdpq_mode_push) for details.
#[inline]
pub fn mode_push() { unsafe { libdragon_sys::rdpq_mode_push(); } }

/// Pop the current render mode from the stack
///
/// See [`rdpq_mode_pop`](libdragon_sys::rdpq_mode_pop) for details.
pub fn mode_pop() { unsafe { libdragon_sys::rdpq_mode_pop(); } }

/// Texture filtering types
///
/// See [`rdpq_filter_s`](libdragon_sys::rdpq_filter_s) for details
pub enum Filter {
    /// Point filtering (aka neartest)
    Point,
    /// Bilinear filtering
    Bilinear,
    /// Median filtering
    Median
}

impl Into<libdragon_sys::rdpq_filter_s> for Filter {
    fn into(self) -> libdragon_sys::rdpq_filter_s {
        match self {
            Filter::Point    => libdragon_sys::rdpq_filter_s_FILTER_POINT,
            Filter::Bilinear => libdragon_sys::rdpq_filter_s_FILTER_BILINEAR,
            Filter::Median   => libdragon_sys::rdpq_filter_s_FILTER_MEDIAN,
        }
    }
}

/// Diethering Configuration
///
/// See [`rdpq_dither_s`](libdragon_sys::rdpq_dither_s)
#[derive(Copy, Clone)]
pub enum Dither {
    /// Dithering: RGB=Square, Alpha=Square
    SquareSquare,       
    /// Dithering: RGB=Square, Alpha=InvSquare
    SquareInvSquare,    
    /// Dithering: RGB=Square, Alpha=Noise
    SquareNoise,        
    /// Dithering: RGB=Square, Alpha=None
    SquareNone,         
                                                                    
    /// Dithering: RGB=Bayer, Alpha=Bayer
    BayerBayer,         
    /// Dithering: RGB=Bayer, Alpha=InvBayer
    BayerInvBayer,      
    /// Dithering: RGB=Bayer, Alpha=Noise
    BayerNoise,         
    /// Dithering: RGB=Bayer, Alpha=None
    BayerNone,          
                                                                    
    /// Dithering: RGB=Noise, Alpha=Square
    NoiseSquare,        
    /// Dithering: RGB=Noise, Alpha=InvSquare
    NoiseInvSquare,     
    /// Dithering: RGB=Noise, Alpha=Noise
    NoiseNoise,         
    /// Dithering: RGB=Noise, Alpha=None
    NoiseNone,          
                                                                    
    /// Dithering: RGB=None, Alpha=Bayer
    NoneBayer,          
    /// Dithering: RGB=None, Alpha=InvBayer
    NoneInvBayer,       
    /// Dithering: RGB=None, Alpha=Noise
    NoneNoise,          
    /// Dithering: RGB=None, Alpha=None
    NoneNone,           
}

impl Into<libdragon_sys::rdpq_dither_s> for Dither {
    fn into(self) -> libdragon_sys::rdpq_dither_s {
        match self {
            Dither::SquareSquare => libdragon_sys::rdpq_dither_s_DITHER_SQUARE_SQUARE,
            Dither::SquareInvSquare => libdragon_sys::rdpq_dither_s_DITHER_SQUARE_INVSQUARE,
            Dither::SquareNoise => libdragon_sys::rdpq_dither_s_DITHER_SQUARE_NOISE,
            Dither::SquareNone => libdragon_sys::rdpq_dither_s_DITHER_SQUARE_NONE,
            Dither::BayerBayer => libdragon_sys::rdpq_dither_s_DITHER_BAYER_BAYER,
            Dither::BayerInvBayer => libdragon_sys::rdpq_dither_s_DITHER_BAYER_INVBAYER,
            Dither::BayerNoise => libdragon_sys::rdpq_dither_s_DITHER_BAYER_NOISE,
            Dither::BayerNone => libdragon_sys::rdpq_dither_s_DITHER_BAYER_NONE,
            Dither::NoiseSquare => libdragon_sys::rdpq_dither_s_DITHER_NOISE_SQUARE,
            Dither::NoiseInvSquare => libdragon_sys::rdpq_dither_s_DITHER_NOISE_INVSQUARE,
            Dither::NoiseNoise => libdragon_sys::rdpq_dither_s_DITHER_NOISE_NOISE,
            Dither::NoiseNone => libdragon_sys::rdpq_dither_s_DITHER_NOISE_NONE,
            Dither::NoneBayer => libdragon_sys::rdpq_dither_s_DITHER_NONE_BAYER,
            Dither::NoneInvBayer => libdragon_sys::rdpq_dither_s_DITHER_NONE_INVBAYER,
            Dither::NoneNoise => libdragon_sys::rdpq_dither_s_DITHER_NONE_NOISE,
            Dither::NoneNone => libdragon_sys::rdpq_dither_s_DITHER_NONE_NONE,
        }
    }
}

/// Types of palettes supported by RDP
///
/// See [`rdpq_tlut_s`](libdragon_sys::rdpq_tlut_s) for details.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tlut {
    /// No palette
    None,
    /// Palette made of [surface::TexFormat::Rgba16] colors
    Rgba16,
    /// Palette made of [surface::TexFormat::Ia16] colors
    Ia16,
}

impl Into<libdragon_sys::rdpq_tlut_s> for Tlut {
    fn into(self) -> libdragon_sys::rdpq_tlut_s {
        match self {
            Tlut::None   => libdragon_sys::rdpq_tlut_s_TLUT_NONE,
            Tlut::Rgba16 => libdragon_sys::rdpq_tlut_s_TLUT_RGBA16,
            Tlut::Ia16   => libdragon_sys::rdpq_tlut_s_TLUT_IA16,
        }
    }
}

impl From<surface::TexFormat> for Tlut {
    /// Converts the specified texture format to the [Tlut] mode that is needed to draw a texture
    /// of this format
    ///
    /// See [`rdpq_tlut_from_format`](libdragon_sys::rdpq_tlut_from_format) for details.
    fn from(format: surface::TexFormat) -> Self {
        match format {
            surface::TexFormat::Ci4 => Self::Rgba16,
            surface::TexFormat::Ci8 => Self::Rgba16,
            _ => Self::None,
        }
    }
}

/// Types of mipmap supported by RDP. Wrapper around `rdpq_mipmap_s`.
///
/// See [`rdpq_mipmap_s`](libdragon_sys::rdpq_mipmap_s) for details
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mipmap {
    /// Mipmap disabled
    None,
    /// Nearest mipmap level
    Nearest,
    /// Interpolate between the two nearest mipmap levels (also known as "trilinear")
    Interpolate,
    /// Interpolate between the two nearest mipmap levels (also known as "trilinear") with sharpening enabled
    InterpolateSharpen,
    /// Interpolate between the two nearest mipmap levels (also known as "trilinear") with detail texture enabled
    InterpolateDetail,
}

impl Into<libdragon_sys::rdpq_mipmap_s> for Mipmap {
    fn into(self) -> libdragon_sys::rdpq_mipmap_s {
        match self {
            Mipmap::None               => libdragon_sys::rdpq_mipmap_s_MIPMAP_NONE,
            Mipmap::Nearest            => libdragon_sys::rdpq_mipmap_s_MIPMAP_NEAREST,
            Mipmap::Interpolate        => libdragon_sys::rdpq_mipmap_s_MIPMAP_INTERPOLATE,
            Mipmap::InterpolateSharpen => libdragon_sys::rdpq_mipmap_s_MIPMAP_INTERPOLATE_SHARPEN,
            Mipmap::InterpolateDetail  => libdragon_sys::rdpq_mipmap_s_MIPMAP_INTERPOLATE_DETAIL,
        }
    }
}

/// Types of antialiasing supported by RDP. Wrapper around `rdpq_antialias_s`.
///
/// See [`rdpq_antialias_s`](libdragon_sys::rdpq_antialias_s) for details.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Antialias {
    /// No antialiasing
    None,
    /// Standard antialiasing
    Standard,
    /// Reduced antialiasing
    Reduced,
}

impl Into<libdragon_sys::rdpq_antialias_s> for Antialias {
    fn into(self) -> libdragon_sys::rdpq_antialias_s {
        match self {
            Antialias::None     => libdragon_sys::rdpq_antialias_s_AA_NONE,
            Antialias::Standard => libdragon_sys::rdpq_antialias_s_AA_STANDARD,
            Antialias::Reduced  => libdragon_sys::rdpq_antialias_s_AA_REDUCED,
        }
    }
}


/// Reset render mode to standard.
///
/// See [`rdpq_set_mode_standard`](libdragon_sys::rdpq_set_mode_standard) for details.
#[inline] pub fn set_mode_standard() { unsafe { libdragon_sys::rdpq_set_mode_standard(); } }

/// Reset render mode to FILL type
///
/// See [`rdpq_set_mode_fill`](libdragon_sys::rdpq_set_mode_fill) for details.
#[inline]
pub fn set_mode_fill(color: graphics::Color) {
    extern "C" { fn __rdpq_set_mode_fill(); }
    unsafe { __rdpq_set_mode_fill(); }
    set_fill_color(color);
}

/// Reset render mode to COPY type.
///
/// See [`rdpq_set_mode_copy`](libdragon_sys::rdpq_set_mode_copy) for details.
#[inline] pub fn set_mode_copy(transparency: bool) { unsafe { libdragon_sys::rdpq_set_mode_copy(transparency); } }

/// Reset render mode to YUV mode.
///
/// See [`rdpq_set_mode_yuv`](libdragon_sys::rdpq_set_mode_yuv) for details.
#[inline] pub fn set_mode_yuv(bilinear: bool) { unsafe { libdragon_sys::rdpq_set_mode_yuv(bilinear); } }

/// Activate antialiasing
///
/// See [`rdpq_mode_antialias`](libdragon_sys::rdpq_mode_antialias) for details.
#[inline] pub fn mode_antialias(mode: Antialias) {
    __mode_change_som(consts::SOM_AA_ENABLE | consts::SOMX_AA_REDUCED,
        (if mode != Antialias::None { consts::SOM_AA_ENABLE } else { 0 })
        | (if mode == Antialias::Reduced { consts::SOMX_AA_REDUCED } else { 0 }));
}

/// Configure the color combiner
///
/// See [`rdpq_mode_combiner`](libdragon_sys::rdpq_mode_combiner) for details.
#[inline] 
pub fn mode_combiner(comb: Combiner) {
    let comb: u64 = comb.into();

    if (comb & consts::COMBINER_2PASS) != 0 {
        unsafe {
            __rdpq_fixup_mode(CMD_SET_COMBINE_MODE_2PASS, ((comb >> 32) & 0x00FFFFFF) as u32, comb as u32);
        }
    } else {
        let mut comb1_mask = consts::COMB1_MASK;
        if ((comb >> 0 ) &  7) == 1 { comb1_mask ^= 1u64 << 0;  }
        if ((comb >> 3 ) &  7) == 1 { comb1_mask ^= 1u64 << 3;  }
        if ((comb >> 6 ) &  7) == 1 { comb1_mask ^= 1u64 << 6;  }
        if ((comb >> 18) &  7) == 1 { comb1_mask ^= 1u64 << 18; }
        if ((comb >> 21) &  7) == 1 { comb1_mask ^= 1u64 << 21; }
        if ((comb >> 24) &  7) == 1 { comb1_mask ^= 1u64 << 24; }
        if ((comb >> 32) & 31) == 1 { comb1_mask ^= 1u64 << 32; }
        if ((comb >> 37) & 15) == 1 { comb1_mask ^= 1u64 << 37; }

        unsafe {
            __rdpq_fixup_mode4(CMD_SET_COMBINE_MODE_1PASS, ((comb >> 32) & 0x00FFFFFF) as u32, comb as u32,
                                                           ((comb1_mask >> 32) & 0x00FFFFFF) as u32, comb1_mask as u32);
        }
    }
}

/// Configure the formula to use for blending.
///
/// See [`rdpq_mode_blender`](libdragon_sys::rdpq_mode_blender) for details.
#[inline]
pub fn mode_blender(blend: Blender) {
    let mut blend: u32 = blend.into();
    if blend != 0 { blend |= consts::SOM_BLENDING as u32; }
    unsafe {
        __rdpq_fixup_mode(CMD_SET_BLENDING_MODE, 0, blend);
    }
}

/// Enable or disable fog
///
/// See [`rdpq_mode_fog`](libdragon_sys::rdpq_mode_fog) for details.
#[inline]
pub fn mode_fog(fog: Blender) {
    let mut fog: u32 = fog.into();
    if fog != 0 { fog |= consts::SOM_BLENDING as u32; }
    if fog != 0 { assert!((fog & (consts::SOMX_BLEND_2PASS as u32)) == 0, "Fogging cannot be used with two-pass blending formulas"); }
    __mode_change_som(consts::SOMX_FOG, if fog != 0 { consts::SOMX_FOG } else { 0 });
    unsafe {
        __rdpq_fixup_mode(CMD_SET_FOG_MODE, 0, fog);
    }
}

/// Change dithering mode
///
/// See [`rdpq_mode_filter`](libdragon_sys::rdpq_mode_filter) for details.
#[inline]
pub fn mode_dithering(dither: Dither) {
    let dither: libdragon_sys::rdpq_dither_s = dither.into();
    __mode_change_som(consts::SOM_RGBDITHER_MASK | consts::SOM_ALPHADITHER_MASK,
                      (dither as u64) << consts::SOM_ALPHADITHER_SHIFT);
}

/// Activate alpha compare feature
///
/// See [`rdpq_mode_alphacompare`](libdragon_sys::rdpq_mode_alphacompare) for details.
#[inline]
pub fn mode_alphacompare(threshold: i32) {
    if threshold == 0 {
        __mode_change_som(consts::SOM_ALPHACOMPARE_MASK, 0);
    } else if threshold > 0 {
        __mode_change_som(consts::SOM_ALPHACOMPARE_MASK, consts::SOM_ALPHACOMPARE_THRESHOLD);
        set_blend_color(graphics::rgba32(0,0,0,threshold as u8));
    } else {
        __mode_change_som(consts::SOM_ALPHACOMPARE_MASK, consts::SOM_ALPHACOMPARE_NOISE);
    }
}

/// Activate z-buffer usage
///
/// See [`rdpq_mode_zbuf`](libdragon_sys::rdpq_mode_zbuf) for details.
#[inline]
pub fn mode_zbuf(compare: bool, update: bool) {
    __mode_change_som(consts::SOM_Z_COMPARE | consts::SOM_Z_WRITE,
        if compare { consts::SOM_Z_COMPARE } else { 0 } |
        if update  { consts::SOM_Z_WRITE   } else { 0 });
}

/// Set a fixed override of Z value
///
/// See [`rdpq_mode_zoverride`](libdragon_sys::rdpq_mode_zoverride) for details.
#[inline]
pub fn mode_zoverride(enable: bool, z: f32, deltaz: i16) {
    if enable { set_prim_depth_raw((z * (0x7FFF as f32)) as u16, deltaz); }
    __mode_change_som(consts::SOM_ZSOURCE_PRIM, if enable { consts::SOM_ZSOURCE_PRIM } else { 0 });
}

/// Activate palette lookup during drawing
///
/// See [`rdpq_mode_tlut`](libdragon_sys::rdpq_mode_tlut) for details.
#[inline]
pub fn mode_tlut(tlut: Tlut) {
    let tlut: libdragon_sys::rdpq_tlut_s = tlut.into();
    __mode_change_som(consts::SOM_TLUT_MASK, (tlut as u64) << consts::SOM_TLUT_SHIFT);
}

/// Activate texture filtering
///
/// See [`rdpq_mode_filter`](libdragon_sys::rdpq_mode_filter) for details.
#[inline]
pub fn mode_filter(filt: Filter) {
    let filt: libdragon_sys::rdpq_filter_s = filt.into();
    __mode_change_som(consts::SOM_SAMPLE_MASK, (filt as u64) << consts::SOM_SAMPLE_SHIFT);
}

/// Activate mip-mapping.
///
/// See [`rdpq_mode_mipmap`](libdragon_sys::rdpq_mode_mipmap) for details.
#[inline]
pub fn mode_mipmap(mode: Mipmap, mut num_levels: usize) {
    if mode == Mipmap::None { num_levels = 0; }
    if num_levels != 0 { num_levels -= 1; }
    let mode: libdragon_sys::rdpq_mipmap_s = mode.into();
    __mode_change_som(consts::SOM_TEXTURE_LOD | consts::SOMX_LOD_INTERPOLATE |
                      consts::SOMX_NUMLODS_MASK | consts::SOM_TEXTURE_SHARPEN | 
                      consts::SOM_TEXTURE_DETAIL, ((mode as u64) << 32) | ((num_levels as u64) << consts::SOMX_NUMLODS_SHIFT));
}

/// Activate perspective correction for textures
///
/// See [`rdpq_mode_persp`](libdragon_sys::rdpq_mode_persp) for details.
#[inline]
pub fn mode_persp(perspective: bool) {
    __mode_change_som(consts::SOM_TEXTURE_PERSP, if perspective { consts::SOM_TEXTURE_PERSP } else { 0 });
}

/// Start a batch of RDP mode changes
///
/// See [`rdpq_mode_begin`](libdragon_sys::rdpq_mode_begin) for details;
#[inline] pub fn mode_begin() { unsafe { libdragon_sys::rdpq_mode_begin(); } }

/// Finish a batch of RDP mode changes
///
/// See [`rdpq_mode_end`](libdragon_sys::rdpq_mode_end) for details;
#[inline] pub fn mode_end() { unsafe { libdragon_sys::rdpq_mode_end(); } }

// rdpq_paragraph.h
#[repr(C, packed)]
pub struct ParagraphChar {
    pub(crate) c: libdragon_sys::rdpq_paragraph_char_s,
}

/// Wrapper around `rdpq_paragraph_t`. 
///
/// See [`rdpq_paragraph_t`](libdragon_sys::rdpq_paragraph_t) for details.
#[repr(C, packed)]
pub struct Paragraph {
    pub(crate) c: *mut libdragon_sys::rdpq_paragraph_t,
}

impl Paragraph {
    /// Calculate the layout of a text using the specified parameters.
    ///
    /// Rust: returns an instance of Paragraph plus the number of bytes (not characters!) read
    /// from `text`. 
    ///
    /// See [`rdpq_paragraph_build`](libdragon_sys::rdpq_paragraph_build) for details.
    pub fn build(parms: TextParms, initial_font_id: u8, text: &str) -> (Paragraph, usize) {
        let parms: libdragon_sys::rdpq_textparms_t = parms.into();
        let ctext = CString::new(text).unwrap();
        let mut len = ctext.to_bytes().len(); // does not contain any trailing \0
        let ptr = unsafe {
            let mut s = len as i32;
            let ptr = libdragon_sys::rdpq_paragraph_build(&parms as *const _, initial_font_id, ctext.as_ptr(), &mut s as *mut _);
            len = s as usize;
            ptr
        };
        (Paragraph { c: ptr, }, len)
    }

    /// Render a text that was laid out by [`rdpq_paragraph_build`]
    ///
    /// See [`rdpq_paragraph_render`](libdragon_sys::rdpq_paragraph_render) for details.
    pub fn render(&self, x0: f32, y0: f32) {
        unsafe {
            libdragon_sys::rdpq_paragraph_render(self.c as *const _, x0, y0);
        }
    }

    /// Bounding box: (top-left x, top-left y, bottom-right x, bottom-right y)
    pub fn bbox(&self) -> (f32, f32, f32, f32) {
        unsafe { 
            let r: &libdragon_sys::rdpq_paragraph_t = &*self.c;
            (r.bbox.x0, r.bbox.y0, r.bbox.x0, r.bbox.y1)
        }
    }

    /// Number of lines of the text
    pub fn nlines(&self) -> usize { unsafe { (*self.c).nlines as usize } }
    /// Total number of chars in this layout
    pub fn nchars(&self) -> usize { unsafe { (*self.c).nchars as usize } }
    /// Capacity of the chars array
    pub fn capacity(&self) -> usize { unsafe { (*self.c).capacity as usize } }
    /// Alignment offset of the text (X coord)
    pub fn x0(&self) -> f32 { unsafe { (*self.c).x0 } }
    /// Alignment offset of the text (Y coord)
    pub fn y0(&self) -> f32 { unsafe { (*self.c).y0 } }
    /// Array of chars
    pub fn chars<'a>(&'a self) -> &'a [ParagraphChar] { 
        // As long as ParagraphChar wraps only rdpq_paragraph_char_t, this cast should be safe
        let ptr: *const ParagraphChar = unsafe { (*self.c).chars.as_ptr() as *const _ };
        unsafe {
            ::core::slice::from_raw_parts(ptr, self.nchars())
        }
    }
}

impl Drop for Paragraph {
    /// Free the memory allocated by [`rdpq_paragraph_build`] or [`rdpq_paragraph_builder_end`].
    ///
    /// See [`rdpq_paragraph_free`](libdragon_sys::rdpq_paragraph_free) for details.
    fn drop(&mut self) { unsafe { libdragon_sys::rdpq_paragraph_free(self.c); } }
}

/// Utility class to make paragraph building more Rust-like Example:
///
/// ```rust
///
/// let p: Paragraph = ParagraphBuilder::begin(parms, 1, None)
///                                        .span("Hello, ")
///                                        .font(2)
///                                        .newline()
///                                        .span("world!")
///                                        .end();
/// ```
///
/// See `rdpq_paragraph.h` in LibDragon for details.
pub struct ParagraphBuilder<'a> {
    pub(crate) _parms: core::pin::Pin<Box<libdragon_sys::rdpq_textparms_t>>,
    pub(crate) phantom: core::marker::PhantomData<&'a u8>,
}

impl<'a> ParagraphBuilder<'a> {
    /// Start a paragraph builder.
    ///
    /// See [`rdpq_paragraph_builder_begin`](libdragon_sys::rdpq_paragraph_builder_begin) for details.
    pub fn begin<'b>(parms: TextParms, initial_font_id: u8, layout: Option<&'b Paragraph>) -> ParagraphBuilder<'b> {
        // text parms have to persist throughout the entire builder
        let parms: libdragon_sys::rdpq_textparms_t = parms.into();
        let pinned = Box::pin(parms);
        let layout_ptr: *mut libdragon_sys::rdpq_paragraph_t = layout.map_or_else(|| ::core::ptr::null_mut(), |p| p.c);
        unsafe {
            libdragon_sys::rdpq_paragraph_builder_begin(pinned.as_ref().get_ref() as *const _, initial_font_id, layout_ptr);
        }
        ParagraphBuilder {
            _parms: pinned,
            phantom: core::marker::PhantomData,
        }
    }

    /// Change the current font
    ///
    /// See [`rdpq_paragraph_builder_font`](libdragon_sys::rdpq_paragraph_builder_font) for details.
    pub fn font(&mut self, font_id: u8) -> &mut Self {
        unsafe { libdragon_sys::rdpq_paragraph_builder_font(font_id); }
        self
    }

    /// Change the current style
    ///
    /// See [`rdpq_paragraph_builder_style`](libdragon_sys::rdpq_paragraph_builder_style) for details.
    pub fn style(&mut self, style_id: u8) -> &mut Self {
        unsafe { libdragon_sys::rdpq_paragraph_builder_style(style_id); }
        self
    }

    /// Add a span of text
    ///
    /// See [`rdpq_paragraph_builder_span`](libdragon_sys::rdpq_paragraph_builder_span) for details.
    pub fn span(&mut self, text: &str) -> &mut Self {
        let ctext = CString::new(text).unwrap();
        let len = ctext.to_bytes().len(); // does not contain any trailing \0
        unsafe { libdragon_sys::rdpq_paragraph_builder_span(ctext.as_ptr(), len as i32); }
        self
    }

    /// Start a new line
    ///
    /// See [`rdpq_paragraph_builder_newline`](libdragon_sys::rdpq_paragraph_builder_newline) for details.
    pub fn newline(&mut self) -> &mut Self {
        unsafe { libdragon_sys::rdpq_paragraph_builder_newline(); }
        self
    }

    /// Finalize the paragraph builder and return the [Paragraph]
    ///
    /// See [`rdpq_paragraph_builder_end`](libdragon_sys::rdpq_paragraph_builder_end) for details.
    pub fn end(&self) -> Paragraph {
        let ptr = unsafe { libdragon_sys::rdpq_paragraph_builder_end() };
        Paragraph { c: ptr, }
    }
}

// rdpq_rect.h

extern "C" {
    fn __rdpq_texture_rectangle_inline(tile: libdragon_sys::rdpq_tile_t,
                                       x0: ::core::ffi::c_int, y0: ::core::ffi::c_int,
                                       x1: ::core::ffi::c_int, y1: ::core::ffi::c_int,
                                       s0: ::core::ffi::c_int, t0: ::core::ffi::c_int);
    fn __rdpq_texture_rectangle_scaled_inline(tile: libdragon_sys::rdpq_tile_t,
                                       x0: ::core::ffi::c_int, y0: ::core::ffi::c_int,
                                       x1: ::core::ffi::c_int, y1: ::core::ffi::c_int,
                                       s0: ::core::ffi::c_int, t0: ::core::ffi::c_int,
                                       s1: ::core::ffi::c_int, t1: ::core::ffi::c_int);
}

// These functions are not part of the public API
#[inline(always)]
fn __rdpq_fill_rectangle_inline(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32) {
    use core::cmp::{min, max};
    x0 = max(x0, 0);
    y0 = max(y0, 0);
    x1 = min(x1, 0xFFF);
    y1 = min(y1, 0xFFF);
    if x0 >= x1 || y0 >= y1 { return; }

    extern "C" { fn __rdpq_fill_rectangle(w0: u32, w1: u32); }
    unsafe {
        __rdpq_fill_rectangle(
            _carg(x1 as u32, 0xFFF, 12) | _carg(y1 as u32, 0xFFF, 0),
            _carg(x0 as u32, 0xFFF, 12) | _carg(y0 as u32, 0xFFF, 0));
    }
}


#[inline(always)]
fn __rdpq_fill_rectangle_fx(x0: i32, y0: i32, x1: i32, y1: i32) { __rdpq_fill_rectangle_inline(x0, y0, x1, y1); }

#[inline(always)]
fn __rdpq_texture_rectangle_fx(tile: Tile, x0: i32, y0: i32, x1: i32, y1: i32, s: i32, t: i32) {
    // Rust TODO: future optimization - rewrite __rdpq_texture_rectangle_inline in Rust, rather than
    // calling the C version.
    unsafe { __rdpq_texture_rectangle_inline(tile.0 as libdragon_sys::rdpq_tile_t, x0, y0, x1, y1, s, t); }
}

#[inline(always)]
fn __rdpq_texture_rectangle_scaled_fx(tile: Tile, x0: i32, y0: i32, x1: i32, y1: i32, s0: i32, t0: i32, s1: i32, t1: i32) {
    // Rust TODO: future optimization - rewrite __rdpq_texture_rectangle_scaled_inline in Rust, rather than
    // calling the C version.
    unsafe { __rdpq_texture_rectangle_scaled_inline(tile.0 as libdragon_sys::rdpq_tile_t, x0, y0, x1, y1, s0, t0, s1, t1); }
}

#[inline(always)]
fn __rdpq_texture_rectangle_raw_fx(tile: Tile, x0: i32, y0: i32, x1: i32, y1: i32, s0: i32, t0: i32, dsdx: i32, dtdy: i32) {
    extern "C" { fn __rdpq_texture_rectangle(w0: u32, w1: u32, w2: u32, w3: u32); }
    unsafe {
        __rdpq_texture_rectangle(
            _carg(x1 as u32, 0xFFF, 12) | _carg(y1 as u32, 0xFFF, 0),
            _carg(tile.0 as u32, 0x7, 24) | _carg(x0 as u32, 0xFFF, 12) | _carg(y0 as u32, 0xFFF, 0),
            _carg(s0 as u32, 0xFFFF, 16) | _carg(t0 as u32, 0xFFFF, 0),
            _carg(dsdx as u32, 0xFFFF, 16) | _carg(dtdy as u32, 0xFFFF, 0));   
    }
}

#[inline(always)]
fn __rdpq_texture_rectangle_flip_raw_fx(tile: Tile, x0: i32, y0: i32, x1: i32, y1: i32, s: i32, t: i32, dsdy: i32, dtdx: i32) {
    extern "C" { fn __rdpq_write16_syncuse(_0: u32, _1: u32, _2: u32, _3: u32, _4: u32, _5: u32); }
    unsafe {
        __rdpq_write16_syncuse(CMD_TEXTURE_RECTANGLE_FLIP,
            _carg(x1 as u32, 0xFFF, 12) | _carg(y1 as u32, 0xFFF, 0),
            _carg(tile.0 as u32, 0x7, 24) | _carg(x0 as u32, 0xFFF, 12) | _carg(y0 as u32, 0xFFF, 0),
            _carg(s as u32, 0xFFFF, 16) | _carg(t as u32, 0xFFFF, 0),
            _carg(dsdy as u32, 0xFFFF, 16) | _carg(dtdx as u32, 0xFFFF, 0),
            libdragon_sys::AUTOSYNC_PIPE | _autosync_tile(tile.0 as u32) | _autosync_tmem(0));
    }
}

/// Draw a filled rectangle (RDP command: FILL_RECTANGLE)
///
/// Takes any type that can be multiplied by i32 and converted to i32.
///
/// See [`rdpq_fill_rectangle`](libdragon_sys::rdpq_fill_rectangle) and `rdpq_rect.h` for details.
#[inline]
pub fn fill_rectangle<T>(x0: T, y0: T, x1: T, y1: T)
    where 
        T: Into<i32> + From<i32> + core::ops::Mul<Output=T> + Copy {
    let x0 = Into::<i32>::into(x0 * From::<i32>::from(4));
    let y0 = Into::<i32>::into(y0 * From::<i32>::from(4));
    let x1 = Into::<i32>::into(x1 * From::<i32>::from(4));
    let y1 = Into::<i32>::into(y1 * From::<i32>::from(4));
    __rdpq_fill_rectangle_fx(x0, y0, x1, y1);
}

/// Draw a textured rectangle (RDP command: TEXTURE_RECTANGLE)
///
/// See [`rdpq_texture_rectangle`](libdragon_sys::rdpq_texture_rectangle) and `rdpq_rect.h` for details.
#[inline]
pub fn texture_rectangle<T>(tile: Tile, x0: T, y0: T, x1: T, y1: T, s: i32, t: i32)
    where 
        T: Into<i32> + From<i32> + core::ops::Mul<Output=T> + Copy {
    let x0 = Into::<i32>::into(x0 * From::<i32>::from(4));
    let y0 = Into::<i32>::into(y0 * From::<i32>::from(4));
    let x1 = Into::<i32>::into(x1 * From::<i32>::from(4));
    let y1 = Into::<i32>::into(y1 * From::<i32>::from(4));
    __rdpq_texture_rectangle_fx(tile, x0, y0, x1, y1, s * 32, t * 32);
}

/// Draw a textured rectangle with scaling (RDP command: TEXTURE_RECTANGLE)
///
/// See [`rdpq_texture_rectangle_scaled`](libdragon_sys::rdpq_texture_rectangle_scaled) and `rdpq_rect.h` for details.
#[inline]
pub fn texture_rectangle_scaled<T>(tile: Tile, x0: T, y0: T, x1: T, y1: T, s0: i32, t0: i32, s1: i32, t1: i32)
    where 
        T: Into<i32> + From<i32> + core::ops::Mul<Output=T> + Copy {
    let x0 = Into::<i32>::into(x0 * From::<i32>::from(4));
    let y0 = Into::<i32>::into(y0 * From::<i32>::from(4));
    let x1 = Into::<i32>::into(x1 * From::<i32>::from(4));
    let y1 = Into::<i32>::into(y1 * From::<i32>::from(4));
    __rdpq_texture_rectangle_scaled_fx(tile, x0, y0, x1, y1, s0 * 32, t0 * 32, s1 * 32, t1 * 32);
}

/// Draw a textured rectangle with scaling -- raw version (RDP command: TEXTURE_RECTANGLE)
///
/// See [`rdpq_texture_rectangle_raw`](libdragon_sys::rdpq_texture_rectangle_raw) and `rdpq_rect.h` for details.
#[inline]
pub fn texture_rectangle_raw<T>(tile: Tile, x0: T, y0: T, x1: T, y1: T, s0: i32, t0: i32, dsdx: T, dtdy: T)
    where 
        T: Into<i32> + From<i32> + core::ops::Mul<Output=T> + Copy {
    let x0 = Into::<i32>::into(x0 * From::<i32>::from(4));
    let y0 = Into::<i32>::into(y0 * From::<i32>::from(4));
    let x1 = Into::<i32>::into(x1 * From::<i32>::from(4));
    let y1 = Into::<i32>::into(y1 * From::<i32>::from(4));
    let dsdx = Into::<i32>::into(dsdx * From::<i32>::from(1024));
    let dtdy = Into::<i32>::into(dtdy * From::<i32>::from(1024));
    __rdpq_texture_rectangle_raw_fx(tile, x0, y0, x1, y1, s0 * 32, t0 * 32, dsdx, dtdy);
}

/// Draw a textured flipped rectangle (RDP command: TEXTURE_RECTANGLE_FLIP)
///
/// See [`rdpq_texture_rectangle_flip_raw`](libdragon_sys::rdpq_texture_rectangle_flip_raw) and `rdpq_rect.h` for details.
#[inline]
pub fn texture_rectangle_flip_raw<T>(tile: Tile, x0: T, y0: T, x1: T, y1: T, s: i32, t: i32, dsdx: T, dtdy: T)
    where 
        T: Into<i32> + From<i32> + core::ops::Mul<Output=T> + Copy {
    let x0 = Into::<i32>::into(x0 * From::<i32>::from(4));
    let y0 = Into::<i32>::into(y0 * From::<i32>::from(4));
    let x1 = Into::<i32>::into(x1 * From::<i32>::from(4));
    let y1 = Into::<i32>::into(y1 * From::<i32>::from(4));
    let dsdx = Into::<i32>::into(dsdx * From::<i32>::from(1024));
    let dtdy = Into::<i32>::into(dtdy * From::<i32>::from(1024));
    __rdpq_texture_rectangle_flip_raw_fx(tile, x0, y0, x1, y1, s * 32, t * 32, dsdx, dtdy);
}

// rdpq_sprite.h

/// Upload a sprite to TMEM, making it ready for drawing
///
/// See [`rdpq_sprite_upload`](libdragon_sys::rdpq_sprite_upload) for details
#[inline]
pub fn sprite_upload(tile: Tile, sprite: &Sprite, parms: TexParms) {
    let parms: libdragon_sys::rdpq_texparms_t = parms.into();
    unsafe {
        libdragon_sys::rdpq_sprite_upload(tile.0 as libdragon_sys::rdpq_tile_t, sprite.as_const_sprite_s() as *mut _, &parms);
    }
}

/// Blit a sprite to the active framebuffer
///
/// See [`rdpq_sprite_blit`](libdragon_sys::rdpq_sprite_blit) for details.
#[inline]
pub fn sprite_blit(sprite: &Sprite, x0: f32, y0: f32, parms: BlitParms) {
    let parms: libdragon_sys::rdpq_blitparms_t = parms.into();
    unsafe {
        libdragon_sys::rdpq_sprite_blit(sprite.as_const_sprite_s() as *mut _, x0, y0, &parms);
    }
}

// rdpq_tex.h
pub const REPEAT_INFINITE: f32 = 2048.0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlitParms {
    pub tile     : Tile,
    pub s0       : i32,
    pub t0       : i32,
    pub width    : i32,
    pub height   : i32,
    pub flip_x   : bool,
    pub flip_y   : bool,
    pub cx       : i32,
    pub cy       : i32,
    pub scale_x  : f32,
    pub scale_y  : f32,
    pub theta    : f32,
    pub filtering: bool,
    pub nx       : i32,
    pub ny       : i32,
}

impl Default for BlitParms {
    fn default() -> Self {
        BlitParms {
            tile     : Tile(0),
            s0       : 0,
            t0       : 0,
            width    : 0,
            height   : 0,
            flip_x   : false,
            flip_y   : false,
            cx       : 0,
            cy       : 0,
            scale_x  : 1.0,
            scale_y  : 1.0,
            theta    : 0.0,
            filtering: false,
            nx       : 0,
            ny       : 0,
        }
    }
}

impl Into<libdragon_sys::rdpq_blitparms_s> for BlitParms {
    fn into(self) -> libdragon_sys::rdpq_blitparms_s {
        assert!(::core::mem::size_of::<Self>() == ::core::mem::size_of::<libdragon_sys::rdpq_blitparms_s>());
        unsafe {
            *::core::mem::transmute::<&Self, *const libdragon_sys::rdpq_blitparms_s>(&self)
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TexParmsST {
    pub translate: f32,
    pub scale_log: i32,
    pub repeats  : f32,
    pub mirror   : bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TexParms {
    pub tmem_addr: i32,
    pub palette  : i32,
    pub s        : TexParmsST,
    pub t        : TexParmsST,
}

impl Into<libdragon_sys::rdpq_texparms_t> for TexParms {
    fn into(self) -> libdragon_sys::rdpq_texparms_t {
        assert!(::core::mem::size_of::<Self>() == ::core::mem::size_of::<libdragon_sys::rdpq_texparms_t>());
        unsafe {
            *::core::mem::transmute::<&Self, *const libdragon_sys::rdpq_texparms_t>(&self)
        }
    }
}


pub type TlutPalette = *mut u16;
pub fn tex_upload_tlut(tlut: TlutPalette, color_idx: i32, num_colors: i32) {
    unsafe {
        libdragon_sys::rdpq_tex_upload_tlut(tlut as *mut u16, color_idx, num_colors);
    }
}

pub fn tex_upload_sub(tile: Tile, tex: &Surface, parms: Option<TexParms>, s0: i32, t0: i32, s1: i32, t1: i32) -> i32 {
    unsafe {
        libdragon_sys::rdpq_tex_upload_sub(tile.0 as libdragon_sys::rdpq_tile_t, tex.ptr,
                                           parms.map_or(::core::ptr::null(), |p| &Into::<libdragon_sys::rdpq_texparms_t>::into(p)) 
                                                    as *const libdragon_sys::rdpq_texparms_t,
                                           s0, t0, s1, t1) as i32
    }
}

// rdpq_text.h
pub fn text_register_font(id: u8, font: &Font) {
    unsafe {
        libdragon_sys::rdpq_text_register_font(id, font.ptr);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub enum Align {
    #[default]
    Left   = 0,
    Center = 1,
    Right  = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub enum VAlign {
    #[default]
    Top    = 0,
    Center = 1,
    Bottom = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub enum TextWrap {
    #[default]
    None = 0,
    Ellipses = 1,
    Char = 2,
    Word = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TextParms {
    pub width       : i16,
    pub height      : i16,
    pub align       : Align,
    pub valign      : VAlign,
    pub indent      : i16,
    pub char_spacing: i16,
    pub line_spacing: i16,
    pub wrap        : TextWrap,
}

impl Into<libdragon_sys::rdpq_textparms_t> for TextParms {
    fn into(self) -> libdragon_sys::rdpq_textparms_t {
        unsafe {
            *core::mem::transmute::<&Self, *const libdragon_sys::rdpq_textparms_t>(&self)
        }
    }
}

pub fn text_print(parms: TextParms, font_id: u8, x0: f32, y0: f32, text: &str) -> i32 {
    let ctext = CString::new(text).unwrap();
    let len = ctext.to_bytes().len(); // does not contain any trailing \0
    unsafe {
        libdragon_sys::rdpq_text_printn(
            &mut Into::<libdragon_sys::rdpq_textparms_t>::into(parms) as *mut libdragon_sys::rdpq_textparms_t,
            font_id, x0, y0, ctext.as_ptr(), len as i32)
    }
}

// rdpq_tri.h

/// Format descriptor of a triangle.
///
/// See [`rdpq_trifmt_t`](libdragon_sys::rdpq_trifmt_t) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TriFmt {
    /// Index of the position component within the vertex array.
    /// See [`rdpq_trifmt_t.pos_offset`](libdragon_sys::rdpq_trifmt_t::pos_offset)
    pub pos_offset: i32,
    /// Index of the shade component witin the vertex arrays.
    /// See [`rdpq_trifmt_t.shade_offset`](libdragon_sys::rdpq_trifmt_t::shade_offset)
    pub shade_offset: i32,
    /// If true, draw the rectangle with flat shading (instead of gouraud shading).
    /// See [`rdpq_trifmt_t.shade_flat`](libdragon_sys::rdpq_trifmt_t::shade_flat)
    pub shade_flat: bool,
    /// Index of the texture component within the vertex arrays.
    /// See [`rdpq_trifmt_t.tex_offset`](libdragon_sys::rdpq_trifmt_t::tex_offset)
    pub tex_offset: i32,
    /// RDP tile descriptor that describes the texture (0-7, see [Tile]).
    /// See [`rdpq_trifmt_t.tex_tile`](libdragon_sys::rdpq_trifmt_t::tex_tile)
    pub tex_tile: Tile,
    /// Number of mipmaps to use for the texture.
    /// See [`rdpq_trifmt_t.tex_mipmaps`](libdragon_sys::rdpq_trifmt_t::tex_mipmaps)
    pub tex_mipmaps: i32,
    /// Index of the depth component within the vertex array.
    /// See [`rdpq_trifmt_t.z_offset`](libdragon_sys::rdpq_trifmt_t::z_offset)
    pub z_offset: i32,
}

/// Format descriptor for a solid-filled triangle.
///
/// See [`TRIFMT_FILL`](libdragon_sys::TRIFMT_FILL) for details.
pub static TRIFMT_FILL: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_FILL) 
};

/// Format descriptor for a shaded triangle.
///
/// See [`TRIFMT_SHADE`](libdragon_sys::TRIFMT_SHADE) for details.
pub static TRIFMT_SHADE: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_SHADE) 
};

/// Format descriptor for a textured triangle.
///
/// See [`TRIFMT_TEX`](libdragon_sys::TRIFMT_TEX) for details.
pub static TRIFMT_TEX: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_TEX) 
};

/// Format descriptor for a shaded, textured triangle.
///
/// See [`TRIFMT_SHADE_TEX`](libdragon_sys::TRIFMT_SHADE_TEX) for details.
pub static TRIFMT_SHADE_TEX: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_SHADE_TEX) 
};

/// Format descriptor for a solid-filled, z-buffered triangle.
///
/// See [`TRIFMT_ZBUF`](libdragon_sys::TRIFMT_ZBUF) for details.
pub static TRIFMT_ZBUF: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_ZBUF) 
};

/// Format descriptor for a z-buffered, shaded triangle.
///
/// See [`TRIFMT_ZBUF_SHADE`](libdragon_sys::TRIFMT_ZBUF_SHADE) for details.
pub static TRIFMT_ZBUF_SHADE: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_ZBUF_SHADE) 
};

/// Format descriptor for a z-buffered, textured triangle.
///
/// See [`TRIFMT_ZBUF_TEX`](libdragon_sys::TRIFMT_ZBUF_TEX) for details.
pub static TRIFMT_ZBUF_TEX: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_ZBUF_TEX) 
};

/// Format descriptor for a z-buffered, shaded, textured triangle.
///
/// See [`TRIFMT_ZBUF_SHADE_TEX`](libdragon_sys::TRIFMT_ZBUF_SHADE_TEX) for details.
pub static TRIFMT_ZBUF_SHADE_TEX: &TriFmt = unsafe { 
    &*core::mem::transmute::<*const libdragon_sys::rdpq_trifmt_t, *const TriFmt>(&libdragon_sys::TRIFMT_ZBUF_SHADE_TEX) 
};

/// Draw a triangle (RDP command: TRI_*)
///
/// See [`rdpq_triangle`](libdragon_sys::rdpq_triangle) for details.
#[inline]
pub fn triangle(fmt: &TriFmt, v1: &[f32], v2: &[f32], v3: &[f32]) {
    assert!(v1.len() >= 2 && v2.len() >= 2 && v3.len() >= 2, "vertices must be two components each");
    unsafe {
        let ptr = core::mem::transmute(fmt);
        libdragon_sys::rdpq_triangle(ptr, v1.as_ptr(), v2.as_ptr(), v3.as_ptr());
    }
}
