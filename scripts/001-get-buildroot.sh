#!/usr/bin/env bash
BUILDROOT_VERSION=2025.02.2

if ! [ -d "buildroot/buildroot-${BUILDROOT_VERSION}" ]; then
  cd buildroot/ \
   && wget -c https://buildroot.org/downloads/buildroot-${BUILDROOT_VERSION}.tar.xz \
   && tar xf buildroot-${BUILDROOT_VERSION}.tar.xz \
   && cd -
fi
