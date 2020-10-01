#[derive(Default)]
pub struct Button {
    is_pressed: bool,
    was_pressed: bool,
}

impl Button {
    pub fn set_state(&mut self, state: bool) { self.is_pressed = state; }
    pub fn is_pressed(&self) -> bool { self.is_pressed }
    pub fn just_pressed(&self) -> bool { self.is_pressed && !self.was_pressed }
    pub fn just_released(&self) -> bool { self.was_pressed && !self.is_pressed }
    pub fn update_previous_state(&mut self) {
        self.was_pressed = self.is_pressed;
    }
}
