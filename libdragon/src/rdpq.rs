use crate::*;

use surface::Surface;
use sprite::Sprite;

// rdpq_macros.h
pub const SOM_SAMPLE_SHIFT   : u32 = libdragon_sys::SOM_SAMPLE_SHIFT;
pub const SOM_SAMPLE_POINT   : u64 = 0u64 << SOM_SAMPLE_SHIFT;
pub const SOM_SAMPLE_BILINEAR: u64 = 2u64 << SOM_SAMPLE_SHIFT;
pub const SOM_SAMPLE_MEDIAN  : u64 = 3u64 << SOM_SAMPLE_SHIFT;
pub const SOM_SAMPLE_MASK    : u64 = 3u64 << SOM_SAMPLE_SHIFT;

pub const SOM_ALPHACOMPARE_SHIFT    : u32 = libdragon_sys::SOM_ALPHACOMPARE_SHIFT;
pub const SOM_ALPHACOMPARE_NONE     : u64 = 0u64 << SOM_ALPHACOMPARE_SHIFT;
pub const SOM_ALPHACOMPARE_THRESHOLD: u64 = 1u64 << SOM_ALPHACOMPARE_SHIFT;
pub const SOM_ALPHACOMPARE_NOISE    : u64 = 3u64 << SOM_ALPHACOMPARE_SHIFT;
pub const SOM_ALPHACOMPARE_MASK     : u64 = 3u64 << SOM_ALPHACOMPARE_SHIFT;

pub const SOM_TLUT_SHIFT         : u32 = libdragon_sys::SOM_TLUT_SHIFT;
pub const SOM_TLUT_NONE          : u64 = 0u64 << SOM_TLUT_SHIFT;
pub const SOM_TLUT_RGBA16        : u64 = 2u64 << SOM_TLUT_SHIFT;
pub const SOM_TLUT_IA16          : u64 = 3u64 << SOM_TLUT_SHIFT;
pub const SOM_TLUT_MASK          : u64 = 3u64 << SOM_TLUT_SHIFT;

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
    fn __rdpq_fixup_mode3(cmd_id: u32, w0: u32, w1: u32, w2: u32);
    fn __rdpq_write8_syncchange(cmd_id: u32, arg0: u32, arg1: u32, autosync: u32);
    fn __rdpq_fixup_write8_syncchange(cmd_id: u32, arg0: u32, arg1: u32, autosync: u32);
}

/// Tile descriptors
///
/// The only valid tiles are 0-7
///
/// See [`rdpq_tile_t`](libdragon_sys::rdpq_tile_t`) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

pub fn debug_start() {
    unsafe {
        libdragon_sys::rdpq_debug_start();
    }
}

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

pub fn debug_log_msg(msg: &str) {
    let cmsg = CString::new(msg).unwrap();
    unsafe {
        libdragon_sys::rdpq_debug_log_msg(cmsg.as_ptr());
    }
}

// rdpq_attach.h
pub fn attach(surf_color: Option<&Surface>, surf_depth: Option<&Surface>) {
    let color_null_surface = Surface::from_ptr(::core::ptr::null_mut());
    let depth_null_surface = Surface::from_ptr(::core::ptr::null_mut());
    unsafe {
        libdragon_sys::rdpq_attach_clear(surf_color.unwrap_or(&color_null_surface).ptr, 
                                         surf_depth.unwrap_or(&depth_null_surface).ptr);
    }
}

pub fn attach_clear(surf_color: Option<&Surface>, surf_depth: Option<&Surface>) {
    let color_null_surface = Surface::from_ptr(::core::ptr::null_mut());
    let depth_null_surface = Surface::from_ptr(::core::ptr::null_mut());
    unsafe {
        libdragon_sys::rdpq_attach_clear(surf_color.unwrap_or(&color_null_surface).ptr, 
                                         surf_depth.unwrap_or(&depth_null_surface).ptr);
    }
}

pub fn detach() {
    extern "C" {
        fn rdpq_detach_cb(cb: Option<unsafe extern "C" fn(arg: *mut core::ffi::c_void)>, arg: *mut ::core::ffi::c_void);
    }
    unsafe {
        rdpq_detach_cb(None, ::core::ptr::null_mut());
    }
}

pub fn detach_show() {
    unsafe {
        libdragon_sys::rdpq_detach_show();
    }
}

#[inline]
pub fn detach_wait() {
    detach();
    rspq::wait();
}

// rdpq_mode.h
#[derive(Copy, Clone)]
pub enum Dither {
    SquareSquare,
    SquareInvSquare,
    SquareNoise,
    SquareNone,

    BayerBayer,
    BayerInvBayer,
    BayerNoise,
    BayerNone,

    NoiseSquare,
    NoiseInvSquare,
    NoiseNoise,
    NoiseNone,

    NoneBayer,
    NoneInvBayer,
    NoneNoise,
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

pub enum Filter {
    Point,
    Bilinear,
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

pub enum Tlut {
    None,
    Rgba16,
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

pub fn set_mode_standard() {
    unsafe {
        libdragon_sys::rdpq_set_mode_standard();
    }
}

pub fn set_mode_copy(transparency: bool) {
    unsafe {
        libdragon_sys::rdpq_set_mode_copy(transparency);
    }
}

pub fn set_mode_fill(color: graphics::Color) {
    extern "C" {
        fn __rdpq_set_mode_fill();
    }
    unsafe {
        __rdpq_set_mode_fill();
    }
    set_fill_color(color);
}

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

#[inline]
pub fn mode_push() {
    unsafe {
        libdragon_sys::rdpq_mode_push();
    }
}

pub fn mode_pop() {
    unsafe {
        libdragon_sys::rdpq_mode_pop();
    }
}

#[inline]
pub fn mode_filter(filt: Filter) {
    let filt = Into::<libdragon_sys::rdpq_filter_s>::into(filt) as u64;
    __mode_change_som(SOM_SAMPLE_MASK, filt << SOM_SAMPLE_SHIFT);
}

#[inline]
pub fn mode_alphacompare(threshold: i32) {
    if threshold == 0 {
        __mode_change_som(SOM_ALPHACOMPARE_MASK, 0);
    } else if threshold > 0 {
        __mode_change_som(SOM_ALPHACOMPARE_MASK, SOM_ALPHACOMPARE_THRESHOLD);
        set_blend_color(graphics::rgba32(0,0,0,threshold as u8));
    } else {
        __mode_change_som(SOM_ALPHACOMPARE_MASK, SOM_ALPHACOMPARE_NOISE);
    }
}

#[inline]
pub fn mode_tlut(tlut: Tlut) {
    __mode_change_som(SOM_TLUT_MASK, (Into::<libdragon_sys::rdpq_tlut_s>::into(tlut) as u64) << SOM_TLUT_SHIFT);
}

// rdpq_sprite.h
//void rdpq_sprite_blit(sprite_t *sprite, float x0, float y0, const rdpq_blitparms_t *parms);
pub fn sprite_blit(sprite: &Sprite, x0: f32, y0: f32, parms: BlitParms) {
    let parms: libdragon_sys::rdpq_blitparms_t = parms.into();
    unsafe {
        libdragon_sys::rdpq_sprite_blit(sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s, x0, y0, &parms);
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

// rdpq_rect.h
extern "C" {
    fn __rdpq_fill_rectangle_offline(x0: ::core::ffi::c_int, 
                                     y0: ::core::ffi::c_int, 
                                     x1: ::core::ffi::c_int, 
                                     y1: ::core::ffi::c_int);
    fn __rdpq_texture_rectangle_offline(tile: libdragon_sys::rdpq_tile_t,
                                        x0: ::core::ffi::c_int,
                                        y0: ::core::ffi::c_int,
                                        x1: ::core::ffi::c_int,
                                        y1: ::core::ffi::c_int,
                                        s0: ::core::ffi::c_int,
                                        t0: ::core::ffi::c_int);
}

#[inline(always)]
pub fn fill_rectangle(x0: i32, y0: i32, x1: i32, y1: i32) {
    __rdpq_fill_rectangle_fx(x0*4, y0*4, x1*4, y1*4);
}

#[inline]
fn __rdpq_fill_rectangle_fx(x0: i32, y0: i32, x1: i32, y1: i32) {
    unsafe {
        __rdpq_fill_rectangle_offline(x0, y0, x1, y1);
    }
}

#[inline(always)]
pub fn texture_rectangle(tile: Tile, x0: i32, y0: i32, x1: i32, y1: i32, s: i32, t: i32) {
    __texture_rectangle_fx(tile, x0*4, y0*4, x1*4, y1*4, s*32, t*32)
}

#[inline]
pub fn __texture_rectangle_fx(tile: Tile, x0: i32, y0: i32, x1: i32, y1: i32, s: i32, t: i32) {
    unsafe {
        __rdpq_texture_rectangle_offline(tile.0 as libdragon_sys::rdpq_tile_t, x0, y0, x1, y1, s, t);
    }
}

// rdpq_font.h
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

pub struct Font {
    pub(crate) ptr: *mut libdragon_sys::rdpq_font_t,
}

impl Font {
    /// See [`rdpq_font_load`](libdragon-sys::rdpq_font_load) for details.
    ///
    /// The Libdragon [`rdpq_font_t`](libdragon-sys::rdpq_font_t) is freed when this object is dropped
    pub fn load(filename: &str) -> Self {
        let cfilename = CString::new(filename).unwrap();
        let ptr = unsafe {
            libdragon_sys::rdpq_font_load(cfilename.as_ptr())
        };
        Self {
            ptr: ptr,
        }
    }

    pub fn style(&mut self, id: u8, style: FontStyle) {
        unsafe {
            libdragon_sys::rdpq_font_style(self.ptr, id, &Into::<libdragon_sys::rdpq_fontstyle_t>::into(style) as *const libdragon_sys::rdpq_fontstyle_t)
        }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            libdragon_sys::rdpq_font_free(self.ptr);
        }
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

