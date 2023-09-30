use crate::{
    screen::Screen,
    config::{NUM_LINES, NUM_BRIGHTNESS_STEPS}
};

pub struct Test {
    centre_dark: u8,
}

impl Test {
    pub fn new() -> Self {
        Self {
            centre_dark: 0,
        }
    }
}

impl super::ModeTrait for Test {
    fn activate(&mut self, screen: &mut Screen) {
        for i in 0..NUM_LINES {
            for u in 0..NUM_LINES {
                screen.frame.set_xy(u, i, NUM_BRIGHTNESS_STEPS);
            }
        }
    }

    fn update(&mut self, screen: &mut Screen) {
        self.centre_dark = self.centre_dark % (NUM_LINES as u8) + 1;
        for i in 0..NUM_LINES {
            for u in 0..NUM_LINES {
                let distance: u8 = ((i as i8 - self.centre_dark as i8).abs() as u8 % NUM_LINES as u8).clamp(1, NUM_BRIGHTNESS_STEPS);
                screen.frame.set_xy(u, i, distance);
            }
        }
    }
}