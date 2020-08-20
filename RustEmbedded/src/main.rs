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

use led_matrix::controls::Color::{Blue, White, Cyan};

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
        Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue,
        Blue, Cyan, White, White, White, White, Cyan, Blue,
        Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, White, Cyan,
        Cyan, Cyan, Cyan, White, White, White, Cyan, White,
        Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, White,
        Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, Cyan,
        Blue, Cyan, Cyan, Cyan, Cyan, Cyan, Cyan, Blue,
        Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue,
    ]); // my incredible attemp to recreate the Demcon logo, don't judge me please
    led_matrix::controller::load_buffer(buffer); 

    // setup status LED
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_blue = gpiob.pb7.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut command_pin = gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

    let serial = Serial::usart2(
        dp.USART2,
        (tx, rx),
        Config::default().baudrate(38_400.bps()), // 38_400 for command mode / 115_200 for data mode / 9_600
        clocks,
        &mut rcc.apb1r1,
    );
    
    let (mut tx, mut rx) = serial.split();
    
    if false {
        command_pin.set_high().unwrap();
        let command = "AT+UART=38400,1,0";
        for s in command.bytes() {
            block!(tx.write(s)).ok();
        }
        block!(tx.write('\r' as u8)).ok();
        block!(tx.write('\n' as u8)).ok();

        let received = block!(rx.read()).unwrap();
        if received == 'O' as u8 {
            led.set_high().unwrap();
        } else {
            led_blue.set_high().unwrap();
        }
    } else {
        command_pin.set_low().unwrap();
    }
    
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
            
            led_blue.set_low().unwrap();

            let new_buffer: [u32; 1586] = led_matrix::controls::create_buffer_from_values(max, incoming_buffer);
            led_matrix::controller::load_buffer(new_buffer);
        }
    }
}
