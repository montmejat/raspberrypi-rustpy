#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::gpio::gpioa::PA0;
use stm32l4xx_hal::gpio::{Output, PushPull};
use stm32l4xx_hal::stm32::{TIM2, RCC};
use stm32l4xx_hal::stm32::tim2;

#[inline(never)]
fn delay(tim2: &tim2::RegisterBlock, ticks: u32) {
    tim2.arr.write(|w| w.arr().bits(ticks));
    tim2.cr1.modify(|_, w| w.cen().set_bit());
    while !tim2.sr.read().uif().bit_is_set() {}
    tim2.sr.modify(|_, w| w.uif().clear_bit());
}

#[inline(never)]
fn send_one(tim2: &tim2::RegisterBlock, led: &mut PA0<Output<PushPull>>) {
    led.set_high().unwrap();
    delay(tim2, 5); // 0.35 usec

    led.set_low().unwrap();
    delay(tim2, 15); // 0.9 usec
}

#[inline(never)]
fn send_zero(tim2: &tim2::RegisterBlock, led: &mut PA0<Output<PushPull>>) {
    led.set_high().unwrap();
    delay(tim2, 15);

    led.set_low().unwrap();
    delay(tim2, 5);
}

#[inline(never)]
fn reset(tim2: &tim2::RegisterBlock, led: &mut PA0<Output<PushPull>>) {
    led.set_low().unwrap();
    delay(tim2, 800);
}

#[entry]
fn main() -> ! {
    // setup the board
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    // setup the peripherals
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    // setup the LEDs
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // timer configuration
    let tim2;
    unsafe {
        tim2 = &*TIM2::ptr(); // general timer 2 pointer 
        let rcc_ptr = &*RCC::ptr(); // RCC pointer

        rcc_ptr.apb1enr1.modify(|_, w| w.tim2en().set_bit()); // power on the tim2 timer
        tim2.cr1.modify(|_, w| w.opm().clear_bit()); // OPM: mode 0 = counter not stopped at event
        tim2.psc.modify(|_, w| w.psc().bits(0)); // prescaler for 1 tick = 1 ms --> 48 MHz / (0 + 1) = 48 MHz
        tim2.arr.modify(|_, w| w.arr().bits(16_000_000)); // 1 tick = .000 000 125 sec
        tim2.cr1.modify(|_, w| w.cen().set_bit()); // CEN: Enable the counter
    }

    let buffer: [u8; 24] = [ 0, 0, 0, 0, 0, 0, 0, 0,
                            1, 1, 1, 1, 1, 1, 1, 1,
                            0, 0, 0, 0, 0, 0, 0, 0 ];

    let mut led_on = false;
    loop {
        // for i in 0..23 {
        //     if buffer[i] == 1 {
        //         send_one(tim2, &mut led);
        //     } else {
        //         send_zero(tim2, &mut led);
        //     }
        // }

        // reset(tim2, &mut led);

        if tim2.sr.read().uif().bit_is_set() {
            led_on = !led_on;
            if led_on {
                led.set_high().unwrap();
            } else {
                led.set_low().unwrap();
            }

            tim2.sr.modify(|_, w| w.uif().clear_bit());
        }
    }
}
