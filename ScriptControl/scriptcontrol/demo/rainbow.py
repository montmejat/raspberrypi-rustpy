import luminolib, time

param = luminolib.Settings()
param.dimmer = luminolib.Settings.SliderValue(0, 100, 40)

led_matrix = luminolib.Led(22)

def start():
    print("Starting rainbow mode!")
    led_matrix.mode = 'rainbow'

def loop():
    time.sleep(10)

def end():
    print("Ending rainbow mode!")