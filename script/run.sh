#!/usr/bin/bash

if [[ $(uname -s) == MSYS* ]]; then
    export PATH=$PATH:/mingw64/bin
fi

mkdir -p ./output
./script/build.sh $1 && \
./build/$1 ${@:2}
