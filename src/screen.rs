use esp_println::println;

use crate::config::{NUM_PIXELS, NUM_BRIGHTNESS_STEPS};
use crate::map::{PORTRAIT_MAP, LANDSCAPE_MAP, LANDSCAPE_FLIPPED_MAP, PORTRAIT_FLIPPED_MAP};
use crate::pins::Pins;
use crate::utils::{ Orientation, PixelState };
use crate::frame::Frame;

pub struct Screen {
    pins: Pins,
    map: [usize; NUM_PIXELS],
    pub frame: Frame,
}

impl Screen {
    pub fn new(pins: Pins, orientation: Orientation, steps: u8) -> Self {
        println!("Instantiating screen with {} steps", steps);
        let map = match orientation {
            Orientation::Landscape => LANDSCAPE_MAP,
            Orientation::Portrait => PORTRAIT_MAP,
            Orientation::LandscapeFlipped => LANDSCAPE_FLIPPED_MAP,
            Orientation::PortraitFlipped => PORTRAIT_FLIPPED_MAP,
        };

        Self {
            pins,
            map,
            frame: Frame::new(steps),
        }
    }

    pub fn display(&mut self) {
        for i in 0..self.frame.steps {
            for u in 0..NUM_PIXELS {
                if self.frame.data[self.map[u]] > i {
                    self.pins.set_data(PixelState::On);
                } else {
                    self.pins.set_data(PixelState::Off)
                };
                self.pins.cycle_clock();
            }
            self.pins.cycle_latch();
        }
    }
}