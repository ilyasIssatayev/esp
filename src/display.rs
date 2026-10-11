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

struct Display {
    ssd1306_driver: Ssd1306<
        I2CInterface<I2c<'static, Blocking>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
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
        Display{
            ssd1306_driver
        }
    }

    pub fn wipe(&mut self){
        self.ssd1306_driver.clear(BinaryColor::Off).unwrap();
    }
}
