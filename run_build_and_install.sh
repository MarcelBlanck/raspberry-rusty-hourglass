#!/bin/sh
cross build -j 6 --target arm-unknown-linux-gnueabihf \
&& ssh pi@hourglass.local "killall -9 hourglass" \
; scp target/arm-unknown-linux-gnueabihf/debug/hourglass pi@hourglass.local:~/ \
&& ssh pi@hourglass.local "~/hourglass" \