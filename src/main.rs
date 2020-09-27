use tokio::time::{self, Duration};
use keyboard_input::*;
use vjoy_wrapper::{VJoyDevice, VJoyAxis};

#[tokio::main]
async fn main() {
    keyboard_input::initialize();
    keyboard_input::block_all_keys();

    let mut vjoy_device = VJoyDevice::new(1, "C:\\Program Files\\vJoy\\x64\\vJoyInterface.dll");

    let mut interval = time::interval(Duration::from_millis(1));
    loop {
        vjoy_device.set_button(1, keyboard_input::key_is_pressed(KeyboardKey::A));
        vjoy_device.send_inputs();

        interval.tick().await;
    }
}
