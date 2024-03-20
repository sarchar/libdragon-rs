use crate::*;

use surface::Surface;

/// Wrapper around [`mpeg2_t`](libdragon_sys::mpeg2).
pub struct Mpeg2 {
    m: libdragon_sys::mpeg2_t,
}

impl Mpeg2 {
    /// Create a new [Mpeg2] object.
    ///
    /// See [`mpeg2_open`](libdragon_sys::mpeg2_open) for details.
    pub fn new(path: &dfs::DfsPathBuf, output_width: usize, output_height: usize) -> Self {
        let path_bytes: &[u8] = path.as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let mut tmp: core::mem::MaybeUninit<libdragon_sys::mpeg2_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            libdragon_sys::mpeg2_open(tmp.as_mut_ptr(), cpath.as_ptr() as *const _, output_width as i32, output_height as i32);
        }

        Self {
            m: unsafe { tmp.assume_init() },
        }
    }

    /// See [`mpeg2_get_framerate`](libdragon_sys::mpeg2_get_framerate) for details.
    #[inline] pub fn get_framerate(&mut self) -> f32 { unsafe { libdragon_sys::mpeg2_get_framerate(&mut self.m as *mut _) } }
    /// See [`mpeg2_next_frame`](libdragon_sys::mpeg2_next_frame) for details.
    #[inline] pub fn next_frame(&mut self) -> bool { unsafe { libdragon_sys::mpeg2_next_frame(&mut self.m as *mut _) } }
    /// See [`mpeg2_draw_frame`](libdragon_sys::mpeg2_draw_frame) for details.
    #[inline] pub fn draw_frame(&mut self, surface: &mut Surface) { unsafe { libdragon_sys::mpeg2_draw_frame(&mut self.m as *mut _, surface.ptr) } }
    /// See [`mpeg2_rewind`](libdragon_sys::mpeg2_rewind) for details.
    #[inline] pub fn rewind(&mut self) { unsafe { libdragon_sys::mpeg2_rewind(&mut self.m as *mut _) } }
}

impl Drop for Mpeg2 {
    /// See [`mpeg2_close`](libdragon_sys::mpeg2_close) for details.
    #[inline]
    fn drop(&mut self) {
        unsafe { libdragon_sys::mpeg2_close(&mut self.m as *mut _); }
    }
}
