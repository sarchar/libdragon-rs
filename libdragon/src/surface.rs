use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexFormat {
    None,
    Rgba16,
    Rgba32,
    Yuv16,
    Ci4,
    Ci8,
    Ia4,
    Ia8,
    Ia16,
    I4,
    I8,
}

impl From<libdragon_sys::tex_format_t> for TexFormat {
    fn from(value: libdragon_sys::tex_format_t) -> Self {
        match value {
            libdragon_sys::tex_format_t_FMT_NONE   => TexFormat::None,
            libdragon_sys::tex_format_t_FMT_RGBA16 => TexFormat::Rgba16,
            libdragon_sys::tex_format_t_FMT_RGBA32 => TexFormat::Rgba32,
            libdragon_sys::tex_format_t_FMT_YUV16  => TexFormat::Yuv16,
            libdragon_sys::tex_format_t_FMT_CI4    => TexFormat::Ci4,
            libdragon_sys::tex_format_t_FMT_CI8    => TexFormat::Ci8,
            libdragon_sys::tex_format_t_FMT_IA4    => TexFormat::Ia4,
            libdragon_sys::tex_format_t_FMT_IA8    => TexFormat::Ia8,
            libdragon_sys::tex_format_t_FMT_IA16   => TexFormat::Ia16,
            libdragon_sys::tex_format_t_FMT_I4     => TexFormat::I4,
            libdragon_sys::tex_format_t_FMT_I8     => TexFormat::I8,
            _ => panic!("invalid tex_format_t"),
        }
    }
}

impl Into<libdragon_sys::tex_format_t> for TexFormat {
    fn into(self) -> libdragon_sys::tex_format_t {
        match self {
            TexFormat::None   => libdragon_sys::tex_format_t_FMT_NONE,
            TexFormat::Rgba16 => libdragon_sys::tex_format_t_FMT_RGBA16,
            TexFormat::Rgba32 => libdragon_sys::tex_format_t_FMT_RGBA32,
            TexFormat::Yuv16  => libdragon_sys::tex_format_t_FMT_YUV16,
            TexFormat::Ci4    => libdragon_sys::tex_format_t_FMT_CI4,
            TexFormat::Ci8    => libdragon_sys::tex_format_t_FMT_CI8,
            TexFormat::Ia4    => libdragon_sys::tex_format_t_FMT_IA4,
            TexFormat::Ia8    => libdragon_sys::tex_format_t_FMT_IA8,
            TexFormat::Ia16   => libdragon_sys::tex_format_t_FMT_IA16,
            TexFormat::I4     => libdragon_sys::tex_format_t_FMT_I4,
            TexFormat::I8     => libdragon_sys::tex_format_t_FMT_I8,
        }
    }
}


pub struct Surface {
    pub(crate) ptr: *mut libdragon_sys::surface_t,
    pub(crate) _backing_surface: Option<core::pin::Pin<Box<libdragon_sys::surface_t>>>,
    pub(crate) needs_free: bool,
}

impl Surface {
    pub(crate) fn from_ptr(ptr: *mut libdragon_sys::surface_t) -> Self {
        Self {
            ptr: ptr,
            _backing_surface: None,
            needs_free: false,
        }
    }

    pub fn alloc(format: TexFormat, width: u32, height: u32) -> Self {
        extern "C" {
            fn surface_alloc_r(surface: *mut libdragon_sys::surface_t, format: libdragon_sys::tex_format_t, width: u32, height: u32);
        }

        let mut backing_surface = Box::pin(unsafe {
            core::mem::MaybeUninit::<libdragon_sys::surface_t>::zeroed().assume_init()
        });

        unsafe {
            surface_alloc_r(backing_surface.as_mut().get_mut() as *mut _, format.into(), width, height);
        }

        Self {
            ptr: backing_surface.as_mut().get_mut(),
            _backing_surface: Some(backing_surface),
            needs_free: true,
        }
    }

    /// Display a buffer on the screen
    ///
    /// See [`display_show`](libdragon_sys::display_show)
    pub fn show(&self) { unsafe { libdragon_sys::display_show(self.ptr); } }
}

impl Drop for Surface {
    fn drop(&mut self) {
        if self.needs_free {
            unsafe {
                libdragon_sys::surface_free(self.ptr);
            }
        }
    }
}
