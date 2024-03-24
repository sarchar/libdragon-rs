use crate::*;

/// Pixel format enum
///
/// See [`tex_format_t`](libdragon_sys::tex_format_t)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TexFormat {
    /// Placeholder for no format defined
    None,
    /// Format RGBA 5551 (16-bit)
    Rgba16,
    /// Format RGBA 8888 (32-bit)
    Rgba32,
    /// Format YUV2 4:2:2 (data interleaved as YUV2)
    Yuv16,
    /// Format CI4: color index 4-bit (paletted, 2 indices per byte)
    Ci4,
    /// Format CI8: color index 8-bit (paletted, 1 index per byte)
    Ci8,
    /// Format IA4: 3-bit intensity + 1-bit alpha (4-bit per pixel)
    Ia4,
    /// Format IA8: 4-bit intensity + 4-bit alpha (8-bit per pixel)
    Ia8,
    /// Format IA16: 8-bit intensity + 8-bit alpha (16-bit per pixel)
    Ia16,
    /// Format I4: 4-bit intensity (4-bit per pixel)
    I4,
    /// Format I8: 8-bit intensity (8-bit per pixel)
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

impl From<TexFormat> for libdragon_sys::tex_format_t {
    fn from(v: TexFormat) -> Self {
        match v {
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

impl TexFormat {
    /// Extract the depth (number of bits per pixel) from a [TexFormat] (eg: `TexFormat::Rgba16` => 16)
    #[inline]
    pub fn bitdepth(self) -> i32 {
        let fmt = Into::<libdragon_sys::tex_format_t>::into(self) as i32;
        4 << (fmt & 0x03)
    }

    /// Convert the specified number of pixels to bytes.
    #[inline]
    pub fn pix2bytes(self, pixels: i32) -> i32 {
        let fmt = Into::<libdragon_sys::tex_format_t>::into(self) as i32;
        ((pixels << ((fmt & 3) + 2)) + 7) >> 3
    }

    /// Convert the specified number of bytes to pixels.
    #[inline]
    pub fn bytes2pix(self, bytes: i32) -> i32 {
        let fmt = Into::<libdragon_sys::tex_format_t>::into(self) as i32;
        (bytes << 1) >> (fmt & 3)
    }

    /// Return the name of the texture format as a string (for debugging purposes)
    ///
    /// Rust note: the enum derives [Debug], but this yields the C formatted enum name
    pub fn name(self) -> String { 
        let fmt = Into::<libdragon_sys::tex_format_t>::into(self);
        let ptr = unsafe { libdragon_sys::tex_format_name(fmt) };
        let c_str = unsafe { CStr::from_ptr(ptr as *const i8) };
        String::from_utf8_lossy(c_str.to_bytes()).to_string()
    }
}

/// A wrapper for LibDragon's `surface_t` struct.
///
/// See [`surface_t`](libdragon_sys::surface_t)
pub struct Surface<'a> {
    pub(crate) ptr: *mut libdragon_sys::surface_t,
    pub(crate) _backing_instance: Option<core::pin::Pin<Box<libdragon_sys::surface_t>>>,
    pub(crate) needs_free: bool,
    pub(crate) is_const: bool,
    pub(crate) phantom: core::marker::PhantomData<&'a u8>,
}

impl<'a> Surface<'a> {
    pub(crate) fn from_ptr(ptr: *mut libdragon_sys::surface_t) -> Self {
        Self {
            ptr: ptr,
            _backing_instance: None,
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    // For LibDragon functions that return a reference to constant surfaces
    pub(crate) fn from_const_ptr(ptr: *const libdragon_sys::surface_t) -> Self {
        Self {
            ptr: ptr as *mut libdragon_sys::surface_t,
            _backing_instance: None,
            needs_free: false,
            is_const: true,
            phantom: core::marker::PhantomData,
        }
    }

    /// Initialize a Surface with the provided buffer
    ///
    /// ex. 
    /// ```rust
    /// let mut buffer = [0u8; 1024];
    /// let surf = surface::Surface::make(&mut buffer, surface::TexFormat::Rgba16, 32, 32, 32);
    /// ```
    ///
    /// See [`surface_make`](libdragon_sys::surface_make) for details.
    pub fn make<'b, T>(buffer: &'b mut [T], format: TexFormat, width: u32, height: u32, stride: u32) -> Surface<'b> {
        let surf = libdragon_sys::surface_t {
            flags: Into::<libdragon_sys::tex_format_t>::into(format) as u16,
            width: width as u16,
            height: height as u16,
            stride: stride as u16,
            buffer: buffer.as_ptr() as *mut _,
        };

        let mut backing_instance = Box::pin(surf);

        Surface {
            ptr: backing_instance.as_mut().get_mut(),
            _backing_instance: Some(backing_instance),
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Initialize a Surface with the provided linear buffer
    ///
    /// See [`surface_make_linear`](libdragon_sys::surface_make_linear) for details.
    pub fn make_linear<'b, T>(buffer: &'b mut [T], format: TexFormat, width: u32, height: u32) -> Surface<'b> {
        Self::make(buffer, format, width, height, format.pix2bytes(width as i32) as u32)
    }
 
    /// Initialize a Surface, pointing to a rectangular portion of this surface
    ///
    /// See [`surface_make_sub`](libdragon_sys::surface_make_sub) for details.
    pub fn make_sub(&self, x0: u32, y0: u32, width: u32, height: u32) -> Self {
        extern "C" {
            fn surface_make_sub_r(ret: *mut libdragon_sys::surface_t, parent: *const libdragon_sys::surface_t, 
                                  x0: u32, y0: u32, width: u32, height: u32);
        }

        let mut backing_surface = Box::pin(unsafe {
            core::mem::MaybeUninit::<libdragon_sys::surface_t>::zeroed().assume_init()
        });

        unsafe {
            surface_make_sub_r(backing_surface.as_mut().get_mut() as *mut _, self.ptr, x0, y0, width, height);
        }

        Self {
            ptr: backing_surface.as_mut().get_mut(),
            _backing_instance: Some(backing_surface),
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Allocate a new surface in memory
    ///
    /// Rust-specific: you do not need to manually free this surface. The memory will
    /// be freed when the object is dropped.
    ///
    /// See [`surface_alloc`](libdragon_sys::surface_alloc)
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
            _backing_instance: Some(backing_surface),
            needs_free: true,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Create a placeholder surface, that can be used during rdpq block recording.
    ///
    /// See [`surface_make_placeholder`](libdragon_sys::surface_make_placeholder)
    pub fn make_placeholder<'b>(index: i32, format: TexFormat, width: u32, height: u32, stride: u32) -> Surface<'b> {
        let surf = libdragon_sys::surface_t {
            flags: (Into::<libdragon_sys::tex_format_t>::into(format) as u16) | ((index as u16) << 8),
            width: width as u16,
            height: height as u16,
            stride: stride as u16,
            buffer: ::core::ptr::null_mut() as *mut _,
        };

        let mut backing_instance = Box::pin(surf);

        Surface {
            ptr: backing_instance.as_mut().get_mut(),
            _backing_instance: Some(backing_instance),
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Create a linear placeholder surface, that can be used during rdpq block recording.
    ///
    /// See [`surface_make_placeholder_linear`](libdragon_sys::surface_make_placeholder_linear) for details.
    pub fn make_placeholder_linear<'b, T>(index: i32, format: TexFormat, width: u32, height: u32) -> Surface<'b> {
        Self::make_placeholder(index, format, width, height, format.pix2bytes(width as i32) as u32)
    }

    /// Returns the pixel format of a surface
    ///
    /// See [`surface_get_format`](libdragon_sys::surface_get_format)
    pub fn get_format(&self) -> TexFormat { ((self.flags() & 0x1F) as libdragon_sys::tex_format_t).into() }

    /// Checks whether this surface owns the buffer that it contains
    ///
    /// See [`surface_has_owned_buffer`](libdragon_sys::surface_has_owned_buffer)
    pub fn has_owned_buffer(&self) -> bool { 
        let valid_buf = unsafe { self.buffer::<u8>() != ::core::ptr::null_mut() };
        valid_buf && (self.flags() & 0x20) != 0
    }

    /// Returns the lookup index of a placeholder surface
    ///
    /// See [`surface_get_placeholder_index`](libdragon_sys::surface_get_placeholder_index)
    pub fn get_placeholder_index(&self) -> i32 { ((self.flags() >> 8) & 0x0F) as i32 }

    /// Access [`surface_t.flags`](libdragon_sys::surface_t::flags)
    pub fn flags(&self) -> u16 { unsafe { (*self.ptr).flags } }
    /// Access [`surface_t.width`](libdragon_sys::surface_t::width)
    pub fn width(&self) -> u16 { unsafe { (*self.ptr).width } }
    /// Access [`surface_t.height`](libdragon_sys::surface_t::height)
    pub fn height(&self) -> u16 { unsafe { (*self.ptr).height } }
    /// Access [`surface_t.stride`](libdragon_sys::surface_t::stride)
    pub fn stride(&self) -> u16 { unsafe { (*self.ptr).stride } }
    /// Unsafe access to [`surface_t.buffer`](libdragon_sys::surface_t::buffer)
    pub unsafe fn buffer_mut<T>(&mut self) -> *mut T { assert!(!self.is_const, "immutable const surface"); unsafe { (*self.ptr).buffer as *mut _ } }
    /// Unsafe access to [`surface_t.buffer`](libdragon_sys::surface_t::buffer)
    pub unsafe fn buffer<T>(&self) -> *const T { unsafe { (*self.ptr).buffer as *const _ } }

    /// Display a buffer on the screen
    ///
    /// See [`display_show`](libdragon_sys::display_show)
    pub fn show(&self) { unsafe { libdragon_sys::display_show(self.ptr); } }
}

impl<'a> Drop for Surface<'a> {
    /// Free the backing memory of a surface
    ///
    /// See [`surface_free`](libdragon_sys::surface_free) for details.
    fn drop(&mut self) {
        if self.needs_free {
            unsafe {
                libdragon_sys::surface_free(self.ptr);
            }
        }
    }
}
