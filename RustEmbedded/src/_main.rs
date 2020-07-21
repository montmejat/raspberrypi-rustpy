#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm::nop;
use cortex_m::asm::delay;
use cortex_m_rt::entry;

use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::gpio::gpiod::PD6;
use stm32l4xx_hal::gpio::{Output, PushPull};
use stm32l4xx_hal::rcc::PllConfig;

#[inline(never)]
fn send_one(led: &mut PD6<Output<PushPull>>) {
    led.set_high();
    delay(14); // 0.35 usec => 43_000_00 = 1 000 000 sec / 0.35 usec = 

    led.set_low();
    delay(38); // 0.9 usec => 43_000_00 = 1 000 000 sec / 0.9 usec = 43_000_00 * 0.9 / 1 000 000 = 38.7
}

#[inline(never)]
fn send_zero(led: &mut PD6<Output<PushPull>>) {
    led.set_high();
    delay(38);

    led.set_low();
    delay(14);
}

#[inline(never)]
fn reset(led: &mut PD6<Output<PushPull>>) {
    led.set_low();
    delay(2000);
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // clock configuration using the default settings (all clocks run at 8 MHz)
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    // let clocks = rcc.cfgr.sysclk(80.mhz()).pclk1(80.mhz()).pclk2(80.mhz()).freeze(&mut flash.acr);
    let plls = PllConfig {
        m: 2,  // / 2
        n: 72, // * 72
        r: 6,  // / 6
    };
    // NOTE: it is up to the user to make sure the pll config matches the given sysclk
    let clocks = rcc
        .cfgr
        .sysclk_with_pll(48.mhz(), plls)
        .pclk1(24.mhz())
        .pclk2(24.mhz())
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    // let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb2);

    let mut led1 = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    // let mut led2 = gpiob.pb7.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    // let mut led3 = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut led3 = gpiod.pd6.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    loop {
        // green
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);

        // red
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);
        send_zero(&mut led3);

        // blue
        send_one(&mut led3);
        send_one(&mut led3);
        send_one(&mut led3);
        send_one(&mut led3);
        send_one(&mut led3);
        send_one(&mut led3);
        send_one(&mut led3);
        send_one(&mut led3);

        // led1.set_high();
        // for _ in 0..(43_000_00) {
        //     nop();
        // }

        // led1.set_low();
        // for _ in 0..(43_000_00) {
        //     nop();
        // }

        reset(&mut led3);
    }
}
