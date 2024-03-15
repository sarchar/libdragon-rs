use crate::*;

/// Wrapper for LibDragon's Sprite structure.
///
/// See [`sprite_t`](libdragon_sys::sprite_t) for details.
pub struct Sprite<'a> {
    ptr: *mut libdragon_sys::sprite_t,
    _data: Option<core::pin::Pin<Box<Vec<u8>>>>,
    phantom: core::marker::PhantomData<&'a u8>,
}

unsafe impl<'a> Send for Sprite<'a> {}
unsafe impl<'a> Sync for Sprite<'a> {}

/// Sprite detail texture information structure
///
/// See [`sprite_detail_t`](libdragon_sys::sprite_detail_t) for details.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SpriteDetail {
    /// Is the detail texture the same as the main surface of the sprite, used for fractal detailing
    use_main_tex: bool,
    /// Blend factor of the detail texture in range of 0 to 1
    blend_factor: f32,
}

impl<'a> Sprite<'a> {
    /// Load a sprite from a filesystem (eg: ROM)
    ///
    /// Rust-specific: the returned sprite 
    ///
    /// See [`sprite_load`](libdragon_sys::sprite_load) for details.
    pub fn load(path: dfs::DfsPathBuf) -> Result<Self> {
        let path_bytes: &[u8] = path.as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let s = unsafe {
            libdragon_sys::sprite_load(cpath.as_ptr())
        };

        Ok(Self {
            ptr: s,
            _data: None,
            phantom: core::marker::PhantomData,
        })
    }

    /// Load a sprite from a buffer
    ///
    /// See [`sprite_load_buf`](libdragon_sys::sprite_load_buf) for details.
    pub fn load_buf<'b, T>(buf: &'b mut [T]) -> Sprite<'b> {
        let ptr = unsafe {
            libdragon_sys::sprite_load_buf(buf.as_mut_ptr() as *mut _, (buf.len() * core::mem::size_of::<T>()) as i32)
        };
        
        Sprite {
            ptr: ptr,
            _data: None,
            phantom: core::marker::PhantomData,
        }
    }

    /// This is not a LibDragon function, and probably not what you want. It's better to use
    /// [Sprite::load_buf].
    pub fn from_data_raw(data: Vec<u8>) -> Self {
        let mut data = Box::pin(data);
        let ptr      = data.as_mut_ptr();
        Self {
            _data: Some(data),
            ptr: ptr as *mut _,
            phantom: core::marker::PhantomData,
        }
    }

    /// Get the sprite texture format
    ///
    /// See [`sprite_get_format`](libdragon_sys::sprite_get_format) for details.
    pub fn get_format(&self) -> surface::TexFormat {
        let f = unsafe { (*self.ptr).__bindgen_anon_1.flags & (libdragon_sys::SPRITE_FLAGS_TEXFORMAT as u8) };
        (f as libdragon_sys::tex_format_t).into()
    }

    /// Create a [surface::Surface] pointing to the full sprite contents.
    ///
    /// See [`sprite_get_pixels`](libdragon_sys::sprite_get_pixels) for details.
    pub fn get_pixels(&self) -> surface::Surface<'a> {
        extern "C" {
            fn sprite_get_pixels_r(surface: *mut libdragon_sys::surface_t, sprite: *mut libdragon_sys::sprite_t);
        }

        // initialize surface_t from libdragon
        let mut surface: core::mem::MaybeUninit<libdragon_sys::surface_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            sprite_get_pixels_r(surface.as_mut_ptr(), self.ptr);
        }

        // pin the structure in place before getting memory address
        let mut backing_instance = Box::pin(unsafe { surface.assume_init() });

        surface::Surface {
            ptr: backing_instance.as_mut().get_mut(),
            _backing_instance: Some(backing_instance),
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Create a [surface::Surface] pointing to the contents of a LOD level.
    ///
    /// See [`sprite_get_lod_pixels`](libdragon_sys::sprite_get_lod_pixels) for details.
    pub fn get_lod_pixels(&self, num_level: u32) -> surface::Surface<'a> {
        extern "C" {
            fn sprite_get_lod_pixels_r(surface: *mut libdragon_sys::surface_t, sprite: *mut libdragon_sys::sprite_t, num_level: i32);
        }

        // initialize surface_t from libdragon
        let mut surface: core::mem::MaybeUninit<libdragon_sys::surface_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            sprite_get_lod_pixels_r(surface.as_mut_ptr(), self.ptr, num_level as i32);
        }

        // pin the structure in place before getting memory address
        let mut backing_instance = Box::pin(unsafe { surface.assume_init() });

        surface::Surface {
            ptr: backing_instance.as_mut().get_mut(),
            _backing_instance: Some(backing_instance),
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Create a [surface::Surface] pointing to the contents of a detail texture.
    ///
    /// See [`sprite_get_detail_pixels`](libdragon_sys::sprite_get_detail_pixels) for details.
    pub fn get_detail_pixels(&self, info: Option<&mut SpriteDetail>, parms: rdpq::TexParms) -> surface::Surface<'a> {
        extern "C" {
            fn sprite_get_detail_pixels_r(surface: *mut libdragon_sys::surface_t, sprite: *mut libdragon_sys::sprite_t, 
                                          info: *mut libdragon_sys::sprite_detail_t, infoparms: *mut libdragon_sys::rdpq_texparms_t);
        }

        // initialize surface_t from libdragon
        let mut surface: core::mem::MaybeUninit<libdragon_sys::surface_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            sprite_get_detail_pixels_r(surface.as_mut_ptr(), self.ptr, 
                                       info.map_or(::core::ptr::null_mut(), |p| core::mem::transmute(p)),
                                       &mut parms.into());
        }

        // pin the structure in place before getting memory address
        let mut backing_instance = Box::pin(unsafe { surface.assume_init() });

        surface::Surface {
            ptr: backing_instance.as_mut().get_mut(),
            _backing_instance: Some(backing_instance),
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Create a [surface::Surface] pointing to the contents of a detail texture.
    ///
    /// See [`sprite_get_tile`](libdragon_sys::sprite_get_tile) for details.
    pub fn get_tile(&self, h: i32, v: i32) -> surface::Surface<'a> {
        extern "C" {
            fn sprite_get_tile_r(surface: *mut libdragon_sys::surface_t, sprite: *mut libdragon_sys::sprite_t, h: i32, v: i32);
        }

        // initialize surface_t from libdragon
        let mut surface: core::mem::MaybeUninit<libdragon_sys::surface_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            sprite_get_tile_r(surface.as_mut_ptr(), self.ptr, h, v);
        }

        // pin the structure in place before getting memory address
        let mut backing_instance = Box::pin(unsafe { surface.assume_init() });

        surface::Surface {
            ptr: backing_instance.as_mut().get_mut(),
            _backing_instance: Some(backing_instance),
            needs_free: false,
            is_const: false,
            phantom: core::marker::PhantomData,
        }
    }

    /// Access the sprite palette (if any)
    ///
    /// See [`sprite_get_palette`](libdragon_sys::sprite_get_palette)
    pub fn get_palette(&self) -> rdpq::TlutPalette<'a> {
        unsafe {
            libdragon_sys::sprite_get_palette(self.ptr) as rdpq::TlutPalette
        }
    }

    /// Get a copy of the RDP texparms, optionally stored within the sprite.
    ///
    /// See [`sprite_get_texparms`](libdragon_sys::sprite_get_texparms) for details.
    pub fn get_texparms(&self) -> rdpq::TexParms {
        let mut parms: core::mem::MaybeUninit<libdragon_sys::rdpq_texparms_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            libdragon_sys::sprite_get_texparms(self.ptr, parms.as_mut_ptr());
        }

        unsafe { core::mem::transmute(parms.assume_init()) }
    }

    /// Return the number of LOD levels stored within the sprite (including the main image).
    ///
    /// See [`sprite_get_lod_count`](libdragon_sys::sprite_get_lod_count) for details.
    pub fn get_lod_count(&self) -> u32 { unsafe { libdragon_sys::sprite_get_lod_count(self.ptr) as u32 } }

    /// Return true if the sprite fits in TMEM without splitting
    ///
    /// See [`sprite_fits_tmem`](libdragon_sys::sprite_fits_tmem) for details.
    pub fn fits_tmem(&self) -> bool { unsafe { libdragon_sys::sprite_fits_tmem(self.ptr) } }

    /// Access `sprite_t.width`
    pub fn width(&self) -> u16 { unsafe { (*self.ptr).width } }
    /// Access `sprite_t.height`
    pub fn height(&self) -> u16 { unsafe { (*self.ptr).height } }
    /// Access `sprite_t.hslices`
    pub fn hslices(&self) -> u8 { unsafe { (*self.ptr).hslices } }
    /// Access `sprite_t.vslices`
    pub fn vslices(&self) -> u8 { unsafe { (*self.ptr).vslices } }

    /// Internal use
    #[doc(hidden)]
    pub(crate) fn as_const_sprite_s(&self) -> *const libdragon_sys::sprite_s {
        self.ptr as *const _
    }
}

impl<'a> Drop for Sprite<'a> {
    /// Deallocate a sprite
    ///
    /// See [`sprite_free`](libdragon_sys::sprite_free) for details.
    fn drop(&mut self) {
        unsafe {
            libdragon_sys::sprite_free(self.ptr);
        }
    }
}
