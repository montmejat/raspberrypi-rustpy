# Quick Start

- Go into the root of the project and build it in release: `cargo build --release`.
- Go where the executable has been built: `cd target/thumbv7em-none-eabihf/release`.
- Copy the executable into a binary file: `arm-none-eabi-objcopy -O binary lumino lumino.bin`
- Remove any old software on the board: `st-flash erase`
- Flash the binaries onto the board: `st-flash write lumino.bin 0x8000000`

# Debugging

Start `openocd` in a terminal from the root of the project and then in an other terminal launch `cargo run`. You will be into a `gdb` session connected to `openocd` waiting at the entry of the `main` function.

If you run into any issues, you can try erasing the flash memory (`st-flash erase`) and unplugging/plugging the board back on. If it's still not working try using `st-util` instead of `openocd`.

# More Info

Micro controller used [Nucleo L496ZG-P](https://www.st.com/en/microcontrollers-microprocessors/stm32l496zg.html?ecmp=tt9470_gl_link_feb2019&rt=db&id=DB3171#resource). More info on the board [here](https://www.st.com/en/evaluation-tools/nucleo-l496zg.html#overview).
