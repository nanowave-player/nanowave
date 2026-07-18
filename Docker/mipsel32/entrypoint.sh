#!/usr/bin/env bash
set -e

SDK=/project/buildroot/buildroot-2025.02.2/output/host
PREFIX=mipsel-buildroot-linux-musl
GCC="$SDK/bin/${PREFIX}-gcc"

if [ -d "$SDK" ] && [ -f "$GCC" ]; then
    SYSROOT=$("$SDK/bin/${PREFIX}-gcc" -print-sysroot)

    export PATH="$SDK/bin:$PATH"

    export CC_mipsel_unknown_linux_musl="${PREFIX}-gcc"
    export CXX_mipsel_unknown_linux_musl="${PREFIX}-g++"
    export AR_mipsel_unknown_linux_musl="${PREFIX}-ar"
    export AS_mipsel_unknown_linux_musl="${PREFIX}-as"
    export LD_mipsel_unknown_linux_musl="${PREFIX}-ld"
    export NM_mipsel_unknown_linux_musl="${PREFIX}-nm"
    export STRIP_mipsel_unknown_linux_musl="${PREFIX}-strip"
    export RANLIB_mipsel_unknown_linux_musl="${PREFIX}-ranlib"

    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_SYSROOT_DIR="$SYSROOT"
    export PKG_CONFIG_LIBDIR="$SYSROOT/usr/lib/pkgconfig:$SYSROOT/usr/share/pkgconfig"
else
    echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
    echo "WARNING:"
    [ -d "$SDK" ] || echo "$SDK is not mounted"
    [ -f "$GCC" ] || echo "$GCC not available"
    echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
fi

exec "$@"