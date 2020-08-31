#![no_std]
#![no_main]

use panic_halt as _;

use embedded_graphics::prelude::*;
use embedded_hal::blocking::delay::DelayMs;
use gd32vf103xx_hal::delay::McycleDelay;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use longan_nano::{lcd, lcd_pins};
use riscv_rt::entry;

mod shiftreg;
use shiftreg::Driver;

mod display;
use display::Display;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks and GPIO
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);
    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    // LCD Display: How does not like ferris
    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);
    let mut display = Display::new(lcd, width, height);
    display.draw_ferris();

    // LED: How does not like a green LED
    gpioa
        .pa1
        .into_push_pull_output_with_state(gd32vf103xx_hal::gpio::State::Low);

    let mut delay = McycleDelay::new(&rcu.clocks);

    // Setup Shiftregister
    let mut shiftreg = match Driver::new(reg_pins!(gpioa), 8) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    delay.delay_ms(2000);
    shiftreg.set(1, true).unwrap();
    shiftreg.set(3, true).unwrap();
    shiftreg.set(5, true).unwrap();
    shiftreg.set(7, true).unwrap();
    shiftreg.update().unwrap();
    shiftreg.update().unwrap();

    loop {
        delay.delay_ms(20)
    }
}
