#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Rtc,
    Delay,
    IO
};

use pins::Pins;
use screen::Screen;
use utils::Orientation;

use crate::config::NUM_LINES;

mod utils;
mod pins;
mod screen;
mod frame;
mod map;
mod config;

#[entry]
fn main() -> ! {
    println!("Initialising system");
    let peripherals = Peripherals::take();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    println!("Disabling watchdogs");
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt = timer_group0.wdt;
    
    // Disable MWDT and RWDT (Watchdog) flash boot protection
    wdt.disable();
    rtc.rwdt.disable();

    let mut delay = Delay::new(&clocks);

    let steps: u8 = 4;
    let mut screen: Screen = Screen::new(Pins::new(io), Orientation::Landscape, steps);
    screen.frame.set_index(0, 4);
    for i in 0..NUM_LINES {
        screen.frame.set_xy(0, i, 4);
        screen.frame.set_xy(NUM_LINES - 1, i, 4);
        screen.frame.set_xy(i, 0, 4);
        screen.frame.set_xy(i, NUM_LINES - 1, 4);
    }

    loop {
        screen.display();
    }
}
