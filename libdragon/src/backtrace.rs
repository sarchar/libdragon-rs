use crate::*;

/// Wrapper around a list of void pointers (a backtrace).
///
/// See [Backtrace::new].
pub struct Backtrace {
    call_stack: Vec<*mut ::core::ffi::c_void>,
}

impl Backtrace {
    /// Walk the stack and return the current call stack.
    ///
    /// `size` is the maximum call stack depth to search
    ///
    /// See [`backtrace`](libdragon_sys::backtrace) for details.
    #[inline]
    pub fn new(size: usize) -> Self {
        let mut ret = Vec::with_capacity(size);
        let r = unsafe {
            libdragon_sys::backtrace(ret.as_mut_ptr(), size as i32)
        };
        ret.truncate(r as usize);
        Self {
            call_stack: ret,
        }
    }

    /// Translate the [Backtrace] to a set of [Symbols].
    ///
    /// See [`backtrace_symbols`](libdragon_sys::backtrace_symbols) for details.
    pub fn symbols<'a>(&'a mut self, size: Option<usize>) -> Symbols<'a> {
        let size = core::cmp::min(size.unwrap_or(self.call_stack.len()), self.call_stack.len());

        let ptrs = unsafe {
            libdragon_sys::backtrace_symbols(self.call_stack.as_mut_ptr(), size as i32)
        };

        Symbols {
            symbols: unsafe { core::slice::from_raw_parts_mut(ptrs, size) },
        }
    }

    /// Translate the [Backtrace] to a se of [BacktraceFrame]s. The provided callback is called
    /// once with each backtrace frame.
    ///
    /// Pass None for `_flags`.  Provide a size if you want fewer frames, otherwise None for all
    /// frames.
    ///
    /// See [`backtrace_symbols_cb`](libdragon_sys::backtrace_symbols_cb) for details.
    pub fn symbols_cb<T: FnMut(BacktraceFrame) -> ()>(&mut self, size: Option<usize>, _flags: Option<()>, cb: T) -> bool {
        let size = core::cmp::min(size.unwrap_or(self.call_stack.len()), self.call_stack.len());

        let cb = Box::new(cb);

        extern "C" fn internal_callback<T: FnMut(BacktraceFrame) -> ()>(ctx: *mut ::core::ffi::c_void, backtrace: *mut libdragon_sys::backtrace_frame_t) {
            let mut cb = unsafe { Box::<T>::from_raw(ctx as *mut _) };
            (cb.as_mut())(BacktraceFrame { ptr: backtrace });
            let _ = Box::leak(cb);
        }

        unsafe {
            let ctx: *mut _ = Box::leak(cb);
            libdragon_sys::backtrace_symbols_cb(self.call_stack.as_mut_ptr(), size as i32, 0u32, Some(internal_callback::<T>), ctx as *mut ::core::ffi::c_void)
        }
    }

    /// Return the size (depth) of this call stack
    #[inline]
    pub fn size(&self) -> usize { self.call_stack.len() }
}

/// Symbols available from [Backtrace::symbols].
///
/// Can access individual elements with [Symbols::symbol] or indexed.
pub struct Symbols<'a> {
    symbols: &'a [*mut ::core::ffi::c_char],
}

impl Symbols<'_> {
    #[inline] pub fn len(&self) -> usize { self.symbols.len() }

    #[inline]
    pub fn symbol(&self, i: usize) -> Result<&str> {
        let c_symbol = unsafe { CStr::from_ptr(self.symbols[i]) };
        Ok(c_symbol.to_str()?)
    }
}

impl core::ops::Index<usize> for Symbols<'_> {
    type Output = str;
    #[inline]
    fn index<'a>(&'a self, i: usize) -> &'a str { self.symbol(i).unwrap() }
}

/// Wrapper around [`backtrace_frame_t`](libdragon_sys::backtrace_frame_t).
pub struct BacktraceFrame {
    ptr: *mut libdragon_sys::backtrace_frame_t,
}

impl BacktraceFrame {
    /// Print a single frame of a backtrace
    ///
    /// See [`backtrace_frame_print`](libdragon_sys::backtrace_frame_print) for details.
    #[inline]
    pub fn print(&mut self, file: &mut dfs::File) {
        unsafe {
            libdragon_sys::backtrace_frame_print(self.ptr, file.fp.unwrap());
        }
    }

    /// Print a single frame of a backtrace, in a compact format
    ///
    /// See [`backtrace_frame_print_compact`](libdragon_sys::backtrace_frame_print_compact) for details.
    #[inline]
    pub fn print_compact(&mut self, file: &mut dfs::File, width: usize) {
        unsafe {
            libdragon_sys::backtrace_frame_print_compact(self.ptr, file.fp.unwrap(), width as i32);
        }
    }

    /// Access [`backtrace_frame_t.addr`](libdragon_sys::backtrace_frame_t::addr)
    #[inline] pub fn addr(&self) -> u32 { unsafe { (*self.ptr).addr } }
    /// Access [`backtrace_frame_t.func`](libdragon_sys::backtrace_frame_t::func)
    #[inline] pub fn func(&self) -> Result<&str> { 
        let c_func = unsafe { CStr::from_ptr((*self.ptr).func) };
        Ok(c_func.to_str()?)
    }
    /// Access [`backtrace_frame_t.func_offset`](libdragon_sys::backtrace_frame_t::func_offset)
    #[inline] pub fn func_offset(&self) -> u32 { unsafe { (*self.ptr).func_offset } }
    /// Access [`backtrace_frame_t.source_file`](libdragon_sys::backtrace_frame_t::source_file)
    #[inline] pub fn source_file(&self) -> Result<&str> { 
        let c_source_file = unsafe { CStr::from_ptr((*self.ptr).source_file) };
        Ok(c_source_file.to_str()?)
    }
    /// Access [`backtrace_frame_t.source_line`](libdragon_sys::backtrace_frame_t::source_line)
    #[inline] pub fn source_line(&self) -> u32 { unsafe { (*self.ptr).source_line as u32 } }
    /// Access [`backtrace_frame_t.is_inline`](libdragon_sys::backtrace_frame_t::is_inline)
    #[inline] pub fn is_inline(&self) -> bool { unsafe { (*self.ptr).is_inline } }
}
