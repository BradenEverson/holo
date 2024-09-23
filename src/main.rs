use holo::lcd::Ili9341Lcd;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 24_000_000, Mode::Mode0)
        .expect("Failed to initialize SPI");

    let mut lcd = Ili9341Lcd::new(25, 24, 18, spi, 240, 240).expect("Failed to initialize LCD Screen");
    lcd.init();

    lcd.draw_image("img/test.png").expect("Failed to write image to lcd");   
}

