use esp_hal::{
    Blocking,
    i2c::master::{Config as I2cConfig, I2c},
    peripherals::Peripherals,
    time::Rate,
};
use ssd1306::{
    I2CDisplayInterface,
    Ssd1306,
    mode::DisplayConfig,
    prelude::{DisplayRotation, DisplaySize128x64, I2CInterface},
};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};
use ssd1306::mode::BufferedGraphicsMode;

pub struct Display {
    ssd1306_driver: Ssd1306<
        I2CInterface<I2c<'static, Blocking>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
    text_style:MonoTextStyle<'static,BinaryColor>,
    line_height: i32,
}

impl Display {
    pub fn new(peripherals: Peripherals) -> Display {
        let i2c_config = I2cConfig::default()
            .with_frequency(Rate::from_khz(400));
        let i2c = I2c::new(peripherals.I2C0,i2c_config)
            .unwrap()
            .with_sda(peripherals.GPIO21)
            .with_scl(peripherals.GPIO22);

        let interface = I2CDisplayInterface::new(i2c);
        let mut ssd1306_driver = Ssd1306::new(
            interface,
            DisplaySize128x64,
            DisplayRotation::Rotate0,
        ).into_buffered_graphics_mode();
        ssd1306_driver.init().unwrap();
        ssd1306_driver.clear(BinaryColor::Off).unwrap();
        _ = ssd1306_driver.flush();

        let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

        Display{
            ssd1306_driver,
            text_style,
            line_height: 8,
        }
    }

    pub fn wipe(&mut self){
        self.ssd1306_driver.clear(BinaryColor::Off).unwrap();
    }

    pub fn draw_line(&mut self, line_pos: i32,text: &str) {
        Text::new(text, Point::new(0,(1 + line_pos)*self.line_height), self.text_style)
            .draw(&mut self.ssd1306_driver)
            .unwrap();

    }

    pub fn draw_logs(&mut self, logs:[&str;8]) {
        self.wipe();
        for (index,&log) in logs.iter().enumerate() {
            self.draw_line(index as i32,log);
        }
        self.ssd1306_driver.flush().unwrap();
    }
}
