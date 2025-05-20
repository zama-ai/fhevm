# Stage 1: Build contracts
FROM ghcr.io/zama-ai/fhevm-node:latest AS contract_builder

USER root

WORKDIR /app
COPY ./contracts ./contracts

# Compiled contracts for listeners
WORKDIR /app/contracts
RUN npm install && npx hardhat compile

# Stage 2: Build fhevm-engine
FROM cgr.dev/chainguard/glibc-dynamic:latest-dev AS builder

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
    openssh && \
    rm -rf /var/cache/apk/*

# Install Rust using rustup
ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTUP_HOME=/root/.rustup
ENV CARGO_HOME=/root/.cargo

SHELL ["/bin/ash", "-o", "pipefail", "-c"]

# Install Rust toolchain - modified to avoid sourcing an external file
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=$RUST_IMAGE_VERSION && \
    export PATH="$CARGO_HOME/bin:$PATH"

RUN cargo install sccache --version ^0.7

# Set environment variables for building
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache
ENV CC=gcc
ENV CXX=g++
ENV RUSTFLAGS="-C target-feature=-crt-static"
ENV OPENSSL_DIR=/usr

WORKDIR /app

COPY ./contracts/contracts/ ./contracts/contracts/
COPY --from=contract_builder /app/contracts/artifacts/contracts /app/contracts/artifacts/contracts
COPY ./fhevm-engine/ ./fhevm-engine/
COPY ./proto/ ./proto/

WORKDIR /app/fhevm-engine

# Build all binaries
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release

# Stage 3: Final image
FROM cgr.dev/chainguard/glibc-dynamic:latest

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /usr/bin/ /usr/bin/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/ssl/certs/ /etc/ssl/certs/
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

COPY --from=builder --chown=fhevm:fhevm /app/fhevm-engine/target/release/coprocessor /usr/local/bin/coprocessor
COPY --from=builder --chown=fhevm:fhevm /app/fhevm-engine/target/release/sns_worker /usr/local/bin/sns_worker
COPY --from=builder --chown=fhevm:fhevm /app/fhevm-engine/target/release/zkproof_worker /usr/local/bin/zkproof_worker
COPY --from=builder --chown=fhevm:fhevm /app/fhevm-engine/target/release/fhevm_listener /usr/local/bin/fhevm_listener
COPY --from=builder --chown=fhevm:fhevm /app/fhevm-engine/target/release/gw_listener /usr/local/bin/gw_listener
COPY --from=builder --chown=fhevm:fhevm /app/fhevm-engine/target/release/transaction_sender /usr/local/bin/transaction_sender
COPY --from=builder --chown=fhevm:fhevm /app/fhevm-engine/target/release/cli /usr/local/bin/cli

USER fhevm:fhevm

CMD ["/usr/local/bin/coprocessor --help"]
