#!/usr/bin/env bash
docker run --rm -it \
    -v "$PWD":/project \
    -w /project/buildroot/buildroot-2025.02.2 \
    -e FORCE_UNSAFE_CONFIGURE=1 \
    hiby-r1-cross \
    make BR2_DEFCONFIG=/project/buildroot/configs/hiby_r1_defconfig defconfig



# mkdir -p toolchain/sdk

#tar -xf \
#    buildroot/buildroot-2025.02.2/output/images/host-sdk.tar.gz \
#    -C toolchain/sdk \
#    --strip-components=1