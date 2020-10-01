use std::time::{Instant, Duration};

use crate::button::Button;
use crate::analog_axis::AnalogAxis;

pub struct AirDodgeLogic {
    air_dodge_input: Button,
    air_dodge_time: Instant,
    is_air_dodging: bool,
    x_level_long: f64,
    y_level_long: f64,
    x_level_medium: f64,
    y_level_medium: f64,
    x_level_short: f64,
    y_level_short: f64,
}

impl AirDodgeLogic {
    pub fn update_axes(&mut self,
        x_axis: &mut AnalogAxis,
        y_axis: &mut AnalogAxis,
        air_dodge: bool,
        shorten: bool,
    ) {
        self.air_dodge_input.update_previous_state();
        self.air_dodge_input.set_state(air_dodge);

        let is_left = x_axis.is_active() && x_axis.value() < 0.0;
        let is_right = x_axis.is_active() && x_axis.value() > 0.0;
        let is_down = y_axis.is_active() && y_axis.value() < 0.0;
        let is_up = y_axis.is_active() && y_axis.value() > 0.0;
        let is_sideways = (is_left || is_right) && !is_down;
        let is_diagonal = (is_left || is_right) && (is_down || is_up);

        let air_dodge_short = is_diagonal && shorten;
        let air_dodge_medium = is_sideways && shorten;
        let air_dodge_long = is_sideways && !shorten;

        if self.air_dodge_input.just_pressed() {
            self.is_air_dodging = true;
            self.air_dodge_time = Instant::now();
        }

        if self.is_air_dodging && !is_up {
            if Instant::now() - self.air_dodge_time < Duration::from_millis(51) {
                if air_dodge_long {
                    x_axis.set_value(x_axis.direction() * self.x_level_long);
                    y_axis.set_value(self.y_level_long);
                }
                else if air_dodge_medium {
                    x_axis.set_value(x_axis.direction() * self.x_level_medium);
                    y_axis.set_value(self.y_level_medium);
                }
                else if air_dodge_short {
                    x_axis.set_value(x_axis.direction() *  self.x_level_short);
                    y_axis.set_value(self.y_level_short);
                }
                else if !is_down {
                    y_axis.set_value(-0.3);
                }
            }
            else {
                self.is_air_dodging = false;
            }
        }
    }
}

impl Default for AirDodgeLogic {
    fn default() -> Self {
        Self {
            air_dodge_input: Default::default(),
            air_dodge_time: Instant::now(),
            is_air_dodging: false,
            x_level_long: 0.925,
            y_level_long: -0.35,
            x_level_medium: 0.8125,
            y_level_medium: -0.575,
            x_level_short: 0.5,
            y_level_short: -0.85,
        }
    }
}
