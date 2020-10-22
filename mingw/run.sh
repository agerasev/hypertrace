#!/usr/bin/env bash

mkdir -p ./output && \
./script/build.sh $1 && \
./build/src/examples/$1 ${@:2}
