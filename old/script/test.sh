#!/usr/bin/env bash

./script/build.sh test && \
./build/test/test $@
