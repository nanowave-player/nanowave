#!/bin/sh

# rustup component add rust-src --toolchain nightly
# experimental
# rustup component add rustc



#export PKG_CONFIG_SYSROOT_DIR=$(pwd)/buildroot/buildroot-2025.02.2/output/host/mipsel-buildroot-linux-gnu/sysroot
#export PKG_CONFIG_PATH=$PKG_CONFIG_SYSROOT_DIR/usr/lib/pkgconfig:$PKG_CONFIG_SYSROOT_DIR/usr/share/pkgconfig
#export PKG_CONFIG_ALLOW_CROSS=1
#export CROSS_CONTAINER_OPTS="-v $(pwd)/buildroot:/project/buildroot:ro"



#export SDK=/home/andreas/projects/baijz/ingenic-toolchain/mips-gcc520-glibc222
# export PKG_CONFIG_SYSROOT_DIR=$SDK/mips-linux-gnu/libc
#export PKG_CONFIG_ALLOW_CROSS=1
# export PKG_CONFIG_PATH=$PKG_CONFIG_SYSROOT_DIR/usr/lib/pkgconfig

# does not work, missing .pc files
# export PKG_CONFIG_SYSROOT_DIR=$PWD/buildroot/buildroot-2025.02.2/output/target
# export PKG_CONFIG_LIBDIR=$PWD/buildroot/buildroot-2025.02.2/output/target/usr/lib/pkgconfig:$PWD/buildroot/buildroot-2025.02.2/output/target/usr/share/pkgconfig

# next try
#export PKG_CONFIG_ALLOW_CROSS=1
#
#export BR2=$PWD/buildroot/buildroot-2025.02.2
#export PATH=$BR2/output/host/bin:$PATH
#export PKG_CONFIG_SYSROOT_DIR=$BR2/output/host/mipsel-buildroot-linux-gnu/sysroot
#export PKG_CONFIG_LIBDIR=$BR2/output/host/mipsel-buildroot-linux-gnu/sysroot/usr/lib/pkgconfig:$BR2/output/host/mipsel-buildroot-linux-gnu/sysroot/usr/share/pkgconfig
#export CC=mips-linux-gnu-gcc
#export CXX=mips-linux-gnu-g++
#export AR=mips-linux-gnu-ar

#echo "CC=$CC"
#which $CC

# find "$BR2/output/staging" -name fontconfig.pc

# find $BR2/output/staging -name "fontconfig.pc"
# find $BR2/output/staging -name "libudev.pc"
# find $BR2/output/staging -name "*.pc" | grep -E "fontconfig|udev"
# pkg-config --variable pc_path pkg-config
# which pkg-config
# pkg-config --cflags --libs fontconfig
# pkg-config --cflags --libs libudev


#echo $PKG_CONFIG_SYSROOT_DIR
#echo $PKG_CONFIG_LIBDIR
#pkg-config --variable pc_path pkg-config
#
#find $BR2/output/host/mipsel-buildroot-linux-gnu/sysroot -name "fontconfig.pc" -o -name "libudev.pc"
#exit

# fake the pc file for udev
#mkdir -p buildroot/buildroot-2025.02.2/output/staging/usr/lib/pkgconfig
#cat > buildroot/buildroot-2025.02.2/output/staging/usr/lib/pkgconfig/libudev.pc <<'EOF'
#prefix=/usr
#exec_prefix=${prefix}
#libdir=/lib
#includedir=${prefix}/include
#
#Name: libudev
#Description: udev library
#Version: 1.6.3
#Libs: -L${libdir} -ludev
#Cflags: -I${includedir}
#EOF


export BR2=$PWD/buildroot/buildroot-2025.02.2
export PATH=$BR2/output/host/bin:$PATH

export CC_mipsel_hardfloat_unknown_linux_gnu=mipsel-linux-gcc
export CXX_mipsel_hardfloat_unknown_linux_gnu=mipsel-linux-g++
export AR_mipsel_hardfloat_unknown_linux_gnu=mipsel-linux-ar

export STAGING_DIR=$BR2/output/host/mipsel-buildroot-linux-gnu/sysroot

export PKG_CONFIG_SYSROOT_DIR="$STAGING_DIR"
export PKG_CONFIG_LIBDIR="$STAGING_DIR/usr/lib/pkgconfig:$STAGING_DIR/usr/share/pkgconfig"
export PKG_CONFIG_ALLOW_CROSS=1

cargo +nightly build \
  --target mipsel-hardfloat-unknown-linux-gnu.json \
  -Z json-target-spec \
  -Z build-std=core,alloc,std,proc_macro,panic_abort \
  --release

# cross +nightly build \
#   --target mipsel-hardfloat-unknown-linux-gnu.json \
#   -Z json-target-spec \
#   -Z build-std=core,alloc,std,proc_macro,panic_abort \
#   --release

# cross +nightly build \
#   --no-default-features \
#   --features "slint/backend-linuxkms-noseat,slint/renderer-software,slint/compat-1-2" \
#   --target mipsel-hardfloat.json \
#   -Z json-target-spec \
#   -Z build-std=core,alloc,std,proc_macro,panic_abort \
#   --release



#cross +nightly build --no-default-features --features "slint/backend-linuxkms-noseat,slint/renderer-software,slint/compat-1-2" \
#    --target mipsel-unknown-linux-gnu \
#    -Z build-std=core,alloc,std,proc_macro,panic_abort \
#    --release




# build binary - ./target/mipsel-unknown-linux-musl/release/nanowave
#cross +nightly build --no-default-features --features "slint/backend-linuxkms-noseat,slint/renderer-software,slint/compat-1-2" \
#    -Z build-std=core,alloc,std,proc_macro,panic_abort \
#    --target mipsel-unknown-linux-musl \
#    --release -vv


 # ./build-mipsel32.sh > build-mipsel32-5.log 2>&1

