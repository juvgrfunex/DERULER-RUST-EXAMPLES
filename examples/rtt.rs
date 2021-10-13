#![no_std]
#![no_main]
use stm32l0xx_hal as _;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("RTT Works!");

    let mut loop_counter: u16 = 0;
    loop {
        rprintln!("Loop iteration {}",loop_counter);
        loop_counter = loop_counter.wrapping_add(1);
    }
}
