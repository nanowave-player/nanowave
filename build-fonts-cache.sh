#!/bin/sh
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
FONTCONFIG_FILE=$DIR/fonts.conf fc-cache -fv