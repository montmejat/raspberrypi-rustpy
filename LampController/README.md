# Raspberry Hardware Script Launching Server

## `Demo.py` script

Simple test script:

```python
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
    print("Ending loop in demo!")

def end():
    print("Ending demo!")
```

## Communication with the server

In the main loop of the server, it first executes the code of the `loop()` function defined in `demo.py` then tells all connected clients that it is listening for 2 seconds. If no new connections have been detected, it will continue executing the `loop()` code.

Commands you can send:

- `pause`: pause the execution of `loop()`.
- `unpause`: unpause the execution of `loop()`.
- `restart`: restart server and execute `start()` again.
- `custom:my_func`: launch custom function defined in `demo.py`.
