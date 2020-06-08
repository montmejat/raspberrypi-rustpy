#[macro_use]
extern crate cpython;

use std::thread;
use std::time::Duration;
// use std::io::{self, BufRead};
// use std::sync::mpsc::{self, TryRecvError};

use cpython::{Python, PyResult};
use rppal::system::DeviceInfo;
use rppal::gpio::Gpio;

const GPIO_ONBOARD_LED: u8 = 23;

fn get_device_info(_py: Python) -> PyResult<String> {
    let device = match DeviceInfo::new() {
        Result::Ok(device) => device.model().to_string(),
        Result::Err(err) => err.to_string(),
    };

    Ok(device)
}

fn turn_on_onboard_led(py: Python, millis: u64) -> PyResult<u8> {
    turn_on_led(py, GPIO_ONBOARD_LED, millis)?;
    Ok(0)
}

fn turn_on_led(_py: Python, led_pin: u8, millis: u64) -> PyResult<u8> {
    let gpio = match Gpio::new() {
        Result::Ok(gpio) => gpio,
        Result::Err(_err) => return Ok(1),
    };

    let mut pin = match gpio.get(led_pin) {
        Result::Ok(led) => led.into_output(),
        Result::Err(_err) => return Ok(2),
    };

    thread::spawn(move || {
        pin.set_high();
        thread::sleep(Duration::from_millis(millis));
    });

    Ok(0)
}

fn blink_led(_py: Python, led_pin: u8, millis: u64) -> PyResult<u8> {
    let gpio = match Gpio::new() {
        Result::Ok(gpio) => gpio,
        Result::Err(_err) => return Ok(1),
    };

    let mut pin = match gpio.get(led_pin) {
        Result::Ok(led) => led.into_output(),
        Result::Err(_err) => return Ok(2),
    };

    thread::spawn(move || {
        loop {
            pin.set_high();
            thread::sleep(Duration::from_millis(millis));
            pin.set_low();
            thread::sleep(Duration::from_millis(millis));
        }
    });

    Ok(0)
}

py_module_initializer!(raspberrypylib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_device_info", py_fn!(py, get_device_info()))?;
    m.add(py, "turn_on_onboard_led", py_fn!(py, turn_on_onboard_led(millis: u64)))?;
    m.add(py, "turn_on_led", py_fn!(py, turn_on_led(led_pin: u8, millis: u64)))?;
    m.add(py, "blink_led", py_fn!(py, blink_led(led_pin: u8, millis: u64)))?;
    Ok(())
});