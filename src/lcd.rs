use std::{error::Error, thread::sleep, time::Duration};

use image::{imageops::FilterType, ImageBuffer, Rgb};
use rppal::{
    gpio::{Gpio, OutputPin},
    spi::Spi,
};

pub struct Ili9341Lcd {
    dc_pin: OutputPin,
    rst_pin: OutputPin,
    bl_pin: OutputPin,
    spi: Spi,

    width: usize,
    height: usize,
}

impl Ili9341Lcd {
    pub fn new(
        dc: u8,
        rst: u8,
        bl: u8,
        spi: Spi,
        width: usize,
        height: usize,
    ) -> Result<Ili9341Lcd, Box<dyn Error>> {
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

    pub fn draw_image(&mut self, img_str: &str) -> Result<(), Box<dyn Error>> {
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

        self.lcd_command(0xEF);
        self.lcd_data(0x03);
        self.lcd_data(0x80);
        self.lcd_data(0x02);

        self.lcd_command(0xCF);
        self.lcd_data(0x00);
        self.lcd_data(0xC1);
        self.lcd_data(0x30);

        self.lcd_command(0xED);
        self.lcd_data(0x64);
        self.lcd_data(0x03);
        self.lcd_data(0x12);
        self.lcd_data(0x81);

        self.lcd_command(0xE8);
        self.lcd_data(0x85);
        self.lcd_data(0x00);
        self.lcd_data(0x78);

        self.lcd_command(0xCB);
        self.lcd_data(0x39);
        self.lcd_data(0x2C);
        self.lcd_data(0x00);
        self.lcd_data(0x34);
        self.lcd_data(0x02);

        self.lcd_command(0xF7);
        self.lcd_data(0x20);

        self.lcd_command(0xEA);
        self.lcd_data(0x00);
        self.lcd_data(0x00);

        self.lcd_command(0xC0); // Power control 1
        self.lcd_data(0x23);

        self.lcd_command(0xC1); // Power control 2
        self.lcd_data(0x10);

        self.lcd_command(0xC5); // VCOM control 1
        self.lcd_data(0x3e);
        self.lcd_data(0x28);

        self.lcd_command(0xC7); // VCOM control 2
        self.lcd_data(0x86);

        self.lcd_command(0x36); // Memory Access Control
        self.lcd_data(0x48);

        self.lcd_command(0x3A); // Pixel Format Set
        self.lcd_data(0x55); // 16-bit/pixel

        self.lcd_command(0xB1); // Frame Rate Control
        self.lcd_data(0x00);
        self.lcd_data(0x18);

        self.lcd_command(0xB6); // Display Function Control
        self.lcd_data(0x08);
        self.lcd_data(0x82);
        self.lcd_data(0x27);

        self.lcd_command(0xF2); // 3Gamma Function Disable
        self.lcd_data(0x00);

        self.lcd_command(0x26); // Gamma curve selected
        self.lcd_data(0x01);

        self.lcd_command(0xE0); // Set Gamma
        self.lcd_data(0x0F);
        self.lcd_data(0x31);
        self.lcd_data(0x2B);
        self.lcd_data(0x0C);
        self.lcd_data(0x0E);
        self.lcd_data(0x08);
        self.lcd_data(0x4E);
        self.lcd_data(0xF1);
        self.lcd_data(0x37);
        self.lcd_data(0x07);
        self.lcd_data(0x10);
        self.lcd_data(0x03);
        self.lcd_data(0x0E);
        self.lcd_data(0x09);
        self.lcd_data(0x00);

        self.lcd_command(0xE1); // Set Gamma
        self.lcd_data(0x00);
        self.lcd_data(0x0E);
        self.lcd_data(0x14);
        self.lcd_data(0x03);
        self.lcd_data(0x11);
        self.lcd_data(0x07);
        self.lcd_data(0x31);
        self.lcd_data(0xC1);
        self.lcd_data(0x48);
        self.lcd_data(0x08);
        self.lcd_data(0x0F);
        self.lcd_data(0x0C);
        self.lcd_data(0x31);
        self.lcd_data(0x36);
        self.lcd_data(0x0F);

        self.lcd_command(0x11); // Exit Sleep
        sleep(Duration::from_millis(120));
        self.lcd_command(0x29); // Display on

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
