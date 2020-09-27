mod gamecube_controller_state;

pub use gamecube_controller_state::*;

macro_rules! create_controller {
    ($($variant:ident),+) => {
        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone)]
        pub enum Action {
            $($variant,)+
        }

        #[allow(non_snake_case)]
        #[derive(Default)]
        pub struct DigitalMeleeController {
            pub controller_state: GameCubeControllerState,
            $($variant: Button,)+
        }

        impl DigitalMeleeController {
            pub fn update(&mut self) {
                self.controller_state.update();
                $(self.$variant.update();)+

                self.controller_state.x_axis.set_value_from_states(
                    self.action_button(Action::Left).is_pressed,
                    self.action_button(Action::Right).is_pressed,
                );
                self.controller_state.y_axis.set_value_from_states(
                    self.action_button(Action::Down).is_pressed,
                    self.action_button(Action::Up).is_pressed,
                );
                self.controller_state.c_x_axis.set_value_from_states(
                    self.action_button(Action::CLeft).is_pressed,
                    self.action_button(Action::CRight).is_pressed,
                );
                self.controller_state.c_y_axis.set_value_from_states(
                    self.action_button(Action::CDown).is_pressed,
                    self.action_button(Action::CUp).is_pressed,
                );

                self.controller_state.a_button.is_pressed = self.action_button(Action::A).is_pressed;
                self.controller_state.b_button.is_pressed = self.action_button(Action::B).is_pressed;
                self.controller_state.z_button.is_pressed = self.action_button(Action::Z).is_pressed;
                self.controller_state.l_button.is_pressed = self.action_button(Action::AirDodge).is_pressed;
                self.controller_state.r_button.is_pressed = self.action_button(Action::Shield).is_pressed;
                self.controller_state.y_button.is_pressed = self.action_button(Action::ShortHop).is_pressed;
                self.controller_state.x_button.is_pressed = self.action_button(Action::FullHop).is_pressed;
                self.controller_state.start_button.is_pressed = self.action_button(Action::Start).is_pressed;
                self.controller_state.d_left_button.is_pressed = self.action_button(Action::DLeft).is_pressed;
                self.controller_state.d_right_button.is_pressed = self.action_button(Action::DRight).is_pressed;
                self.controller_state.d_down_button.is_pressed = self.action_button(Action::DDown).is_pressed;
                self.controller_state.d_up_button.is_pressed = self.action_button(Action::DUp).is_pressed;
            }

            fn action_button(&self, action: Action) -> &Button {
                match action {
                    $(Action::$variant => &self.$variant,)+
                }
            }

            pub fn set_action_state(&mut self, action: Action, state: bool) {
                match action {
                    $(Action::$variant => self.$variant.is_pressed = state,)+
                }
            }
        }
    };
}

create_controller!(
    Left,
    Right,
    Down,
    Up,
    XMod,
    YMod,
    Tilt,
    CLeft,
    CRight,
    CDown,
    CUp,
    ShortHop,
    FullHop,
    A,
    B,
    BUp,
    BSide,
    Z,
    Shield,
    ToggleLightShield,
    AirDodge,
    Start,
    DLeft,
    DRight,
    DDown,
    DUp,
    ChargeSmash,
    InvertXAxis
);
