#![no_std]
#![no_main]

mod led_matrix;
mod bluetooth;

// you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;

#[macro_use(block)]
extern crate nb;

use cortex_m_rt::entry;

use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::serial::{Config, Serial};

use led_matrix::controls::Color::{Blue, White, Red, Yellow, Cyan, Black};

#[entry]
fn main() -> ! {
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let c1 = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).into_af1(&mut gpioa.moder, &mut gpioa.afrl);
    let mut pwm = dp.TIM2.pwm(c1, 800.khz(), clocks, &mut rcc.apb1r1);
    
    let max = led_matrix::controller::init(&mut pwm);
    let buffer: [u32; 1586] = led_matrix::controls::create_buffer_from_colors(max, [
        Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, Cyan,
        Cyan, Blue, Blue, Blue, Blue, Blue, Cyan, Cyan,
        Blue, Blue, Blue, Black, White, Black, Cyan, Cyan,
        Blue, Cyan, Blue, Yellow, Yellow, Yellow, Cyan, Cyan,
        Cyan, Blue, Blue, Blue, Blue, Blue, Blue, Cyan,
        Cyan, Blue, Blue, Yellow, Yellow, Blue, Blue, Cyan,
        Cyan, White, Blue, Yellow, Yellow, Blue, White, Cyan,
        Cyan, Cyan, Red, Cyan, Cyan, Red, Cyan, Cyan,
    ]);
    led_matrix::controller::load_buffer(buffer);

    // setup status LED
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_blue = gpiob.pb7.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

    let serial = Serial::usart2(
        dp.USART2,
        (tx, rx),
        Config::default().baudrate(9_600.bps()),
        clocks,
        &mut rcc.apb1r1,
    );
    
    let (_tx, mut rx) = serial.split();

    led.set_high().unwrap(); // configuration done led

    loop {
        let mut received = block!(rx.read()).unwrap();
        if received == '#' as u8 { // new message incoming
            led_blue.set_high().unwrap(); // bluetooth led
            let mut incoming_buffer = [0 as u8; 192];
            let mut i = 0;

            incoming_buffer[i] = block!(rx.read()).unwrap();
            loop {
                if i < 191 {
                    i += 1;
                }

                received = block!(rx.read()).unwrap();
                if received != '?' as u8 {
                    incoming_buffer[i] = received;
                } else {
                    break;
                }                                
            }

            let new_buffer: [u32; 1586] = led_matrix::controls::create_buffer_from_values(max, incoming_buffer);
            led_matrix::controller::load_buffer(new_buffer);

            led_blue.set_low().unwrap(); // bluetooth led
        }
    }
}
