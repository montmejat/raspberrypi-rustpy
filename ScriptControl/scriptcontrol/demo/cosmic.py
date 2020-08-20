import luminolib, time

param = luminolib.Settings()
param.dimmer = luminolib.Settings.SliderValue(0, 100, 40)

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