# syntax=docker/dockerfile:1

# RUST_IMAGE_VERSION arg can be used to override the default version
ARG RUST_IMAGE_VERSION=1.89.0

# Stage 1: Build gw-listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

# The profile used to run `cargo build`
ARG LTO_RELEASE=release

# Use root user for build stage
USER root

WORKDIR /app

# Copy sources
COPY test-suite/benchmarks/gateway-stress test-suite/benchmarks/gateway-stress

# Build with improved caching
WORKDIR /app/test-suite/benchmarks/gateway-stress
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --profile=${LTO_RELEASE}

# Stage 2: Runtime image
FROM cgr.dev/chainguard/busybox:latest-glibc AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/test-suite/benchmarks/gateway-stress/target/release/gateway-stress /bin/gateway-stress

USER fhevm:fhevm

ENTRYPOINT ["/bin/gateway-stress"]

HEALTHCHECK --start-period=5s --interval=1m --timeout=3s --retries=3 \
    CMD ["ls", "/bin/gateway-stress"]

FROM prod AS dev
