#!/usr/bin/env bash
docker run --rm -it \
    -v "$PWD":/project \
    -w /project/buildroot/buildroot-2025.02.2 \
    -e FORCE_UNSAFE_CONFIGURE=1 \
    hiby-r1-cross \
    make -j$(nproc)
