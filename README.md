# Rusty Raspberry Hourglas

The project is in early state, the documentation will be finalized later.

Currently here you find more or less just a few notes for myself.

# Crosscompile

For crosscompilation the rust cross tool is a good option.

https://github.com/rust-embedded/cross

// Install
$ cargo install cross

// (ONCE PER BOOT)
// Start the Docker daemon, if it's not already running
$ sudo systemctl start docker

// MAGIC! This Just Works
$ cross build --target arm-unknown-linux-gnueabihf

// EVEN MORE MAGICAL! This also Just Works
$ cross test --target arm-unknown-linux-gnueabihf

// Obviously, this also Just Works
$ cross rustc --target arm-unknown-linux-gnueabihf --release -- -C lto

// Copy over to the Pi then log in and run
$ scp target/arm-unknown-linux-gnueabihf/debug/hourglas pi@rasp-hourglas.local:~/

// Or do build, copy and run in one line to be the laziest dev out there, eventually later I'll add a script
$ cross build --release --target arm-unknown-linux-gnueabihf && scp target/arm-unknown-linux-gnueabihf/debug/hourglas pi@rasp-hourglas.local:~/ && ssh pi@rasp-hourglas.local "~/hourglas"

# Hardware

* Raspberry Pi Zero W
* http://www.waveshare.com/wiki/2.23inch_OLED_HAT
* 

## Setup Display Connection

SPI 0

## Setup I2S Sound Breakout Connection

5V Raspberry -> Vin
GND Raspberry -> GND
PIN18 Raspberry -> BCLK
PIN19 Raspberry -> LRC
PIN21 Raspberry -> DIN

## Check I2S Sound Breakout Connection

# License

Licensed, at your option, under either the
[Apache License, Version 2.0](LICENSE-APACHE) or the [MIT license](LICENSE-MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache License, Version 2.0,
shall be dual licensed as above, without any additional terms or conditions.
