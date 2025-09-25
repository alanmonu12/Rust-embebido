#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _;

extern crate cortex_m_rt;
extern crate cortex_m;

use cortex_m_rt::{entry, exception, ExceptionFrame};

use stm32l4xx_hal::{
    delay::Delay, pac, prelude::*,
};

// use core::fmt;

// use fmt::Write as FmtWrite;

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing...");

    // Peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr
        .sysclk(80.MHz())
        .pclk1(80.MHz())
        .hclk(80.MHz())
        .pclk2(80.MHz())
        .freeze(&mut flash.acr, &mut pwr);

        // Delay (se necesita para calibración del ADC)
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    rprintln!("Perifericos creados.");

    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
        delay.delay_ms(500u32);
    }
}

#[exception]
fn SysTick() {
    
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    use cortex_m::peripheral::SCB;

    rprintln!("💥 HARD FAULT!");
    rprintln!("PC  = {:#010X}", ef.pc());
    rprintln!("LR  = {:#010X}", ef.lr());
    rprintln!("xPSR= {:#010X}", ef.xpsr());

    // Leer el registro HFSR para ver qué causó el HardFault
    let scb = unsafe { &*SCB::PTR };
    rprintln!("HFSR= {:#010X}", scb.hfsr.read());

    loop {}
}
