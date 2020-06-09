all:
	cd RPiPython/ && $(MAKE) && cp target/release/libraspberry_pi_lib.so ../LampController/raspberrypylib.so && cp Lib/rpipy.py ../LampController/rpipy.py
