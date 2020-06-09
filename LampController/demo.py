from gpiozero import LED
from time import sleep
import rpipy

led = LED(26)

def start():
    print("Device info:", rpipy.get_device_info(), "| temp:", rpipy.measure_temp())

def loop():
    print("Looping in demo!")
    led.on()
    sleep(1)
    led.off()
    sleep(1)
