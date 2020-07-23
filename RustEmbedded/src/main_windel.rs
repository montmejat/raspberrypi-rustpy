#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32l4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world w0000000000000t!").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    // let mut flash = dp.FLASH.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze(&mut flash.acr);

    hprintln!("Clocks pclk1: {}", clocks.pclk1().0);

    // the board has 4 leds on PD12, PD13, PD14 and PD15
    // Head for PD13 (an LED)

    let gpiod = dp.GPIOD.split(&mut rcc.ahb2);
    let gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led1 = gpiod.pd14.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    // led2 as pwm output:
    // PD14 --> AF2 = TIM4_CH3
    // let mut led2 = gpiod.pd14.into_push_pull_output();

    // Ugh, pwm config:
    let channels = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).into_af1(&mut gpioa.moder, &mut gpioa.afrl);
    let pwm = dp.TIM2.pwm(channels, 1.hz(), clocks, &mut rcc.apb1r1);

    let (mut ch1, mut ch2) = pwm;
    let max_duty = ch1.get_max_duty();
    ch1.set_duty(max_duty / 10);
    ch1.enable();

    let max_duty = ch2.get_max_duty();
    ch2.set_duty(max_duty / 10);
    ch2.enable();
    // let tim4 = dp.TIM4.pwm(); //  psc();

    // led2.set_high().unwrap();

    // Connect DMA to PD12:
    let delta = max_duty as u32 / 6;
    let buffer: [u32; 6] = [delta, delta * 2, delta * 3, delta * 4, delta * 5, delta * 6];
    use stm32l4xx_hal::stm32::{RCC, DMA1, TIM4};

    // Enable DMA1 clock:
    unsafe {
        let rcc = &*RCC::ptr();
        rcc.ahb1enr.modify(|_, w| w.dma1en().set_bit());
    }
    // DMA1, Channel 2, stream 6 = TIM4_UP DMA request.
    unsafe {
        let dma1 = &*DMA1::ptr();
        // Configure stream 6:

        // Configure buffer:
        dma1.st[6].m0ar.write(|w| w.m0a().bits(buffer.as_ptr() as u32));
        dma1.st[6].ndtr.write(|w| w.ndt().bits(buffer.len() as u16)        );
        
        // CCR1 register of TIM4, channel 1, offset = 0x34, TIM4 at 0x4000 0800
        dma1.st[6].par.write(|w| w.pa().bits(0x4000_0834));

        dma1.st[6].cr.write(|w|
            w
            .chsel().bits(2)  // channel 2
            .msize().bits(2)  // 32 bit memory
            .psize().bits(2)  // 32 bit peripheral
            .minc().set_bit()  // increment memory address
            .pinc().clear_bit() // do not increment peripheral address
            .circ().set_bit() // circular mode for demo
            .dir().bits(1) // memory to peripheral
            .en().set_bit() // enable the dma stream
        );

        let tim4 = &*TIM4::ptr();
        tim4.dier.write(|w|
            w
            .tde().set_bit()  // enable dma request trigger
            .ude().set_bit()  // Enable update event dma request
        );

    }

    // Manual blink loop:
    let mut delay = stm32l4xx_hal::delay::Delay::new(cp.SYST, clocks);
    loop {
        led1.set_high().unwrap();
        delay.delay_ms(300_u32);
        led1.set_low().unwrap();
        delay.delay_ms(300_u32);
    }
}
