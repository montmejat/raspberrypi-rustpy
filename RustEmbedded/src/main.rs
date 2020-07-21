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
    let buffer: [u32; 10] = [ 0, max, 0, max, 0, max / 8, max / 4, max / 2, max, 0 ];

    pwm.set_duty(0);
    pwm.enable();

    let tim2;
    unsafe {
        tim2 = &*TIM2::ptr(); // general timer 2 pointer
        let rcc_ptr = &*RCC::ptr(); // RCC pointer
        let dma1 = &*DMA1::ptr(); // DMA 1 pointer
        
        // // timer configuration
        // rcc_ptr.apb1enr1.modify(|_, w| w.tim2en().set_bit()); // power on the tim2 timer
        // tim2.cr1.modify(|_, w| w.opm().clear_bit()); // OPM: mode 0 = counter not stopped at event
        // tim2.psc.modify(|_, w| w.psc().bits(15_999)); // prescaler for 1 tick = 1 ms - 16 MHz / (15999 + 1) = 1 KHz
        // tim2.arr.modify(|_, w| w.arr().bits(500)); // update event every 1 second
        // tim2.cr1.modify(|_, w| w.cen().set_bit()); // CEN: Enable the counter

        // timer for DMA configuration
        tim2.dier.modify(|_, w| w.tde().set_bit()); // enable DMA trigger
        tim2.dier.modify(|_, w| w.cc1de().set_bit()); // enable capture/compare 1 DMA request
        tim2.dier.modify(|_, w| w.ude().set_bit()); // enable update DMA request
        tim2.dier.modify(|_, w| w.uie().set_bit()); // enable update interrupt enable

        let a = tim2.ccr1.read().bits();

        // DMA configuration
        dma1.cmar5.write(|w| w.ma().bits(buffer.as_ptr() as u32)); // write the buffer to the memory adress
        dma1.cndtr5.write(|w| w.ndt().bits(buffer.len() as u16)); // number of data to transfer register
        dma1.cselr.write(|w| w.c1s().bits(2)); // set CxS[3:0] to 0100 to map the DMA request to timer 2 channel 1
        dma1.cpar5.write(|w| w.pa().bits(tim2.ccr1.read().bits())); // set the DMA peripheral address register to the capture/compare 1 of TIM2
        dma1.ccr5.modify(|_, w| w
            .mem2mem().clear_bit() // memory-to-memory disabled
            .pl().high() // set highest priority
            .msize().bits(32) // size in memory of each transfer
            .psize().bits(32) // size of peripheral
            .minc().set_bit() // memory increment mode enabled
            .pinc().clear_bit() // peripheral increment mode disabled
            .circ().clear_bit() // circular mode disabled (not sure here)
            .dir().set_bit() // data transfer direction: 1 = read from memory
            .teie().set_bit() // transfer error interrupt enabled
            .htie().set_bit() // half transfer interrupt enabled
            .tcie().set_bit() // transfer complete interrupt enabled
            .en().set_bit() // channel enable
        );
    }

    led.set_high();

    // let mut led_on = false;
    loop {
        // if tim2.sr.read().uif().bit_is_set() {
        //     led_on = !led_on;
        //     if led_on {
        //         match led.set_high() {
        //             Ok(_) => {},
        //             Err(_) => {},
        //         }
        //     } else {
        //         match led.set_low() {
        //             Ok(_) => {},
        //             Err(_) => {},
        //         }
        //     }
        //     tim2.sr.modify(|_, w| w.uif().clear_bit());
        // }
    }
}
