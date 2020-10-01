use std::time::{Instant, Duration};

use crate::button::Button;
use crate::analog_axis::AnalogAxis;

pub struct BackdashOutOfCrouchFix {
    x_axis_output: f64,
    down_input: Button,
    left_input: Button,
    right_input: Button,
    delay_backdash: bool,
    backdash_time: Instant,
    backdash_fix_duration: Duration,
}

impl BackdashOutOfCrouchFix {
    pub fn x_axis_output(&self) -> f64 { self.x_axis_output }

    pub fn update_state(
        &mut self,
        x_axis: &AnalogAxis,
        left: bool,
        right: bool,
        down: bool,
    ) {
        self.x_axis_output = x_axis.value();

        self.down_input.update_previous_state();
        self.left_input.update_previous_state();
        self.right_input.update_previous_state();

        self.down_input.set_state(down);
        self.left_input.set_state(left);
        self.right_input.set_state(right);

        if self.down_input.is_pressed() && (self.left_input.just_pressed()
                                         || self.right_input.just_pressed()) {
            self.delay_backdash = true;
            self.backdash_time = Instant::now();
        }

        if self.down_input.just_released() {
            self.delay_backdash = false;
        }

        if self.delay_backdash {
            self.x_axis_output = 0.0;
            if Instant::now() - self.backdash_time >= self.backdash_fix_duration {
                self.delay_backdash = false;
            }
        }
    }
}

impl Default for BackdashOutOfCrouchFix {
    fn default() -> Self {
        Self {
            x_axis_output: 0.0,
            down_input: Default::default(),
            left_input: Default::default(),
            right_input: Default::default(),
            delay_backdash: false,
            backdash_time: Instant::now(),
            backdash_fix_duration: Duration::from_millis(50)
        }
    }
}
