use std::time::{Instant, Duration};

use crate::button::Button;

pub struct JumpLogic {
    pub short_hop_output: bool,
    pub full_hop_output: bool,

    short_hop_input: Button,
    full_hop_input: Button,
    is_short_hopping: bool,
    is_full_hopping: bool,
    short_hop_time: Instant,
    full_hop_time: Instant,
}

impl Default for JumpLogic {
    fn default() -> Self {
        Self {
            short_hop_output: false,
            full_hop_output: false,
            short_hop_input: Default::default(),
            full_hop_input: Default::default(),
            is_short_hopping: false,
            is_full_hopping: false,
            short_hop_time: Instant::now(),
            full_hop_time: Instant::now(),
        }
    }
}

impl JumpLogic {
    pub fn update(&mut self, short_hop: bool, full_hop: bool) {
        self.short_hop_input.update_previous_state();
        self.full_hop_input.update_previous_state();
        self.short_hop_input.set_state(short_hop);
        self.full_hop_input.set_state(full_hop);


        // Short hop handling.
        let start_short_hop = self.short_hop_input.just_pressed()
                           || (self.is_full_hopping && self.full_hop_input.just_pressed());

        if start_short_hop {
            self.short_hop_output = true;
            self.is_short_hopping = true;
            self.short_hop_time = Instant::now();
        }

        if self.is_short_hopping && Instant::now() - self.short_hop_time >= Duration::from_millis(25) {
            self.short_hop_output = false;
            self.is_short_hopping = false;
        }


        // Full hop handling.
        let start_full_hop = self.full_hop_input.just_pressed();

        if start_full_hop {
            self.is_full_hopping = true;
            self.full_hop_output = true;
            self.full_hop_time = Instant::now();
        }

        if self.is_full_hopping && !self.full_hop_input.is_pressed() {
            if Instant::now() - self.full_hop_time >= Duration::from_millis(134) {
                self.full_hop_output = false;
            }

            // Wait one extra frame so you can't miss a double jump by
            // pushing the full hop button on the same frame of release.
            if Instant::now() - self.full_hop_time >= Duration::from_millis(150) {
                self.is_full_hopping = false;
            }
        }
    }
}