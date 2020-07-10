from gpiozero import LED
from time import sleep
import rpipy

led = LED(26)

class Settings:
    def __init__(self):
        self.my_var = 50
        self.my_message = "Hello!"
        self.slider_var = self.SliderValue(0, 100, 50)
        self.another_slider = self.SliderValue(10, 40, 20)

    class SliderValue:
        def __init__(self, min, max, value=0):
            self.min = min
            self.max = max
            self.value = value

param = Settings()

def start():
    print("Device info:", rpipy.get_device_info(), "| temp:", rpipy.measure_temp())

def loop():
    print("Looping in demo! My var =", param.my_var, "| My message:", param.my_message, "| My slider value:", param.slider_var.value)
    led.on()
    sleep(1)
    led.off()
    sleep(1)
    param.my_var += 1
    print("Ending loop in demo!")

def end():
    print("Ending demo!")

def my_func():
    print("Some custom stuff!")