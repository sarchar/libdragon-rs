#![allow(non_upper_case_globals)]

use crate::*;

use surface::Surface;

/// Valid interlace modes
///
/// See [`interlace_mode_t`](libdragon_sys::interlace_mode_t)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterlaceMode {
    /// Video output is not interlaced
    Off,
    /// Video output is interlaced and buffer is swapped on odd and even fields
    Half,
    /// Video output is interlaced and buffer is swapped only on even fields
    Full
}

impl Into<libdragon_sys::interlace_mode_t> for InterlaceMode {
    fn into(self) -> libdragon_sys::interlace_mode_t {
        match self {
            InterlaceMode::Off  => libdragon_sys::interlace_mode_t_INTERLACE_OFF,
            InterlaceMode::Half => libdragon_sys::interlace_mode_t_INTERLACE_HALF,
            InterlaceMode::Full => libdragon_sys::interlace_mode_t_INTERLACE_FULL,
        }
    }
}

// Private import of the resolution structures from LibDragon
static RESOLUTION_256x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_256x240 };
static RESOLUTION_320x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_320x240 };
static RESOLUTION_512x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_512x240 };
static RESOLUTION_640x240: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_640x240 };
static RESOLUTION_512x480: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_512x480 };
static RESOLUTION_640x480: &libdragon_sys::resolution_t = unsafe { &libdragon_sys::RESOLUTION_640x480 };

/// Video resolutions
///
/// Rust-specific: instead of structs on the C side, enumerations are used in Rust.
/// If a custom resolution is desired, use [Resolution::Custom].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    /// 256x240 mode
    _256x240,
    /// 320x240 mode
    _320x240,
    /// 512x240 mode
    _512x240,
    /// 640x480 mode
    _640x240,
    /// 512x480 mode
    _512x480,
    /// 640x480 mode
    _640x480,
    /// User-defined mode
    /// 
    /// First parameter is width, second is height, third is the [InterlaceMode]
    Custom(u32, u32, InterlaceMode),
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
            Resolution::Custom(w, h, i) => {
                libdragon_sys::resolution_t {
                    width: w as i32,
                    height: h as i32,
                    interlaced: i.into(),
                }
            }
        }
    }
}

/// Valid bit depths
///
/// See [`bitdepth_t`](libdragon_sys::bitdepth_t)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitDepth {
    /// 16 bits per pixel (5-5-5-1)
    Bpp16,
    /// 32 bits per pixel (8-8-8-8)
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

/// Valid gamma correction settings
///
/// See [`gamma_t`](libdragon_sys::gamma_t)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gamma {
    /// Uncorrected gamma, should be used by default and with assets built by libdragon tools
    None,
    /// Corrected gamma, should be used on a 32-bit framebuffer
    /// only when assets have been produced in linear color space and accurate blending is important
    Correct,
    /// Corrected gamma with hardware dithered output
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

/// Valid display filter options
///
/// See [`filter_options_t`](libdragon_sys::filter_options_t)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterOptions {
    /// All display filters are disabled
    Disabled,

    /// Resize the output image with a bilinear filter
    ///
    /// See [`filter_options_t`](libdragon_sys::filter_options_t_FILTERS_RESAMPLE)
    Resample,

    /// Reconstruct a 32-bit output from dithered 16-bit framebuffer
    Dedither,

    /// Resize the output image with a bilinear filter
    ///
    /// See [`filter_options_t`](libdragon_sys::filter_options_t_FILTERS_RESAMPLE_ANTIALIAS)
    ResampleAntialias,

    /// Resize the output image with a bilinear filter
    ///
    /// See [`filter_options_t`](libdragon_sys::filter_options_t_FILTERS_RESAMPLE_ANTIALIAS)
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

extern "C" {
    pub fn display_init_r(
        res: *const libdragon_sys::resolution_t,
        bit: libdragon_sys::bitdepth_t,
        num_buffers: u32,
        gamma: libdragon_sys::gamma_t,
        filters: libdragon_sys::filter_options_t);
}

/// Initialize the display to a particular resolution and bit depth
///
/// See [`display_init`](libdragon_sys::display_init) for details.
pub fn init(resolution: Resolution, depth: BitDepth, num_buffers: u32, gamma: Gamma, filter: FilterOptions) {
    unsafe {
        display_init_r(&resolution.into() as *const libdragon_sys::resolution_t, 
                       depth.into(), num_buffers, gamma.into(), filter.into())
    }
}

/// Close the display
///
/// See [`display_close`](libdragon_sys::display_close) for details.
pub fn close() { unsafe { libdragon_sys::display_close(); } }

/// Get a display buffer for rendering
///
/// See [`display_get`](libdragon_sys::display_get) for details.
pub fn get<'a>() -> Surface<'a> {
    let ptr = unsafe { libdragon_sys::display_get() };
    Surface::from_ptr(ptr)
}

/// Try getting a display surface
///
/// See [`display_try_get`](libdragon_sys::display_try_get) for details.
pub fn try_get<'a>() -> Option<Surface<'a>> {
    let ptr = unsafe { libdragon_sys::display_try_get() };

    if ptr == ::core::ptr::null_mut() {
        None
    } else {
        Some(Surface::from_ptr(ptr))
    }
}

/// Get the currently configured width of a display in pixels
///
/// See [`display_get_width`](libdragon_sys::display_get_width) for details.
pub fn get_width() -> u32 { unsafe { libdragon_sys::display_get_width() } }

/// Get the currently configured height of a display in pixels
///
/// See [`display_get_height`](libdragon_sys::display_get_height) for details.
pub fn get_height() -> u32 { unsafe { libdragon_sys::display_get_height() } }

/// Get the currently configured bitdepth of a display (in bytes per pixels)
///
/// See [`display_get_bitdepth`](libdragon_sys::display_get_bitdepth) for details.
pub fn get_bitdepth() -> BitDepth { unsafe { libdragon_sys::display_get_bitdepth() }.into() }

/// Get the currently configured number of buffers
///
/// See [`display_get_num_buffers`](libdragon_sys::display_get_num_buffers) for details.
pub fn get_num_buffers() -> u32 { unsafe { libdragon_sys::display_get_num_buffers() } }

/// Get the current number of frames per second being rendered
///
/// See [`display_get_fps`](libdragon_sys::display_get_fps) for details.
pub fn get_fps() -> f32 { unsafe { libdragon_sys::display_get_fps() } }


