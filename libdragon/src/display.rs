#![allow(non_upper_case_globals)]

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

#[derive(Copy, Clone)]
pub struct Display {
    pub(crate) ptr: *mut libdragon_sys::surface_t,
}


impl Display {
    pub fn get() -> Self {
        let ptr = unsafe {
            libdragon_sys::display_get()
        };
        Self {
            ptr: ptr,
        }
    }

    pub fn try_get() -> Option<Self> {
        let ptr = unsafe {
            libdragon_sys::display_try_get()
        };

        if ptr == ::core::ptr::null_mut() {
            None
        } else {
            Some(Self {
                ptr: ptr,
            })
        }
    }

    pub fn show(&self) {
        unsafe {
            libdragon_sys::display_show(self.ptr);
        }
    }

    pub fn get_width(&self) -> u32 {
        unsafe {
            libdragon_sys::display_get_width()
        }
    }

    pub fn get_height(&self) -> u32 {
        unsafe {
            libdragon_sys::display_get_height()
        }
    }

    pub fn get_bitdepth(&self) -> BitDepth {
        let s = unsafe {
            libdragon_sys::display_get_bitdepth()
        };
        s.into()
    }

    pub fn get_num_buffers(&self) -> u32 {
        unsafe {
            libdragon_sys::display_get_num_buffers()
        }
    }

    pub fn get_fps(&self) -> f32 {
        unsafe {
            libdragon_sys::display_get_fps()
        }
    }
}
