import luminolib, time

class Settings:
    def __init__(self):
        self.dimmer = self.SliderValue(0, 40, 100)

    class SliderValue:
        def __init__(self, min, max, value=0):
            self.min = min
            self.max = max
            self.value = value

param = Settings()
led_matrix = luminolib.Led(23)
turned_on_light = 0

def start():
    print("Starting rainbow mode!")
    
    radiatio_map = [
        (0, 255, 0), (0, 0, 255), (0, 0, 255), (0, 255, 0), (0, 0, 255), (0, 0, 255), (0, 255, 0), (0, 0, 255),
        (0, 0, 255), (0, 255, 0), (0, 0, 255), (0, 0, 255), (0, 0, 255), (0, 0, 255), (0, 0, 255), (0, 0, 255),
        (0, 0, 255), (0, 0, 255), (0, 0, 255), (0, 0, 255), (0, 0, 255), (0, 255, 0), (0, 0, 255), (0, 255, 0),
        (0, 0, 0), (0, 0, 255), (0, 0, 255), (0, 255, 0), (0, 0, 255), (0, 0, 255), (0, 0, 255),
    ]

    for i in range(led_matrix.leds_count):
        led = led_matrix.get(i)
        green, red, blue = radiatio_map[i]
        led.green = int(green * param.dimmer.value / 100)
        led.red = int(red * param.dimmer.value / 100)
        led.blue = int(blue * param.dimmer.value / 100)

def loop():
    # try to vary some intensities of the leds
    time.sleep(5)

def end():
    print("Ending rainbow mode!")