all:
	cd RPiPython/ && $(MAKE) && cp target/release/libraspberry_pi_lib.so ../ScriptControl/scriptcontrol/demo/raspberrypylib.so && cp Lib/rpipy.py ../ScriptControl/scriptcontrol/demo/rpipy.py
