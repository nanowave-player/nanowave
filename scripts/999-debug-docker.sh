#!/usr/bin/env bash
docker run --rm -it \
    -v "$PWD":/project \
    -w /project/buildroot/buildroot-2025.02.2 \
    hiby-r1-cross-gnu /bin/bash

# pkg-config --libs --cflags fontconfig
