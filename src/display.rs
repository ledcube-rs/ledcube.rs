use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitive_style;
use embedded_graphics::primitives::Rectangle;
use longan_nano::lcd;

const FERRIS: &[u8] = include_bytes!("ferris.raw");

pub struct Display {
    driver: lcd::Lcd,
    width: i32,
    height: i32,
}

impl Display {
    pub fn new(driver: lcd::Lcd, width: i32, height: i32) -> Display {
        Display {
            driver,
            width,
            height,
        }
    }

    pub fn draw_ferris(&mut self) {
        // Clear screen
        Rectangle::new(
            Point::new(0, 0),
            Point::new(self.width - 1, self.height - 1),
        )
        .into_styled(primitive_style!(fill_color = Rgb565::BLACK))
        .draw(&mut self.driver)
        .unwrap();

        // Load Image Data
        let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&FERRIS, 86, 64);
        Image::new(
            &raw_image,
            Point::new(self.width / 2 - 43, self.height / 2 - 32),
        )
        .draw(&mut self.driver)
        .unwrap();
    }
}
