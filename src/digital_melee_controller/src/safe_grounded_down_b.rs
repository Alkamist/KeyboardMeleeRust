use std::time::{Instant, Duration};

use crate::button::Button;
use crate::analog_axis::AnalogAxis;

pub struct SafeGroundedDownB {
    x_axis_output: f64,
    y_axis_output: f64,
    b_input: Button,
    is_doing_safe_b: bool,
    safe_b_time: Instant,
    safe_b_duration: Duration,
}

impl SafeGroundedDownB {
    pub fn x_axis_output(&self) -> f64 { self.x_axis_output }
    pub fn y_axis_output(&self) -> f64 { self.y_axis_output }

    pub fn update_state(
        &mut self,
        x_axis: &AnalogAxis,
        y_axis: &AnalogAxis,
        b: bool,
        down: bool,
        up: bool,
    ) {
        self.x_axis_output = x_axis.value();
        self.y_axis_output = y_axis.value();

        self.b_input.update_previous_state();
        self.b_input.set_state(b);

        if self.b_input.just_pressed() && (down || up) {
            self.is_doing_safe_b = true;
            self.safe_b_time = Instant::now();
        }

        if self.is_doing_safe_b {
            if Instant::now() - self.safe_b_time < self.safe_b_duration {
                self.x_axis_output = x_axis.direction() * 0.5875;
                self.y_axis_output = y_axis.direction() * 0.6;
            }
            else {
                self.is_doing_safe_b = false;
            }
        }
    }
}

impl Default for SafeGroundedDownB {
    fn default() -> Self {
        Self {
            x_axis_output: 0.0,
            y_axis_output: 0.0,
            b_input: Default::default(),
            is_doing_safe_b: false,
            safe_b_time: Instant::now(),
            safe_b_duration: Duration::from_millis(25),
        }
    }
}
