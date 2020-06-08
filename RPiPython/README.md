# Raspberry Pi Hardware Library

## Example

`cd Lib/`

```[python]
import rpipy

print("Device info:", rpipy.get_device_info(), "| temp:", rpipy.measure_temp())
rpipy.blink_led(26, 500)

wait = input("Press any key to end program") # blink led on GPIO 26 every 500 ms
```
