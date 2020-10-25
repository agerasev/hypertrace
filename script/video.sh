#!/usr/bin/env bash

ffmpeg -framerate 60 -i output/%d.png -crf 18 output.mp4
