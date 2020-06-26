# Raspberry Hardware Demo Script Server

## `Demo.py` script

Simple test script:

```python
from gpiozero import LED
from time import sleep
import rpipy

led = LED(26)

class Settings:
    def __init__(self):
        self.my_var = 50
        self.my_message = "Hello!"
        self.slider_var = self.SliderValue(0, 100, 50)

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
    print("Ending loop in demo!")

def end():
    print("Ending demo!")

def my_func():
    print("Some custom stuff!")
```

## Communication with the server

### How it works

In the main loop of the server, it first executes the code of the `loop()` function defined in `demo.py` then tells all connected clients that it is listening for 2 seconds. If no new connections have been detected, it will continue executing the `loop()` code.

Commands you can send:

Basic control:

- `{ 'type': 'get', 'value': 'state' }`: get the state of the controller. Returns a CBOR encoded dictionnary `{ 'paused': 'true/false' }`
- `{ 'type': 'get', 'value': 'settings'}`: get the names and values of the variables saved in the `Settings` class. Returns a CBOR encoded dictionnary `{ 'var1': 'value', 'var2': 'value' }`
- `{ 'type': 'action', 'value': 'pause' }`: pause the execution of `loop()`.
- `{ 'type': 'action', 'value': 'unpause' }`: unpause the execution of `loop()`.
- `{ 'type': 'action', 'value': 'restart', 'key': 'test' }`: execute `start()` again. Needs an admin key.
- `{ 'type': 'action', 'value': 'save', 'arg': 'filename' }`: saves all the variables from `demo.py`. If the filename is not specified it is `demo_vars.pkl` by default. The filename extension must be specified if custom. 
- `{ 'type': 'action', 'value': 'load', 'arg': 'filename' }`: loads all the variables from `demo.py`. Same option for the filename as with the `save` command.

More options:

- `{ 'type': 'call', 'value': 'my_func' }`: launch custom function defined in `demo.py`.
- `{ 'type': 'set', 'var': 'my_val', 'value': 50, 'cast': '[int|float|bool|str]' }`: change a variable to the given value. Passing the type in `cast` is optionnal.

### Test it yourself

Start the app:

`python3 demo_controller_app.py`

In an other shell, you can try sending it commands:

```python
>>> import cbor, zmq
>>> context = zmq.Context()
>>> socket = context.socket(zmq.REQ)
>>> # send request
>>> socket.connect("tcp://localhost:5555")
>>> message = { 'type': 'set', 'var': 'my_var', 'value': '40' }
>>> socket.send(cbor.dumps(message))
>>> # get response 
>>> data = socket.recv()
```

## More stuff

Possibility to hide server debug messages (although print messages from `demo.py` will still show up) when calling `start_server` and `app_loop`:

```python
def start_server(print_debug=True):
def app_loop(server_socket, print_debug=True, send_debug_to_client=True):
```
