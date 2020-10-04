use std::thread;
use std::time::{Instant, Duration};
use keyboard_melee_controller::KeyboardMeleeController;

fn main() {
    let mut controller = KeyboardMeleeController::new();

    let update_rate = Duration::from_millis(1);

    //let mut time_of_last_loop = Instant::now();
    let mut time_of_next_loop = Instant::now();
    loop {
        //println!("{:?}", Instant::now() - time_of_last_loop);
        //time_of_last_loop = Instant::now();

        time_of_next_loop += update_rate;

        controller.update();

        let now = Instant::now();
        if time_of_next_loop > now {
            thread::sleep(time_of_next_loop - now);
        }
    }
}
