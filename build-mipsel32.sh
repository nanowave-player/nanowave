#!/bin/sh



# rustup component add rust-src --toolchain nightly


# experimental
# rustup component add rustc

cross +nightly build --no-default-features --features "slint/backend-linuxkms-noseat,slint/renderer-software,slint/compat-1-2" \
    -Z build-std=core,alloc,std,proc_macro,panic_abort \
    --target mipsel-unknown-linux-musl \
    --release