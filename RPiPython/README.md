# Raspberry Pi Hardware Library

## Example

The make command will build the cargo projet and copy the compiled Rust library next to the Python library. 

`make`

Make sure to be in the same directory as the Python library. 

`cd Lib/`

You can test the following code:

`python3 demo.py`

```
import rpipy

print("Device info:", rpipy.get_device_info(), "| temp:", rpipy.measure_temp())
rpipy.blink_led(26, 500)

wait = input("Press any key to end program") # blink led on GPIO 26 every 500 ms
```

```
Device info: Raspberry Pi 3 B | temp: 46.2'C
Press enter to end program

```
