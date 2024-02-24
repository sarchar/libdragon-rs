use crate::*;
use display::Surface;

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
//const _AUTOSYNC_TILE(n)  (1    << (0+(n)))
//const _AUTOSYNC_TMEM(n)  (1    << (8+(n)))

pub fn init() {
    unsafe {
        libdragon_sys::rdpq_init();
    }
}

pub fn debug_start() {
    unsafe {
        libdragon_sys::rdpq_debug_start();
    }
}

pub fn debug_log_msg(msg: &str) {
    let cmsg = CString::new(msg).unwrap();
    unsafe {
        libdragon_sys::rdpq_debug_log_msg(cmsg.as_ptr());
    }
}

pub fn set_blend_color(color: graphics::Color) {
    unsafe {
        __rdpq_write8_syncchange(libdragon_sys::RDPQ_CMD_SET_BLEND_COLOR, 0, color.to_packed32(), 
                                 libdragon_sys::AUTOSYNC_PIPE);
    }
}

pub fn set_fill_color(color: graphics::Color) {
    extern "C" {
        fn __rdpq_set_fill_color(c: ::core::ffi::c_uint);
    }
    let c = color.to_packed32();
    unsafe {
        __rdpq_set_fill_color(c);
    }
}

// rdpq_attach.h
pub fn attach(surf_color: Option<&Surface>, surf_depth: Option<&Surface>) {
    let color_null_surface = Surface { ptr: ::core::ptr::null_mut(), _backing_surface: None };
    let depth_null_surface = Surface { ptr: ::core::ptr::null_mut(), _backing_surface: None };
    unsafe {
        libdragon_sys::rdpq_attach_clear(surf_color.unwrap_or(&color_null_surface).ptr, 
                                         surf_depth.unwrap_or(&depth_null_surface).ptr);
    }
}

pub fn attach_clear(surf_color: Option<&Surface>, surf_depth: Option<&Surface>) {
    let color_null_surface = Surface { ptr: ::core::ptr::null_mut(), _backing_surface: None };
    let depth_null_surface = Surface { ptr: ::core::ptr::null_mut(), _backing_surface: None };
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

extern "C" {
    fn __rdpq_fixup_mode3(cmd_id: u32, w0: u32, w1: u32, w2: u32);
    fn __rdpq_write8_syncchange(cmd_id: u32, arg0: u32, arg1: u32, autosync: u32);
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
pub fn sprite_blit(sprite: &graphics::Sprite, x0: f32, y0: f32, parms: BlitParms) {
    let parms: libdragon_sys::rdpq_blitparms_t = parms.into();
    unsafe {
        libdragon_sys::rdpq_sprite_blit(sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s, x0, y0, &parms);
    }
}

// rdpq_tex.h
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tile(pub u32);

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
#[derive(Debug, Copy, Clone)]
pub struct TexParmST {
    translate: f32,
    scale_log: i32,
    repeats  : f32,
    mirror   : bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TexParms {
    tmem_addr: i32,
    palette  : i32,
    s        : TexParmST,
    t        : TexParmST,
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

