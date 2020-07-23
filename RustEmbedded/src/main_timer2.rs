#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;

use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::stm32::{TIM3, RCC};

#[entry]
fn main() -> ! {
    let p = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let c1 = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper)
        .into_af1(&mut gpioa.moder, &mut gpioa.afrl);

    let mut pwm = p.TIM2.pwm(c1, 800.khz(), clocks, &mut rcc.apb1r1);
    
    let max = pwm.get_max_duty();
    let one_duty = (max * 80 / 125) as u32;
    let zero_duty = (max * 45 / 125) as u32;

    let buffer: [u32; 24] = [zero_duty, zero_duty, zero_duty, zero_duty, zero_duty, zero_duty, zero_duty, zero_duty,
                            one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty, one_duty,
                            zero_duty, zero_duty, zero_duty, zero_duty, zero_duty, zero_duty, zero_duty, zero_duty];
    
    let tim3;
    unsafe {
        tim3 = &*TIM3::ptr();
        let rcc_ptr = &*RCC::ptr();

        rcc_ptr.apb1enr1.modify(|_, w| w.tim3en().set_bit()); // power on the tim2 timer
        tim3.cr1.modify(|_, w| w.opm().clear_bit()); // OPM: mode 0 = counter not stopped at event
        tim3.psc.modify(|_, w| w.psc().bits(1)); // prescaler --> 16 MHz (APB1_CLOCK) / (15999 + 1) = 1 kHz
        tim3.arr.modify(|_, w| w.arr().bits(10)); // 1 tick = .000 000 125 sec
        tim3.cr1.modify(|_, w| w.cen().set_bit()); // CEN: Enable the counter
    }

    pwm.set_duty(0);
    pwm.enable();

    let mut led_on = false;
    loop {
        for _ in 0..63 {
            for i in 0..23 {
                loop {
                    if tim3.sr.read().uif().bit_is_set() {
                        pwm.set_duty(buffer[i]);
                        tim3.sr.modify(|_, w| w.uif().clear_bit());
                        break;
                    }
                }
            }
        }

        // reset
        tim3.arr.modify(|_, w| w.arr().bits(40));
        tim3.sr.modify(|_, w| w.uif().clear_bit());
        while !tim3.sr.read().uif().bit_is_set() {}

        tim3.sr.modify(|_, w| w.uif().clear_bit());
        tim3.arr.modify(|_, w| w.arr().bits(10));

        // if tim3.sr.read().uif().bit_is_set() {
        //     led_on = !led_on;

        //     if led_on {
        //         led.set_high();
        //         pwm.set_duty(max / 4);
        //     } else {
        //         led.set_low();
        //         pwm.set_duty(0);
        //     }

        //     tim3.sr.modify(|_, w| w.uif().clear_bit());
        // }
    }
}
