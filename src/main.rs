use keyboard_melee_controller::KeyboardMeleeController;

#[tokio::main]
async fn main() {
    let mut controller = KeyboardMeleeController::new();
    controller.run().await;
}
