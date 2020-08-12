#!/usr/bin/env bash

if [ -z $1 ]; then
    TEST=unit
else
    TEST=$1
fi && \
./script/build.sh test_${TEST} && \
./build/test/test_${TEST} ${@:2}
