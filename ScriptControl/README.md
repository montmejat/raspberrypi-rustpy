# Raspberry Hardware Demo Script Server

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

### How it works

In the main loop of the server, it first executes the code of the `loop()` function defined in `demo.py` then tells all connected clients that it is listening for 2 seconds. If no new connections have been detected, it will continue executing the `loop()` code.

Commands you can send:

Basic control:

- `{ 'type': 'action', 'value': 'pause' }`: pause the execution of `loop()`.
- `{ 'type': 'action', 'value': 'unpause' }`: unpause the execution of `loop()`.
- `{ 'type': 'action', 'value': 'restart', 'key': 'test' }`: execute `start()` again. Needs an admin key.
- `{ 'type': 'action', 'value': 'save', 'arg': 'filename' }`: saves all the variables from `demo.py`. If the filename is not specified it is `demo_vars.pkl` by default. The filename extension must be specified if custom. 
- `{ 'type': 'action', 'value': 'load', 'arg': 'filename' }`: loads all the variables from `demo.py`. Same option for the filename as with the `save` command.

More options:

- `{ 'type': 'call', 'value': 'my_func' }`: launch custom function defined in `demo.py`.
- `{ 'type': 'set', 'var': 'my_val', 'value': 50, 'cast': '[int|float|bool|str]' }`: change a variable to the given value. Passing the type is optionnal.

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
