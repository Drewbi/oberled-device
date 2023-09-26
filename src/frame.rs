#[derive(Copy, Clone)]
pub struct Frame {
    pub data: [u8; 255],
    pub steps: u8,
}

impl Frame {
    pub fn new(steps: u8) -> Self {
        Self {
            data: [0; 255],
            steps,
        }
    }

    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    pub fn set_xy(&mut self, x: usize, y: usize, value: u8) {
        if x > 0 && x < 16 && y > 0 && y < 16 {
            self.data[x + y * 16] = value;
        } 
    }

    pub fn set_index(&mut self, index: usize, value: u8) {
        if index > 0 && index < 256 {
            self.data[index] = value
        }
    }
}