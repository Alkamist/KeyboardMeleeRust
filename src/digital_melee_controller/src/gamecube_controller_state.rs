use crate::button::Button;
use crate::analog_axis::AnalogAxis;
use crate::analog_slider::AnalogSlider;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameCubeControllerButton {
    A,
    B,
    X,
    Y,
    Z,
    L,
    R,
    Start,
    DLeft,
    DRight,
    DUp,
    DDown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameCubeControllerAxis {
    X,
    Y,
    CX,
    CY,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameCubeControllerSlider {
    L,
    R,
}

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

    pub fn button(&self, input: GameCubeControllerButton) -> &Button {
        match input {
            GameCubeControllerButton::A => &self.a_button,
            GameCubeControllerButton::B => &self.b_button,
            GameCubeControllerButton::X => &self.x_button,
            GameCubeControllerButton::Y => &self.y_button,
            GameCubeControllerButton::Z => &self.z_button,
            GameCubeControllerButton::L => &self.l_button,
            GameCubeControllerButton::R => &self.r_button,
            GameCubeControllerButton::Start => &self.start_button,
            GameCubeControllerButton::DLeft => &self.d_left_button,
            GameCubeControllerButton::DUp => &self.d_right_button,
            GameCubeControllerButton::DRight => &self.d_up_button,
            GameCubeControllerButton::DDown => &self.d_down_button,
        }
    }

    pub fn axis(&self, input: GameCubeControllerAxis) -> &AnalogAxis {
        match input {
            GameCubeControllerAxis::X => &self.x_axis,
            GameCubeControllerAxis::Y => &self.y_axis,
            GameCubeControllerAxis::CX => &self.c_x_axis,
            GameCubeControllerAxis::CY => &self.c_y_axis,
        }
    }

    pub fn slider(&self, input: GameCubeControllerSlider) -> &AnalogSlider {
        match input {
            GameCubeControllerSlider::L => &self.l_analog,
            GameCubeControllerSlider::R => &self.r_analog,
        }
    }
}
