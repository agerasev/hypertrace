#!/usr/bin/bash

lcov --capture --directory . --output-file .coverage.info && \
lcov --quiet --remove .coverage.info '/usr/*' --output-file .coverage.info && \
lcov --list .coverage.info
