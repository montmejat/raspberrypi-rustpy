import luminolib

param = luminolib.Settings()
led_matrix = luminolib.Led(22)
turned_on_light = 0

def start():
    print("Starting rainbow mode!")

def loop():
    global turned_on_light
    
    led = led_matrix.get(turned_on_light)
    led.green = 5
    led.red = 5
    led.blue = 5

    if turned_on_light > 0:
        led = led_matrix.get(turned_on_light - 1)
        led.green = 0
        led.red = 0
        led.blue = 0

    turned_on_light += 1
    if turned_on_light > 21:
        turned_on_light = 0

def end():
    print("Ending rainbow mode!")