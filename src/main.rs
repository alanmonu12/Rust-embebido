#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32l4;

use cortex_m_semihosting::hprintln;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32l4::stm32l4x6;

// Crates pata usar el SYST
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

fn mDelay (delay: u32, s: &mut SYST) {
    let _tmp = s.has_wrapped();

    let mut tmp_delay = delay;

    if tmp_delay < 0xFFFF_FFFF {
        tmp_delay = tmp_delay + 1;
    }

    while tmp_delay != 0 {
        if s.has_wrapped() != false {
            tmp_delay = tmp_delay - 1;
        }
    }
}

#[entry]
fn main() -> ! {
    let peripherals = stm32l4x6::Peripherals::take().unwrap();
    // Perifericos definidos por ARM
    let core_peripherals = cortex_m::Peripherals::take().unwrap();

    // Periferico para el SysTick
    let mut syst = core_peripherals.SYST;
 
    let gpioa = &peripherals.GPIOA;
    
    let rcc = &peripherals.RCC;
    
    let flash = &peripherals.FLASH;

    let pwr = &peripherals.PWR;

    hprintln!("Perifericos creados {}", 1).unwrap();

    // Se habilita el reloj del sistema
    rcc.apb2enr.write(|w| w.syscfgen().set_bit());
    // Se habilita el reloj de la toda la parte de alimetnación
    rcc.apb1enr1.write(|w| w.pwren().set_bit());
    // Se configura la latencia de la memorua flash
    flash.acr.write(|w| unsafe{w.latency().bits(0b000)});
    // Voltage scaling range 1
    pwr.cr1.write(|w| unsafe{w.vos().bits(0b01)});
    // Se habilita el HSI
    rcc.cr.write(|w| w.hsion().set_bit());

    // Esperamos a que el HSI se active
    while !(rcc.cr.read().hsirdy().bit_is_set()){

    }

    rcc.icscr.write(|w| unsafe{w.hsitrim().bits(0b10000)});

    // Selecionamos el HSI como reloj del sistema
    rcc.cfgr.write(|w| unsafe{w.sw().bits(0b01)});

    while !(rcc.cfgr.read().sws().bits() == 0x0000_0001) {
        hprintln!("{}", rcc.cfgr.read().sws().bits()).unwrap(); 
    }

    // Configuramos el prescaler para al AHB en 1
    //rcc.cfgr.write(|w| unsafe{w.hpre().bits(0b0000)});
    //rcc.cfgr.write(|w| unsafe{w.ppre1().bits(0b000)});
    //rcc.cfgr.write(|w| unsafe{w.ppre2().bits(16)});

    // Se configura el SysTick para tener un interrupcion cada 1ms
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(15_999);
    syst.clear_current();
    syst.enable_counter();
    //syst.enable_interrupt();
    
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
        mDelay(100, &mut syst);
        gpioa.bsrr.write(|w| w.br5().set_bit());
        mDelay(100, &mut syst);
    }
}

#[exception]
fn SysTick() {
    
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    hprintln!("{:#?}", ef).unwrap();

    loop {}
}
