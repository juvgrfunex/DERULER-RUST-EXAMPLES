#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal as _;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    // allocate usize on heap
    let _heap_alloc = Box::new(0_usize);

    // try to allocate 1024 bytes on the heap
    // this will fail because there is not enough free memory
    let _too_large_vec = vec![[0u8; 1024]];

    unreachable!();
}

#[alloc_error_handler]
fn oom(layout: Layout) -> ! {
    rprintln!("Out of Memory Error");
    rprintln!("{:?}", layout);
    loop {}
}
