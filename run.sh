#!/bin/sh

mkdir -p build && \
cd build && \

cmake .. && \
make && \

cd .. && \

./build/hypertrace $@

