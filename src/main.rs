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
    IO, Rng,
};

use pins::{Pins, BUTTON_MUTEX};
use screen::Screen;
use utils::Orientation;

use crate::{
    config::{TICKS_PER_UPDATE, NUM_BRIGHTNESS_STEPS},
    modes::Modes
};

mod utils;
mod pins;
mod screen;
mod frame;
mod map;
mod config;
mod modes;

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

    let mut tick_timer = timer_group0.timer0;
    tick_timer.start(5u64.secs());
    // tick_timer.listen();

    let pins = Pins::new(io);

    let mut screen: Screen = Screen::new(pins, Orientation::Landscape, NUM_BRIGHTNESS_STEPS);
    
    // let mut delay = Delay::new(&clocks);

    // println!("Initialising screen mode");
    // unsafe {
    //     SCREEN_MODE = Some(VolatileRef::new(NonNull::new(&mut SCREEN_MODE_STORAGE as *mut _).unwrap()));
    // }

    let rng = Rng::new(peripherals.RNG);
    let mut modes: Modes = Modes::new(&mut screen, rng);

    let mut old_time = tick_timer.now();

    loop {
        let new_time = tick_timer.now();
        // println!("old {}", old_time);
        // println!("new {}", new_time);
        if new_time < old_time {
            old_time = new_time;
        }
        let delta = new_time - old_time;
        
        if delta >= TICKS_PER_UPDATE {
            modes.update(&mut screen);
            old_time = new_time;
        }
        screen.display();
    }
}

// fn current_time_millis(clocks) -> u64 {
//     clocks
// }

#[ram]
#[interrupt]
unsafe fn GPIO() {
    critical_section::with(|cs| {
        println!("button interrupt");
        if let Some(ref mut button_pin) = *BUTTON_MUTEX.borrow(cs).borrow_mut() {
            if button_pin.is_low().unwrap() {
                println!("Button is low.");
            }
            button_pin.clear_interrupt();
        }
    });
}