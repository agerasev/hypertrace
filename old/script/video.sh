#!/usr/bin/env bash

if [ -z $1 ]; then
    export FRAMERATE=25
else
	export FRAMERATE=$1
fi && \

ffmpeg -framerate $FRAMERATE -i output/%d.png output.mp4
