use crate::*;

use display::Display;

pub fn make_color(r: i32, g: i32, b: i32, a: i32) -> u32 {
    unsafe {
        libdragon_sys::graphics_make_color(r, g, b, a)
    }
}

pub struct Graphics {
    surface: Display,
}

impl Graphics {
    pub fn new(surface: Display) -> Self {
        Self {
            surface: surface
        }
    }

    pub fn set_color(&self, forecolor: u32, backcolor: u32) {
        unsafe {
            libdragon_sys::graphics_set_color(forecolor, backcolor);
        }
    }

    pub fn fill_screen(&mut self, color: u32) {
        unsafe {
            libdragon_sys::graphics_fill_screen(self.surface.ptr, color);
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        unsafe {
            libdragon_sys::graphics_draw_line(self.surface.ptr, x0, y0, x1, y1, color);
        }
    }

    pub fn draw_text(&mut self, x: i32, y: i32, msg: &str) {
        let cmsg = CString::new(msg).unwrap();
        unsafe {
            libdragon_sys::graphics_draw_text(self.surface.ptr, x, y, cmsg.as_ptr());
        }
    }
}
