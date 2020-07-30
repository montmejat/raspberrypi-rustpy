pub mod controller {
    // use stm32l4xx_hal::prelude::*;
    // use stm32l4xx_hal::rcc::{Clocks, Rcc};
    // use stm32l4xx_hal::serial::{Config, Serial, Tx, Rx};
    // use stm32l4xx_hal::stm32::USART2;
    // use stm32l4xx_hal::device::gpioa::{moder, afrl};
    // use stm32l4xx_hal::gpio::gpioa::{PA2, PA3};
    // use stm32l4xx_hal::gpio::Alternate;
    
    // pub fn init(rcc: &mut Rcc, clocks: Clocks, tx: PA2<Alternate<AF7, MODE>>, rx: PA3<Alternate<AF7, MODE>>) -> (Tx<USART2>, Rx<USART2>) {
    //     let dp = unsafe { stm32l4xx_hal::stm32::Peripherals::steal() };

    //     let serial = Serial::usart2(
    //         dp.USART2,
    //         (tx, rx),
    //         Config::default().baudrate(9_600.bps()),
    //         clocks,
    //         &mut rcc.apb1r1,
    //     );
        
    //     serial.split()
    // }
}