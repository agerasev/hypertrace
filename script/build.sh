#!/usr/bin/bash

mkdir -p build && \
cd build && \

cmake .. && \
if [ -z $1 ]; then
    make
else
    make $@
fi && \

cd ..
