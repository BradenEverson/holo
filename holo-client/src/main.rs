//! Main firmware for the physical hologram, pings the attached server for images every time a new
//! image is requested

use holo_client::lcd::St7789Lcd;
use image::{DynamicImage, ImageReader};
use reqwest::Client;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::{thread, time::Duration};

/// The server hosting the random image collection
pub const IMG_SOURCE: &str = "https://holoserve-fe9fc0f69a47.herokuapp.com/img";

#[tokio::main]
async fn main() {
    // Setup
    for _ in 0..15 {
        let _ = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 24_000_000, Mode::Mode3)
            .expect("Failed to initialize SPI");
    }
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 24_000_000, Mode::Mode3)
        .expect("Failed to initialize SPI");

    let mut lcd =
        St7789Lcd::new(25, 24, 18, spi, 240, 240).expect("Failed to initialize LCD Screen");
    lcd.init();

    // HTTP client for fetching images
    let client = Client::new();

    loop {
        match fetch_image(&client).await {
            Ok(image) => {
                println!("Image fetched successfully, drawing...");
                if let Err(e) = lcd.draw_image(image) {
                    eprintln!("Failed to draw image: {}", e);
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch image: {}", err);
            }
        }

        thread::sleep(Duration::from_secs(60)); // Sleep for 1 minute
    }
}

/// Fetches an image from IMG_SOURCE and returns it as raw pixel data
async fn fetch_image(client: &Client) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let response = client.get(IMG_SOURCE).send().await?;
    let bytes = response.bytes().await?;
    let img = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    Ok(img)
}
