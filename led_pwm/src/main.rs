#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal::{pac, prelude::*, pwm, rcc::Config};

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

    let btn1 = gpiob.pb3.into_pull_up_input();
    let btn2 = gpioa.pa15.into_pull_up_input();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    // Initialize TIM2 for PWM
    //
    // Pin PA0, channel 1 for green led
    // Pin PA1, channel 2 for red led
    // PIN PA2, channel 3 for blue led
    let mut pwm = pwm::Timer::new(dp.TIM2, 10_000.Hz(), &mut rcc)
        .channel1
        .assign(gpioa.pa0);

    let max_duty = pwm.get_max_duty();
    let one_percent_duty = max_duty / 100;
    let mut led_on: bool = true;
    let mut led_brightness: u16 = 20;

    rprintln!("LED brightness set to {}%", led_brightness);
    pwm.enable();

    loop {
        match (btn1.is_low().unwrap(), btn2.is_low().unwrap()) {
            (true, true) => {
                if led_on {
                    led_on = false;
                    pwm.disable();
                } else {
                    led_on = true;
                    pwm.enable();
                }
            }
            (true, false) => {
                if led_brightness < 100 {
                    led_brightness += 1;
                    rprintln!("LED brightness set to {}%", led_brightness);
                }
            }
            (false, true) => {
                if led_brightness > 0 {
                    led_brightness -= 1;
                    rprintln!("LED brightness set to {}%", led_brightness);
                }
            }
            (false, false) => {}
        }
        pwm.set_duty(led_brightness * one_percent_duty);
        delay.delay_ms(250_u16);
    }
}
