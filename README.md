# Raspberry Pi Hardware Control in Rust and Python

Tested on Raspberry Pi 3 running on Raspberry Pi OS (32 bit).

![Smarphone Screenshot](Documentation/architecture.png)

## How to use

- Launch a `make all` in the root of the project to build the RustPython Library and copy the files to the demo folder. Go into each individual directories to learn more. 
- Connect to the bluetooth dongle by using `sudo bluetoothctl` and connecting with `sudo rfcomm connect hci0 hc05_addr`.