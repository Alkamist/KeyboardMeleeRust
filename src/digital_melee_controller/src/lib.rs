mod button;
mod analog_axis;
mod gamecube_controller_state;
mod jump_logic;
mod stick_tilter;
mod air_dodge_logic;
mod delayed_button;
mod a_stick;

pub use crate::button::Button;
pub use crate::analog_axis::AnalogAxis;
pub use crate::gamecube_controller_state::GameCubeControllerState;
use crate::jump_logic::JumpLogic;
use crate::stick_tilter::StickTilter;
use crate::air_dodge_logic::AirDodgeLogic;
use crate::a_stick::AStick;

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
            pub fn update_action_buttons_previous_states(&mut self) {
                $(self.action_states.$variant.update_previous_state();)+
            }

            fn action_button(&self, action: Action) -> &Button {
                match action {
                    $(Action::$variant => &self.action_states.$variant,)+
                }
            }

            pub fn set_action_state(&mut self, action: Action, state: bool) {
                match action {
                    $(Action::$variant => self.action_states.$variant.set_state(state),)+
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

pub struct DigitalMeleeController {
    pub controller_state: GameCubeControllerState,
    action_states: ActionStates,
    jump_logic: JumpLogic,
    tilt_modifier: StickTilter,
    shield_tilter: StickTilter,
    air_dodge_logic: AirDodgeLogic,
    a_stick: AStick,
    use_c_stick_tilting: bool,
}

impl DigitalMeleeController {
    pub fn update_previous_state(&mut self) {
        self.controller_state.update_previous_state();
        self.update_action_buttons_previous_states();
    }

    pub fn process_actions(&mut self) {
        self.update_axes_with_directional_buttons();
        self.handle_a_stick();
        self.handle_tilt_modifier();
        self.handle_shield_tilt();
        self.handle_air_dodge_logic();
        self.handle_jump_logic();

        self.controller_state.b_button.set_state(self.action_button(Action::B).is_pressed());
        self.controller_state.z_button.set_state(self.action_button(Action::Z).is_pressed());
        self.controller_state.l_button.set_state(self.action_button(Action::AirDodge).is_pressed());
        self.controller_state.r_button.set_state(self.action_button(Action::Shield).is_pressed());
        self.controller_state.y_button.set_state(self.jump_logic.short_hop_output);
        self.controller_state.x_button.set_state(self.jump_logic.full_hop_output);
        self.controller_state.start_button.set_state(self.action_button(Action::Start).is_pressed());
        self.controller_state.d_left_button.set_state(self.action_button(Action::DLeft).is_pressed());
        self.controller_state.d_right_button.set_state(self.action_button(Action::DRight).is_pressed());
        self.controller_state.d_down_button.set_state(self.action_button(Action::DDown).is_pressed());
        self.controller_state.d_up_button.set_state(self.action_button(Action::DUp).is_pressed());
    }

    pub fn update_axes_with_directional_buttons(&mut self) {
        self.controller_state.x_axis.set_value_from_states(
            self.action_button(Action::Left).is_pressed(),
            self.action_button(Action::Right).is_pressed(),
        );
        self.controller_state.y_axis.set_value_from_states(
            self.action_button(Action::Down).is_pressed(),
            self.action_button(Action::Up).is_pressed(),
        );
        self.controller_state.c_x_axis.set_value_from_states(
            self.action_button(Action::CLeft).is_pressed(),
            self.action_button(Action::CRight).is_pressed(),
        );
        self.controller_state.c_y_axis.set_value_from_states(
            self.action_button(Action::CDown).is_pressed(),
            self.action_button(Action::CUp).is_pressed(),
        );
    }

    pub fn handle_a_stick(&mut self) {
        if self.use_c_stick_tilting && !self.action_button(Action::Shield).is_pressed() {
            let a_stick_modifier = self.action_button(Action::Tilt).is_pressed();
            self.a_stick.update_state(
                &self.controller_state.x_axis,
                &self.controller_state.y_axis,
                self.action_button(Action::A).is_pressed(),
                self.action_button(Action::CLeft).is_pressed() && a_stick_modifier,
                self.action_button(Action::CRight).is_pressed() && a_stick_modifier,
                self.action_button(Action::CDown).is_pressed() && a_stick_modifier,
                self.action_button(Action::CUp).is_pressed() && a_stick_modifier,
            );
            self.controller_state.a_button.set_state(self.a_stick.output_state());
            self.controller_state.x_axis.set_value(self.a_stick.x_axis_output());
            self.controller_state.y_axis.set_value(self.a_stick.y_axis_output());
            if a_stick_modifier {
                self.controller_state.c_x_axis.set_value(0.0);
                self.controller_state.c_y_axis.set_value(0.0);
            }
        }
        else {
            self.controller_state.a_button.set_state(self.action_button(Action::A).is_pressed());
        }
    }

    pub fn handle_tilt_modifier(&mut self) {
        let allow_tilt = self.action_button(Action::Tilt).is_pressed();
        let hold_tilt = self.action_button(Action::Shield).is_pressed();
        self.tilt_modifier.tilt_axes(
            &mut self.controller_state.x_axis,
            &mut self.controller_state.y_axis,
            allow_tilt,
            false,
            hold_tilt,
        );
    }

    pub fn handle_shield_tilt(&mut self) {
        let allow_tilt = self.action_button(Action::Shield).is_pressed();
        let reset_tilt = self.action_button(Action::Shield).just_pressed();
        self.shield_tilter.tilt_axes(
            &mut self.controller_state.x_axis,
            &mut self.controller_state.y_axis,
            allow_tilt,
            reset_tilt,
            false,
        );
    }

    pub fn handle_air_dodge_logic(&mut self) {
        let air_dodge = self.action_button(Action::AirDodge).is_pressed();
        let shorten_air_dodge = self.action_button(Action::Tilt).is_pressed();
        self.air_dodge_logic.update_axes(
            &mut self.controller_state.x_axis,
            &mut self.controller_state.y_axis,
            air_dodge,
            shorten_air_dodge,
        );
    }

    pub fn handle_jump_logic(&mut self) {
        self.jump_logic.update(
            self.action_button(Action::ShortHop).is_pressed(),
            self.action_button(Action::FullHop).is_pressed(),
        );
    }
}

impl Default for DigitalMeleeController {
    fn default() -> Self {
        Self {
            controller_state: Default::default(),
            action_states: Default::default(),
            jump_logic: Default::default(),
            tilt_modifier: StickTilter::new(0.65),
            shield_tilter: StickTilter::new(0.6625),
            air_dodge_logic: Default::default(),
            a_stick: Default::default(),
            use_c_stick_tilting: true,
        }
    }
}
