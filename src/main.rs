use tokio::time::{self, Duration};
use keyboard_input::*;

#[tokio::main]
async fn main() {
    keyboard_input::initialize();
    keyboard_input::block_all_keys();

    let mut interval = time::interval(Duration::from_millis(100));
    loop {
        if keyboard_input::key_is_pressed(KeyboardKey::A) {
            println!("Yee");
        }

        interval.tick().await;
    }
}
