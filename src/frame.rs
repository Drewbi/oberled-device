use esp_println::println;

use crate::config::{ NUM_PIXELS, NUM_LINES };

#[derive(Copy, Clone)]
pub struct Frame {
    pub data: [u8; NUM_PIXELS],
    pub steps: u8,
}

impl Frame {
    pub fn new(steps: u8) -> Self {
        Self {
            data: [0; NUM_PIXELS],
            steps,
        }
    }

    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    pub fn set_xy(&mut self, x: usize, y: usize, value: u8) {
        if x < NUM_LINES && y < NUM_LINES {
            self.data[x + y * NUM_LINES] = value;
        } else {
            println!("Exceeded range with x {} y {} - value {}", x, y, value);
        }
    }
    
    pub fn set_index(&mut self, index: usize, value: u8) {
        if index < NUM_PIXELS {
            self.data[index] = value
        } else {
            println!("Exceeded range with index {} - value {}", index, value);
        }
    }
}