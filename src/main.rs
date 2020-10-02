use keyboard_melee_controller::KeyboardMeleeController;

#[tokio::main]
async fn main() {
    let mut controller = KeyboardMeleeController::new(1, "C:\\Program Files\\vJoy\\x64\\vJoyInterface.dll");
    controller.run().await;
}
