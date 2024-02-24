use crate::*;

use display::Surface;

pub fn make_color(r: i32, g: i32, b: i32, a: i32) -> u32 {
    unsafe {
        libdragon_sys::graphics_make_color(r, g, b, a)
    }
}

// color_t is a simple type that can be exposed directly
pub struct Color {
    pub c: libdragon_sys::color_t,
}

impl Color {
    #[inline]
    pub fn to_packed16(&self) -> u16 {
        ((self.c.r as u16) << 11) | (((self.c.g as u16) >> 3) << 6) | (((self.c.b as u16) >> 3) << 1) | ((self.c.a >> 7) as u16)
    }

    #[inline]
    pub fn to_packed32(&self) -> u32 {
        ((self.c.r as u32) << 24) | ((self.c.g as u32) << 16) | ((self.c.b as u32) << 8) | (self.c.a as u32)
    }

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

extern "C" {
    fn graphics_convert_color_r(color: *const libdragon_sys::color_t) -> u32;
    fn sprite_get_pixels_r(surface: *mut libdragon_sys::surface_t, sprite: *mut libdragon_sys::sprite_t);
}

pub fn convert_color( color: Color ) -> u32 {
    unsafe {
        graphics_convert_color_r(&color.c as *const libdragon_sys::color_t)
    }
}

pub fn rgba32(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color {
        c: libdragon_sys::color_t {
            r: r, g: g, b: b, a: a,
        }
    }
}

pub struct Graphics {
    surface: Surface,
}

impl Graphics {
    pub fn new(surface: Surface) -> Self {
        Self {
            surface: surface
        }
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn set_color(&self, forecolor: u32, backcolor: u32) {
        unsafe {
            libdragon_sys::graphics_set_color(forecolor, backcolor);
        }
    }

    pub fn set_default_font(&self) {
        unsafe {
            libdragon_sys::graphics_set_default_font();
        }
    }

    // take ownership of sprite and pin its memory in place
    // the caller of this function is required to keep the memory available until the font has been
    // changed to something else, otherwise Bad Things Will Happen
    pub fn set_font_sprite(&self, sprite: Sprite) -> core::pin::Pin<Box<Sprite>> {
        let pinned = Box::pin(sprite);
        unsafe {
            libdragon_sys::graphics_set_font_sprite(pinned.as_const_sprite_s() as *mut libdragon_sys::sprite_s);
        }
        pinned
    }

    pub fn fill_screen(&mut self, color: u32) {
        unsafe {
            libdragon_sys::graphics_fill_screen(self.surface.ptr, color);
        }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_pixel(self.surface.ptr, x, y, color);
        }
    }

    pub fn draw_pixel_trans(&mut self, x: i32, y: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_pixel_trans(self.surface.ptr, x, y, color);
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_line(self.surface.ptr, x0, y0, x1, y1, color);
        }
    }

    pub fn draw_line_trans(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_line_trans(self.surface.ptr, x0, y0, x1, y1, color);
        }
    }

    pub fn draw_box(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_box(self.surface.ptr, x0, y0, x1, y1, color);
        }
    }

    pub fn draw_character(&mut self, x: i32, y: i32, c: char) {
        unsafe {
            libdragon_sys::graphics_draw_character(self.surface.ptr, x, y, c as ::core::ffi::c_char);
        }
    }

    pub fn draw_text(&mut self, x: i32, y: i32, msg: &str) {
        let cmsg = CString::new(msg).unwrap();
        unsafe {
            libdragon_sys::graphics_draw_text(self.surface.ptr, x, y, cmsg.as_ptr());
        }
    }

    pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite) {
        unsafe {
            libdragon_sys::graphics_draw_sprite(self.surface.ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s);
        }
    }

    pub fn draw_sprite_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: i32) {
        unsafe {
            libdragon_sys::graphics_draw_sprite_stride(self.surface.ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s, 
                                                offset as ::core::ffi::c_int);
        }
    }

    pub fn draw_sprite_trans(&mut self, x: i32, y: i32, sprite: &Sprite) {
        unsafe {
            libdragon_sys::graphics_draw_sprite_trans(self.surface.ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s);
        }
    }

    pub fn draw_sprite_trans_stride(&mut self, x: i32, y: i32, sprite: &Sprite, offset: i32) {
        unsafe {
            libdragon_sys::graphics_draw_sprite_trans_stride(self.surface.ptr, x, y, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_s, 
                                                             offset as ::core::ffi::c_int);
        }
    }
}

pub struct Sprite {
    ptr: *mut u8,
    _data: Option<core::pin::Pin<Box<Vec<u8>>>>,
}

unsafe impl Send for Sprite {}
unsafe impl Sync for Sprite {}

impl Sprite {
    pub fn load(path: dfs::DfsPathBuf) -> Result<Self> {
        let path_bytes: &[u8] = path.as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let s = unsafe {
            libdragon_sys::sprite_load(cpath.as_ptr()) as *mut u8
        };

        Ok(Self {
            ptr: s,
            _data: None,
        })
    }

    pub fn from_data(data: Vec<u8>) -> Self {
        let mut data = Box::pin(data);
        let ptr  = data.as_mut_ptr();
        Self {
            _data: Some(data),
            ptr: ptr,
        }
    }

    pub(crate) fn as_const_sprite_s(&self) -> *const libdragon_sys::sprite_s {
        unsafe {
            core::mem::transmute(self.ptr)
        }
    }

    pub fn width(&self) -> u16 {
        let spr = self.ptr as *const libdragon_sys::sprite_s;
        unsafe { (*spr).width }
    }

    pub fn height(&self) -> u16 {
        let spr = self.ptr as *const libdragon_sys::sprite_s;
        unsafe { (*spr).height }
    }

    pub fn hslices(&self) -> u8 {
        let spr = self.ptr as *const libdragon_sys::sprite_s;
        unsafe { (*spr).hslices }
    }

    pub fn vslices(&self) -> u8 {
        let spr = self.ptr as *const libdragon_sys::sprite_s;
        unsafe { (*spr).vslices }
    }

    pub fn get_format(&self) -> display::TextureFormat {
        let spr = self.ptr as *const libdragon_sys::sprite_s;
        let f = unsafe { (*spr).__bindgen_anon_1.flags & (libdragon_sys::SPRITE_FLAGS_TEXFORMAT as u8) };
        (f as libdragon_sys::tex_format_t).into()
    }

    pub fn get_palette(&self) -> rdpq::TlutPalette {
        unsafe {
            libdragon_sys::sprite_get_palette(self.ptr as *mut libdragon_sys::sprite_t) as rdpq::TlutPalette
        }
    }

    pub fn get_pixels(&self) -> display::Surface {
        // initialize surface_t from libdragon
        let mut surface: core::mem::MaybeUninit<libdragon_sys::surface_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            sprite_get_pixels_r(surface.as_mut_ptr(), self.ptr as *mut libdragon_sys::sprite_t);
        }

        // pin the structure in place before getting memory address
        let mut backing_surface = Box::pin(unsafe { surface.assume_init() });

        display::Surface {
            ptr: unsafe { 
                core::mem::transmute(backing_surface.as_mut()) 
            },
            _backing_surface: Some(backing_surface),
        }
    }

}
