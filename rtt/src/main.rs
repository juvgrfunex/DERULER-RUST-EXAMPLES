#![no_std]
#![no_main]
use stm32l0xx_hal as _;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("RTT Works!");
    loop {
    }
}
