use crate::*;

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
        let ptr      = data.as_mut_ptr();
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

    pub fn get_format(&self) -> surface::TexFormat {
        let spr = self.ptr as *const libdragon_sys::sprite_s;
        let f = unsafe { (*spr).__bindgen_anon_1.flags & (libdragon_sys::SPRITE_FLAGS_TEXFORMAT as u8) };
        (f as libdragon_sys::tex_format_t).into()
    }

    pub fn get_palette(&self) -> rdpq::TlutPalette {
        unsafe {
            libdragon_sys::sprite_get_palette(self.ptr as *mut libdragon_sys::sprite_t) as rdpq::TlutPalette
        }
    }

    pub fn get_pixels(&self) -> surface::Surface {
        // initialize surface_t from libdragon
        let mut surface: core::mem::MaybeUninit<libdragon_sys::surface_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            sprite_get_pixels_r(surface.as_mut_ptr(), self.ptr as *mut libdragon_sys::sprite_t);
        }

        // pin the structure in place before getting memory address
        let mut backing_instance = Box::pin(unsafe { surface.assume_init() });

        surface::Surface {
            ptr: backing_instance.as_mut().get_mut(),
            _backing_instance: Some(backing_instance),
            needs_free: false,
            phantom: core::marker::PhantomData,
        }
    }

}

extern "C" {
    fn sprite_get_pixels_r(surface: *mut libdragon_sys::surface_t, sprite: *mut libdragon_sys::sprite_t);
}

