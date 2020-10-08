#!/usr/bin/env bash

mkdir -p build && \
cd build && \

cmake .. -DUSE_PCH=ON && \
if [ -z $1 ]; then
    make
else
    make $@
fi && \

cd ..
