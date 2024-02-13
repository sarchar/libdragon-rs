
#[derive(Copy, Clone, Debug)]
pub enum RenderMode {
    Manual = 0,
    Automatic = 1,
}

pub const CONSOLE_WIDTH     : u32 = libdragon_sys::CONSOLE_WIDTH;
pub const CONSOLE_HEIGHT    : u32 = libdragon_sys::CONSOLE_HEIGHT;
pub const TAB_WIDTH         : u32 = libdragon_sys::TAB_WIDTH;
pub const HORIZONTAL_PADDING: u32 = libdragon_sys::HORIZONTAL_PADDING;
pub const VERTICAL_PADDING  : u32 = libdragon_sys::VERTICAL_PADDING;

pub fn init() {
    unsafe {
        libdragon_sys::console_init();        
    }
}

pub fn close() {
    unsafe {
        libdragon_sys::console_close();
    }
}

pub fn set_debug(debug: bool) {
    unsafe {
        libdragon_sys::console_set_debug(debug);
    }
}

pub fn set_render_mode(mode: RenderMode) {
    unsafe {
        libdragon_sys::console_set_render_mode((mode as u8) as ::std::os::raw::c_int);
    }
}

pub fn clear() {
    unsafe {
        libdragon_sys::console_clear();
    }
}

pub fn render() {
    unsafe {
        libdragon_sys::console_render();
    }
}
