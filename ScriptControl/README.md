# Raspberry Hardware Script Launching Server

## `Demo.py` script

Simple test script:

```python
from gpiozero import LED
from time import sleep
import rpipy

led = LED(26)
my_var = 50
my_message = "Hello!"

def start():
    print("Device info:", rpipy.get_device_info(), "| temp:", rpipy.measure_temp())

def loop():
    print("Looping in demo! My var =", my_var, "My message:", my_message)
    led.on()
    sleep(1)
    led.off()
    sleep(1)
    print("Ending loop in demo!")

def end():
    print("Ending demo!")

def my_func():
    print("Some custom stuff!")
```

## Communication with the server

In the main loop of the server, it first executes the code of the `loop()` function defined in `demo.py` then tells all connected clients that it is listening for 2 seconds. If no new connections have been detected, it will continue executing the `loop()` code.

Commands you can send:

Basic control:

- `pause`: pause the execution of `loop()`.
- `unpause`: unpause the execution of `loop()`.
- `restart`: execute `start()` again.
- `save:filename`: saves all the variables from `demo.py`. If the filename is not specified it is `demo_vars.pkl` by default. The filename extension must be specified if custom. 
- `load:filename`: loads all the variables from `demo.py`. Same option for the filename as with the `save` command.

More options:

- `func:my_func`: launch custom function defined in `demo.py`.
- `var:my_var=value`: change a variable to the given value. Passing the type is also possible: `var:my_var=int|float|str|bool(value)`.

## More stuff

Possibility to hide server debug messages (although print messages from `demo.py` will still show up) when calling `start_server` and `app_loop`:

```python
def start_server(print_debug=True):
def app_loop(server_socket, print_debug=True, send_debug_to_client=True):
```