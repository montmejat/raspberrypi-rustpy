import os
import raspberrypylib

def get_device_info():
    return raspberrypylib.get_device_info()

def measure_temp():
    temp = os.popen("vcgencmd measure_temp").readline()
    return (temp.replace("temp=","").replace("\n", ""))

# def turn_on_onboard_led(time):
#     ret = raspberrypylib.turn_on_onboard_led(time)
#     if ret != 0:
#         print("Error turning onboard led...")

def turn_on_led(led_pin, time):
    ret = raspberrypylib.set_high(led_pin, time)
    if ret != 0:
        print("Error turning on led...")

def blink_led(led_pin, time):
    ret = raspberrypylib.blink_led(led_pin, time)
    if ret != 0:
        print("Error blinking led...")

def send_pwm(pin, period, pulse_width):
    ret = raspberrypylib.send_pwm(pin, period, pulse_width)
    if ret != 0:
        print("Error sending pwm...")