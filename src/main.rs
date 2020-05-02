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
    let gpioa = &peripherals.GPIOA;
    let rcc = &peripherals.RCC;
    hprintln!("Perifericos creados").unwrap();
    
    // Se debe habilitar el reloj para el GPIOA
    rcc.ahb2enr.write(|w| w.gpioaen().set_bit());

    // Se configura el periferico para funcionar como salida
    gpioa.moder.write(|w| w.moder5().bits(0b01));
    gpioa.otyper.write(|w| w.ot5().bit(false));
    gpioa.ospeedr.write(|w| w.ospeedr5().bits(0b00));
    gpioa.pupdr.write(|w| unsafe{w.pupdr5().bits(0b00)});
    hprintln!("Se configuro el pin 5 del puerto A").unwrap();
    
    
    loop {
        gpioa.bsrr.write(|w| w.bs5().set_bit());
        cortex_m::asm::delay(2000000);
        gpioa.bsrr.write(|w| w.br5().set_bit());
        cortex_m::asm::delay(2000000);
    }
}
