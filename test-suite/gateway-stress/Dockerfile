# syntax=docker/dockerfile:1

ARG RUST_IMAGE_VERSION

# Stage 1: Build gateway-stress tool
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

ARG LTO_RELEASE=release
ARG SCCACHE_BUCKET
ARG SCCACHE_REGION=eu-west-3
ARG SCCACHE_S3_PREFIX
ENV RUSTC_WRAPPER=${SCCACHE_BUCKET:+sccache}
ENV SCCACHE_IDLE_TIMEOUT=0
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET}
ENV SCCACHE_REGION=${SCCACHE_REGION}
ENV SCCACHE_S3_KEY_PREFIX=${SCCACHE_S3_PREFIX}
ENV AWS_REGION=${SCCACHE_REGION}

USER root

WORKDIR /app

COPY test-suite/gateway-stress test-suite/gateway-stress

# Build with improved caching
WORKDIR /app/test-suite/gateway-stress
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ]; then \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo build --profile=${LTO_RELEASE}

# Stage 2: Runtime image
FROM cgr.dev/chainguard/busybox:latest-glibc AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/test-suite/gateway-stress/target/release/gateway-stress /bin/gateway-stress

USER fhevm:fhevm

ENTRYPOINT ["/bin/gateway-stress"]

HEALTHCHECK --start-period=5s --interval=1m --timeout=3s --retries=3 \
    CMD ["ls", "/bin/gateway-stress"]

FROM prod AS dev
