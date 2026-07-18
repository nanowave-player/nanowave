#!/usr/bin/env bash
set -euo pipefail

IMAGE=hiby-r1-cross
TAG=latest

docker build \
    -t ${IMAGE}:${TAG} \
    -f Docker/mipsel32/Dockerfile \
    .