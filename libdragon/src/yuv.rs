use crate::*;

use surface::Surface;

/// Initialize the YUV conversion library
#[inline]
pub fn init() { unsafe { libdragon_sys::yuv_init(); } }

/// Shutdown the YUV conversion library
#[inline]
pub fn close() { unsafe { libdragon_sys::yuv_close(); } }

/// Re-expose [`yuv_colorspace_t`](libdragon_sys::yuv_colorspace_t) as [Colorspace]
pub type Colorspace = libdragon_sys::yuv_colorspace_t;
/// Wrapper around [`yuv_blitter_t`](libdragon_sys::yuv_blitter_t).
#[repr(C)]
pub struct Blitter<'a> {
    b: libdragon_sys::yuv_blitter_t,
    phantom: core::marker::PhantomData<&'a u8>
}

/// See [`YUV_BT601_TV`](libdragon_sys::YUV_BT601_TV)
pub static BT601_TV  : &Colorspace = unsafe { core::mem::transmute(&libdragon_sys::YUV_BT601_TV  ) };
/// See [`YUV_BT601_FULL`](libdragon_sys::YUV_BT601_FULL)
pub static BT601_FULL: &Colorspace = unsafe { core::mem::transmute(&libdragon_sys::YUV_BT601_FULL) };
/// See [`YUV_BT709_TV`](libdragon_sys::YUV_BT709_TV)
pub static BT709_TV  : &Colorspace = unsafe { core::mem::transmute(&libdragon_sys::YUV_BT709_TV  ) };
/// See [`YUV_BT709_FULL`](libdragon_sys::YUV_BT709_FULL)
pub static BT709_FULL: &Colorspace = unsafe { core::mem::transmute(&libdragon_sys::YUV_BT709_FULL) };

/// Calculate coefficients for a new YUV colorspace.
///
/// See [`yuv_new_colorspace`](libdragon_sys::yuv_new_colorspace) for details.
#[inline]
#[allow(non_snake_case)]
pub fn new_colorspace(Kr: f32, Kb: f32, y0: i32, yrange: i32, crange: i32) -> Colorspace {
    extern "C" {
        fn yuv_new_colorspace_r(res: *mut libdragon_sys::yuv_colorspace_t,
                                Kr: f32, Kb: f32, y0: ::core::ffi::c_int,
                                yrange: ::core::ffi::c_int,
                                crange: ::core::ffi::c_int);
    }
    let mut tmp: core::mem::MaybeUninit<libdragon_sys::yuv_colorspace_t> = core::mem::MaybeUninit::uninit();
    unsafe { 
        yuv_new_colorspace_r(tmp.as_mut_ptr(), Kr, Kb, y0, yrange, crange); 
        core::mem::transmute(tmp.assume_init())
    }
}

pub trait ColorspaceImpl {
    /// Convert a single YUV pixel into RGB.
    ///
    /// See [`yuv_to_rgb`](libdragon_sys::yuv_to_rgb) for details.
    #[inline]
    fn to_rgb(&self, y: u8, u: u8, v: u8) -> graphics::Color {
        graphics::Color {
            c: unsafe {
                libdragon_sys::yuv_to_rgb(y, u, v, self as *const Self as *const _)
            },
        }
    }

    /// Create a YUV blitter optimized for rendering multiple frames with some possible
    /// transformation.
    ///
    /// See [`yuv_blitter_new`](libdragon_sys::yuv_blitter_new) for details.
    #[inline]
    fn blitter_new(&self, video_width: u32, video_height: u32, x0: f32, y0: f32, parms: rdpq::BlitParms) -> Blitter {
        assert!(::core::mem::size_of::<libdragon_sys::yuv_blitter_t>() == 4, "if this struct gets larger, write a C wrapper");
        let parms: libdragon_sys::rdpq_blitparms_t = parms.into();
        Blitter {
            b: unsafe { 
                libdragon_sys::yuv_blitter_new(video_width as i32, video_height as i32, x0, y0, 
                                               &parms, self as *const Self as *const _)
            },
            phantom: core::marker::PhantomData,
        }
    }

    /// Blit a 3-planes YUV frame into the current RDP framebuffer.
    ///
    /// See [`yuv_tex_blit`](libdragon_sys::yuv_tex_blit) for details.
    #[inline]
    fn tex_blit(&self, yp: &mut Surface, up: &mut Surface, vp: &mut Surface, x0: f32, y0: f32, parms: rdpq::BlitParms) {
        let parms: libdragon_sys::rdpq_blitparms_t = parms.into();
        unsafe {
            libdragon_sys::yuv_tex_blit(yp.ptr, up.ptr, vp.ptr, x0, y0, &parms, self as *const Self as *const _);
        }
    }
}

impl ColorspaceImpl for Colorspace {}

/// YUV blitter zoom configuration
pub enum Zoom {
    /// Zoom the frame, keeping the aspect ratio
    KeepAspect,
    /// Zoom the frame, irrespective of aspect ratio
    Full,
    /// Do not zoom the frame to fit the output buffer
    None
}

impl Into<u32> for Zoom {
    fn into(self) -> u32 {
        match self {
            Zoom::KeepAspect => libdragon_sys::yuv_zoom_t_YUV_ZOOM_KEEP_ASPECT,
            Zoom::Full => libdragon_sys::yuv_zoom_t_YUV_ZOOM_FULL,
            Zoom::None => libdragon_sys::yuv_zoom_t_YUV_ZOOM_NONE,
        }
    }
}

/// YUV blitter output buffer alignment
pub enum Align {
    /// Align to the center of the output buffer
    Center,
    /// Align to left/top of the output buffer
    Min,
    /// Align to right/bottom of the output buffer
    Max
}

impl Into<u32> for Align {
    fn into(self) -> u32 {
        match self {
            Align::Center => libdragon_sys::yuv_align_t_YUV_ALIGN_CENTER,
            Align::Min => libdragon_sys::yuv_align_t_YUV_ALIGN_MIN,
            Align::Max => libdragon_sys::yuv_align_t_YUV_ALIGN_MAX,
        }
    }
}

/// YUV full motion video blitter configuration.
///
/// See [`yuv_fmv_parms_t`](libdragon_sys::yuv_fmv_parms_t) for details.
pub struct FmvParms<'a> {
    /// Color space to use during conversion (default: [BT601_TV]),
    pub cs: &'a Colorspace,
    /// Frame horizontal alignment to the output buffer (default: [Align::Center])
    pub halign: Align,
    /// Frame vertical alignment to the output buffer (default: [Align::Center])
    pub valign: Align,
    /// Frame zooming algorithm to use (default: [Zoom::KeepAspect])
    pub zoom: Zoom,
    /// Color to use to clear the reset of the output buffer (default: 0x000000FF).
    pub bkg_color: graphics::Color,
}

impl Default for FmvParms<'_> {
    fn default() -> Self {
        Self {
            cs: BT601_TV,
            halign: Align::Center,
            valign: Align::Center,
            zoom: Zoom::KeepAspect,
            bkg_color: graphics::Color::default(),
        }
    }
}

impl Into<libdragon_sys::yuv_fmv_parms_t> for FmvParms<'_> {
    fn into(self) -> libdragon_sys::yuv_fmv_parms_t {
        libdragon_sys::yuv_fmv_parms_t {
            cs: unsafe { ::core::mem::transmute(self.cs) },
            halign: self.halign.into(),
            valign: self.valign.into(),
            zoom: self.zoom.into(),
            bkg_color: self.bkg_color.c,
        }
    }
}

/// Create a YUV blitter optimized for FMV drawing (full screen movie player)
///
/// See [`yuv_blitter_new_fmv`](libdragon_sys::yuv_blitter_new_fmv) for details.
#[inline]
pub fn blitter_new_fmv<'a>(video_width: u32, video_height: u32, screen_width: u32, screen_height: u32, parms: FmvParms<'a>) -> Blitter<'a> {
    assert!(::core::mem::size_of::<libdragon_sys::yuv_blitter_t>() == 4, "if this struct gets larger, write a C wrapper");
    let parms: libdragon_sys::yuv_fmv_parms_t = parms.into();
    Blitter {
        b: unsafe { 
            libdragon_sys::yuv_blitter_new_fmv(video_width as i32, video_height as i32,
                                               screen_width as i32, screen_height as i32, &parms)
        },
        phantom: core::marker::PhantomData,
    }
}

impl Blitter<'_> {
    /// Perform a YUV blit using a blitter, with the specified surfaces
    ///
    /// See [`yuv_blitter_run`](libdragon_sys::yuv_blitter_run) for details.
    #[inline]
    pub fn run(&mut self, yp: &mut Surface, up: &mut Surface, vp: &mut Surface) {
        unsafe {
            libdragon_sys::yuv_blitter_run(self as *mut Self as *mut _, yp.ptr, up.ptr, vp.ptr);
        }
    }
}

impl Drop for Blitter<'_> {
    #[inline]
    fn drop(&mut self) { unsafe { libdragon_sys::yuv_blitter_free(self as *mut Self as *mut _); } }
}
