#!/usr/bin/env bash

if [ "$1" = "clean" ]; then
  docker run --rm -it \
      -u $(id -u):$(id -g) \
      -v "$PWD":/home/andreas/projects/nanowave-player/nanowave/ \
      -w /home/andreas/projects/nanowave-player/nanowave/buildroot/buildroot-2025.02.2 \
      -e FORCE_UNSAFE_CONFIGURE=1 \
      hiby-r1-cross \
      make distclean
fi

docker run --rm -it \
    -u $(id -u):$(id -g) \
    -v "$PWD":/home/andreas/projects/nanowave-player/nanowave/ \
    -w /home/andreas/projects/nanowave-player/nanowave/buildroot/buildroot-2025.02.2 \
    -e FORCE_UNSAFE_CONFIGURE=1 \
    hiby-r1-cross \
    make BR2_DEFCONFIG=/home/andreas/projects/nanowave-player/nanowave/buildroot/configs/hiby_r1_defconfig defconfig



# mkdir -p toolchain/sdk

#tar -xf \
#    buildroot/buildroot-2025.02.2/output/images/host-sdk.tar.gz \
#    -C toolchain/sdk \
#    --strip-components=1