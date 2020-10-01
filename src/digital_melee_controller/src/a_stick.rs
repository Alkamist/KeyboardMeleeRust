use std::time::{Instant, Duration};

use crate::delayed_button::DelayedButton;
use crate::analog_axis::AnalogAxis;

pub struct AStick {
    output_state: bool,
    x_axis_output: f64,
    y_axis_output: f64,
    output_x_axis: AnalogAxis,
    output_y_axis: AnalogAxis,
    output_button: DelayedButton,
    neutral_button: DelayedButton,
    left_button: DelayedButton,
    right_button: DelayedButton,
    down_button: DelayedButton,
    up_button: DelayedButton,
    axis_hold_duration: Duration,
    activation_time: Instant,
}

impl AStick {
    pub fn output_state(&self) -> bool { self.output_state }
    pub fn x_axis_output(&self) -> f64 { self.x_axis_output }
    pub fn y_axis_output(&self) -> f64 { self.y_axis_output }

    pub fn update_state(
        &mut self,
        x_axis: &AnalogAxis,
        y_axis: &AnalogAxis,
        neutral: bool,
        left: bool,
        right: bool,
        down: bool,
        up: bool,
    ) {
        self.x_axis_output = x_axis.value();
        self.y_axis_output = y_axis.value();

        self.output_button.update_previous_state();
        self.neutral_button.update_previous_state();
        self.left_button.update_previous_state();
        self.right_button.update_previous_state();
        self.down_button.update_previous_state();
        self.up_button.update_previous_state();

        self.neutral_button.set_state(neutral);
        self.left_button.set_state(left);
        self.right_button.set_state(right);
        self.down_button.set_state(down);
        self.up_button.set_state(up);

        let turn_around_left_tilt = self.left_button.just_pressed() && x_axis.value() > 0.0;
        let turn_around_right_tilt = self.right_button.just_pressed() && x_axis.value() < 0.0;

        if turn_around_left_tilt || turn_around_right_tilt {
            self.activation_time = Instant::now();
            self.output_button.set_delay(Duration::from_millis(34));
            self.axis_hold_duration = Duration::from_millis(67);
        }
        else if self.left_button.just_pressed() || self.right_button.just_pressed() {
            self.activation_time = Instant::now();
            self.output_button.set_delay(Duration::from_millis(17));
            self.axis_hold_duration = Duration::from_millis(67);
        }

        if self.down_button.just_pressed() || self.up_button.just_pressed() {
            self.activation_time = Instant::now();
            self.output_button.set_delay(Duration::from_millis(0));
            self.axis_hold_duration = Duration::from_millis(67);
        }

        if self.neutral_button.just_pressed() {
            self.activation_time = Instant::now();
            self.output_button.set_delay(Duration::from_millis(0));
            self.axis_hold_duration = Duration::from_millis(25);
        }

        self.output_button.set_state(self.neutral_button.is_pressed()
                                  || self.left_button.is_pressed()
                                  || self.right_button.is_pressed()
                                  || self.down_button.is_pressed()
                                  || self.up_button.is_pressed());

        self.output_x_axis.set_value_from_states(
            self.left_button.is_pressed(),
            self.right_button.is_pressed(),
        );
        self.output_y_axis.set_value_from_states(
            self.down_button.is_pressed(),
            self.up_button.is_pressed(),
        );

        if Instant::now() - self.activation_time <= self.axis_hold_duration {
            let should_bias_x = !(self.left_button.is_pressed()
                                  || self.right_button.is_pressed()
                                  || self.neutral_button.is_pressed());
            let mut x_bias = 0.0;
            if should_bias_x {
                x_bias = 0.35 * x_axis.direction();
            }
            self.x_axis_output = self.output_x_axis.value() * 0.6 + x_bias;

            let should_bias_y = y_axis.is_active()
                                && !(self.down_button.is_pressed()
                                     || self.up_button.is_pressed()
                                     || self.neutral_button.is_pressed());
            let mut y_bias = 0.0;
            if should_bias_y {
                y_bias = 0.5 * y_axis.direction();
            }
            self.y_axis_output = self.output_y_axis.value() * 0.6 + y_bias;
        }

        self.output_state = self.output_button.is_pressed();
    }
}

impl Default for AStick {
    fn default() -> Self {
        Self {
            output_state: false,
            x_axis_output: 0.0,
            y_axis_output: 0.0,
            output_x_axis: Default::default(),
            output_y_axis: Default::default(),
            output_button: Default::default(),
            neutral_button: DelayedButton::new(Duration::from_millis(0), Duration::from_millis(34)),
            left_button: DelayedButton::new(Duration::from_millis(0), Duration::from_millis(50)),
            right_button: DelayedButton::new(Duration::from_millis(0), Duration::from_millis(50)),
            down_button: DelayedButton::new(Duration::from_millis(0), Duration::from_millis(34)),
            up_button: DelayedButton::new(Duration::from_millis(0), Duration::from_millis(34)),
            axis_hold_duration: Duration::from_millis(0),
            activation_time: Instant::now(),
        }
    }
}
