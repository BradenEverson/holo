use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::gpio::{Gpio, OutputPin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut spi = Spi::new(
        Bus::Spi0,        // Use SPI bus 0
        SlaveSelect::Ss0, // Use Chip Select 0 (CE0)
        15_000_000,       // SPI clock speed (15 MHz for stability)
        Mode::Mode3,      // SPI mode 3 (CPOL=1, CPHA=1)
    )
    .expect("Failed to initialize SPI");

    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    let mut dc_pin = gpio.get(25).expect("Failed to get GPIO25").into_output();
    let mut rst_pin = gpio.get(24).expect("Failed to get GPIO24").into_output();
    let mut bl_pin = gpio.get(23).expect("Failed to get GPIO18").into_output();

    init_display(&mut spi, &mut dc_pin, &mut rst_pin, &mut bl_pin);

    draw_image(&mut spi, &mut dc_pin);
}

fn init_display(spi: &mut Spi, dc: &mut OutputPin, rst: &mut OutputPin, bl: &mut OutputPin) {
    rst.set_high();
    sleep(Duration::from_millis(100));
    rst.set_low();
    sleep(Duration::from_millis(100));
    rst.set_high();
    sleep(Duration::from_millis(100));

    bl.set_high();

    send_command(spi, dc, 0x01);
    sleep(Duration::from_millis(150));

    send_command(spi, dc, 0x11); 
    sleep(Duration::from_millis(500));

    send_command(spi, dc, 0x3A); 
    send_data(spi, dc, &[0x55]); 

    send_command(spi, dc, 0x36); 
    send_data(spi, dc, &[0x00]); 

    send_command(spi, dc, 0x29); 
    sleep(Duration::from_millis(100));
}

fn draw_image(spi: &mut Spi, dc: &mut OutputPin) {
    set_window(spi, dc, 0, 0, 239, 239);

    send_command(spi, dc, 0x2C); // Memory Write

    let width = 240;
    let height = 240;
    let mut image_data = vec![0u8; width * height * 2]; // Each pixel takes 2 bytes

    for y in 0..height {
        for x in 0..width {
            let r = ((x * 31) / width) as u16;
            let g = ((y * 63) / height) as u16;
            let b = (((x + y) * 31) / (width + height)) as u16;

            let color = ((r & 0x1F) << 11) | ((g & 0x3F) << 5) | (b & 0x1F);

            let index = ((x + y * width) * 2) as usize;
            image_data[index] = (color >> 8) as u8;
            image_data[index + 1] = (color & 0xFF) as u8;
        }
    }

    dc.set_high(); // Data mode
    spi.write(&image_data).expect("Failed to write image data");
}

fn send_command(spi: &mut Spi, dc: &mut OutputPin, command: u8) {
    dc.set_low(); // Command mode
    spi.write(&[command]).expect("Failed to send command");
}

fn send_data(spi: &mut Spi, dc: &mut OutputPin, data: &[u8]) {
    dc.set_high(); // Data mode
    spi.write(data).expect("Failed to send data");
}

fn set_window(spi: &mut Spi, dc: &mut OutputPin, x0: u16, y0: u16, x1: u16, y1: u16) {
    // Column Address Set
    send_command(spi, dc, 0x2A);
    send_data(
        spi,
        dc,
        &[
            (x0 >> 8) as u8,
            (x0 & 0xFF) as u8,
            (x1 >> 8) as u8,
            (x1 & 0xFF) as u8,
        ],
    );

    // Row Address Set
    send_command(spi, dc, 0x2B);
    send_data(
        spi,
        dc,
        &[
            (y0 >> 8) as u8,
            (y0 & 0xFF) as u8,
            (y1 >> 8) as u8,
            (y1 & 0xFF) as u8,
        ],
    );
}
