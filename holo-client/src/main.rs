use holo::lcd::St7789Lcd;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() {
    // Setup
    for _ in 0..10 {
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 24_000_000, Mode::Mode3)
            .expect("Failed to initialize SPI");

        let mut lcd = St7789Lcd::new(25, 24, 18, spi, 240, 240).expect("Failed to initialize LCD Screen");
        lcd.init();
    }

    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 24_000_000, Mode::Mode3)
        .expect("Failed to initialize SPI");

    let mut lcd = St7789Lcd::new(25, 24, 18, spi, 240, 240).expect("Failed to initialize LCD Screen");
    lcd.init();

    lcd.draw_image("img/test.png").expect("Failed to write image to lcd");   
}

