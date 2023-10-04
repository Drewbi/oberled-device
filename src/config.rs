pub const NUM_PIXELS: usize = 256;
pub const NUM_LINES: usize = 16;

pub const NUM_BRIGHTNESS_STEPS: u8 = 3;

const FPS: u64 = 20;
const MAGIC_TICKS_PER_SECOND: u64 = 50 * 1000 * 1000;
pub const TICKS_PER_UPDATE: u64 = MAGIC_TICKS_PER_SECOND / FPS;