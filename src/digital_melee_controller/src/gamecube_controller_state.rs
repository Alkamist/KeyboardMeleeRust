pub struct AnalogAxis {
    pub value: f64,
    pub previous_value: f64,
    pub dead_zone: f64,
    pub was_active: bool,
    pub high_state_was_first: bool,
}

impl Default for AnalogAxis {
    fn default() -> Self {
        Self {
            value: 0.0,
            previous_value: 0.0,
            dead_zone: 0.2875,
            was_active: false,
            high_state_was_first: true,
        }
    }
}

impl AnalogAxis {
    pub fn direction(&self) -> f64 {
        if self.value > 0.0 { 1.0 }
        else if self.value < 0.0 { -1.0 }
        else { 0.0 }
    }
    pub fn just_crossed_center(&self) -> bool {
        (self.value < 0.0 && self.previous_value >= 0.0)
        || (self.value > 0.0 && self.previous_value <= 0.0)
    }
    pub fn is_active(&self) -> bool { self.value.abs() >= self.dead_zone }
    pub fn just_activated(&self) -> bool { self.just_crossed_center() || self.is_active() && !self.was_active }
    pub fn just_deactivated(&self) -> bool { self.was_active && !self.is_active() }

    pub fn set_value_from_states(&mut self, low: bool, high: bool) {
        if high && !low {
            self.high_state_was_first = true;
        }
        else if low && !high {
            self.high_state_was_first = false;
        }

        let low_and_high = low && high;
        let only_low = low && !high;
        let only_high = high && !low;

        if only_low || (low_and_high && self.high_state_was_first) {
            self.value = -1.0;
        }
        else if only_high || (low_and_high && !self.high_state_was_first) {
            self.value = 1.0;
        }
        else {
            self.value = 0.0;
        }
    }

    pub fn update(&mut self) {
        self.previous_value = self.value;
        self.was_active = self.is_active();
    }
}

#[derive(Default)]
pub struct Button {
    pub is_pressed: bool,
    pub was_pressed: bool,
}

impl Button {
    pub fn new() -> Self {
        Self {
            is_pressed: false,
            was_pressed: false,
        }
    }

    pub fn just_pressed(&self) -> bool { self.is_pressed && !self.was_pressed }
    pub fn just_released(&self) -> bool { self.was_pressed && !self.is_pressed }
    pub fn update(&mut self) {
        self.was_pressed = self.is_pressed;
    }
}

#[derive(Default)]
pub struct GameCubeControllerState {
    pub x_axis: AnalogAxis,
    pub y_axis: AnalogAxis,
    pub c_x_axis: AnalogAxis,
    pub c_y_axis: AnalogAxis,
    pub a_button: Button,
    pub b_button: Button,
    pub x_button: Button,
    pub y_button: Button,
    pub z_button: Button,
    pub l_button: Button,
    pub r_button: Button,
    pub start_button: Button,
    pub d_left_button: Button,
    pub d_right_button: Button,
    pub d_down_button: Button,
    pub d_up_button: Button,
    //l_analog,
    //r_analog,
}

impl GameCubeControllerState {
    pub fn update(&mut self) {
        self.x_axis.update();
        self.y_axis.update();
        self.c_x_axis.update();
        self.c_y_axis.update();
        self.a_button.update();
        self.b_button.update();
        self.x_button.update();
        self.y_button.update();
        self.z_button.update();
        self.l_button.update();
        self.r_button.update();
        self.start_button.update();
        self.d_left_button.update();
        self.d_right_button.update();
        self.d_down_button.update();
        self.d_up_button.update();
    }
}
