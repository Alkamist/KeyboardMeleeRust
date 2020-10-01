pub struct AnalogAxis {
    value: f64,
    previous_value: f64,
    dead_zone: f64,
    was_active: bool,
    high_state_was_first: bool,
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
    pub fn value(&self) -> f64 { self.value }
    pub fn set_value(&mut self, value: f64) { self.value = value; }

    pub fn dead_zone(&self) -> f64 { self.dead_zone }

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

    pub fn update_previous_state(&mut self) {
        self.previous_value = self.value;
        self.was_active = self.is_active();
    }
}
