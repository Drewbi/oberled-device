use esp_println::println;
use hal::{
    IO,
    prelude::*,
    gpio::{
        GpioPin, 
        Gpio27Signals,
        Gpio14Signals,
        Gpio12Signals,
        Output, 
        PushPull, 
        Bank0GpioRegisterAccess, 
        DualCoreInteruptStatusRegisterAccessBank0, 
        InputOutputAnalogPinType
    }, 
};

#[derive(PartialEq)]
pub enum PixelValue {
    Off,
    On
}

pub struct Screen {
    data_pin: GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio27Signals, 27>,
    clock_pin: GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio14Signals, 14>,
    latch_pin: GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio12Signals, 12>,
}

impl Screen {
    pub fn new(io: IO) -> Screen {
        println!("Instantiating Screen");

        let mut data_pin = io.pins.gpio27.into_push_pull_output();
        let mut clock_pin = io.pins.gpio14.into_push_pull_output();
        let mut latch_pin = io.pins.gpio12.into_push_pull_output();
        let mut enable_pin = io.pins.gpio26.into_push_pull_output();
        // let mut button_pin = io.pins.gpio25.into_pull_up_input();

        enable_pin.set_low().unwrap();
        data_pin.set_low().unwrap();
        clock_pin.set_low().unwrap();
        latch_pin.set_low().unwrap();

        Screen {
            data_pin,
            clock_pin,
            latch_pin,
        }
    }

    pub fn write_value(&mut self, value: PixelValue) {
        if value == PixelValue::On {
            self.data_pin.set_high().unwrap();
        } else {
            self.data_pin.set_low().unwrap();
        }
        self.clock_pin.set_high().unwrap();
        self.clock_pin.set_low().unwrap();
    }

    pub fn cycle_latch(&mut self) {
        self.latch_pin.set_high().unwrap();
        self.latch_pin.set_low().unwrap();
    }
}