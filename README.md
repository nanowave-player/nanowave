# nanowave
Home of the nanowave player


# development

## Building mipsel32 (Hiby R1)

Prepare build environment requires a few steps to build a custom docker image including a mipsel32 toolchain via buildroot.

```sh 
# download buildroot
./scripts/001-get-buildroot.sh

# build custom docker image hiby-r1-cross:latest
./scripts/002-build-docker.sh

# transfer external config to buildroots .config
./scripts/003-build-buildroot-config.sh

# now build toolchain - may take a while
./scripts/004-build-buildroot.sh 

# now build sdk
./scripts/005-build-buildroot-sdk.sh

# build binary - later located at target/mipsel-unknown-linux-musl/release/nanowave
./build-mipsel32.sh
```

