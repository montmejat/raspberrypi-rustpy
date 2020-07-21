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

fn write_all(chan: C1, buffer: [u8]) {
    unsafe { 
        (*TIM2::ptr()).dier.modify(|_, w| {
            w.tde().set_bit().cc1de().set_bit()
        }); 
    }

    let buffer: &[u8] = buffer.borrow();
    chan.cmar().write(|w| unsafe {
        w.ma().bits(buffer.as_ptr() as usize as u32)
    });
    chan.cndtr().write(|w| unsafe{
        w.ndt().bits(u16(buffer.len()).unwrap())
    });
    chan.cpar().write(|w| unsafe {
        w.pa().bits(&(*TIM2::ptr()).ccr1 as *const _ as usize as u32)
    });

    // atomic::compiler_fence(Ordering::SeqCst);

    chan.ccr().modify(|_, w| {
        w.mem2mem().clear_bit()
            // priority
            .pl().high()
            // size in memory
            .msize().bit8()
            .psize().bit32()
            .minc().set_bit()
            .pinc().clear_bit()
            .circ().clear_bit()
            .dir().set_bit()
            .teie().set_bit()
            .htie().set_bit()
            .tcie().set_bit()
            // enable
            .en().set_bit()
    });
}

#[entry]
fn main() -> ! {
    let p = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let c1 = gpioa
        .pa0
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
        .into_af1(&mut gpioa.moder, &mut gpioa.afrl);

    let channels = p.DMA1.split(&mut rcc.ahb1);

    let mut pwm = p
        .TIM2
        .pwm(c1, 800.khz(), clocks, &mut rcc.apb1r1)
        .3;

    let one_duty = (max * 16 / 25) as u8;

    let buf = singleton!(: [u8; 25] = [one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, 
                                    one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, 
                                    one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, 0]).unwrap();
    
    // set duty to zero
    pwm.set_duty(0);
    // enable pwm output
    pwm.enable();
    // enable the dma and wait for it to finish
    write_all(channels.5, buf);

    loop {

    }
}
