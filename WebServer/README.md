# Rocket RPi Web-App Controller

To build and run:

`ROCKET_ENV=staging cargo run`

`ROCKET_ENV=staging` will enable the server to be accessible from the local network. 

## How to use

![Interface Screenshot](doc/screenshot_desktop.png)

You can use the app through the interface or send the following requests:

- `raspberry_ip:8000/pause`
- `raspberry_ip:8000/unpause`