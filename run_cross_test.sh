#!/bin/sh
export RUST_BACKTRACE=full
cross test -j 6 --target arm-unknown-linux-gnueabihf
