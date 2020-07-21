#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[macro_use(singleton)]
extern crate cortex_m;

use cortex_m::asm::nop;
use cortex_m::asm::delay;
use cortex_m_rt::entry;

use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::gpio::gpiod::PD6;
use stm32l4xx_hal::gpio::{Output, PushPull};
use stm32l4xx_hal::rcc::PllConfig;
use stm32l4xx_hal::pwm::C1;
use stm32l4xx_hal::stm32::TIM2;

#[entry]
fn main() -> ! {
    let p = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // DMA
    let channels = p.DMA1.split(&mut rcc.ahb1);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let c1 = gpioa
        .pa0
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
        .into_af1(&mut gpioa.moder, &mut gpioa.afrl);

    let mut pwm = p
        .TIM2
        .pwm(
            c1,
            800.khz(),
            clocks,
            &mut rcc.apb1r1,
        );
    let max = pwm.get_max_duty();

    let one_duty = (max * 16 / 25) as u8;
    let buf = singleton!(: [u8; 25] = [one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, 
                                    one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, 
                                    one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, 0]).unwrap();
    
    pwm.set_duty(0);
    pwm.enable();
    pwm.set_duty(one_duty as u32);

    loop {
    }
}
