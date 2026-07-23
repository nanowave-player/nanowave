#!/usr/bin/env bash

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
BUILDROOT_DIR="$DIR/../buildroot/"

cd "$BUILDROOT_DIR/buildroot-2025.02.2" || exit 1

if [ "$1" = "clean" ]; then
  make distclean
fi

make BR2_DEFCONFIG=../configs/hiby_r1_defconfig defconfig
cd -
