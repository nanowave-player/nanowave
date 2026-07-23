#!/usr/bin/env bash
set -e

SDK=/home/andreas/projects/nanowave-player/nanowave/buildroot/buildroot-2025.02.2/output/host
PREFIX=mipsel-buildroot-linux-gnu
GCC="$SDK/bin/${PREFIX}-gcc"

if [ -d "$SDK" ] && [ -f "$GCC" ]; then
    SYSROOT=$("$SDK/bin/${PREFIX}-gcc" -print-sysroot)

    export PATH="$SDK/bin:$PATH"

    export CC_mipsel_unknown_linux_gnu="${PREFIX}-gcc"
    export CXX_mipsel_unknown_linux_gnu="${PREFIX}-g++"
    export AR_mipsel_unknown_linux_gnu="${PREFIX}-ar"
    export AS_mipsel_unknown_linux_gnu="${PREFIX}-as"
    export LD_mipsel_unknown_linux_gnu="${PREFIX}-ld"
    export NM_mipsel_unknown_linux_gnu="${PREFIX}-nm"
    export STRIP_mipsel_unknown_linux_gnu="${PREFIX}-strip"
    export RANLIB_mipsel_unknown_linux_gnu="${PREFIX}-ranlib"

    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_SYSROOT_DIR="$SYSROOT"
    export PKG_CONFIG_LIBDIR="$SYSROOT/usr/lib/pkgconfig:$SYSROOT/usr/share/pkgconfig"
else
    echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
    echo "WARNING:"
    echo "-> this might be ok, if you are still configuring and building buildroot"
    echo "-> this is NOT ok, if you are trying to build the rust app"
    echo ""
    [ -d "$SDK" ] || echo "$SDK is not mounted"
    [ -f "$GCC" ] || echo "$GCC not available"
    echo ""
    echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
fi

exec "$@"