FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0-dev AS builder

ARG RUST_IMAGE_VERSION=1.85.0

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
    openssh \
    postgresql-client \
    vim && \
    rm -rf /var/cache/apk/*

# Install Rust in a location accessible to all users
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH="/usr/local/cargo/bin:${PATH}"

SHELL ["/bin/ash", "-o", "pipefail", "-c"]

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=$RUST_IMAGE_VERSION && \
    chmod -R a+w /usr/local/cargo && \
    chmod -R a+w /usr/local/rustup

RUN cargo install sccache --version ^0.7

# Set environment variables for building
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache
ENV CC=gcc
ENV CXX=g++
ENV RUSTFLAGS="-C target-feature=-crt-static"
ENV OPENSSL_DIR=/usr


FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0-dev AS prod

# Runtime base
COPY --from=builder /lib/ /lib/
COPY --from=builder /lib64/ /lib64/
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /usr/lib64/ /usr/lib64/
COPY --from=builder /usr/lib/gcc/ /usr/lib/gcc/
COPY --from=builder /usr/libexec/ /usr/libexec/
COPY --from=builder /usr/include/ /usr/include/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/bin/ /usr/bin/

# OpenSSL
COPY --from=builder /etc/ssl/ /etc/ssl/
COPY --from=builder /usr/include/openssl/ /usr/include/openssl/
COPY --from=builder /usr/lib/libssl* /usr/lib/
COPY --from=builder /usr/lib/libcrypto* /usr/lib/

# User and group information
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

# Rust and Cargo
COPY --from=builder /usr/local/cargo /usr/local/cargo
COPY --from=builder /usr/local/rustup /usr/local/rustup
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH="/usr/local/cargo/bin:${PATH}"
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache
ENV RUSTFLAGS="-C target-feature=-crt-static"

USER fhevm:fhevm

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD cargo --version || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
