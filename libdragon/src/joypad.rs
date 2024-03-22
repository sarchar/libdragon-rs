
/// Count of Joypad ports
pub const JOYPAD_PORT_COUNT: usize = libdragon_sys::JOYPAD_PORT_COUNT as usize;

/// Maximum range of a Nintendo 64 controller analog stick
pub const JOYPAD_RANGE_N64_STICK_MAX  : u32 = libdragon_sys::JOYPAD_RANGE_N64_STICK_MAX;
/// Maximum range of a GameCube controller analog stick
pub const JOYPAD_RANGE_GCN_STICK_MAX  : u32 = libdragon_sys::JOYPAD_RANGE_GCN_STICK_MAX;
/// Maximum range of a GameCube controller analog C-stick
pub const JOYPAD_RANGE_GCN_CSTICK_MAX : u32 = libdragon_sys::JOYPAD_RANGE_GCN_CSTICK_MAX;
/// Maximum range of a GameCube controller analog trigger
pub const JOYPAD_RANGE_GCN_TRIGGER_MAX: u32 = libdragon_sys::JOYPAD_RANGE_GCN_TRIGGER_MAX;

/// Joypad Styles
#[derive(Debug, Copy, Clone)]
pub enum Style {
    /// Unsupported Joypad Style
    None,
    /// Nintendo 64 Joypad Style
    ///
    /// See [JOYPAD_STYLE_N64](libdragon_sys::joypad_style_t_JOYPAD_STYLE_N64)
    N64,
    /// GameCube Joypad Style
    ///
    /// See [JOYPAD_STYLE_GCN](libdragon_sys::joypad_style_t_JOYPAD_STYLE_GCN)
    GCN,
    /// Mouse Joypad Style
    ///
    /// See [JOYPAD_STYLE_MOUSE](libdragon_sys::joypad_style_t_JOYPAD_STYLE_MOUSE)
    Mouse,
}

/// Joypad Axis enumeration values.
///
/// See [`joypad_axis_t`](libdragon_sys::joypad_axis_t)
#[derive(Debug, Copy, Clone)]
pub enum Axis {
    /// Joypad stick X axis.
    StickX,
    /// Joypad stick Y axis.
    StickY,
    /// Joypad C-stick X axis.
    CStickX,
    /// Joypad C-stick Y axis.
    CStickY,
    /// Joypad analog L trigger axis.
    AnalogL,
    /// Joypad analog R trigger axis.
    AnalogR,
}

impl From<Axis> for libdragon_sys::joypad_axis_t {
    fn from(v: Axis) -> Self {
        match v {
            Axis::StickX  => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_STICK_X,
            Axis::StickY  => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_STICK_Y,
            Axis::CStickX => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_CSTICK_X,
            Axis::CStickY => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_CSTICK_Y,
            Axis::AnalogL => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_ANALOG_L,
            Axis::AnalogR => libdragon_sys::joypad_axis_t_JOYPAD_AXIS_ANALOG_R,
        }
    }
}

/// Joypad 2D axes enumeration
///
/// See [`joypad_2d_t`](libdragon_sys::joypad_2d_t)
#[derive(Debug, Copy, Clone)]
pub enum TwoD {
    /// Analog stick 2D axis.
    Stick,
    /// D-Pad 2D axis.
    Dpad,
    /// C buttons 2D axes.
    C,
    /// Left-Hand 2D axes: Analog stick or D-pad.
    LeftHand,
    /// Right-Hand 2D axes: Analog stick or C buttons.
    RightHand,
    /// Any 2D axes: Analog stick, D-Pad, or C buttons.
    Any,
}

impl From<TwoD> for libdragon_sys::joypad_2d_t {
    fn from(v: TwoD) -> Self {
        match v {
            TwoD::Stick     => libdragon_sys::joypad_2d_t_JOYPAD_2D_STICK,
            TwoD::Dpad      => libdragon_sys::joypad_2d_t_JOYPAD_2D_DPAD,
            TwoD::C         => libdragon_sys::joypad_2d_t_JOYPAD_2D_C,
            TwoD::LeftHand  => libdragon_sys::joypad_2d_t_JOYPAD_2D_LH,
            TwoD::RightHand => libdragon_sys::joypad_2d_t_JOYPAD_2D_RH,
            TwoD::Any       => libdragon_sys::joypad_2d_t_JOYPAD_2D_ANY,
        }
    }
}

/// Joypad 8-way directional enumeration
#[derive(Debug, Copy, Clone)]
pub enum EightWay {
    /// 8-way no direction.
    None,
    /// 8-way right direction.
    Right,
    /// 8-way up-right direction.
    UpRight,
    /// 8-way up direction.
    Up,
    /// 8-way up-left direction.
    UpLeft,
    /// 8-way left direction.
    Left,
    /// 8-way down-left direction.
    DownLeft,
    /// 8-way down direction.
    Down,
    /// 8-way down-right direction.
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

/// Initialize the Joypad subsystem.
///
/// See [`joypad_init`](libdragon_sys::joypad_init) for details.
pub fn init() { unsafe { libdragon_sys::joypad_init(); } }

/// Close the Joypad subsystem.
///
/// See [`joypad_close`](libdragon_sys::joypad_close) for details.
pub fn close() { unsafe { libdragon_sys::joypad_close(); } }

/// Fetch the current Joypad input state.
///
/// See [`joypad_poll`](libdragon_sys::joypad_poll) for details.
pub fn poll() { unsafe { libdragon_sys::joypad_poll(); } }

/// Rust-specific: convenience function to iterate through all Joypad ports
///
/// See the `joypadtest` example application for a demonstration
pub fn foreach(cb: impl Fn(Port) -> ()) {
    for port in 0..libdragon_sys::JOYPAD_PORT_COUNT as usize {
        cb(Port::get(port));
    }
}

/// Joypad Buttons
#[derive(Debug, Clone, Copy, Default)]
pub struct Buttons {
    /// State of the A button
    pub a      : bool,
    /// State of the B button
    pub b      : bool,
    /// State of the Z button
    pub z      : bool,
    /// State of the Start button
    pub start  : bool,
    /// State of the D-Pad Up button
    pub d_up   : bool,
    /// State of the D-Pad Down button
    pub d_down : bool,
    /// State of the D-Pad Left button
    pub d_left : bool,
    /// State of the D-Pad Right button
    pub d_right: bool,

    /// State of the Y button (GCN only)
    pub y      : bool,
    /// State of the X button (GCN only)
    pub x      : bool,

    /// State of the L button
    pub l      : bool,
    /// State of the R button
    pub r      : bool,

    /// State of the C-Up button.
    ///
    /// For GameCube controllers, the value will be
    /// emulated based on the C-Stick Y axis position.
    pub c_up   : bool,

    /// State of the C-Down button.
    ///
    /// For GameCube controllers, the value will be
    /// emulated based on the C-Stick Y axis position.
    pub c_down : bool,

    /// State of the C-Left button.
    ///
    /// For GameCube controllers, the value will be
    /// emulated based on the C-Stick X axis position.
    pub c_left : bool,

    /// State of the C-Right button.
    ///
    /// For GameCube controllers, the value will be
    /// emulated based on the C-Stick X axis position.
    pub c_right: bool,
}

/// Joypad Inputs Unified State Structure
///
/// See [`joypad_inputs_s`](libdragon_sys::joypad_inputs_s)
#[derive(Debug, Clone, Copy, Default)]
pub struct Inputs {
    /// Structure containing digital button inputs state.
    pub btn    : Buttons,

    /// Position of the analog joystick X axis. (-127, +127)
    ///
    /// See [`joypad_inputs_s.stick_x`](libdragon_sys::joypad_inputs_s::stick_x)
    pub stick_x: i8,

    /// Position of the analog joystick Y axis. (-127, +127)
    ///
    /// See [`joypad_inputs_s.stick_y`](libdragon_sys::joypad_inputs_s::stick_y)
    pub stick_y: i8,

    /// Position of the analog "C-Stick" X axis. (-127, +127)
    ///
    /// See [`joypad_inputs_s.cstick_x`](libdragon_sys::joypad_inputs_s::cstick_x)
    pub cstick_x: i8,

    /// Position of the analog "C-Stick" Y axis. (-127, +127)
    ///
    /// See [`joypad_inputs_s.cstick_y`](libdragon_sys::joypad_inputs_s::cstick_y)
    pub cstick_y: i8,

    /// Position of the analog L trigger. (0, 255)
    ///
    /// See [`joypad_inputs_s.analog_l`](libdragon_sys::joypad_inputs_s::analog_l)
    pub analog_l: u8,

    /// Position of the analog R trigger. (0, 255)
    ///
    /// See [`joypad_inputs_s.analog_r`](libdragon_sys::joypad_inputs_s::analog_r)
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

/// Rust-specific: [Port] is a wrapper object around LibDragon's `joypad_*` functions
/// that take a [`joypad_port_t`](libdragon_sys::joypad_port_t) as its first parameter.
///
/// First, get a Port instance with [Port::get] or [Port::get_port_1] through [Port::get_port_4].
/// Then, the remainder of the functions in the struct map directly to LibDragon functions.
pub struct Port {
    pub port: usize
}

impl Port {
    /// Return a [Port] instance for a given port index
    pub fn get(port: usize) -> Self {
        assert!(port < JOYPAD_PORT_COUNT);

        Self {
            port: port
        }
    }

    /// Convenience functions
    pub fn get_port_1() -> Self { Self::get(0) }
    pub fn get_port_2() -> Self { Self::get(1) }
    pub fn get_port_3() -> Self { Self::get(2) }
    pub fn get_port_4() -> Self { Self::get(3) }

    /// Whether a Joybus device is plugged in to a Joypad port.
    ///
    /// See [`joypad_is_connected`](libdragon_sys::joypad_is_connected)
    pub fn is_connected(&self) -> bool {
        unsafe {
            libdragon_sys::joypad_is_connected(self.port as libdragon_sys::joypad_port_t)
        }
    }

    /// Get the Joybus device identifier for a Joypad port.
    ///
    /// See [`joypad_get_identifier`](libdragon_sys::joypad_get_identifier)
    pub fn get_identifier(&self) -> Identifier {
        unsafe {
            libdragon_sys::joypad_get_identifier(self.port as libdragon_sys::joypad_port_t) as Identifier
        }
    }

    /// Get the Joypad style for a Joypad port.
    ///
    /// See [`joypad_get_style`](libdragon_sys::joypad_get_style)
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

    /// Is rumble supported for a Joypad port?
    ///
    /// See [`joypad_get_rumble_supported`](libdragon_sys::joypad_get_rumble_supported)
    pub fn get_rumble_supported(&self) -> bool {
        unsafe {
            libdragon_sys::joypad_get_rumble_supported(self.port as libdragon_sys::joypad_port_t)
        }
    }

    /// Is rumble active for a Joypad port?
    ///
    /// See [`joypad_get_rumble_active`](libdragon_sys::joypad_get_rumble_active)
    pub fn get_rumble_active(&self) -> bool {
        unsafe {
            libdragon_sys::joypad_get_rumble_active(self.port as libdragon_sys::joypad_port_t)
        }
    }

    /// Activate or deactivate rumble on a Joypad port.
    ///
    /// See [`joypad_set_rumble_active`](libdragon_sys::joypad_set_rumble_active)
    pub fn set_rumble_active(&self, active: bool) {
        unsafe {
            libdragon_sys::joypad_set_rumble_active(self.port as libdragon_sys::joypad_port_t, active);
        }
    }

    /// Get the current Joypad inputs state for a Joypad port.
    ///
    /// See [`joypad_get_inputs`](libdragon_sys::joypad_get_inputs)
    pub fn get_inputs(&self) -> Inputs {
        unsafe {
            libdragon_sys::joypad_get_inputs(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    /// Get the current Joypad buttons state for a Joypad port.
    ///
    /// See [`joypad_get_buttons`](libdragon_sys::joypad_get_buttons)
    pub fn get_buttons(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    /// Get the current Joypad buttons that were pressed since the last
    /// time Joypads were read for a Joypad port.
    ///
    /// See [`joypad_get_buttons_pressed`](libdragon_sys::joypad_get_buttons_pressed)
    pub fn get_buttons_pressed(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons_pressed(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    /// Get the Joypad buttons that were released since the last
    /// time Joypads were read for a Joypad port.
    ///
    /// See [`joypad_get_buttons_released`](libdragon_sys::joypad_get_buttons_released)
    pub fn get_buttons_released(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons_released(self.port as libdragon_sys::joypad_port_t).into()
        }
    }

    /// Get the Joypad buttons that are held down since the last
    /// time Joypads were read for a Joypad port.
    ///
    /// See [`joypad_get_buttons_held`](libdragon_sys::joypad_get_buttons_held)
    pub fn get_buttons_held(&self) -> Buttons {
        unsafe {
            libdragon_sys::joypad_get_buttons_held(self.port as libdragon_sys::joypad_port_t).into()
        }
    }
    
    /// Get the 8-way direction for a Joypad port's direction axes.
    ///
    /// See [`joypad_get_direction`](libdragon_sys::joypad_get_direction)
    pub fn get_direction(&self, axes: TwoD) -> EightWay {
        unsafe {
            libdragon_sys::joypad_get_direction(self.port as libdragon_sys::joypad_port_t, axes.into()).into()
        }
    }

    /// Get the direction for a "press" of an axis on a Joypad port.
    ///
    /// See [`joypad_get_axis_pressed`](libdragon_sys::joypad_get_axis_pressed)
    pub fn get_axis_pressed(&self, axis: Axis) -> i32 {
        unsafe {
            libdragon_sys::joypad_get_axis_pressed(self.port as libdragon_sys::joypad_port_t, axis.into()) as i32
        }
    }

    /// Get the direction for a "release" of an axis on a Joypad port.
    ///
    /// See [`joypad_get_axis_released`](libdragon_sys::joypad_get_axis_released)
    pub fn get_axis_released(&self, axis: Axis) -> i32 {
        unsafe {
            libdragon_sys::joypad_get_axis_released(self.port as libdragon_sys::joypad_port_t, axis.into()) as i32
        }
    }

    /// Get the direction for an axis is held on a Joypad port.
    ///
    /// See [`joypad_get_axis_held`](libdragon_sys::joypad_get_axis_held)
    pub fn get_axis_held(&self, axis: Axis) -> i32 {
        unsafe {
            libdragon_sys::joypad_get_axis_held(self.port as libdragon_sys::joypad_port_t, axis.into()) as i32
        }
    }
}

/// Base trait for other modules to extend [Port]
pub trait BasePort {
    /// Return the [Port]'s controller index
    fn port(&self) -> usize;
}

impl BasePort for Port {
    /// Return the [Port]'s controller index
    fn port(&self) -> usize { self.port }
}
