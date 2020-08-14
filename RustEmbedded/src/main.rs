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
use stm32l4xx_hal::delay::Delay;

use led_matrix::controls::Color::{Blue, White, Red, Yellow, Cyan, Black};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
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

    // led_blue.set_high().unwrap();
    let hsv_lights: [u8; 61] = 
    [ 0, 4, 8, 13, 17, 21, 25, 30, 34, 38, 42, 47, 51, 55, 59, 64, 68, 72, 76,
    81, 85, 89, 93, 98, 102, 106, 110, 115, 119, 123, 127, 132, 136, 140, 144,
    149, 153, 157, 161, 166, 170, 174, 178, 183, 187, 191, 195, 200, 204, 208,
    212, 217, 221, 225, 229, 234, 238, 242, 246, 251, 255 ];
    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        let mut received = block!(rx.read()).unwrap();
        if received == '#' as u8 { // new message incoming
            led_blue.set_high().unwrap(); // bluetooth led

            received = block!(rx.read()).unwrap();
            if received == '/' as u8 { // set to rainbow mode
                loop {
                    let mut buffer = [0 as u8; 192];
        
                    for angle in 0..360 {
                        let green;
                        let red;
                        let blue;
                        if angle < 60 {
                            red = 255; green = hsv_lights[angle]; blue = 0;
                        } else if angle < 120 {
                            red = hsv_lights[120-angle]; green = 255; blue = 0;
                        } else if angle < 180 {
                            red = 0; green = 255; blue = hsv_lights[angle-120];
                        } else if angle < 240 {
                            red = 0; green = hsv_lights[240-angle]; blue = 255;
                        } else if angle < 300 {
                            red = hsv_lights[angle-240]; green = 0; blue = 255;
                        } else {
                            red = 255; green = 0; blue = hsv_lights[360-angle];
                        } 
        
                        for i in (0..192).step_by(3) {
                            buffer[i] = 255 - green;
                            buffer[i + 1] = 255 - red;
                            buffer[i + 2] = 255 - blue;
                        }
        
                        let new_buffer: [u32; 1586] = led_matrix::controls::create_buffer_from_values(max, buffer);
                        led_matrix::controller::load_buffer(new_buffer);
        
                        timer.delay_ms(10 as u32);
                    }
                }
            } else if received == '&' as u8 { // set to send buffer mode
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
                
                led_blue.set_low().unwrap(); // bluetooth led

                let new_buffer: [u32; 1586] = led_matrix::controls::create_buffer_from_values(max, incoming_buffer);
                led_matrix::controller::load_buffer(new_buffer);
            }
        }
    }
}
