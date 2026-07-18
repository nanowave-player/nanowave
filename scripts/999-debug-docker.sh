#!/usr/bin/env bash
docker run --rm -it \
    -v "$PWD":/project \
    -w /project/buildroot/buildroot-2025.02.2 \
    hiby-r1-cross /bin/bash

# pkg-config --libs --cflags fontconfig
