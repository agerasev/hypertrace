#!/usr/bin/bash

mkdir -p build && cd build && \
mkdir -p kernel host && \
cd kernel && cmake -DCMAKE_TOOLCHAIN_FILE=../../kernel/spirv.cmake ../../kernel/ && make VERBOSE=1 && cd ..  && \
cd host && cmake ../../host/ && make VERBOSE=1 && cd ..  && \
echo "Success!"
