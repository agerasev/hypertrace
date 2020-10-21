#!/usr/bin/env bash

mkdir -p build && \
cd build && \

if [ -z $SINGLE_THREAD ]; then
    NCPU=$(grep -c ^processor /proc/cpuinfo)
else
    NCPU=1
fi

cmake .. -DUSE_PCH=ON && \
if [ -z $1 ]; then
    make -j $NCPU
else
    make -j $NCPU $@
fi && \

cd ..
