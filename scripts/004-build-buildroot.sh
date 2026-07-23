#!/usr/bin/env bash
docker run --rm -it \
    -u $(id -u):$(id -g) \
    -v "$PWD":/home/andreas/projects/nanowave-player/nanowave/ \
    -w /home/andreas/projects/nanowave-player/nanowave/buildroot/buildroot-2025.02.2 \
    -e FORCE_UNSAFE_CONFIGURE=1 \
    hiby-r1-cross \
    make -j$(nproc)
