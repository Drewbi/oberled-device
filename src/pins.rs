use hal::{
    IO,
    prelude::*,
    gpio::{
        GpioPin, 
        Output,
        Input,
        PushPull,
        PullUp, 
        Bank0GpioRegisterAccess, 
        DualCoreInteruptStatusRegisterAccessBank0, 
        InputOutputAnalogPinType,
        Gpio12Signals,
        Gpio14Signals,
        Gpio25Signals,
        Gpio26Signals,
        Gpio27Signals,
    },
};

use crate::utils::PixelState;

pub struct Pins {
    data_pin: GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio27Signals, 27>,
    clock_pin: GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio14Signals, 14>,
    latch_pin: GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio12Signals, 12>,
    enable_pin: GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio26Signals, 26>,

    button_pin: GpioPin<Input<PullUp>, Bank0GpioRegisterAccess, DualCoreInteruptStatusRegisterAccessBank0, InputOutputAnalogPinType, Gpio25Signals, 25>,
}

impl Pins {
    pub fn new(io: IO) -> Self {
        let mut data_pin = io.pins.gpio27.into_push_pull_output();
        let mut clock_pin = io.pins.gpio14.into_push_pull_output();
        let mut latch_pin = io.pins.gpio12.into_push_pull_output();
        let mut enable_pin = io.pins.gpio26.into_push_pull_output();
        
        let button_pin = io.pins.gpio25.into_pull_up_input();

        enable_pin.set_low().unwrap();
        data_pin.set_low().unwrap();
        clock_pin.set_low().unwrap();
        latch_pin.set_low().unwrap();

        Self {
            data_pin,
            clock_pin,
            latch_pin,
            enable_pin,
            button_pin,
        }
    }

    pub fn set_data(&mut self, value: PixelState) {
        match value {
            PixelState::On => self.data_pin.set_high().unwrap(),
            PixelState::Off => self.data_pin.set_low().unwrap(),
        }
    }

    pub fn cycle_clock(&mut self) {
        self.clock_pin.set_high().unwrap();
        self.clock_pin.set_low().unwrap();
    }

    pub fn cycle_latch(&mut self) {
        self.latch_pin.set_high().unwrap();
        self.latch_pin.set_low().unwrap();
    }
}