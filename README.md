# Rusty Raspberry Hourglass

This hobby project is in early state, the documentation and rebuild instructions will be finalized later.

Currently this file contains more or less just a few notes for myself.

# Motivation

My 4y old son is allowed 20 minutes screentime a day. Sometimes he earns a few bonus minutes while playing. I wanted a visual and accustic representation for him to get a feeling for the time passing and what it visually means to get 5 bonus minutes compared to 2 bonus minutes.

But I think the hourglass can be used for basically anything an hourglass can be used for.

# Interesting aspects

1. Using the Rust Programming Language
2. Controlling the Raspberry Pi Zero's GPIO pins, SPI and I2S using Rust and crates
3. Setting up a webserver in Rust
4. Testing with mockall and Dependency Inversion via Rust's generics

# Crosscompile

For crosscompilation the rust cross tool is the best option and easy to use.

```
// (ONCE, MANDATORY)
// Install cross with cargo
// More info on: https://github.com/rust-embedded/cross
$ cargo install cross

// (ONCE PER BOOT, MANDATORY)
// Start the Docker daemon, or start the Docker gui app
$ sudo systemctl start docker

// (ONCE, MANDATORY)
// Build the docker image for cross compilation in the repo root path
$ docker build -t hourglass-crossbuild:local .

// (ONCE, OPTIONAL)
// Generating a ssh key and copying it over to the pi makes life easy
$ ssh-copy-id <USERNAME>@<HOSTNAME or IP>

// To build and run on target execute the script (make sure dockerdaemon is running)
$ ./run_build_and_install.sh

// Run tests in the docker container with
$ ./run_cross_test.sh
```

# Hardware

* Raspberry Pi Zero W
* http://www.waveshare.com/wiki/2.23inch_OLED_HAT
* SparkFun I2S Audio Breakout, MAX98357A
* 8 Ohm Speaker

## Setup Display Connection

* SPI 0
* Reset pin on BMC 25
* DC pin on BMC 24

## Setup I2S Sound Breakout Connection

https://learn.adafruit.com/adafruit-max98357-i2s-class-d-mono-amp/raspberry-pi-usage

* 5V Raspberry -> Vin
* GND Raspberry -> GND
* PIN18 Raspberry -> BCLK
* PIN19 Raspberry -> LRC
* PIN21 Raspberry -> DIN.

# Tools

I created a few tools in the tools/ directory. To use them please install python3. I recommend using a virtual environment in the root path of the repo.

```
python3 -m venv env
source env/bin/activate
pip3 install -r requirements.txt
```

You will have to run `source env/bin/activate` everytime when you navigate to the root directory of the repo with a new terminal window.

## png_to_bw_array.py

The script will output a Rust array declaration and initialization for copy-paste usage.

The script needs a path to a greyscale png image and will transform it to black-white dotmatrix image data.

Usage:
```
python3 png_to_bw_array.py [threshold 0-255] [path to greyscale png file]
```

# License

Licensed, at your option, under either the [Apache License, Version 2.0](LICENSE-APACHE) or the [MIT license](LICENSE-MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache License, Version 2.0, shall be dual licensed as above, without any additional terms or conditions.
