use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::gpio::gpioa::{PA0, PA12, PA4};
use gd32vf103xx_hal::gpio::{Output, PushPull};

/// Sets up the needed GPIO pins for the Shiftregister
/// Wiring:
/// PA12: Data
/// PA4: Clock
/// PA0: Latch
///
/// Setup:
/// ```
/// let gpioa = dp.GPIOA.split(&mut rcu);
/// let reg_pins = reg_pins!(gpioa);
/// ```
#[macro_export]
macro_rules! reg_pins {
    ($gpioa:ident) => {{
        $crate::shiftreg::Pins {
            data: $gpioa
                .pa12
                .into_push_pull_output_with_state(gd32vf103xx_hal::gpio::State::Low),
            clock: $gpioa
                .pa4
                .into_push_pull_output_with_state(gd32vf103xx_hal::gpio::State::Low),
            latch: $gpioa
                .pa0
                .into_push_pull_output_with_state(gd32vf103xx_hal::gpio::State::Low),
        }
    }};
}

const MAX_BITS: usize = 2048;

pub struct Driver {
    registers: [bool; MAX_BITS],
    pins: Pins,
    bits: usize,
}

pub struct Pins {
    pub data: PA12<Output<PushPull>>,
    pub clock: PA4<Output<PushPull>>,
    pub latch: PA0<Output<PushPull>>,
}

impl Driver {
    pub fn new(pins: Pins, bits: usize) -> Result<Driver, &'static str> {
        if bits >= MAX_BITS {
            return Err("bits exceed MAX_BITS in shifregister");
        }
        Ok(Driver {
            pins,
            registers: [false; MAX_BITS],
            bits,
        })
    }

    pub fn set(&mut self, index: usize, bit: bool) -> Result<(), &'static str> {
        if index >= self.bits {
            return Err("index out of range");
        }
        self.registers[index] = bit;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), &'static str> {
        for bit in 0..self.bits {
            self.pins.latch.set_low().unwrap();
            match self.registers[bit] {
                true => self.pins.data.set_high().unwrap(),
                false => self.pins.data.set_low().unwrap(),
            }
            self.pins.clock.set_high().unwrap();
            self.pins.clock.set_low().unwrap();
            self.pins.latch.set_high().unwrap();
        }
        Ok(())
    }
}
