#![no_std]
#![no_main]

// you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;

use cortex_m_rt::entry;

mod led_matrix;
use led_matrix::controls::Color::{Blue, White, Red, Yellow, Cyan, Black};

#[entry]
fn main() -> ! {
    let max = led_matrix::controller::init();

    let buffer: [u32; 1586] = led_matrix::controls::create_buffer(max, [
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

    loop {
    }
}
