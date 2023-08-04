#!/usr/bin/env bash

mkdir -p build && \
cd build && \

cmake ../src && \
if [ -z $1 ]; then
    make
else
    make $@
fi && \

cd ..
