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
        let mut controller = DigitalMeleeController::default();
        let config = KeyboardMeleeControllerConfig::load("config.json");
        controller.set_use_short_hop_macro(config.use_short_hop_macro);
        controller.set_use_c_stick_tilting(config.use_c_stick_tilting);
        controller.set_use_extra_b_buttons(config.use_extra_b_buttons);

        keyboard_input::start_hook();
        keyboard_input::block_all_keys();

        Self {
            controller,
            vjoy_device: VJoyDevice::new(config.vjoy_device_id, &config.vjoy_dll_path),
            config: config,
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

type KeyBinds = HashMap<Action, Vec<KeyboardKey>>;
type VJoyButtonBinds = HashMap<GameCubeControllerButton, u32>;
type VJoyAxisBinds = HashMap<GameCubeControllerAxis, VJoyAxis>;
type VJoySliderBinds = HashMap<GameCubeControllerSlider, VJoyAxis>;

#[derive(Serialize, Deserialize)]
struct KeyboardMeleeControllerConfig {
    #[serde(default = "KeyboardMeleeControllerConfig::default_key_binds")]
    pub key_binds: KeyBinds,

    #[serde(default = "KeyboardMeleeControllerConfig::default_use_short_hop_macro")]
    pub use_short_hop_macro: bool,

    #[serde(default = "KeyboardMeleeControllerConfig::default_use_c_stick_tilting")]
    pub use_c_stick_tilting: bool,

    #[serde(default = "KeyboardMeleeControllerConfig::default_use_extra_b_buttons")]
    pub use_extra_b_buttons: bool,

    #[serde(default = "KeyboardMeleeControllerConfig::default_vjoy_dll_path")]
    pub vjoy_dll_path: String,

    #[serde(default = "KeyboardMeleeControllerConfig::default_vjoy_device_id")]
    pub vjoy_device_id: u32,

    #[serde(default = "KeyboardMeleeControllerConfig::default_vjoy_button_binds")]
    pub vjoy_button_binds: VJoyButtonBinds,

    #[serde(default = "KeyboardMeleeControllerConfig::default_vjoy_axis_binds")]
    pub vjoy_axis_binds: VJoyAxisBinds,

    #[serde(default = "KeyboardMeleeControllerConfig::default_vjoy_slider_binds")]
    pub vjoy_slider_binds: VJoySliderBinds,
}

impl KeyboardMeleeControllerConfig {
    fn save(&self, file_name: &str) -> std::io::Result<()> {
        let mut config = File::create(file_name)?;
        let output = serde_json::to_string_pretty(&self)?;
        config.write_all(output.as_bytes())?;
        Ok(())
    }

    fn load(file_name: &str) -> KeyboardMeleeControllerConfig {
        let output_config: KeyboardMeleeControllerConfig;

        if let Ok(config_string) = fs::read_to_string(file_name) {
            match serde_json::from_str(&config_string) {
                Ok(value) => output_config = value,
                Err(e) => {
                    println!("Could not parse config.json, loading default config:\n {}", e);
                    output_config = KeyboardMeleeControllerConfig::default();
                }
            }
        }
        else {
            output_config = KeyboardMeleeControllerConfig::default();
            if let Err(e) = output_config.save("config.json") {
                println!("Could not save config.json:\n {}", e);
            }
        }

        output_config
    }

    fn default_use_short_hop_macro() -> bool { true }
    fn default_use_c_stick_tilting() -> bool { true }
    fn default_use_extra_b_buttons() -> bool { true }

    fn default_vjoy_device_id() -> u32 { 1 }
    fn default_vjoy_dll_path() -> String { "C:\\Program Files\\vJoy\\x64\\vJoyInterface.dll".to_string() }

    fn default_key_binds() -> KeyBinds {
        let mut binds = HashMap::new();
        binds.insert(Action::Left, vec![KeyboardKey::A]);
        binds.insert(Action::Right, vec![KeyboardKey::D]);
        binds.insert(Action::Down, vec![KeyboardKey::S]);
        binds.insert(Action::Up, vec![KeyboardKey::W]);
        binds.insert(Action::XMod, vec![KeyboardKey::LeftAlt]);
        binds.insert(Action::YMod, vec![KeyboardKey::Space]);
        binds.insert(Action::Tilt, vec![KeyboardKey::CapsLock]);
        binds.insert(Action::CLeft, vec![KeyboardKey::L]);
        binds.insert(Action::CRight, vec![KeyboardKey::Slash]);
        binds.insert(Action::CDown, vec![KeyboardKey::Apostrophe]);
        binds.insert(Action::CUp, vec![KeyboardKey::P]);
        binds.insert(Action::ShortHop, vec![KeyboardKey::LeftBracket,
                                            KeyboardKey::Minus]);
        binds.insert(Action::FullHop, vec![KeyboardKey::BackSlash]);
        binds.insert(Action::A, vec![KeyboardKey::RightWindows]);
        binds.insert(Action::B, vec![KeyboardKey::RightAlt]);
        binds.insert(Action::BUp, vec![KeyboardKey::Period]);
        binds.insert(Action::BSide, vec![KeyboardKey::Backspace]);
        binds.insert(Action::Z, vec![KeyboardKey::Equals]);
        binds.insert(Action::Shield, vec![KeyboardKey::RightBracket]);
        binds.insert(Action::ToggleLightShield, vec![KeyboardKey::Space]);
        binds.insert(Action::AirDodge, vec![KeyboardKey::Semicolon]);
        binds.insert(Action::Start, vec![KeyboardKey::Key5]);
        binds.insert(Action::DLeft, vec![KeyboardKey::V]);
        binds.insert(Action::DRight, vec![KeyboardKey::N]);
        binds.insert(Action::DDown, vec![KeyboardKey::B]);
        binds.insert(Action::DUp, vec![KeyboardKey::G]);
        binds.insert(Action::ChargeSmash, vec![KeyboardKey::Space]);
        binds.insert(Action::InvertXAxis, vec![KeyboardKey::Enter]);
        binds
    }

    fn default_vjoy_button_binds() -> VJoyButtonBinds {
        let mut binds = HashMap::new();
        binds.insert(GameCubeControllerButton::A, 1);
        binds.insert(GameCubeControllerButton::B, 2);
        binds.insert(GameCubeControllerButton::X, 3);
        binds.insert(GameCubeControllerButton::Y, 4);
        binds.insert(GameCubeControllerButton::Z, 5);
        binds.insert(GameCubeControllerButton::L, 6);
        binds.insert(GameCubeControllerButton::R, 7);
        binds.insert(GameCubeControllerButton::Start, 8);
        binds.insert(GameCubeControllerButton::DLeft, 9);
        binds.insert(GameCubeControllerButton::DUp, 10);
        binds.insert(GameCubeControllerButton::DRight, 11);
        binds.insert(GameCubeControllerButton::DDown, 12);
        binds
    }

    fn default_vjoy_axis_binds() -> VJoyAxisBinds {
        let mut binds = HashMap::new();
        binds.insert(GameCubeControllerAxis::X, VJoyAxis::X);
        binds.insert(GameCubeControllerAxis::Y, VJoyAxis::Y);
        binds.insert(GameCubeControllerAxis::CX, VJoyAxis::XRotation);
        binds.insert(GameCubeControllerAxis::CY, VJoyAxis::YRotation);
        binds
    }

    fn default_vjoy_slider_binds() -> VJoySliderBinds {
        let mut binds = HashMap::new();
        binds.insert(GameCubeControllerSlider::L, VJoyAxis::Slider0);
        binds
    }
}

impl Default for KeyboardMeleeControllerConfig {
    fn default() -> Self {
        Self {
            key_binds: KeyboardMeleeControllerConfig::default_key_binds(),
            use_short_hop_macro: KeyboardMeleeControllerConfig::default_use_short_hop_macro(),
            use_c_stick_tilting: KeyboardMeleeControllerConfig::default_use_c_stick_tilting(),
            use_extra_b_buttons: KeyboardMeleeControllerConfig::default_use_extra_b_buttons(),
            vjoy_dll_path: KeyboardMeleeControllerConfig::default_vjoy_dll_path(),
            vjoy_device_id: KeyboardMeleeControllerConfig::default_vjoy_device_id(),
            vjoy_button_binds: KeyboardMeleeControllerConfig::default_vjoy_button_binds(),
            vjoy_axis_binds: KeyboardMeleeControllerConfig::default_vjoy_axis_binds(),
            vjoy_slider_binds: KeyboardMeleeControllerConfig::default_vjoy_slider_binds(),
        }
    }
}
