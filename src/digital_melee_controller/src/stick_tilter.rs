use std::time::{Instant, Duration};

use crate::analog_axis::AnalogAxis;

pub struct StickTilter {
    is_tilting: bool,
    tilt_level: f64,
    tilt_time: Instant,
}

impl StickTilter {
    pub fn new(tilt_level: f64) -> Self {
        Self {
            tilt_level,
            ..Default::default()
        }
    }

    pub fn tilt_axes(
        &mut self,
        x_axis: &mut AnalogAxis,
        y_axis: &mut AnalogAxis,
        allow_tilt: bool,
        reset_tilt: bool,
        hold_tilt: bool,
    ) {
        let reset_tilt_conditions = x_axis.just_activated() || x_axis.just_crossed_center()
                                 || y_axis.just_activated() || y_axis.just_crossed_center()
                                 || reset_tilt;

        if allow_tilt && reset_tilt_conditions {
            self.tilt_time = Instant::now();
            self.is_tilting = true;
        }

        if self.is_tilting || (allow_tilt && hold_tilt) {
            set_magnitude(x_axis, y_axis, self.tilt_level);

            if Instant::now() - self.tilt_time >= Duration::from_millis(117) {
                self.is_tilting = false;
            }
        }
    }
}

impl Default for StickTilter {
    fn default() -> Self {
        Self {
            is_tilting: false,
            tilt_level: 1.0,
            tilt_time: Instant::now(),
        }
    }
}

pub fn set_magnitude(x_axis: &mut AnalogAxis, y_axis: &mut AnalogAxis, scale_value: f64) {
    scale_axes(x_axis, y_axis, scale_value);
    scale_axes(y_axis, x_axis, scale_value);
}

fn bipolar_max(value: f64, magnitude: f64) -> f64 {
    if value > 0.0 { value.max(magnitude) }
    else if value < 0.0 { value.min(-magnitude) }
    else { 0.0 }
}

fn scale_axes(axis_a: &mut AnalogAxis, axis_b: &mut AnalogAxis, scale_value: f64) {
    let axis_a_magnitude = axis_a.value().abs();
    if axis_a_magnitude > scale_value {
        let scale_factor = scale_value / axis_a_magnitude;
        axis_a.set_value(axis_a.direction() * scale_value);
        axis_b.set_value(bipolar_max(axis_b.value() * scale_factor, axis_b.dead_zone()));
    }
}
