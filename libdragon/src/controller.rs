
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct ControllerData {
    pub A      : bool,
    pub B      : bool,
    pub Z      : bool,
    pub start  : bool,
    pub up     : bool,
    pub down   : bool,
    pub left   : bool,
    pub right  : bool,
    pub L      : bool,
    pub R      : bool,
    pub C_up   : bool,
    pub C_down : bool,
    pub C_left : bool,
    pub C_right: bool,
    pub x      : i8,
    pub y      : i8,
}

impl From<libdragon_sys::SI_condat> for ControllerData {
    fn from(value: libdragon_sys::SI_condat) -> Self {
        let state = unsafe { &value.__bindgen_anon_1.__bindgen_anon_2 };
        Self {
            A      : state.A()       != 0,
            B      : state.B()       != 0,
            Z      : state.Z()       != 0,
            start  : state.start()   != 0,
            up     : state.up()      != 0,
            down   : state.down()    != 0,
            left   : state.left()    != 0,
            right  : state.right()   != 0,
            L      : state.L()       != 0,
            R      : state.R()       != 0,
            C_up   : state.C_up()    != 0,
            C_down : state.C_down()  != 0,
            C_left : state.C_left()  != 0,
            C_right: state.C_right() != 0,
            x      : state.x() as i8,
            y      : state.y() as i8,
        }
    }
}


pub fn init() {
    unsafe {
        libdragon_sys::controller_init();
    }
}

pub fn scan() {
    unsafe {
        libdragon_sys::controller_scan();
    }
}

pub fn get_present() -> [bool; 4] {
    let mask = unsafe {
        libdragon_sys::get_controllers_present() as u32
    };

    let p0 = (mask & libdragon_sys::CONTROLLER_1_INSERTED) != 0;
    let p1 = (mask & libdragon_sys::CONTROLLER_2_INSERTED) != 0;
    let p2 = (mask & libdragon_sys::CONTROLLER_3_INSERTED) != 0;
    let p3 = (mask & libdragon_sys::CONTROLLER_4_INSERTED) != 0;

    [p0, p1, p2, p3]
}

pub fn get_keys_down() -> [ControllerData; 4] {
    let keys_down_sys = unsafe {
        libdragon_sys::get_keys_down()
    };

    keys_down_sys.c.map(|v| v.into())
}

pub fn get_keys_up() -> [ControllerData; 4] {
    let keys_down_sys = unsafe {
        libdragon_sys::get_keys_up()
    };

    keys_down_sys.c.map(|v| v.into())
}

pub fn get_keys_held() -> [ControllerData; 4] {
    let keys_down_sys = unsafe {
        libdragon_sys::get_keys_held()
    };

    keys_down_sys.c.map(|v| v.into())
}

pub fn get_keys_pressed() -> [ControllerData; 4] {
    let keys_down_sys = unsafe {
        libdragon_sys::get_keys_pressed()
    };

    keys_down_sys.c.map(|v| v.into())
}

pub fn rumble_start(i: usize) {
    unsafe {
        libdragon_sys::rumble_start(i as ::std::os::raw::c_int);
    }
}

pub fn rumble_stop(i: usize) {
    unsafe {
        libdragon_sys::rumble_stop(i as ::std::os::raw::c_int);
    }
}
