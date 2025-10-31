FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0-dev AS prod

ARG RUST_IMAGE_VERSION=1.91.0

USER root

RUN mkdir -p /app && \
    addgroup -g 10001 fhevm && \
    adduser -D -u 10000 -G fhevm fhevm && \
    mkdir -p /app /home/fhevm

RUN apk add --no-cache \
    curl \
    wget \
    bash \
    git \
    make \
    perl \
    binutils \
    ca-certificates \
    gcc \
    libstdc++ \
    linux-headers \
    build-base \
    openssl-dev \
    protoc \
    protobuf \
    postgresql-client \
    vim && \
    rm -rf /var/cache/apk/*

# Install Rust in a location accessible to all users
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH="/usr/local/cargo/bin:${PATH}"

SHELL ["/bin/ash", "-o", "pipefail", "-c"]

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=$RUST_IMAGE_VERSION && \
    chmod -R a+w /usr/local/cargo && \
    chmod -R a+w /usr/local/rustup

# Set environment variables for building
ENV RUSTFLAGS="-C target-feature=-crt-static" \
    CC=gcc \
    CXX=g++ \
    OPENSSL_DIR=/usr

USER fhevm:fhevm

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD cargo --version || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
