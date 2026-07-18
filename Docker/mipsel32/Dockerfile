FROM debian:bookworm

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    build-essential \
    bc bison flex rsync cpio texinfo patch \
    git wget curl file unzip xz-utils \
    python3 python3-pip \
    cmake ninja-build meson \
    pkg-config \
    clang libclang-dev llvm \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# RUN rustup toolchain install nightly
# RUN rustup target add --toolchain nightly mipsel-unknown-linux-musl
# RUN rustup component add --toolchain nightly rust-src
# RUN rustup component add rust-src --toolchain nightly


RUN rustup toolchain install nightly
RUN rustup component add rust-src --toolchain nightly

COPY Docker/mipsel32/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

WORKDIR /project

ENTRYPOINT ["/entrypoint.sh"]
CMD ["bash"]