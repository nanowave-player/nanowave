#!/usr/bin/env bash
docker run --rm -it --entrypoint /bin/bash \
    -v "$PWD":/project \
    -w /project/buildroot/buildroot-2025.02.2 \
    hiby-r1-cross