import rpipy

print("Device info:", rpipy.get_device_info(), "| temp:", rpipy.measure_temp())
rpipy.blink_led(26, 500)

input("Press enter to end program\n")