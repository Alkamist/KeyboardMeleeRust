mod gamecube_controller_state;
mod jump_logic;
mod stick_tilter;
mod air_dodge_logic;

pub use gamecube_controller_state::{
    GameCubeControllerState,
    Button,
    AnalogAxis,
};
use jump_logic::JumpLogic;
use stick_tilter::StickTilter;
use air_dodge_logic::AirDodgeLogic;

macro_rules! define_actions {
    ($($variant:ident),+) => {
        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone)]
        pub enum Action {
            $($variant,)+
        }

        #[allow(non_snake_case)]
        #[derive(Default)]
        pub struct ActionStates {
            pub $($variant: Button,)+
        }

        impl DigitalMeleeController {
            pub fn update_action_buttons(&mut self) {
                $(self.action_states.$variant.update();)+
            }

            fn action_button(&self, action: Action) -> &Button {
                match action {
                    $(Action::$variant => &self.action_states.$variant,)+
                }
            }

            pub fn set_action_state(&mut self, action: Action, state: bool) {
                match action {
                    $(Action::$variant => self.action_states.$variant.is_pressed = state,)+
                }
            }
        }
    };
}

define_actions!(
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

#[derive(Default)]
pub struct DigitalMeleeController {
    pub controller_state: GameCubeControllerState,
    action_states: ActionStates,
    jump_logic: JumpLogic,
    tilt_modifier: StickTilter,
    shield_tilter: StickTilter,
    air_dodge_logic: AirDodgeLogic,
}

impl DigitalMeleeController {
    pub fn new() -> Self {
        Self {
            tilt_modifier: StickTilter::new(0.65),
            shield_tilter: StickTilter::new(0.6625),
            ..Default::default()
        }
    }

    pub fn update_state(&mut self) {
        self.controller_state.update();
        self.update_action_buttons();
    }

    pub fn process_actions(&mut self) {
        // Use directional buttons to update analog axes.
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

        // Handle tilting the stick with the tilt modifier.
        let allow_tilt = self.action_button(Action::Tilt).is_pressed;
        let hold_tilt = self.action_button(Action::Shield).is_pressed;
        self.tilt_modifier.tilt_axes(
            &mut self.controller_state.x_axis,
            &mut self.controller_state.y_axis,
            allow_tilt,
            false,
            hold_tilt,
        );

        // Handle tilting shield.
        let allow_tilt = self.action_button(Action::Shield).is_pressed;
        let reset_tilt = self.action_button(Action::Shield).just_pressed();
        self.shield_tilter.tilt_axes(
            &mut self.controller_state.x_axis,
            &mut self.controller_state.y_axis,
            allow_tilt,
            reset_tilt,
            false,
        );

        // Handle air dodge angle logic.
        let air_dodge = self.action_button(Action::AirDodge).is_pressed;
        let shorten_air_dodge = self.action_button(Action::Tilt).is_pressed;
        self.air_dodge_logic.update_axes(
            &mut self.controller_state.x_axis,
            &mut self.controller_state.y_axis,
            air_dodge,
            shorten_air_dodge,
        );

        // Handle short hop and full hop macros.
        self.jump_logic.update(
            self.action_button(Action::ShortHop).is_pressed,
            self.action_button(Action::FullHop).is_pressed,
        );

        self.controller_state.a_button.is_pressed = self.action_button(Action::A).is_pressed;
        self.controller_state.b_button.is_pressed = self.action_button(Action::B).is_pressed;
        self.controller_state.z_button.is_pressed = self.action_button(Action::Z).is_pressed;
        self.controller_state.l_button.is_pressed = self.action_button(Action::AirDodge).is_pressed;
        self.controller_state.r_button.is_pressed = self.action_button(Action::Shield).is_pressed;
        self.controller_state.y_button.is_pressed = self.jump_logic.short_hop_output;
        self.controller_state.x_button.is_pressed = self.jump_logic.full_hop_output;
        self.controller_state.start_button.is_pressed = self.action_button(Action::Start).is_pressed;
        self.controller_state.d_left_button.is_pressed = self.action_button(Action::DLeft).is_pressed;
        self.controller_state.d_right_button.is_pressed = self.action_button(Action::DRight).is_pressed;
        self.controller_state.d_down_button.is_pressed = self.action_button(Action::DDown).is_pressed;
        self.controller_state.d_up_button.is_pressed = self.action_button(Action::DUp).is_pressed;
    }
}
