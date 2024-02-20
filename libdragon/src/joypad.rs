
pub const JOYPAD_PORT_COUNT: usize = libdragon_sys::JOYPAD_PORT_COUNT as usize;

pub const JOYPAD_RANGE_N64_STICK_MAX  : u32 = libdragon_sys::JOYPAD_RANGE_N64_STICK_MAX;
pub const JOYPAD_RANGE_GCN_STICK_MAX  : u32 = libdragon_sys::JOYPAD_RANGE_GCN_STICK_MAX;
pub const JOYPAD_RANGE_GCN_CSTICK_MAX : u32 = libdragon_sys::JOYPAD_RANGE_GCN_CSTICK_MAX;
pub const JOYPAD_RANGE_GCN_TRIGGER_MAX: u32 = libdragon_sys::JOYPAD_RANGE_GCN_TRIGGER_MAX;

#[derive(Debug, Copy, Clone)]
pub enum Style {
    None,
    N64,
    GCN,
    Mouse,
}

#[derive(Debug, Copy, Clone)]
pub enum AccessoryType {
    None,
    Unknown,
    ControllerPak,
    RumblePak,
    TransferPak,
    BioSensor,
    SnapStation,
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    StickX,
    StickY,
    CStickX,
    CStickY,
    AnalogL,
    AnalogR,
}

impl Into<libdragon_sys::joypad_axis_t> for Axis {
    fn into(self) -> libdragon_sys::joypad_axis_t {
        match self {
            Axis::StickX  => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_STICK_X,
            Axis::StickY  => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_STICK_Y,
            Axis::CStickX => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_CSTICK_X,
            Axis::CStickY => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_CSTICK_Y,
            Axis::AnalogL => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_ANALOG_L,
            Axis::AnalogR => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_ANALOG_R,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TwoD {
    Stick,
    Dpad,
    C,
    LeftHand,
    RightHand,
    Any,
}

impl Into<libdragon_sys::joypad_2d_t> for TwoD {
    fn into(self) -> libdragon_sys::joypad_2d_t {
        match self {
            TwoD::Stick     => libdragon_sys::joypad_2d_t_JOYPAD_2D_STICK,
            TwoD::Dpad      => libdragon_sys::joypad_2d_t_JOYPAD_2D_DPAD,
            TwoD::C         => libdragon_sys::joypad_2d_t_JOYPAD_2D_C,
            TwoD::LeftHand  => libdragon_sys::joypad_2d_t_JOYPAD_2D_LH,
            TwoD::RightHand => libdragon_sys::joypad_2d_t_JOYPAD_2D_RH,
            TwoD::Any       => libdragon_sys::joypad_2d_t_JOYPAD_2D_ANY,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EightWay {
    None,
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}

impl From<libdragon_sys::joypad_8way_t> for EightWay {
    fn from(value: libdragon_sys::joypad_8way_t) -> Self {
        match value {
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_NONE       => EightWay::None,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_RIGHT      => EightWay::Right,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_UP_RIGHT   => EightWay::UpRight,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_UP         => EightWay::Up,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_UP_LEFT    => EightWay::UpLeft,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_LEFT       => EightWay::Left,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_DOWN_LEFT  => EightWay::DownLeft,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_DOWN       => EightWay::Down,
            libdragon_sys::joypad_8way_t_JOYPAD_8WAY_DOWN_RIGHT => EightWay::DownRight,
            _ => panic!("invalid value from libdragon"),
        }
    }
}

/// See [`joypad_init`](libdragon_sys::joypad_init)
pub fn init() {
    unsafe {
        libdragon_sys::joypad_init();
    }
}

pub fn close() {
    unsafe {
        libdragon_sys::joypad_close();
    }
}

pub fn poll() {
    unsafe {
        libdragon_sys::joypad_poll();
    }
}

pub fn foreach(cb: impl Fn(Port) -> ()) {
    for port in 0..libdragon_sys::JOYPAD_PORT_COUNT as usize {
        cb(Port::get(port));
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Buttons {
    pub a      : bool,
    pub b      : bool,
    pub z      : bool,
    pub start  : bool,
    pub d_up   : bool,
    pub d_down : bool,
    pub d_left : bool,
    pub d_right: bool,

    // GCN only
    pub y      : bool,
    pub x      : bool,
    pub l      : bool,
    pub r      : bool,

    pub c_up   : bool,
    pub c_down : bool,
    pub c_left : bool,
    pub c_right: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Inputs {
    pub btn    : Buttons,
    pub stick_x: i8,
    pub stick_y: i8,

    pub cstick_x: i8,
    pub cstick_y: i8,

    pub analog_l: u8,
    pub analog_r: u8,
}

impl From<libdragon_sys::joypad_buttons_t> for Buttons {
    fn from(value: libdragon_sys::joypad_buttons_t) -> Self {
        let btns = unsafe { &value.__bindgen_anon_1 };
        Self {
            a      : btns.a()       != 0,
            b      : btns.b()       != 0,
            z      : btns.z()       != 0,
            start  : btns.start()   != 0,
            d_up   : btns.d_up()    != 0,
            d_down : btns.d_down()  != 0,
            d_left : btns.d_left()  != 0,
            d_right: btns.d_right() != 0,
            y      : btns.y()       != 0,
            x      : btns.x()       != 0,
            l      : btns.l()       != 0,
            r      : btns.r()       != 0,
            c_up   : btns.c_up()    != 0,
            c_down : btns.c_down()  != 0,
            c_left : btns.c_left()  != 0,
            c_right: btns.c_right() != 0,
        }
    }
}

impl From<libdragon_sys::joypad_inputs_t> for Inputs {
    fn from(value: libdragon_sys::joypad_inputs_t) -> Self {
        Self {
            stick_x    : value.stick_x,
            stick_y    : value.stick_y,
            cstick_x   : value.cstick_x,
            cstick_y   : value.cstick_y,
            analog_l   : value.analog_l,
            analog_r   : value.analog_r,
            btn        : value.btn.into(),
        }
    }
}

// TODO move identifier to joybus module
pub type Identifier = libdragon_sys::joybus_identifier_t;

pub struct Port {
    pub port: usize
}

impl Port {
    pub fn get(port: usize) -> Self {
        assert!(port < JOYPAD_PORT_COUNT);

        Self {
            port: port
        }
    }

    pub fn get_port_1() -> Self { Self::get(0) }
    pub fn get_port_2() -> Self { Self::get(1) }
    pub fn get_port_3() -> Self { Self::get(2) }
    pub fn get_port_4() -> Self { Self::get(3) }

    pub fn is_connected(&self) -> bool {
        unsafe {
            libdragon_sys::joypad_is_connected(self.port as libdragon_sys::joypad_port_t)
        }
    }

    pub fn get_identifier(&self) -> Identifier {
        unsafe {
            libdragon_sys::joypad_get_identifier(self.port as libdragon_sys::joypad_port_t) as Identifier
        }
    }

    pub fn get_style(&self) -> Style {
        let s = unsafe {
            libdragon_sys::joypad_get_style(self.port as libdragon_sys::joypad_port_t)
        };

        match s {
            libdragon_sys::joypad_style_t_JOYPAD_STYLE_NONE  => Style::None,
            libdragon_sys::joypad_style_t_JOYPAD_STYLE_N64   => Style::N64,
            libdragon_sys::joypad_style_t_JOYPAD_STYLE_GCN   => Style::GCN,
            libdragon_sys::joypad_style_t_JOYPAD_STYLE_MOUSE => Style::Mouse,
            _ => todo!("invalid response from joypad_get_style")
        }
    }

    pub fn get_accessory_type(&self) -> AccessoryType {
        let s = unsafe {
            libdragon_sys::joypad_get_accessory_type(self.port as libdragon_sys::joypad_port_t)
        };

        match s {
            libdragon_sys::joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_NONE           => AccessoryType::None,
            libdragon_sys::joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_UNKNOWN        => AccessoryType::Unknown,
            libdragon_sys::joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_CONTROLLER_PAK => AccessoryType::ControllerPak,
            libdragon_sys::joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_RUMBLE_PAK     => AccessoryType::RumblePak,
            libdragon_sys::joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_TRANSFER_PAK   => AccessoryType::TransferPak,
            libdragon_sys::joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_BIO_SENSOR     => AccessoryType::BioSensor,
            libdragon_sys::joypad_accessory_type_t_JOYPAD_ACCESSORY_TYPE_SNAP_STATION   => AccessoryType::SnapStation,
            _ => todo!("invalid response from joypad_get_accessory_type")
        }
    }

    pub fn get_rumble_supported(&self) -> bool {
        unsafe {
            libdragon_sys::joypad_get_rumble_supported(self.port as libdragon_sys::joypad_port_t)
        }
    }

    pub fn get_rumble_active(&self) -> bool {
        unsafe {
            libdragon_sys::joypad_get_rumble_active(self.port as libdragon_sys::joypad_port_t)
        }
    }

    pub fn set_rumble_active(&self, active: bool) {
        unsafe {
            libdragon_sys::joypad_set_rumble_active(self.port as libdragon_sys::joypad_port_t, active);
        }
    }

    pub fn get_inputs(&self) -> Inputs {
        unsafe {
            libdragon_sys::joypad_get_inputs(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    pub fn get_buttons(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    pub fn get_buttons_pressed(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons_pressed(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    pub fn get_buttons_released(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons_released(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    pub fn get_buttons_held(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons_held(self.port as libdragon_sys::joypad_port_t).into()
        }
    }
    
    pub fn get_direction(&self, axes: TwoD) -> EightWay {
        unsafe {
            libdragon_sys::joypad_get_direction(self.port as libdragon_sys::joypad_port_t, axes.into()).into()
        }
    }

    pub fn get_axis_pressed(&self, axis: Axis) -> i32 {
        unsafe {
            libdragon_sys::joypad_get_axis_pressed(self.port as libdragon_sys::joypad_port_t, axis.into()) as i32
        }
    }

    pub fn get_axis_released(&self, axis: Axis) -> i32 {
        unsafe {
            libdragon_sys::joypad_get_axis_released(self.port as libdragon_sys::joypad_port_t, axis.into()) as i32
        }
    }

    pub fn get_axis_held(&self, axis: Axis) -> i32 {
        unsafe {
            libdragon_sys::joypad_get_axis_held(self.port as libdragon_sys::joypad_port_t, axis.into()) as i32
        }
    }
}


