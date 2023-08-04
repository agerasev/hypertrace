#!/usr/bin/env bash

mkdir -p ./output
./script/build.sh "${1}_${2}" && \
./build/examples/$1/host/"${1}_${2}" ${@:3}
