#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    // Acquire the Peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIO
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure the pins
    let mut led_red = gpioa.pa1.into_push_pull_output();
    let mut led_green = gpioa.pa0.into_push_pull_output();
    let mut led_blue = gpioa.pa2.into_push_pull_output();
    let btn1 = gpiob.pb3.into_pull_up_input();
    let btn2 = gpioa.pa15.into_pull_up_input();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    let mut color: i8 = 0;
    loop {
        // Check if any button is pressed
        if btn1.is_low().unwrap() {
            // Button 1 is pressed
            color += 1; // Increase color value by 1
        } else if btn2.is_low().unwrap() {
            // Button 2 is pressed
            color -= 1; // Decrease color value by 1
        }

        if color < 0 {
            // Color is negative, wrap around to max value 2
            color = 2;
        } else if color > 2 {
            // Color is more than 2, wrap around to first value 0
            color = 0;
        }

        // Output the selected color
        match color {
            0 => {
                // Red color
                led_red.set_high().unwrap();
                led_green.set_low().unwrap();
                led_blue.set_low().unwrap();
            }
            1 => {
                // Green color
                led_red.set_low().unwrap();
                led_green.set_high().unwrap();
                led_blue.set_low().unwrap();
            }
            2 => {
                // Blue color
                led_red.set_low().unwrap();
                led_green.set_low().unwrap();
                led_blue.set_high().unwrap();
            }
            _ => {
                unreachable!();
            }
        }
        delay.delay_ms(1000_u16);
    }
}
