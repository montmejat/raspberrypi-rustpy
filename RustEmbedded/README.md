# Quick Start

`arm-none-eabi-objcopy -O binary app app.bin`
`st-flash erase`
`st-flash write app.bin 0x8000000`