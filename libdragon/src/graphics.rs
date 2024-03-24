use crate::*;

use surface::Surface;
use sprite::Sprite;

/// Create a 32-bit RGBA color from four components [0-255]
///
/// See [`graphics_make_color`](libdragon_sys::graphics_make_color) for details.
#[inline]
pub fn make_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    unsafe {
        libdragon_sys::graphics_make_color(r as i32, g as i32, b as i32, a as i32)
    }
}

/// Wrapper around a [`color_t`](libdragon_sys::color_t)
///
/// The `color_t` is directly accessible via [Color::c].
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub c: libdragon_sys::color_t,
}

impl Color {
    /// Convert a [Color] to the 16-bit packed format used by a [TexFormat::Rgba16] surface (RGBA 5551)
    ///
    /// See [`color_to_packed16`](libdragon_sys::color_to_packed16) for details.
    #[inline]
    pub fn to_packed16(&self) -> u16 {
        ((self.c.r as u16) << 11) | (((self.c.g as u16) >> 3) << 6) | (((self.c.b as u16) >> 3) << 1) | ((self.c.a >> 7) as u16)
    }

    /// Convert a [Color] to the 32-bit packed format used by a [TexFormat::Rgba32] surface (RGBA 8888)
    ///
    /// See [`color_to_packed32`](libdragon_sys::color_to_packed32) for details.
    #[inline]
    pub fn to_packed32(&self) -> u32 {
        ((self.c.r as u32) << 24) | ((self.c.g as u32) << 16) | ((self.c.b as u32) << 8) | (self.c.a as u32)
    }

    /// Create a [Color] from the 16-bit packed format used by [TexFormat::Rgba16] surface (RGBA 8888 )
    ///
    /// See [`color_from_packed16`](libdragon_sys::color_from_packed16) for details.
    #[inline]
    pub fn from_packed16(c: u16) -> Self {
        Self {
            c: libdragon_sys::color_t {
                r: (((c >> 11) & 0x1F) << 3) as u8,
                g: (((c >>  6) & 0x1F) << 3) as u8,
                b: (((c >>  1) & 0x1F) << 3) as u8,
                a: if (c & 0x01) != 0 { 0xFF } else { 0x00 },
            }
        }
    }

    /// Create a [Color] from the 32-bit packed format used by [TexFormat::Rgba32] surface (RGBA 8888 )
    ///
    /// See [`color_from_packed32`](libdragon_sys::color_from_packed32) for details.
    #[inline]
    pub fn from_packed32(c: u32) -> Self {
        Self {
            c: libdragon_sys::color_t {
                r: ((c >> 24) & 0xFF) as u8,
                g: ((c >> 16) & 0xFF) as u8,
                b: ((c >>  8) & 0xFF) as u8,
                a: ((c >>  0) & 0xFF) as u8,
            }
        }
    }
}

/// Convert a [Color] to a [`libdragon_sys::color_t`].
impl From<Color> for libdragon_sys::color_t {
    fn from(c: Color) -> Self {
        unsafe {
            *core::mem::transmute::<&Color, *const Self>(&c)
        }
    }
}

impl Default for Color {
    /// Create a default [Color] (opaque black)
    fn default() -> Self { Color::from_packed32(0x0000_00FF) }
}

extern "C" {
    fn graphics_convert_color_r(color: *const libdragon_sys::color_t) -> u32;
}

/// Convert a [Color] to a packed format compatible with the current display.
///
/// See [`graphics_convert_color`](libdragon_sys::graphics_convert_color) for details.
pub fn convert_color(color: Color) -> u32 {
    unsafe {
        graphics_convert_color_r(&color.c as *const libdragon_sys::color_t)
    }
}

/// Create a [Color] from the R,G,B,A components in the RGBA16 range (that is: RGB in 0-31, A in 0-1).
///
/// See the `RGBA16` macro in LibDragon's `graphics.h` file.
#[inline]
pub fn rgba16(r: u8, g: u8, b: u8, a: u8) -> Color {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    Color {
        c: libdragon_sys::color_t {
            r: ((r << 3) | (r >> 3)) as u8, 
            g: ((g << 3) | (g >> 3)) as u8, 
            b: ((b << 3) | (b >> 3)) as u8, 
            a: if a != 0 { 0xFF } else { 0 },
        }
    }
}


/// Create a [Color] from the R,G,B,A components in the RGBA32 range (0-255).
///
/// See the `RGBA32` macro in LibDragon's `graphics.h` file.
#[inline]
pub fn rgba32(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color {
        c: libdragon_sys::color_t {
            r: r, g: g, b: b, a: a,
        }
    }
}

/// [Graphics] is a wrapper struct around a collection of functions that render to a [Surface].
pub struct Graphics<'a> {
    surface: Option<Surface<'a>>,
}

impl<'a> Graphics<'a> {
    /// Create a new [Graphics] interface from a given [Surface].  Ownership is given to the
    /// [Graphics] class and can be given back via [Graphics::finish].  Alternatively, a reference
    /// to the surface can be obtained from [Graphics::surface].
    #[inline]
    pub fn new(surface: Surface<'a>) -> Self {
        Self {
            surface: Some(surface)
        }
    }

    /// Return a reference to the [Surface] this [Graphics] context owns.
    #[inline]
    pub fn surface(&self) -> Option<&Surface<'a>> {
        self.surface.as_ref()
    }

    /// Return a mutable reference to the [Surface] this [Graphics] context owns.
    #[inline]
    pub fn surface_mut(&mut self) -> Option<&mut Surface<'a>> {
        self.surface.as_mut()
    }

    /// Take ownership of the underlying Surface. The `graphics_*` underlying functions can
    /// no longer be called and will cause an assert if used.
    #[inline]
    pub fn finish(&mut self) -> Surface<'a> {
        core::mem::replace(&mut self.surface, None).unwrap()
    }

    /// Fill the entire screen with a particular color
    ///
    /// See [`graphics_fill_screen`](libdragon_sys::graphics_fill_screen) for details.
    pub fn fill_screen(&mut self, color: u32) {
        unsafe {
            libdragon_sys::graphics_fill_screen(self.surface.as_mut().unwrap().ptr, color);
        }
    }

    /// Draw a pixel to a given display context
    ///
    /// See [`graphics_draw_pixel`](libdragon_sys::graphics_draw_pixel) for details.
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_pixel(self.surface.as_mut().unwrap().ptr, x, y, color);
        }
    }

    /// Draw a pixel to a given display context with alpha support
    ///
    /// See [`graphics_draw_pixel_trans`](libdragon_sys::graphics_draw_pixel_trans) for details.
    pub fn draw_pixel_trans(&mut self, x: i32, y: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_pixel_trans(self.surface.as_mut().unwrap().ptr, x, y, color);
        }
    }

    /// Draw a line to a given display context
    ///
    /// See [`graphics_draw_line`](libdragon_sys::graphics_draw_line) for details.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_line(self.surface.as_mut().unwrap().ptr, x0, y0, x1, y1, color);
        }
    }

    /// Draw a line to a given display context with alpha support
    ///
    /// See [`graphics_draw_line_trans`](libdragon_sys::graphics_draw_line_trans) for details.
    pub fn draw_line_trans(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_line_trans(self.surface.as_mut().unwrap().ptr, x0, y0, x1, y1, color);
        }
    }

    /// Draw a filled rectangle to a given display context
    ///
    /// See [`graphics_draw_box`](libdragon_sys::graphics_draw_box) for details.
    pub fn draw_box(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_box(self.surface.as_mut().unwrap().ptr, x0, y0, x1, y1, color);
        }
    }

    /// Draw a filled rectangle to a given display context with alpha support
    ///
    /// See [`graphics_draw_box_trans`](libdragon_sys::graphics_draw_box_trans) for details.
    pub fn draw_box_trans(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_box_trans(self.surface.as_mut().unwrap().ptr, x0, y0, x1, y1, color);
        }
    }

    /// Draw a character to the screen using the built-in font
    ///
    /// See [`graphics_draw_character`](libdragon_sys::graphics_draw_character) for details.
    pub fn draw_character(&mut self, x: i32, y: i32, c: char) -> Result<()> {
        unsafe {
            libdragon_sys::graphics_draw_character(self.surface.as_mut().unwrap().ptr, x, y, 
                                                   c.as_ascii().ok_or(LibDragonError::Utf8Error { error: None })?
                                                               .to_u8() as ::core::ffi::c_char);
        }
        Ok(())
    }

    /// Draw a string slice to a display context
    ///
    /// See [`graphics_draw_text`](libdragon_sys::graphics_draw_text) for details.
    pub fn draw_text(&mut self, x: i32, y: i32, msg: &str) {
        let cmsg = CString::new(msg).unwrap();
        unsafe {
            libdragon_sys::graphics_draw_text(self.surface.as_mut().unwrap().ptr, x, y, cmsg.as_ptr());
        }
    }

    /// Draw a sprite to a display context
    ///
    /// See [`graphics_draw_sprite`](libdragon_sys::graphics_draw_sprite) for details.
    pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite) {
        unsafe {
            libdragon_sys::graphics_draw_sprite(self.surface.as_mut().unwrap().ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s);
        }
    }

    /// Draw a sprite from a spritemap to a display context
    ///
    /// See [`graphics_draw_sprite_stride`](libdragon_sys::graphics_draw_sprite_stride) for details.
    pub fn draw_sprite_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: i32) {
        unsafe {
            libdragon_sys::graphics_draw_sprite_stride(self.surface.as_mut().unwrap().ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s, 
                                                offset as ::core::ffi::c_int);
        }
    }

    /// Draw a sprite to a display context with alpha transparency
    ///
    /// See [`graphics_draw_sprite_trans`](libdragon_sys::graphics_draw_sprite_trans) for details.
    pub fn draw_sprite_trans(&mut self, x: i32, y: i32, sprite: &Sprite) {
        unsafe {
            libdragon_sys::graphics_draw_sprite_trans(self.surface.as_mut().unwrap().ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s);
        }
    }

    /// Draw a sprite from a spritemap to a display context with alpha transparency
    ///
    /// See [`graphics_draw_sprite_trans_stride`](libdragon_sys::graphics_draw_sprite_trans_stride) for details.
    pub fn draw_sprite_trans_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: i32) {
        unsafe {
            libdragon_sys::graphics_draw_sprite_trans_stride(self.surface.as_mut().unwrap().ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s, 
                                                             offset as ::core::ffi::c_int);
        }
    }
}

/// Set the current forecolor and backcolor for text operations.
///
/// This function changes the _global state_ of foreground and background colors.
///
/// See [`graphics_set_color`](libdragon_sys::graphics_set_color) for details.
#[inline]
pub fn set_color(forecolor: u32, backcolor: u32) {
    unsafe {
        libdragon_sys::graphics_set_color(forecolor, backcolor);
    }
}

/// Set the current font.
///
/// This function changes the _global state_ of the current font.
///
/// See [`graphics_set_default_font`](libdragon_sys::graphics_set_default_font) for details.
pub fn set_default_font() {
    unsafe {
        libdragon_sys::graphics_set_default_font();
    }
}

/// Set the current font. Should be set before using any of the [Graphics] draw functions.
///
/// Rust: take ownership of [Sprite] and pin its memory in place. The caller of this function 
/// is required to keep the memory available until the font has been changed to something else, 
/// otherwise Bad Things Will Happen.
///
/// See [`graphics_set_font_sprite`](libdragon_sys::graphics_set_font_sprite) for details.
#[inline]
pub fn set_font_sprite<'b>(sprite: Sprite<'b>) -> core::pin::Pin<Box<Sprite<'b>>> {
    let pinned = Box::pin(sprite);
    unsafe {
        libdragon_sys::graphics_set_font_sprite(pinned.as_const_sprite_s() as *mut libdragon_sys::sprite_s);
    }
    pinned
}


