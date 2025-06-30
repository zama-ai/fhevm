# syntax=docker/dockerfile:1

# RUST_IMAGE_VERSION arg can be used to override the default version
ARG RUST_IMAGE_VERSION=1.87.0

## Multistage build to reduce image size
# First stage sets up basic Rust build environment
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS base

# The profile used to run `cargo build`
ARG LTO_RELEASE=release

# Use root user for build stage
USER root

RUN cargo install sccache --version ^0.7
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apk,sharing=locked \
    --mount=type=cache,target=/var/lib/apk,sharing=locked \
    apk add --no-cache \
    openssh-keyscan \
    protoc \
    gcc

WORKDIR /app
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY kms-connector ./kms-connector

# Setup SSH and git
RUN mkdir -p -m 0600 /root/.ssh && \
    ssh-keyscan -H github.com >> ~/.ssh/known_hosts && \
    mkdir -p /app/kms-connector/bin

# Configure git with secure token handling
RUN --mount=type=secret,id=BLOCKCHAIN_ACTIONS_TOKEN,env=BLOCKCHAIN_ACTIONS_TOKEN \
    git config --global url."https://$BLOCKCHAIN_ACTIONS_TOKEN@github.com".insteadOf ssh://git@github.com

# Build with improved caching
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked cd kms-connector && \
    cargo build --profile=${LTO_RELEASE} -p gw-listener

# Change user to limit root access
RUN chown -R fhevm:fhevm /app/kms-connector

# Final runtime stage for the three sub-components
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod
COPY --from=base /app/kms-connector/target/release/gw-listener /app/kms-connector/bin/
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -f "gw-listener" > /dev/null || exit 1
ENTRYPOINT ["/app/kms-connector/bin/gw-listener", "start"]

FROM prod AS dev
