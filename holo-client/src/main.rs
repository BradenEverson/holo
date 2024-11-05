//! Main firmware for the physical hologram, pings the attached server for images every time a new
//! image is requested

use std::{thread, time::Duration};

use holo_client::lcd::St7789Lcd;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

#[tokio::main]
async fn main() {
    // Setup
    for _ in 0..10 {
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 24_000_000, Mode::Mode3)
            .expect("Failed to initialize SPI");

        let mut lcd =
            St7789Lcd::new(25, 24, 18, spi, 240, 240).expect("Failed to initialize LCD Screen");
        lcd.init();
    }

    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 24_000_000, Mode::Mode3)
        .expect("Failed to initialize SPI");

    let mut lcd =
        St7789Lcd::new(25, 24, 18, spi, 240, 240).expect("Failed to initialize LCD Screen");
    lcd.init();

    loop {
        /*if let Some(img) = choose_random_file("img/", &mut rng) {
            println!("Got image {:?}", img);
            lcd.draw_image(&img).expect("Failed to draw image");
        } else {
            println!("No images, trying again later")
        }*/
        thread::sleep(Duration::from_secs(/*10 **/ 60))
    }
}
