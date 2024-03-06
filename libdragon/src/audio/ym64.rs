
use crate::*;

/// Ym64 structure. This is a wrapper around LibDragon's `ym64player_t`.
///
/// See [`struct ym64player_t`](libdragon_sys::ym64player_t) for details.
pub struct Ym64 {
    ptr: *mut libdragon_sys::ym64player_t,
    backing_instance: Option<core::pin::Pin<Box<libdragon_sys::ym64player_t>>>,
    song_info: libdragon_sys::ym64player_songinfo_t,
}

impl Ym64 {
    /// Open a YM64 file for playback
    ///
    /// See [`ym64player_open`](libdragon_sys::ym64player_open) for details.
    pub fn open(path: dfs::DfsPathBuf) -> Result<Self> {
        let path_bytes: &[u8] = path.as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let mut obj: core::mem::MaybeUninit<libdragon_sys::ym64player_t> = core::mem::MaybeUninit::uninit();
        let mut info: core::mem::MaybeUninit<libdragon_sys::ym64player_songinfo_t> = core::mem::MaybeUninit::uninit();
        unsafe {
            libdragon_sys::ym64player_open(obj.as_mut_ptr(), cpath.as_ptr(), info.as_mut_ptr());
        }

        let mut backing_instance = Box::pin(unsafe { obj.assume_init() });
        Ok(Self {
            ptr: backing_instance.as_mut().get_mut(),
            backing_instance: Some(backing_instance),
            song_info: unsafe { info.assume_init() },
        })
    }

    /// Return the number of channels used in the mixer for playback.
    ///
    /// See [`ym64player_num_channels`](libdragon_sys::ym64player_num_channels) for details.
    pub fn num_channels(&self) -> i32 {
        unsafe {
            libdragon_sys::ym64player_num_channels(self.ptr)
        }
    }

    /// Start playback of a YM file
    ///
    /// See [`ym64player_play`](libdragon_sys::ym64player_play) for details.
    pub fn play(&mut self, first_ch: i32) {
        unsafe {
            libdragon_sys::ym64player_play(self.ptr, first_ch);
        }
    }

    /// Stop YM playback
    ///
    /// See [`ym64player_stop`](libdragon_sys::ym64player_stop) for details.
    pub fn stop(&mut self) {
        unsafe {
            libdragon_sys::ym64player_stop(self.ptr);
        }
    }

    /// Read the total duration in the YM module.
    ///
    /// See [`ym64player_duration`](libdragon_sys::ym64player_duration) for details.
    pub fn duration(&self, len: &mut i32, secs: &mut f32) {
        unsafe {
            libdragon_sys::ym64player_duration(self.ptr, len as *mut _, secs as *mut _);
        }
    }

    /// Read the current position of the YM module.
    ///
    /// See [`ym64player_tell`](libdragon_sys::ym64player_tell) for details.
    pub fn tell(&self, pos: &mut i32, secs: &mut f32) {
        unsafe {
            libdragon_sys::ym64player_tell(self.ptr, pos as *mut _, secs as *mut _);
        }
    }

    /// Seek to a specific position in the YM module.
    ///
    /// See [`ym64player_seek`](libdragon_sys::ym64player_seek) for details.
    pub fn seek(&mut self, pos: i32) -> bool {
        unsafe {
            libdragon_sys::ym64player_seek(self.ptr, pos)
        }
    }

    /// Access the libxm context
    ///
    /// See [`struct ym64player_t`](libdragon_sys::ym64player_t) for details.
    pub fn ctx(&mut self) -> *mut libdragon_sys::xm_context_t {
        todo!("need an XmContext wrapper")
    }

    /// Access the [Waveform](crate::audio::mixer::Waveform) for this Ym64.
    ///
    /// See [`struct ym64player_t`](libdragon_sys::ym64player_t::wave) for details.
    pub fn wave(&mut self) -> audio::mixer::Waveform {
        audio::mixer::Waveform::from_ptr(unsafe { &mut (*self.ptr).wave })
    }

    /// See [`struct ym64player_t`](libdragon_sys::ym64player_t) for details.
    pub fn nframes(&self) -> u32 { unsafe { (*self.ptr).nframes as u32 } }
    pub fn curframe(&self) -> i32 { unsafe { (*self.ptr).curframe } }
    pub fn first_ch(&self) -> i32 { unsafe { (*self.ptr).first_ch } }

    /// See [`string ym64player_songinfo_t`](libdragon_sys::ym64player_songinfo_t)
    pub fn name(&self) -> Result<&str> {
        let name_str = unsafe {
            CStr::from_ptr(self.song_info.name.as_ptr() as *const i8)
        };
        Ok(name_str.to_str()?)
    }

    pub fn author(&self) -> Result<&str> {
        let author_str = unsafe {
            CStr::from_ptr(self.song_info.author.as_ptr() as *const i8)
        };
        Ok(author_str.to_str()?)
    }
    
    pub fn comment(&self) -> Result<&str> {
        let comment_str = unsafe {
            CStr::from_ptr(self.song_info.comment.as_ptr() as *const i8)
        };
        Ok(comment_str.to_str()?)
    }
}

impl Drop for Ym64 {
    /// Uses [`ym64player_close`](libdragon_sys::ym64player_close).
    fn drop(&mut self) {
        if let Some(_) = core::mem::replace(&mut self.backing_instance, None) {
            unsafe {
                libdragon_sys::ym64player_close(self.ptr);
                self.ptr = core::ptr::null_mut();
            }
        }
    }
}
