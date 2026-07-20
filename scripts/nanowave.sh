#!/bin/sh
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

export FONTCONFIG_PATH="$DIR/fonts"
export FONTCONFIG_FILE="$DIR/fonts/fonts.conf"

# font config debug output
export FC_DEBUG=2048

RUST_BACKTRACE=1 ./nanowave