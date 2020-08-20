import luminolib, time

param = luminolib.Settings()
param.ms_sleep = luminolib.Settings.SliderValue(0, 1000, 0)

led_matrix = luminolib.Led(22)
hsv_lights = [ 0, 4, 8, 13, 17, 21, 25, 30, 34, 38, 42, 47, 51, 55, 59, 64, 68, 72, 76,
            81, 85, 89, 93, 98, 102, 106, 110, 115, 119, 123, 127, 132, 136, 140, 144,
            149, 153, 157, 161, 166, 170, 174, 178, 183, 187, 191, 195, 200, 204, 208,
            212, 217, 221, 225, 229, 234, 238, 242, 246, 251, 255 ]
angle = 0

def start():
    print("Starting rainbow mode!")

def loop():
    global angle

    print(f'Looping beautiful colors! Angle {angle}')
    angle += 1

    if angle < 60:
        red = 255
        green = hsv_lights[angle]
        blue = 0
    elif angle < 120:
        red = hsv_lights[120 - angle]
        green = 255
        blue = 0
    elif angle < 180:
        red = 0
        green = 255
        blue = hsv_lights[angle - 120]
    elif angle < 240:
        red = 0
        green = hsv_lights[240 - angle]
        blue = 255
    elif angle < 300:
        red = hsv_lights[angle - 240]
        green = 0
        blue = 255
    else:
        red = 255
        green = 0
        blue = hsv_lights[360 - angle]

    print(f'   -> values: (R: {red}, G: {green}, B: {blue})')

    for i in range(led_matrix.leds_count):
        led = led_matrix.get(i)
        led.red = red
        led.green = green
        led.blue = blue
    
    if angle > 359:
        angle = 0
    
    time.sleep(ms_sleep / 1000)

def end():
    print("Ending rainbow mode!")