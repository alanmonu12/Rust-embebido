#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32l4;

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use stm32l4::stm32l4x6;

#[entry]
fn main() -> ! {
    let peripherals = stm32l4x6::Peripherals::take().unwrap();
    hprintln!("Perifericos creados").unwrap();
    loop {
        // your code goes here
    }
}
