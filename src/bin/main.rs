#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use log::info;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::time::Rate;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};
use core::fmt::Write;
use heapless::String;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32 -o esp-backtrace -o log

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let i2c_config = I2cConfig::default()
        .with_frequency(Rate::from_khz(400));
    let i2c = I2c::new(peripherals.I2C0,i2c_config)
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let mut counter = 0;
    loop {
        display.clear(BinaryColor::Off).unwrap();
        let mut message: String<32> = String::new();
        write!(message, "counter {}", counter).unwrap();
        Text::new(&message, Point::new(8,16), text_style)
            .draw(&mut display)
            .unwrap();

        Rectangle::new(Point::new(8,25),Size::new(50,28))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On,1))
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        counter += 1;
        delay.delay_millis(1000);
    }
}
