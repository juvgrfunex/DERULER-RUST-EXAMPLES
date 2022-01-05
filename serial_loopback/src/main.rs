#![no_std]
#![no_main]
use core::fmt::Write;
use nb::block;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config, serial};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Choose TX / RX pins
    let tx_pin = gpiob.pb6;
    let rx_pin = gpiob.pb7;

    // Configure the serial peripheral.
    let serial = dp
        .USART1
        .usart(tx_pin, rx_pin, serial::Config::default(), &mut rcc)
        .unwrap();

    let (mut tx, mut rx) = serial.split();
    writeln!(tx, "Hello from DERULER!").unwrap();
    loop {
        // Echo what is received on the serial link.
        let received = block!(rx.read()).unwrap();
        block!(tx.write(received)).ok();
    }
}
