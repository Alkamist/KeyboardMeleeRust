use std::time::{Instant, Duration};

use crate::button::Button;

pub struct DelayedButton {
    is_pressed: bool,
    was_pressed: bool,
    should_press: bool,
    delay: Duration,
    min_hold_time: Duration,
    output_press_time: Instant,
    input_press_time: Instant,
    input_button: Button,
}

impl DelayedButton {
    pub fn new(delay: Duration, min_hold_time: Duration) -> Self {
        Self {
            delay,
            min_hold_time,
            ..Default::default()
        }
    }

    pub fn set_state(&mut self, state: bool) {
        self.input_button.set_state(state);

        if self.input_button.just_pressed() {
            self.should_press = true;
            self.input_press_time = Instant::now();
        }

        if self.should_press && Instant::now() - self.input_press_time >= self.delay {
            self.output_press_time = Instant::now();
            self.should_press = false;
            self.is_pressed = true;
        }

        let stop_press = self.is_pressed
                      && !self.input_button.is_pressed()
                      && Instant::now() - self.output_press_time >= self.min_hold_time;

        if stop_press {
            self.is_pressed = false;
        }
    }

    pub fn set_delay(&mut self, delay: Duration) { self.delay = delay; }

    pub fn is_pressed(&self) -> bool { self.is_pressed }
    pub fn was_pressed(&self) -> bool { self.was_pressed }
    pub fn just_pressed(&self) -> bool { self.is_pressed && !self.was_pressed }
    pub fn just_released(&self) -> bool { self.was_pressed && !self.is_pressed }
    pub fn update_previous_state(&mut self) {
        self.input_button.update_previous_state();
        self.was_pressed = self.is_pressed;
    }
}

impl Default for DelayedButton {
    fn default() -> Self {
        Self {
            is_pressed: false,
            was_pressed: false,
            should_press: false,
            delay: Duration::from_millis(0),
            min_hold_time: Duration::from_millis(0),
            output_press_time: Instant::now(),
            input_press_time: Instant::now(),
            input_button: Default::default(),
        }
    }
}