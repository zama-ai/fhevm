# syntax=docker/dockerfile:1

# RUST_IMAGE_VERSION arg can be used to override the default version
ARG RUST_IMAGE_VERSION=1.87.0

# Stage 1: Build kms-worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

# The profile used to run `cargo build`
ARG LTO_RELEASE=release

# Use root user for build stage
USER root

WORKDIR /app

COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY kms-connector ./kms-connector

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apk,sharing=locked \
    --mount=type=cache,target=/var/lib/apk,sharing=locked \
    apk add --no-cache \
    gcc \
    protoc

# Build with improved caching
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked cd kms-connector && \
    cargo build --profile=${LTO_RELEASE} -p kms-worker

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/kms-connector/target/release/kms-worker /app/kms-connector/bin/kms-worker

USER fhevm:fhevm

ENTRYPOINT ["/app/kms-connector/bin/kms-worker", "start"]

FROM prod AS dev
