use std::{error::Error, path::Path, thread::sleep, time::Duration};

use image::{imageops::FilterType, ImageBuffer, Rgb};
use rppal::{
    gpio::{Gpio, OutputPin},
    spi::Spi,
};

pub struct St7789Lcd {
    dc_pin: OutputPin,
    rst_pin: OutputPin,
    bl_pin: OutputPin,
    spi: Spi,

    width: usize,
    height: usize,
}

impl St7789Lcd {
    pub fn new(
        dc: u8,
        rst: u8,
        bl: u8,
        spi: Spi,
        width: usize,
        height: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        Ok(Self {
            dc_pin: gpio.get(dc)?.into_output(),
            rst_pin: gpio.get(rst)?.into_output(),
            bl_pin: gpio.get(bl)?.into_output(),

            spi,

            width,
            height,
        })
    }

    pub fn draw_image<P: AsRef<Path>>(&mut self, img_str: P) -> Result<(), Box<dyn Error>> {
        let img = image::open(img_str)?;
        let img = img.resize(self.width as u32, self.height as u32, FilterType::Nearest);

        self.send_image(&img.to_rgb8())
    }

    pub fn send_image(
        &mut self,
        img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        self.lcd_command(0x2A);
        self.lcd_data(0x00);
        self.lcd_data(0x00);
        self.lcd_data((self.width - 1 >> 8) as u8);
        self.lcd_data((self.width - 1 & 0xFF) as u8);

        self.lcd_command(0x2B);
        self.lcd_data(0x00);
        self.lcd_data(0x00);
        self.lcd_data((self.height - 1 >> 8) as u8);
        self.lcd_data((self.height - 1 & 0xFF) as u8);

        self.lcd_command(0x2C);

        for pixel in img.pixels() {
            let r = pixel[0] >> 3;
            let g = pixel[1] >> 2;
            let b = pixel[2] >> 3;
            let color: u16 = ((r as u16) << 11) | ((g as u16) << 5) | (b as u16);

            self.dc_pin.set_high();
            self.spi.write(&[(color >> 8) as u8, color as u8])?;
        }

        Ok(())
    }

    pub fn init(&mut self) {
        self.rst_pin.set_low();
        sleep(Duration::from_millis(100));
        self.rst_pin.set_high();
        sleep(Duration::from_millis(100));

        self.lcd_command(0x36); // Memory Data Access Control
        self.lcd_data(0x00); // Set rotation, RGB format

        self.lcd_command(0x3A); // Interface Pixel Format
        self.lcd_data(0x55); // 16-bit color format

        self.lcd_command(0xB2); // Porch Setting
        self.lcd_data(0x0C);
        self.lcd_data(0x0C);
        self.lcd_data(0x00);
        self.lcd_data(0x33);
        self.lcd_data(0x33);

        self.lcd_command(0xB7); // Gate Control
        self.lcd_data(0x35); // Default value

        self.lcd_command(0xBB); // VCOM Setting
        self.lcd_data(0x19); // Default value

        self.lcd_command(0xC0); // LCM Control
        self.lcd_data(0x2C); // Default value

        self.lcd_command(0xC2); // VDV and VRH Command Enable
        self.lcd_data(0x01); // Default value

        self.lcd_command(0xC3); // VRH Set
        self.lcd_data(0x12); // Default value

        self.lcd_command(0xC4); // VDV Set
        self.lcd_data(0x20); // Default value

        self.lcd_command(0xC6); // Frame Rate Control in Normal Mode
        self.lcd_data(0x0F); // Default value

        self.lcd_command(0xD0); // Power Control
        self.lcd_data(0xA4); // Default value
        self.lcd_data(0xA1); // Default value

        self.lcd_command(0xE0); // Positive Voltage Gamma Control
        self.lcd_data(0xD0);
        self.lcd_data(0x04);
        self.lcd_data(0x0D);
        self.lcd_data(0x11);
        self.lcd_data(0x13);
        self.lcd_data(0x2B);
        self.lcd_data(0x3F);
        self.lcd_data(0x54);
        self.lcd_data(0x4C);
        self.lcd_data(0x18);
        self.lcd_data(0x0D);
        self.lcd_data(0x0B);
        self.lcd_data(0x1F);
        self.lcd_data(0x23);

        self.lcd_command(0xE1); // Negative Voltage Gamma Control
        self.lcd_data(0xD0);
        self.lcd_data(0x04);
        self.lcd_data(0x0C);
        self.lcd_data(0x11);
        self.lcd_data(0x13);
        self.lcd_data(0x2C);
        self.lcd_data(0x3F);
        self.lcd_data(0x44);
        self.lcd_data(0x51);
        self.lcd_data(0x2F);
        self.lcd_data(0x1F);
        self.lcd_data(0x1F);
        self.lcd_data(0x20);
        self.lcd_data(0x23);

        self.lcd_command(0x21); // Inversion On
        self.lcd_command(0x11); // Sleep Out
        sleep(Duration::from_millis(120));
        self.lcd_command(0x29); // Display ON
        self.bl_pin.set_high();
    }

    fn lcd_data(&mut self, data: u8) {
        self.dc_pin.set_high();
        self.spi.write(&[data]).expect("Failed to send data");
    }

    fn lcd_command(&mut self, cmd: u8) {
        self.dc_pin.set_low();
        self.spi.write(&[cmd]).expect("Failed to send command");
    }
}
