#!/bin/sh
cross build -j 6 --target arm-unknown-linux-gnueabihf \
&& ssh pi@hourglass "sudo systemctl stop hourglass.service ; killall -9 hourglass" \
; scp target/arm-unknown-linux-gnueabihf/debug/hourglass pi@hourglass:~/ \
; ssh pi@hourglass "sudo systemctl start hourglass.service"