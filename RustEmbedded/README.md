# Quick Start

`arm-none-eabi-objcopy -O binary app app.bin`

`st-flash erase`

`st-flash write app.bin 0x8000000`

# Debugging

Start `openocd` in a terminal from the root of the project and then in an other terminal launch `cargo run`.

If you run into any issues, you can try erasing the flash memory (`st-flash erase`) and unplugging/plugging the board back on.

# More Info

Micro controller used [Nucleo L496ZG-P](https://www.st.com/en/microcontrollers-microprocessors/stm32l496zg.html?ecmp=tt9470_gl_link_feb2019&rt=db&id=DB3171#resource)
