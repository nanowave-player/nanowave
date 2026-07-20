#!/bin/sh

# rustup component add rust-src --toolchain nightly
# experimental
# rustup component add rustc



export CROSS_CONTAINER_OPTS="-v $(pwd)/buildroot:/project/buildroot:ro"

# build binary - ./target/mipsel-unknown-linux-musl/release/nanowave
cross +nightly build --no-default-features --features "slint/backend-linuxkms-noseat,slint/renderer-software,slint/compat-1-2" \
    -Z build-std=core,alloc,std,proc_macro,panic_abort \
    --target mipsel-unknown-linux-musl \
    --release -vv


 # ./build-mipsel32.sh > build-mipsel32-5.log 2>&1

