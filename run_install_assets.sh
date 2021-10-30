#!/bin/sh
ssh pi@hourglass "mkdir ~/html" \
&& ssh pi@hourglass "mkdir ~/audio" \
&& scp html/index.html pi@hourglass:~/html \
&& scp audio/424244__aceinet__number-90-flange-the-hammer-on-e.wav pi@hourglass:~/audio