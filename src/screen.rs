use esp_println::println;

use crate::pins::Pins;
use crate::utils::{ Orientation, PixelState };
use crate::frame::Frame;

pub struct Screen {
    pins: Pins,
    orientation: Orientation,
    pub frame: Frame,
}

impl Screen {
    pub fn new(pins: Pins, orientation: Orientation, steps: u8) -> Self {
        println!("Instantiating screen with {} steps", steps);
        Self {
            pins,
            orientation,
            frame: Frame::new(steps),
        }
    }

    pub fn display(&mut self) {
        for i in 0..self.frame.steps {
            for u in 0..255 {
                if self.frame.data[u] > i {
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