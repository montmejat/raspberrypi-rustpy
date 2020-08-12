# LED matrix to control via the webserver
class Led:
    def __init__(self, leds_count):
        self.leds = []
        self.leds_count = leds_count

        for i in range(self.leds_count):
            self.leds.append(self.Led(0, 0, 0))

    def get(self, i):
        return self.leds[i]

    class Led:
        def __init__(self, green, red, blue):
            self.green = green
            self.red = red
            self.blue = blue

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