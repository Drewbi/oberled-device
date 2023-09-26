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

mod screen;
use screen::{ PixelValue, Screen };

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

    let mut screen = Screen::new(io);

    loop {
        for _ in 0..255 {
            screen.write_value(PixelValue::On);
            screen.cycle_latch();
            delay.delay_ms(100u8);
        }
        for _ in 0..255 {
            screen.write_value(PixelValue::Off);
            screen.cycle_latch();
            delay.delay_ms(10u8);
        }
    }
}
