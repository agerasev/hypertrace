#!/usr/bin/env bash

if [[ $(uname -s) == MSYS* ]]; then
    export PATH=$PATH:/mingw64/bin
fi

if [ -z $1 ]; then
    export FRAMERATE=25
else
	export FRAMERATE=$1
fi && \

ffmpeg -framerate $FRAMERATE -i output/%d.png output.mp4
