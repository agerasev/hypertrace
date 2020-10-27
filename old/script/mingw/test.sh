#!/usr/bin/env bash

if [ -z $1 ]; then
    TEST=unit
else
    TEST=$1
fi && \
./script-mingw/build.sh test_${TEST} && \
./build/src/test/test_${TEST} ${@:2}
