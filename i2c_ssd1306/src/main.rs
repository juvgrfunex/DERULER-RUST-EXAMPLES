#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Acquire the Peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIO
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure the pins
    let scl = gpiob.pb6.into_open_drain_output();
    let sda = gpiob.pb7.into_open_drain_output();
    let btn1 = gpiob.pb3.into_pull_up_input();
    let btn2 = gpioa.pa15.into_pull_up_input();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    // Setup I2C Peripheral
    let i2c = dp.I2C1.i2c(sda, scl, 400_000.Hz(), &mut rcc);

    // Configure and init ssd1306 display
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Create primary image
    let primary_raw: ImageRaw<BinaryColor> =
        ImageRaw::new(include_bytes!("../primary_image.raw"), 64);
    let primary_image = Image::new(&primary_raw, Point::new(32, 0));

    // Create secondary image
    let secondary_raw: ImageRaw<BinaryColor> =
        ImageRaw::new(include_bytes!("../secondary_image.raw"), 64);
    let secondary_image = Image::new(&secondary_raw, Point::new(32, 0));

    // Draw primary image
    primary_image.draw(&mut display).unwrap();
    display.flush().unwrap();

    loop {
        // Check if any button is pressed
        if btn1.is_low().unwrap() {
            // Button 1 is pressed
            primary_image.draw(&mut display).unwrap(); // Draw primary image
            display.flush().unwrap();
        } else if btn2.is_low().unwrap() {
            // Button 2 is pressed
            secondary_image.draw(&mut display).unwrap(); // Draw secondary image
            display.flush().unwrap();
        }
        delay.delay_ms(250u16);
    }
}
