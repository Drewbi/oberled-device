use esp_println::println;
use hal::Rng;

use crate::{
    screen::Screen,
    config::{NUM_BRIGHTNESS_STEPS, NUM_PIXELS},
};

pub struct Noisy<'a> {
    rng: Rng<'a>,
    grid: [u8; 256],
}

impl<'a> Noisy<'a> {
    pub fn new(rng: Rng<'a>) -> Self {
        Self {
            rng,
            grid: [0; 256]
        }
    }
}

impl super::ModeTrait for Noisy<'_> {
    fn activate(&mut self, screen: &mut Screen) {
        for i in 0..NUM_PIXELS {
            self.grid[i] = (((self.rng.random() as u8) as f32 / 255 as f32) * NUM_BRIGHTNESS_STEPS as f32) as u8;
            screen.frame.set_index(i, self.grid[i]);
        }
    }

    fn update(&mut self, screen: &mut Screen) {
        for i in 0..NUM_PIXELS {
            screen.frame.set_index(i, self.grid[i]);
            let val = self.rng.random() as u8;
            if val > 250 {
                if self.grid[i] < NUM_BRIGHTNESS_STEPS {
                    self.grid[i] += 1;
                }
            } else if val < 5 {
                if self.grid[i] > 0 {
                    self.grid[i] -= 1;
                }
            }
        }
    }
}

