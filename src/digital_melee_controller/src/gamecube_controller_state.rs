use crate::button::Button;
use crate::analog_axis::AnalogAxis;
use crate::analog_slider::AnalogSlider;

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
    pub l_analog: AnalogSlider,
    pub r_analog: AnalogSlider,
}

impl GameCubeControllerState {
    pub fn update_previous_state(&mut self) {
        self.x_axis.update_previous_state();
        self.y_axis.update_previous_state();
        self.c_x_axis.update_previous_state();
        self.c_y_axis.update_previous_state();
        self.a_button.update_previous_state();
        self.b_button.update_previous_state();
        self.x_button.update_previous_state();
        self.y_button.update_previous_state();
        self.z_button.update_previous_state();
        self.l_button.update_previous_state();
        self.r_button.update_previous_state();
        self.start_button.update_previous_state();
        self.d_left_button.update_previous_state();
        self.d_right_button.update_previous_state();
        self.d_down_button.update_previous_state();
        self.d_up_button.update_previous_state();
        self.l_analog.update_previous_state();
        self.r_analog.update_previous_state();
    }
}
