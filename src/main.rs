use tokio::time::{self, Duration};
use keyboard_input::{self, KeyboardKey};
use vjoy_wrapper::{VJoyDevice, VJoyAxis};
use digital_melee_controller::*;

#[tokio::main]
async fn main() {
    keyboard_input::initialize();
    keyboard_input::block_all_keys();

    let mut vjoy_device = VJoyDevice::new(1, "C:\\Program Files\\vJoy\\x64\\vJoyInterface.dll");
    let mut controller = DigitalMeleeController::default();

    let mut interval = time::interval(Duration::from_millis(1));
    loop {
        controller.set_action_state(Action::Left, keyboard_input::key_is_pressed(KeyboardKey::A));
        controller.set_action_state(Action::Right, keyboard_input::key_is_pressed(KeyboardKey::D));
        controller.set_action_state(Action::Down, keyboard_input::key_is_pressed(KeyboardKey::S));
        controller.set_action_state(Action::Up, keyboard_input::key_is_pressed(KeyboardKey::W));
        controller.set_action_state(Action::XMod, keyboard_input::key_is_pressed(KeyboardKey::LeftAlt));
        controller.set_action_state(Action::YMod, keyboard_input::key_is_pressed(KeyboardKey::Space));
        controller.set_action_state(Action::YMod, keyboard_input::key_is_pressed(KeyboardKey::CapsLock));
        controller.set_action_state(Action::CLeft, keyboard_input::key_is_pressed(KeyboardKey::L));
        controller.set_action_state(Action::CRight, keyboard_input::key_is_pressed(KeyboardKey::Slash));
        controller.set_action_state(Action::CDown, keyboard_input::key_is_pressed(KeyboardKey::Apostrophe));
        controller.set_action_state(Action::CUp, keyboard_input::key_is_pressed(KeyboardKey::P));
        controller.set_action_state(Action::ShortHop, keyboard_input::key_is_pressed(KeyboardKey::LeftBracket));
        controller.set_action_state(Action::FullHop, keyboard_input::key_is_pressed(KeyboardKey::BackSlash));
        controller.set_action_state(Action::A, keyboard_input::key_is_pressed(KeyboardKey::RightWindows));
        controller.set_action_state(Action::B, keyboard_input::key_is_pressed(KeyboardKey::RightAlt));
        controller.set_action_state(Action::BUp, keyboard_input::key_is_pressed(KeyboardKey::Backspace));
        controller.set_action_state(Action::BSide, keyboard_input::key_is_pressed(KeyboardKey::Enter));
        controller.set_action_state(Action::Z, keyboard_input::key_is_pressed(KeyboardKey::Equals));
        controller.set_action_state(Action::Shield, keyboard_input::key_is_pressed(KeyboardKey::RightBracket));
        controller.set_action_state(Action::ToggleLightShield, keyboard_input::key_is_pressed(KeyboardKey::Space));
        controller.set_action_state(Action::AirDodge, keyboard_input::key_is_pressed(KeyboardKey::Semicolon));
        controller.set_action_state(Action::Start, keyboard_input::key_is_pressed(KeyboardKey::Key5));
        controller.set_action_state(Action::DLeft, keyboard_input::key_is_pressed(KeyboardKey::V));
        controller.set_action_state(Action::DRight, keyboard_input::key_is_pressed(KeyboardKey::N));
        controller.set_action_state(Action::DDown, keyboard_input::key_is_pressed(KeyboardKey::B));
        controller.set_action_state(Action::DUp, keyboard_input::key_is_pressed(KeyboardKey::G));
        controller.set_action_state(Action::ChargeSmash, keyboard_input::key_is_pressed(KeyboardKey::Space));
        controller.set_action_state(Action::InvertXAxis, keyboard_input::key_is_pressed(KeyboardKey::RightShift));
        controller.update();

        vjoy_device.set_button(1, controller.controller_state.a_button.is_pressed);
        vjoy_device.set_button(2, controller.controller_state.b_button.is_pressed);
        vjoy_device.set_button(3, controller.controller_state.x_button.is_pressed);
        vjoy_device.set_button(4, controller.controller_state.y_button.is_pressed);
        vjoy_device.set_button(5, controller.controller_state.z_button.is_pressed);
        vjoy_device.set_button(6, controller.controller_state.l_button.is_pressed);
        vjoy_device.set_button(7, controller.controller_state.r_button.is_pressed);
        vjoy_device.set_button(8, controller.controller_state.start_button.is_pressed);
        vjoy_device.set_button(9, controller.controller_state.d_left_button.is_pressed);
        vjoy_device.set_button(10, controller.controller_state.d_up_button.is_pressed);
        vjoy_device.set_button(11, controller.controller_state.d_right_button.is_pressed);
        vjoy_device.set_button(12, controller.controller_state.d_down_button.is_pressed);
        vjoy_device.set_axis(VJoyAxis::X, controller.controller_state.x_axis.value);
        vjoy_device.set_axis(VJoyAxis::Y, controller.controller_state.y_axis.value);
        vjoy_device.set_axis(VJoyAxis::XRotation, controller.controller_state.c_x_axis.value);
        vjoy_device.set_axis(VJoyAxis::YRotation, controller.controller_state.c_y_axis.value);
        vjoy_device.send_inputs();

        interval.tick().await;
    }
}
