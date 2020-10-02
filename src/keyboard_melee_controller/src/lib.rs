use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;

use serde::{Serialize, Deserialize};
use tokio::time::{self, Duration};
use keyboard_input::{self, KeyboardKey};
use vjoy_device::{VJoyDevice, VJoyAxis};
use digital_melee_controller::{
    Action,
    GameCubeControllerButton,
    GameCubeControllerAxis,
    GameCubeControllerSlider,
    DigitalMeleeController,
};

pub struct KeyboardMeleeController {
    pub controller: DigitalMeleeController,
    pub vjoy_device: VJoyDevice,
    config: KeyboardMeleeControllerConfig,
}

impl KeyboardMeleeController {
    pub fn new() -> Self {
        let output_config: KeyboardMeleeControllerConfig;
        if let Ok(config) = KeyboardMeleeControllerConfig::load("config.json") {
            output_config = config;
        }
        else {
            output_config = KeyboardMeleeControllerConfig::default();
            output_config.save("config.json").unwrap();
        }

        keyboard_input::start_hook();
        keyboard_input::block_all_keys();

        Self {
            controller: DigitalMeleeController::default(),
            vjoy_device: VJoyDevice::new(output_config.vjoy_device_id, &output_config.vjoy_dll_path),
            config: output_config,
        }
    }

    pub async fn run(&mut self) {
        let mut interval = time::interval(Duration::from_millis(1));
        loop {
            self.update_controller_state_with_keys();
            self.update_vjoy_device_buttons();
            self.update_vjoy_device_axes();
            self.update_vjoy_device_sliders();
            self.vjoy_device.send_inputs();
            interval.tick().await;
        }
    }

    fn update_controller_state_with_keys(&mut self) {
        self.controller.update_previous_state();
        for (action, keybinds) in &self.config.key_binds {
            self.controller.set_action_state(
                *action,
                {
                    let mut state = false;
                    for keybind in &*keybinds {
                        if keyboard_input::key_is_pressed(*keybind) {
                            state = true;
                        }
                    }
                    state
                }
            );
        }
        self.controller.process_actions();
    }

    fn update_vjoy_device_buttons(&mut self) {
        for (button_variant, button_id) in &self.config.vjoy_button_binds {
            let button = self.controller.controller_state.button(*button_variant);
            self.vjoy_device.set_button(*button_id, button.is_pressed());
        }
    }

    fn update_vjoy_device_axes(&mut self) {
        for (axis_variant, vjoy_axis) in &self.config.vjoy_axis_binds {
            let axis = self.controller.controller_state.axis(*axis_variant);
            self.vjoy_device.set_axis(*vjoy_axis, axis.value());
        }
    }

    fn update_vjoy_device_sliders(&mut self) {
        for (slider_variant, vjoy_slider) in &self.config.vjoy_slider_binds {
            let slider = self.controller.controller_state.slider(*slider_variant);
            self.vjoy_device.set_axis(*vjoy_slider, slider.value());
        }
    }
}

#[derive(Serialize, Deserialize)]
struct KeyboardMeleeControllerConfig {
    pub key_binds: HashMap<Action, Vec<KeyboardKey>>,
    pub vjoy_dll_path: String,
    pub vjoy_device_id: u32,
    pub vjoy_button_binds: HashMap<GameCubeControllerButton, u32>,
    pub vjoy_axis_binds: HashMap<GameCubeControllerAxis, VJoyAxis>,
    pub vjoy_slider_binds: HashMap<GameCubeControllerSlider, VJoyAxis>,
}

impl KeyboardMeleeControllerConfig {
    fn save(&self, file_name: &str) -> std::io::Result<()> {
        let mut config = File::create(file_name)?;
        let output = serde_json::to_string_pretty(&self).unwrap();
        config.write_all(output.as_bytes())?;
        Ok(())
    }

    fn load(file_name: &str) -> std::io::Result<KeyboardMeleeControllerConfig> {
        let config_string = fs::read_to_string(file_name)?;
        let config: KeyboardMeleeControllerConfig = serde_json::from_str(&config_string)?;
        Ok(config)
    }

    fn apply_default_key_binds(&mut self) {
        self.key_binds.clear();
        self.key_binds.insert(Action::Left, vec![KeyboardKey::A]);
        self.key_binds.insert(Action::Right, vec![KeyboardKey::D]);
        self.key_binds.insert(Action::Down, vec![KeyboardKey::S]);
        self.key_binds.insert(Action::Up, vec![KeyboardKey::W]);
        self.key_binds.insert(Action::XMod, vec![KeyboardKey::LeftAlt]);
        self.key_binds.insert(Action::YMod, vec![KeyboardKey::Space]);
        self.key_binds.insert(Action::Tilt, vec![KeyboardKey::CapsLock]);
        self.key_binds.insert(Action::CLeft, vec![KeyboardKey::L]);
        self.key_binds.insert(Action::CRight, vec![KeyboardKey::Slash]);
        self.key_binds.insert(Action::CDown, vec![KeyboardKey::Apostrophe]);
        self.key_binds.insert(Action::CUp, vec![KeyboardKey::P]);
        self.key_binds.insert(Action::ShortHop, vec![KeyboardKey::LeftBracket,
                                                     KeyboardKey::Minus]);
        self.key_binds.insert(Action::FullHop, vec![KeyboardKey::BackSlash]);
        self.key_binds.insert(Action::A, vec![KeyboardKey::RightWindows]);
        self.key_binds.insert(Action::B, vec![KeyboardKey::RightAlt]);
        self.key_binds.insert(Action::BUp, vec![KeyboardKey::Backspace]);
        self.key_binds.insert(Action::BSide, vec![KeyboardKey::Enter]);
        self.key_binds.insert(Action::Z, vec![KeyboardKey::Equals]);
        self.key_binds.insert(Action::Shield, vec![KeyboardKey::RightBracket]);
        self.key_binds.insert(Action::ToggleLightShield, vec![KeyboardKey::Space]);
        self.key_binds.insert(Action::AirDodge, vec![KeyboardKey::Semicolon]);
        self.key_binds.insert(Action::Start, vec![KeyboardKey::Key5]);
        self.key_binds.insert(Action::DLeft, vec![KeyboardKey::V]);
        self.key_binds.insert(Action::DRight, vec![KeyboardKey::N]);
        self.key_binds.insert(Action::DDown, vec![KeyboardKey::B]);
        self.key_binds.insert(Action::DUp, vec![KeyboardKey::G]);
        self.key_binds.insert(Action::ChargeSmash, vec![KeyboardKey::Space]);
        self.key_binds.insert(Action::InvertXAxis, vec![KeyboardKey::RightShift]);
    }

    fn apply_default_vjoy_button_binds(&mut self) {
        self.vjoy_button_binds.clear();
        self.vjoy_button_binds.insert(GameCubeControllerButton::A, 1);
        self.vjoy_button_binds.insert(GameCubeControllerButton::B, 2);
        self.vjoy_button_binds.insert(GameCubeControllerButton::X, 3);
        self.vjoy_button_binds.insert(GameCubeControllerButton::Y, 4);
        self.vjoy_button_binds.insert(GameCubeControllerButton::Z, 5);
        self.vjoy_button_binds.insert(GameCubeControllerButton::L, 6);
        self.vjoy_button_binds.insert(GameCubeControllerButton::R, 7);
        self.vjoy_button_binds.insert(GameCubeControllerButton::Start, 8);
        self.vjoy_button_binds.insert(GameCubeControllerButton::DLeft, 9);
        self.vjoy_button_binds.insert(GameCubeControllerButton::DUp, 10);
        self.vjoy_button_binds.insert(GameCubeControllerButton::DRight, 11);
        self.vjoy_button_binds.insert(GameCubeControllerButton::DDown, 12);
    }

    fn apply_default_vjoy_axis_binds(&mut self) {
        self.vjoy_axis_binds.clear();
        self.vjoy_axis_binds.insert(GameCubeControllerAxis::X, VJoyAxis::X);
        self.vjoy_axis_binds.insert(GameCubeControllerAxis::Y, VJoyAxis::Y);
        self.vjoy_axis_binds.insert(GameCubeControllerAxis::CX, VJoyAxis::XRotation);
        self.vjoy_axis_binds.insert(GameCubeControllerAxis::CY, VJoyAxis::YRotation);
    }

    fn apply_default_vjoy_slider_binds(&mut self) {
        self.vjoy_slider_binds.clear();
        self.vjoy_slider_binds.insert(GameCubeControllerSlider::L, VJoyAxis::Slider0);
    }
}

impl Default for KeyboardMeleeControllerConfig {
    fn default() -> Self {
        let mut output = Self {
            key_binds: HashMap::new(),
            vjoy_dll_path: "C:\\Program Files\\vJoy\\x64\\vJoyInterface.dll".to_string(),
            vjoy_device_id: 1,
            vjoy_button_binds: HashMap::new(),
            vjoy_axis_binds: HashMap::new(),
            vjoy_slider_binds: HashMap::new(),
        };

        output.apply_default_key_binds();
        output.apply_default_vjoy_button_binds();
        output.apply_default_vjoy_axis_binds();
        output.apply_default_vjoy_slider_binds();

        output
    }
}
