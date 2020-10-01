extern crate libloading as lib;

use std::os::raw::{
    c_uchar,
    c_long,
    c_uint,
};

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Default, Copy, Clone)]
struct JoystickPositionV2 {
    /// JOYSTICK_POSITION
    bDevice: c_uchar, // Index of device. 1-based.
    wThrottle: c_long,
    wRudder: c_long,
    wAileron: c_long,
    wAxisX: c_long,
    wAxisY: c_long,
    wAxisZ: c_long,
    wAxisXRot: c_long,
    wAxisYRot: c_long,
    wAxisZRot: c_long,
    wSlider: c_long,
    wDial: c_long,
    wWheel: c_long,
    wAxisVX: c_long,
    wAxisVY: c_long,
    wAxisVZ: c_long,
    wAxisVBRX: c_long,
    wAxisVBRY: c_long,
    wAxisVBRZ: c_long,
    lButtons: c_long, // 32 buttons: 0x00000001 means button1 is pressed, 0x80000000 -> button32 is pressed
    bHats: c_uint, // Lower 4 bits: HAT switch or 16-bit of continuous HAT switch
    bHatsEx1: c_uint, // Lower 4 bits: HAT switch or 16-bit of continuous HAT switch
    bHatsEx2: c_uint, // Lower 4 bits: HAT switch or 16-bit of continuous HAT switch
    bHatsEx3: c_uint, // Lower 4 bits: HAT switch or 16-bit of continuous HAT switch

    /// JOYSTICK_POSITION_V2 Extension
    lButtonsEx1: c_long, // Buttons 33-64
    lButtonsEx2: c_long, // Buttons 65-96
    lButtonsEx3: c_long, // Buttons 97-128
}

#[derive(Debug, Copy, Clone)]
pub enum VJoyAxis {
    X,
    Y,
    Z,
    XRotation,
    YRotation,
    Slider0,
}

pub struct VJoyDevice {
    device_id: u32,
    vjoy: lib::Library,
    state: JoystickPositionV2,
}

impl VJoyDevice {
    pub fn new(device_id: u32, path: &str) -> Self {
        let output = Self {
            device_id,
            vjoy: lib::Library::new(path).expect("Could not load vJoyInterface.dll."),
            state: JoystickPositionV2::default(),
        };
        if output.v_joy_is_enabled() {
            output.acquire_vjoy_device();
        }
        output
    }

    pub fn set_button(&mut self, button_id: u32, state: bool) {
        let bit_index: u32 = button_id - 1;
        if state {
            self.state.lButtons = self.state.lButtons | (1 << bit_index);
        }
        else {
            self.state.lButtons = self.state.lButtons & !(1 << bit_index);
        }
    }

    pub fn set_axis(&mut self, axis: VJoyAxis, value: f64) {
        match axis {
            VJoyAxis::X => self.state.wAxisX = Self::get_scaled_axis_value(value),
            VJoyAxis::Y => self.state.wAxisY = Self::get_scaled_axis_value(value),
            VJoyAxis::Z => self.state.wAxisZ = Self::get_scaled_axis_value(value),
            VJoyAxis::XRotation => self.state.wAxisXRot = Self::get_scaled_axis_value(value),
            VJoyAxis::YRotation => self.state.wAxisYRot = Self::get_scaled_axis_value(value),
            VJoyAxis::Slider0 => self.state.wSlider = Self::get_scaled_slider_value(value),
        }
    }

    pub fn send_inputs(&self) {
        unsafe {
            let func = self.vjoy.get::<lib::Symbol<unsafe extern fn(u32, JoystickPositionV2) -> u32>>(b"UpdateVJD")
                .expect(&format!("Could not update vJoy device {}.", self.device_id));
            if func(self.device_id, self.state) == 0 {
                panic!("Could not update vJoy device {}.", self.device_id);
            };
        }
    }

    fn get_scaled_axis_value(value: f64) -> c_long {
        let scaled_value = 0.5 * (0.626 * value + 1.0);
        return (scaled_value * (0x8000 as f64)) as c_long;
    }

    fn get_scaled_slider_value(value: f64) -> c_long {
        return (value * (0x8000 as f64)) as c_long;
    }

    fn v_joy_is_enabled(&self) -> bool {
        unsafe {
            if let Ok(func) = self.vjoy.get::<lib::Symbol<unsafe extern fn() -> u32>>(b"vJoyEnabled") {
                if func() == 1 { true } else { false }
            }
            else {
                false
            }
        }
    }

    fn acquire_vjoy_device(&self) {
        unsafe {
            let func = self.vjoy.get::<lib::Symbol<unsafe extern fn(u32) -> u32>>(b"AcquireVJD")
                .expect(&format!("Could not acquire vJoy device {}.", self.device_id));
            if func(self.device_id) == 0 {
                panic!("Could not acquire vJoy device {}.", self.device_id);
            };
        }
    }

    fn free_vjoy_device(&self) {
        unsafe {
            let func = self.vjoy.get::<lib::Symbol<unsafe extern fn(u32)>>(b"RelinquishVJD")
                .expect(&format!("Could not free vJoy device {}.", self.device_id));
            func(self.device_id);
        }
    }
}

impl Drop for VJoyDevice {
    fn drop(&mut self) {
        self.free_vjoy_device();
    }
}
