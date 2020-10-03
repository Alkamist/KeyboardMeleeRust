use std::time::Duration;
use keyboard_melee_controller::KeyboardMeleeController;

#[tokio::main]
async fn main() {
    let mut controller = KeyboardMeleeController::new();

    let mut interval = tokio::time::interval(Duration::from_millis(1));
    loop {
        controller.update();
        interval.tick().await;
    }
}
