#![no_main]
#![no_std]

extern crate panic_halt;
use cortex_m_rt::entry;
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

    // Configure the pins
    let mut led_red = gpioa.pa1.into_push_pull_output();
    let mut led_green = gpioa.pa0.into_push_pull_output();
    let mut led_blue = gpioa.pa2.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    loop {
        led_red.set_high().unwrap();
        led_green.set_high().unwrap();
        led_blue.set_high().unwrap();
        delay.delay_ms(1000_u16);

        led_red.set_low().unwrap();
        led_green.set_low().unwrap();
        led_blue.set_low().unwrap();
        delay.delay_ms(1000_u16);
    }
}
