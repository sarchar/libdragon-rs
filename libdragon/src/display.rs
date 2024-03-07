#![allow(non_upper_case_globals)]

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureFormat {
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

impl From<libdragon_sys::tex_format_t> for TextureFormat {
    fn from(value: libdragon_sys::tex_format_t) -> Self {
        match value {
            libdragon_sys::tex_format_t_FMT_NONE   => TextureFormat::None,
            libdragon_sys::tex_format_t_FMT_RGBA16 => TextureFormat::Rgba16,
            libdragon_sys::tex_format_t_FMT_RGBA32 => TextureFormat::Rgba32,
            libdragon_sys::tex_format_t_FMT_YUV16  => TextureFormat::Yuv16,
            libdragon_sys::tex_format_t_FMT_CI4    => TextureFormat::Ci4,
            libdragon_sys::tex_format_t_FMT_CI8    => TextureFormat::Ci8,
            libdragon_sys::tex_format_t_FMT_IA4    => TextureFormat::Ia4,
            libdragon_sys::tex_format_t_FMT_IA8    => TextureFormat::Ia8,
            libdragon_sys::tex_format_t_FMT_IA16   => TextureFormat::Ia16,
            libdragon_sys::tex_format_t_FMT_I4     => TextureFormat::I4,
            libdragon_sys::tex_format_t_FMT_I8     => TextureFormat::I8,
            _ => panic!("invalid tex_format_t"),
        }
    }
}

impl Into<libdragon_sys::tex_format_t> for TextureFormat {
    fn into(self) -> libdragon_sys::tex_format_t {
        match self {
            TextureFormat::None   => libdragon_sys::tex_format_t_FMT_NONE,
            TextureFormat::Rgba16 => libdragon_sys::tex_format_t_FMT_RGBA16,
            TextureFormat::Rgba32 => libdragon_sys::tex_format_t_FMT_RGBA32,
            TextureFormat::Yuv16  => libdragon_sys::tex_format_t_FMT_YUV16,
            TextureFormat::Ci4    => libdragon_sys::tex_format_t_FMT_CI4,
            TextureFormat::Ci8    => libdragon_sys::tex_format_t_FMT_CI8,
            TextureFormat::Ia4    => libdragon_sys::tex_format_t_FMT_IA4,
            TextureFormat::Ia8    => libdragon_sys::tex_format_t_FMT_IA8,
            TextureFormat::Ia16   => libdragon_sys::tex_format_t_FMT_IA16,
            TextureFormat::I4     => libdragon_sys::tex_format_t_FMT_I4,
            TextureFormat::I8     => libdragon_sys::tex_format_t_FMT_I8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitDepth {
    Bpp16,
    Bpp32,
}

impl Into<libdragon_sys::bitdepth_t> for BitDepth {
    fn into(self) -> libdragon_sys::bitdepth_t {
        match self {
            BitDepth::Bpp16 => libdragon_sys::bitdepth_t_DEPTH_16_BPP,
            BitDepth::Bpp32 => libdragon_sys::bitdepth_t_DEPTH_32_BPP,
        }
    }
}

impl From<libdragon_sys::bitdepth_t> for BitDepth {
    fn from(value: libdragon_sys::bitdepth_t) -> Self {
        match value {
            libdragon_sys::bitdepth_t_DEPTH_16_BPP => BitDepth::Bpp16,
            libdragon_sys::bitdepth_t_DEPTH_32_BPP => BitDepth::Bpp32,
            _ => panic!("invalid bitdepth_t value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gamma {
    None,
    Correct,
    CorrectDither
}

impl Into<libdragon_sys::gamma_t> for Gamma {
    fn into(self) -> libdragon_sys::gamma_t {
        match self {
            Gamma::None => libdragon_sys::gamma_t_GAMMA_NONE,
            Gamma::Correct => libdragon_sys::gamma_t_GAMMA_CORRECT,
            Gamma::CorrectDither => libdragon_sys::gamma_t_GAMMA_CORRECT_DITHER,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterOptions {
    Disabled,
    Resample,
    Dedither,
    ResampleAntialias,
    ResampleAntialiasDedither,
}

impl Into<libdragon_sys::filter_options_t> for FilterOptions {
    fn into(self) -> libdragon_sys::filter_options_t {
        match self {
            FilterOptions::Disabled => libdragon_sys::filter_options_t_FILTERS_DISABLED,
            FilterOptions::Resample => libdragon_sys::filter_options_t_FILTERS_RESAMPLE,
            FilterOptions::Dedither => libdragon_sys::filter_options_t_FILTERS_DEDITHER,
            FilterOptions::ResampleAntialias => libdragon_sys::filter_options_t_FILTERS_RESAMPLE_ANTIALIAS,
            FilterOptions::ResampleAntialiasDedither => libdragon_sys::filter_options_t_FILTERS_RESAMPLE_ANTIALIAS_DEDITHER,
        }
    }
}

// Re-export the resolution structs
static RESOLUTION_256x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_256x240 };
static RESOLUTION_320x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_320x240 };
static RESOLUTION_512x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_512x240 };
static RESOLUTION_640x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_640x240 };
static RESOLUTION_512x480: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_512x480 };
static RESOLUTION_640x480: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_640x480 };

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    _256x240,
    _320x240,
    _512x240,
    _640x240,
    _512x480,
    _640x480,
}

impl Into<libdragon_sys::resolution_t> for Resolution {
    fn into(self) -> libdragon_sys::resolution_t {
        match self {
            Resolution::_256x240 => *RESOLUTION_256x240,
            Resolution::_320x240 => *RESOLUTION_320x240,
            Resolution::_512x240 => *RESOLUTION_512x240,
            Resolution::_640x240 => *RESOLUTION_640x240,
            Resolution::_512x480 => *RESOLUTION_512x480,
            Resolution::_640x480 => *RESOLUTION_640x480,
        }
    }
}

extern "C" {
    pub fn display_init_r(
        res: *const libdragon_sys::resolution_t,
        bit: libdragon_sys::bitdepth_t,
        num_buffers: u32,
        gamma: libdragon_sys::gamma_t,
        filters: libdragon_sys::filter_options_t);
}

pub fn init(resolution: Resolution, depth: BitDepth, num_buffers: u32, gamma: Gamma, filter: FilterOptions) {
    unsafe {
        display_init_r(&resolution.into() as *const libdragon_sys::resolution_t, 
                       depth.into(), num_buffers, gamma.into(), filter.into())
    }
}

pub fn close() {
    unsafe {
        libdragon_sys::display_close();
    }
}

pub fn get() -> Surface {
    let ptr = unsafe {
        libdragon_sys::display_get()
    };

    Surface {
        ptr: ptr,
        _backing_surface: None,
        needs_free: false,
    }
}

pub fn try_get() -> Option<Surface> {
    let ptr = unsafe {
        libdragon_sys::display_try_get()
    };

    if ptr == ::core::ptr::null_mut() {
        None
    } else {
        Some(Surface {
            ptr: ptr,
            _backing_surface: None,
            needs_free: false,
        })
    }
}

pub fn get_width() -> u32 {
    unsafe {
        libdragon_sys::display_get_width()
    }
}

pub fn get_height() -> u32 {
    unsafe {
        libdragon_sys::display_get_height()
    }
}

pub fn get_bitdepth() -> BitDepth {
    let s = unsafe {
        libdragon_sys::display_get_bitdepth()
    };
    s.into()
}

pub fn get_num_buffers() -> u32 {
    unsafe {
        libdragon_sys::display_get_num_buffers()
    }
}

pub fn get_fps() -> f32 {
    unsafe {
        libdragon_sys::display_get_fps()
    }
}

pub struct Surface {
    pub(crate) ptr: *mut libdragon_sys::surface_t,
    pub(crate) _backing_surface: Option<core::pin::Pin<Box<libdragon_sys::surface_t>>>,
    pub(crate) needs_free: bool,
}

impl Surface {
    pub fn alloc(format: TextureFormat, width: u32, height: u32) -> Self {
        extern "C" {
            fn surface_alloc_r(surface: *mut libdragon_sys::surface_t, format: libdragon_sys::tex_format_t, width: u32, height: u32);
        }

        let mut backing_surface = Box::pin(unsafe {
            core::mem::MaybeUninit::<libdragon_sys::surface_t>::zeroed().assume_init()
        });

        unsafe {
            surface_alloc_r(backing_surface.as_mut().get_mut() as *mut _, format.into(), width, height);
        }

        display::Surface {
            ptr: backing_surface.as_mut().get_mut(),
            _backing_surface: Some(backing_surface),
            needs_free: true,
        }
    }

    pub fn show(&self) {
        unsafe {
            libdragon_sys::display_show(self.ptr);
        }
    }
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
