# syntax=docker/dockerfile:1

ARG RUST_IMAGE_VERSION

# Stage 1: Build gateway-stress tool
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

# The profile used to run `cargo build`
ARG CARGO_PROFILE=release

USER root

WORKDIR /app

COPY test-suite/gateway-stress test-suite/gateway-stress

# Build with improved caching
WORKDIR /app/test-suite/gateway-stress
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo build --profile=${CARGO_PROFILE}

# Stage 2: Runtime image
FROM cgr.dev/chainguard/busybox:latest-glibc AS prod

ARG CARGO_PROFILE=release

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/test-suite/gateway-stress/target/${CARGO_PROFILE}/gateway-stress /bin/gateway-stress

USER fhevm:fhevm

ENTRYPOINT ["/bin/gateway-stress"]

HEALTHCHECK --start-period=5s --interval=1m --timeout=3s --retries=3 \
    CMD ["ls", "/bin/gateway-stress"]

FROM prod AS dev
