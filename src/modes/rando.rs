use crate::{
    screen::Screen,
    config::{NUM_LINES, NUM_PIXELS, NUM_BRIGHTNESS_STEPS}
};
use hal::Rng;

pub struct Rando<'a> {
    rng: Rng<'a>,
}

impl<'a> Rando<'a> {
    pub fn new(rng: Rng<'a>) -> Self {
        Self {
            rng,
        }
    }
}

impl super::ModeTrait for Rando<'_> {
    fn activate(&mut self, screen: &mut Screen) {
        for i in 0..NUM_LINES {
            for u in 0..NUM_LINES {
                screen.frame.set_xy(u, i, 0);
            }
        }
    }

    fn update(&mut self, screen: &mut Screen) {
        for i in 0..NUM_PIXELS {
            screen.frame.set_index(i, 0);
        }
        for i in 0..NUM_LINES {
            let rand_i = self.rng.random() % NUM_PIXELS as u32;
            screen.frame.set_index(rand_i as usize, NUM_BRIGHTNESS_STEPS);
        }
    }
}