#!/bin/sh
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

export FONTCONFIG_PATH="$DIR/fonts"
export FONTCONFIG_FILE="$DIR/fonts/fonts.conf"

# font config debug output
# export FC_DEBUG=2048

# debug call with strace
# strace -f -e ioctl,read,epoll_wait,epoll_ctl ./nanowave
# timerfd_create problem
# strace -f -e trace=timerfd_create,timerfd_settime ./nanowave
# no filter debug call
# strace -f -yy -o trace.log ./nanowave

RUST_BACKTRACE=1 ./nanowave