#!/bin/sh

./script/build.sh test && \
./build/test && \

# Show coverage info
lcov --capture --directory . --output-file .coverage.info && \
lcov --quiet --remove .coverage.info '/usr/*' --output-file .coverage.info && \
lcov --list .coverage.info
