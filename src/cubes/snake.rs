/// Snake shaped LED Cube
///
/// Despite it looks elegant I would not recommend building the Cube like that.
/// It is cumbersome to solder and if an LED breaks down somewhere inside the cube well good luck fixing that.
/// Because of that I'm like not going to add many effects for this wiring schema.
///
/// I call it snake shaped because all the cathodes are soldered together in a plane following an `S` or snake shaped form.
/// This gives you many planes that are then soldered together on the anodes.
///
/// Top view or one single plane of the layout looks like this (5x5x5, `<|` represents a LED):
/// cathode ---<|---<|---<|---<|---<|
///                                 -
///            <|---<|---<|---<|---<|
///             -
///            <|---<|---<|---<|---<|
///                                 -
///            <|---<|---<|---<|---<|
///             -
///            <|---<|---<|---<|---<|
///
/// Then each anode in the plane is soldered to the one exactly below. Side view:
/// cathode ---<|---<|---<|---<|---<|
///             +    +    +    +    +
/// cathode    <|---<|---<|---<|---<|
///             +    +    +    +    +
/// cathode    <|---<|---<|---<|---<|
///             +    +    +    +    +
/// cathode    <|---<|---<|---<|---<|
///             +    +    +    +    +
/// cathode    <|---<|---<|---<|---<|
///             +    +    +    +    +
///             +    +    +    +    +
///             ++++++++++++++++++++++ anode
///
use embedded_hal::blocking::delay::DelayMs;
use gd32vf103xx_hal::delay::McycleDelay;

use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::shiftreg::Driver;

pub struct Cube {
    pub edge_length: usize,
    pub tick_ms: u32,
}

#[allow(unused)]
impl Cube {
    pub fn effect_light_all(&self, driver: &mut Driver, delay: &mut McycleDelay) {
        for i in 0..self.edge_length * self.edge_length + self.edge_length {
            driver.set(i, true).unwrap();
        }
        driver.update().unwrap();
    }

    pub fn effect_light_none(&self, driver: &mut Driver, delay: &mut McycleDelay) {
        self.effect_light_all(driver, delay);
        for i in 0..self.edge_length * self.edge_length {
            driver.set(i, false).unwrap();
        }
        driver.update().unwrap();
    }

    pub fn effect_shift_planes2(
        &self,
        driver: &mut Driver,
        delay: &mut McycleDelay,
        n_loops: usize,
    ) {
        self.effect_light_all(driver, delay);
        for _ in 0..n_loops {
            let mut position = (self.edge_length * self.edge_length);
            for i in position..position + self.edge_length {
                driver.set(i, false).unwrap();
            }
            for i in 1..self.edge_length * 2 - 1 {
                driver.set(position, true).unwrap();
                driver.update().unwrap();

                delay.delay_ms(self.tick_ms);

                driver.set(position, false).unwrap();
                driver.update().unwrap();

                if i < self.edge_length {
                    position += 1
                } else {
                    position -= 1
                }
            }
        }
    }

    pub fn effect_shift_planes_fill_up(
        &self,
        driver: &mut Driver,
        delay: &mut McycleDelay,
        n_loops: usize,
    ) {
        self.effect_light_all(driver, delay);
        for _ in 0..n_loops {
            let position = (self.edge_length * self.edge_length);
            for i in position..position + self.edge_length {
                driver.set(i, false).unwrap();
            }
            driver.update().unwrap();
            for i in position..position + self.edge_length {
                driver.set(i, true).unwrap();
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
            }
            for i in position..position + self.edge_length {
                driver.set(i, false).unwrap();
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
            }
        }
    }
    pub fn effect_shift_planes_fill_down(
        &self,
        driver: &mut Driver,
        delay: &mut McycleDelay,
        n_loops: usize,
    ) {
        self.effect_light_all(driver, delay);
        for _ in 0..n_loops {
            let position = (self.edge_length * self.edge_length);
            for i in position..position + self.edge_length {
                driver.set(i, false).unwrap();
            }
            driver.update().unwrap();
            for i in 1..self.edge_length + 1 {
                driver.set(position + self.edge_length - i, true).unwrap();
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
            }
            for i in 1..self.edge_length + 1 {
                driver.set(position + self.edge_length - i, false).unwrap();
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
            }
        }
    }

    pub fn effect_shift_planes(
        &self,
        driver: &mut Driver,
        delay: &mut McycleDelay,
        n_loops: usize,
    ) {
        self.effect_light_all(driver, delay);
        for _ in 0..n_loops {
            let mut position = (self.edge_length * self.edge_length);
            for i in 1..self.edge_length * 2 - 1 {
                driver.set(position, false).unwrap();
                driver.update().unwrap();

                delay.delay_ms(self.tick_ms);

                driver.set(position, true).unwrap();
                driver.update().unwrap();

                if i < self.edge_length {
                    position += 1
                } else {
                    position -= 1
                }
            }
        }
    }

    pub fn effect_shift_walls2(
        &self,
        driver: &mut Driver,
        delay: &mut McycleDelay,
        n_loops: usize,
    ) {
        self.effect_light_none(driver, delay);
        let mut position = 0;
        for _ in 0..n_loops {
            for n in 1..self.edge_length * 2 - 1 {
                for i in 0..self.edge_length {
                    driver.set(i + position, true).unwrap();
                }
                driver.update().unwrap();

                delay.delay_ms(self.tick_ms);

                for i in 0..self.edge_length {
                    driver.set(i + position, false).unwrap();
                }
                driver.update().unwrap();

                if n < self.edge_length {
                    position += self.edge_length
                } else {
                    position -= self.edge_length
                }
            }
        }
    }

    pub fn effect_shift_walls(&self, driver: &mut Driver, delay: &mut McycleDelay, n_loops: usize) {
        self.effect_light_all(driver, delay);
        for _ in 0..n_loops {
            let mut position = 0;
            for n in 1..self.edge_length * 2 - 1 {
                for i in 0..self.edge_length {
                    driver.set(i + position, false).unwrap();
                }
                driver.update().unwrap();

                delay.delay_ms(self.tick_ms);

                for i in 0..self.edge_length {
                    driver.set(i + position, true).unwrap();
                }
                driver.update().unwrap();

                if n < self.edge_length {
                    position += self.edge_length
                } else {
                    position -= self.edge_length
                }
            }
        }
    }

    pub fn effect_random_rain(&self, driver: &mut Driver, delay: &mut McycleDelay, n_loops: usize) {
        self.effect_light_none(driver, delay);
        let mut rng = Pcg64::from_seed([42u8; 32]);
        let mut randint = rng.next_u32();
        let mask: u32 = 0;
        for _ in 0..n_loops {
            for bit in 0..self.edge_length * self.edge_length {
                if bit % 32 == 0 {
                    randint = rng.next_u32();
                }
                let index = bit % 32;
                if 1u32 << index & randint == 0 {
                    driver.set(index as usize, false).unwrap()
                } else {
                    driver.set(index as usize, true).unwrap()
                }
            }
            driver.update().unwrap();
            delay.delay_ms(self.tick_ms);
        }
    }

    pub fn effect_snake_walk(&self, driver: &mut Driver, delay: &mut McycleDelay, n_loops: usize) {
        self.effect_light_none(driver, delay);
        for _ in 0..n_loops {
            for i in 0..self.edge_length * self.edge_length {
                driver.set(i as usize, true).unwrap();
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
                driver.set(i as usize, false).unwrap();
                driver.update().unwrap();
            }
        }
    }

    pub fn effect_scissors(&self, driver: &mut Driver, delay: &mut McycleDelay, n_loops: usize) {
        for _ in 0..n_loops {
            self.effect_light_none(driver, delay);
            for i in 0..self.edge_length {
                for n in 0..self.edge_length {
                    driver.set(i + n * 5, true).unwrap();
                    if i > 0 {
                        driver.set(i - 1 + n * 5, false).unwrap();
                    }
                }
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
            }
        }
    }

    pub fn effect_scissors_fill(
        &self,
        driver: &mut Driver,
        delay: &mut McycleDelay,
        n_loops: usize,
    ) {
        for _ in 0..n_loops {
            self.effect_light_none(driver, delay);
            for i in 0..self.edge_length {
                for n in 0..self.edge_length {
                    driver.set(i + n * 5, true).unwrap();
                }
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
            }
            for i in 0..self.edge_length {
                for n in 0..self.edge_length {
                    driver.set(i + n * 5, false).unwrap();
                }
                driver.update().unwrap();
                delay.delay_ms(self.tick_ms);
            }
        }
    }
}
