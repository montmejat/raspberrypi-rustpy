from gpiozero import LED
from time import sleep
import rpipy

led = LED(26)
my_var = 50
my_message = "Hello!"

def start():
    print("Device info:", rpipy.get_device_info(), "| temp:", rpipy.measure_temp())

def loop():
    print("Looping in demo! My var =", my_var, "| My message:", my_message)
    led.on()
    sleep(1)
    led.off()
    sleep(1)
    print("Ending loop in demo!")

def end():
    print("Ending demo!")

def my_func():
    print("Some custom stuff!")