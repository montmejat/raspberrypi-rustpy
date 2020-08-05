from gpiozero import LED
from time import sleep
import rpipy

# LED plugged to the RasperryPi
led = LED(26)

# LED matrix to control via the webserver
class Led:
    def __init__(self, leds_count):
        self.leds = []
        self.leds_count = leds_count

        for _ in range(leds_count):
            self.leds.append(self.Led(0, 0, 0))
    
    def __eq__(self, obj):
        if self.leds_count != obj.leds_count:
            return False

        for i in range(self.leds_count):
            if self.leds.get(i) != obj.leds.get(i):
                return False
        
        return True

    def get(self, i):
        return self.leds[i]

    class Led:
        def __init__(self, green, red, blue):
            self.green = green
            self.red = red
            self.blue = blue

        def __eq__(self, obj):
            return (self.green == obj.green and self.red == obj.red and self.blue == obj.blue)

# Personnal settings to also control via the webserver
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
led_matrix = Led(64)

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