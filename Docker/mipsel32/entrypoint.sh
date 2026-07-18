#!/usr/bin/env bash
set -e

SDK=${SDK_ROOT:-/opt/sdk}

PREFIX=$(
    find "$SDK/bin" -maxdepth 1 -type f -name "*-gcc" \
    | head -n1 \
    | xargs basename \
    | sed 's/-gcc$//'
)

if [ -z "$PREFIX" ]; then
    echo "No cross compiler found in $SDK/bin"
    exit 1
fi

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

exec "$@"