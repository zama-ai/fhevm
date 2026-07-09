FROM cgr.dev/zama.ai/glibc-dynamic:16-dev AS prod

ARG RUST_IMAGE_VERSION
ARG SCCACHE_VERSION=v0.15.0

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

RUN set -eux; \
    arch="$(uname -m)"; \
    case "$arch" in \
        x86_64)  triple=x86_64-unknown-linux-musl ;; \
        aarch64) triple=aarch64-unknown-linux-musl ;; \
        *) echo "unsupported arch: $arch" >&2; exit 1 ;; \
    esac; \
    curl -fsSL "https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/sccache-${SCCACHE_VERSION}-${triple}.tar.gz" \
        | tar -xz --strip-components=1 -C /usr/local/bin "sccache-${SCCACHE_VERSION}-${triple}/sccache"; \
    chmod +x /usr/local/bin/sccache; \
    sccache --version

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
