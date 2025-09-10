# syntax=docker/dockerfile:1

# RUST_IMAGE_VERSION arg can be used to override the default version
ARG RUST_IMAGE_VERSION=1.89.0

# Stage 1: Build tx-sender
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

# The profile used to run `cargo build`
ARG LTO_RELEASE=release

# Use root user for build stage
USER root

WORKDIR /app

# Copy git directory to include commit hash in build info
COPY .git ./.git

# Copy sources
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY kms-connector ./kms-connector

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apk,sharing=locked \
    --mount=type=cache,target=/var/lib/apk,sharing=locked \
    apk add --no-cache \
    gcc \
    protoc

# Build with improved caching
WORKDIR /app/kms-connector
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    git config --global --add safe.directory /app && \
    cargo build --profile=${LTO_RELEASE} -p tx-sender

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/kms-connector/target/release/tx-sender /app/kms-connector/bin/tx-sender

USER fhevm:fhevm

ENTRYPOINT ["/app/kms-connector/bin/tx-sender", "start"]

HEALTHCHECK --start-period=5s --interval=1m --timeout=3s --retries=3 \
    CMD ["/app/kms-connector/bin/tx-sender", "health", "--endpoint", "http://127.0.0.1:9100/healthz"]

FROM prod AS dev
