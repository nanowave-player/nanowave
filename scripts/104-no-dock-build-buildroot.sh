#!/usr/bin/env bash

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
BUILDROOT_DIR="$DIR/../buildroot/"

cd "$BUILDROOT_DIR/buildroot-2025.02.2" || exit 1
make -j$(nproc)
cd -
