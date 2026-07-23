#!/usr/bin/env bash

# how to docker as non root
# https://oneuptime.com/blog/post/2026-01-16-docker-run-non-root-user/view

# output/images/host-sdk.tar.gz
docker run --rm -it \
    -u $(id -u):$(id -g) \
    -v "$PWD":/home/andreas/projects/nanowave-player/nanowave/ \
    -w /home/andreas/projects/nanowave-player/nanowave/buildroot/buildroot-2025.02.2 \
    -e FORCE_UNSAFE_CONFIGURE=1 \
    hiby-r1-cross \
    make sdk


# mkdir -p toolchain/sdk

#tar -xf \
#    buildroot/buildroot-2025.02.2/output/images/host-sdk.tar.gz \
#    -C toolchain/sdk \
#    --strip-components=1