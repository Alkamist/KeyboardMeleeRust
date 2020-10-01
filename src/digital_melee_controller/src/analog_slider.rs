pub struct AnalogSlider {
    value: f64,
    previous_value: f64,
}

impl Default for AnalogSlider {
    fn default() -> Self {
        Self {
            value: 0.0,
            previous_value: 0.0,
        }
    }
}

impl AnalogSlider {
    pub fn value(&self) -> f64 { self.value }
    pub fn set_value(&mut self, value: f64) { self.value = value; }

    pub fn update_previous_state(&mut self) {
        self.previous_value = self.value;
    }
}
