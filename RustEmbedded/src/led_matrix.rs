pub mod controller {
    use stm32l4xx_hal::prelude::*;
    use stm32l4xx_hal::stm32::{TIM2, DMA1, RCC};

    pub fn init() -> u32 {
        // setup the board
        let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

        // setup the peripherals
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let _channels = dp.DMA1.split(&mut rcc.ahb1);

        // setup the PWM
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let c1 = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).into_af1(&mut gpioa.moder, &mut gpioa.afrl);
        let mut pwm = dp.TIM2.pwm(c1, 800.khz(), clocks, &mut rcc.apb1r1);
        let max = pwm.get_max_duty();

        pwm.set_duty(0);
        pwm.enable();

        let tim2;
        let rcc_ptr;
        let dma1;
        unsafe {
            tim2 = &*TIM2::ptr(); // general timer 2 pointer
            rcc_ptr = &*RCC::ptr(); // RCC pointer
            dma1 = &*DMA1::ptr(); // DMA 1 pointer
        }

        rcc_ptr.ahb1enr.modify(|_, w| w.dma1en().set_bit()); // enable DMA1 clock: peripheral clock enable register

        // timer for DMA configuration
        tim2.dier.write(|w| w.tde().set_bit()); // enable DMA trigger
        tim2.dier.write(|w| w.ude().set_bit()); // enable update DMA request

        // DMA configuration
        dma1.cselr.write(|w| w.c2s().bits(0b0100)); // set CxS[3:0] to 0100 to map the DMA request to timer 2 channel 1
        dma1.cpar2.write(|w| w.pa().bits(0x4000_0034)); // set the DMA peripheral address register to the capture/compare 1 of TIM2
        dma1.ccr2.modify(|_, w| w
            .mem2mem().clear_bit() // memory-to-memory disabled
            .pl().high() // set highest priority
            .minc().set_bit() // memory increment mode enabled
            .pinc().clear_bit() // peripheral increment mode disabled
            .circ().clear_bit() // circular mode: the dma transfer is repeated automatically when finished
            .dir().set_bit() // data transfer direction: 1 = read from memory
            .teie().set_bit() // transfer error interrupt enabled
            .htie().set_bit() // half transfer interrupt enabled
            .tcie().set_bit() // transfer complete interrupt enabled
        );

        unsafe {
            dma1.ccr2.modify(|_, w| w
                .msize().bits(2) // size in memory of each transfer: b10 = 32 bits long
                .psize().bits(2) // size of peripheral: b10 = 32 bits long
            );
        }

        dma1.ccr2.modify(|_, w| w
            .en().set_bit() // channel enable
        );

        max
    }

    pub fn load_buffer(buffer: [u32; 1586]) {
        let dma1 = unsafe { &*DMA1::ptr() }; // disable
        dma1.ccr2.modify(|_, w| w.en().clear_bit());
        dma1.cmar2.write(|w| w.ma().bits(buffer.as_ptr() as u32)); // write the buffer to the memory address
        dma1.cndtr2.write(|w| w.ndt().bits(buffer.len() as u16)); // number of data to transfer register
        dma1.ccr2.modify(|_, w| w.en().set_bit()); // enable
    }
}

pub mod controls {
    #[derive(PartialEq)]
    pub enum Color {
        Black,
        White,
        Red,
        Blue,
        Green,
        Yellow,
        Cyan,
    }

    #[inline(never)]
    pub fn create_buffer(max: u32, input: [Color; 64]) -> [u32; 1586] {
        let one_duty = (max * 90 / 125) as u32;
        let zero_duty = (max * 35 / 125) as u32;

        let mut buffer = [max; 1586];
        
        for i in 0..64 {
            if input[i] == Color::White {
                for k in 0..24 {
                    buffer[i * 24 + k] = one_duty;
                }
            } else if input[i] == Color::Black {
                for k in 0..24 {
                    buffer[i * 24 + k] = zero_duty;
                }
            } else if input[i] == Color::Green {
                for k in 0..8 {
                    buffer[i * 24 + k] = one_duty;
                }
                for k in 8..16 {
                    buffer[i * 24 + k] = zero_duty;
                }
                for k in 16..24 {
                    buffer[i * 24 + k] = zero_duty;
                }
            } else if input[i] == Color::Red {
                for k in 0..8 {
                    buffer[i * 24 + k] = zero_duty;
                }
                for k in 8..16 {
                    buffer[i * 24 + k] = one_duty;
                }
                for k in 16..24 {
                    buffer[i * 24 + k] = zero_duty;
                }
            } else if input[i] == Color::Blue {
                for k in 0..8 {
                    buffer[i * 24 + k] = zero_duty;
                }
                for k in 8..16 {
                    buffer[i * 24 + k] = zero_duty;
                }
                for k in 16..24 {
                    buffer[i * 24 + k] = one_duty;
                }
            } else if input[i] == Color::Yellow {
                for k in 0..8 {
                    buffer[i * 24 + k] = one_duty;
                }
                for k in 8..16 {
                    buffer[i * 24 + k] = one_duty;
                }
                for k in 16..24 {
                    buffer[i * 24 + k] = zero_duty;
                }
            } else if input[i] == Color::Cyan {
                for k in 0..8 {
                    buffer[i * 24 + k] = one_duty;
                }
                for k in 8..16 {
                    buffer[i * 24 + k] = zero_duty;
                }
                for k in 16..24 {
                    buffer[i * 24 + k] = one_duty;
                }
            }
        }

        // reset
        for i in 1536..1586 {
            buffer[i] = 0;
        }

        buffer
    }
}