#!/usr/bin/env bash

mkdir -p ./output
./script/build.sh $1 && \
./build/$1 ${@:2}
