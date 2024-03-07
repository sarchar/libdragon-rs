
/// Console Render Modes
#[derive(Copy, Clone, Debug)]
pub enum RenderMode {
    /// Manual Rendering
    Manual = 0,
    /// Automatic Rendering
    Automatic = 1,
}

/// The console width in characters
pub const CONSOLE_WIDTH     : u32 = libdragon_sys::CONSOLE_WIDTH;
/// The console height in characters
pub const CONSOLE_HEIGHT    : u32 = libdragon_sys::CONSOLE_HEIGHT;
/// Tab width
pub const TAB_WIDTH         : u32 = libdragon_sys::TAB_WIDTH;
/// Padding from the left and right ends of the screen in pixels
pub const HORIZONTAL_PADDING: u32 = libdragon_sys::HORIZONTAL_PADDING;
/// Padding from the top and bottom ends of the screen in pixels
pub const VERTICAL_PADDING  : u32 = libdragon_sys::VERTICAL_PADDING;

/// Initialize the console
///
/// See [`console_init`](libdragon_sys::console_init) for details.
pub fn init() {
    unsafe {
        libdragon_sys::console_init();        
    }
}

/// Close the console
///
/// See [`console_close`](libdragon_sys::console_close) for details.
pub fn close() {
    unsafe {
        libdragon_sys::console_close();
    }
}

/// Send console output to debug channel
///
/// See [`console_set_debug`](libdragon_sys::console_set_debug) for details.
pub fn set_debug(debug: bool) {
    unsafe {
        libdragon_sys::console_set_debug(debug);
    }
}

/// Set the console rendering mode
///
/// See [`console_set_render_mode`](libdragon_sys::console_set_render_mode) for details.
pub fn set_render_mode(mode: RenderMode) {
    unsafe {
        libdragon_sys::console_set_render_mode((mode as u8) as ::core::ffi::c_int);
    }
}

/// Clear the console
///
/// See [`console_clear`](libdragon_sys::console_clear) for details.
pub fn clear() { unsafe { libdragon_sys::console_clear(); } }

pub fn render() { unsafe { libdragon_sys::console_render(); } }
