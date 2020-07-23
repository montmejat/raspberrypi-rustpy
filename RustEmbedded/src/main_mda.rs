#![no_std]
#![no_main]

// you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;

use cortex_m_rt::entry;

use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::stm32::{TIM2, DMA1, RCC};

#[entry]
fn main() -> ! {
    // setup the board
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    // setup the peripherals
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr); // sets by default the clock to 16mhz ?!
    let _channels = dp.DMA1.split(&mut rcc.ahb1);

    // setup the LEDs
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    
    // setup the PWM
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let c1 = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).into_af1(&mut gpioa.moder, &mut gpioa.afrl);
    let mut pwm = dp.TIM2.pwm(c1, 1.hz(), clocks, &mut rcc.apb1r1);
    let max = pwm.get_max_duty();

    // general variables
    let buffer: [u32; 10] = [ 0, 0, max, max, 0, max / 8, max / 8, max / 2, max / 2, 0 ];

    pwm.set_duty(0);
    pwm.enable();

    let tim2;
    unsafe {
        tim2 = &*TIM2::ptr(); // general timer 2 pointer
        let rcc_ptr = &*RCC::ptr(); // RCC pointer
        let dma1 = &*DMA1::ptr(); // DMA 1 pointer
    
        rcc_ptr.ahb1enr.modify(|_, w| w.dma1en().set_bit()); // enable DMA1 clock: peripheral clock enable register

        // timer configuration
        // rcc_ptr.apb1enr1.modify(|_, w| w.tim4en().set_bit()); // power on the tim4 timer
        // tim4.cr1.modify(|_, w| w.opm().clear_bit()); // OPM: mode 0 = counter not stopped at event
        // tim4.psc.modify(|_, w| w.psc().bits(15_999)); // prescaler for 1 tick = 1 ms - 16 MHz / (15999 + 1) = 1 KHz
        // tim4.arr.modify(|_, w| w.arr().bits(1_000)); // update event every 1 second
        // // tim3.ccer.modify(|_, w| w.cc1e().set_bit()); // capture mode enabled: OC1 signal is output on the corresponding output pin
        // tim4.cr1.modify(|_, w| w.cen().set_bit()); // CEN: Enable the counter

        // timer for DMA configuration
        tim2.dier.write(|w| w.tde().set_bit()); // enable DMA trigger
        tim2.dier.write(|w| w.ude().set_bit()); // enable update DMA request
        // tim2.dier.write(|w| w.cc1de().set_bit()); // enable capture/compare 1 DMA request
        // tim2.dier.write(|w| w.uie().set_bit()); // enable update interrupt enable

        let _a = &tim2.ccr1 as *const _ as u32; // very different from 0x4000_0034

        // DMA configuration
        dma1.cselr.write(|w| w.c2s().bits(0b0100)); // set CxS[3:0] to 0100 to map the DMA request to timer 2 channel 1
        dma1.cmar2.write(|w| w.ma().bits(buffer.as_ptr() as u32)); // write the buffer to the memory adress
        dma1.cndtr2.write(|w| w.ndt().bits(buffer.len() as u16)); // number of data to transfer register
        dma1.cpar2.write(|w| w.pa().bits(0x4000_0034)); // set the DMA peripheral address register to the capture/compare 1 of TIM2
        dma1.ccr2.modify(|_, w| w
            .mem2mem().clear_bit() // memory-to-memory disabled
            .pl().high() // set highest priority
            .msize().bits(2) // size in memory of each transfer: b10 = 32 bits long
            .psize().bits(2) // size of peripheral: b10 = 32 bits long --> 32 or 16 ?? 
            .minc().set_bit() // memory increment mode enabled
            .pinc().clear_bit() // peripheral increment mode disabled
            .circ().set_bit() // circular mode: the dma transfer is repeated automatically when finished
            .dir().set_bit() // data transfer direction: 1 = read from memory
            .teie().set_bit() // transfer error interrupt enabled
            .htie().set_bit() // half transfer interrupt enabled
            .tcie().set_bit() // transfer complete interrupt enabled
            .en().set_bit() // channel enable
        );
    }

    led.set_high().unwrap();

    // let mut led_on = false;
    loop {
        // if tim4.sr.read().uif().bit_is_set() {
        //     led_on = !led_on;
        //     if led_on {
        //         led.set_high().unwrap();
        //     } else {
        //         led.set_low().unwrap();
        //     }
        //     tim4.sr.modify(|_, w| w.uif().clear_bit());
        // }
    }
}
