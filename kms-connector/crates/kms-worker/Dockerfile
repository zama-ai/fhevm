# syntax=docker/dockerfile:1

ARG RUST_IMAGE_VERSION

# Stage 1: Build kms-worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

# The profile used to run `cargo build`
ARG CARGO_PROFILE=release
ARG SCCACHE_BUCKET
ARG SCCACHE_REGION=eu-west-3
ARG SCCACHE_S3_PREFIX
ARG SCCACHE_VERSION=v0.15.0
ENV SCCACHE_IDLE_TIMEOUT=0
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET}
ENV SCCACHE_REGION=${SCCACHE_REGION}
ENV SCCACHE_S3_KEY_PREFIX=${SCCACHE_S3_PREFIX}
ENV AWS_REGION=${SCCACHE_REGION}

# Use root user for build stage
USER root

# TODO(sccache): temporary inline install for testing; remove once sccache
# ships in the rust-glibc golden image and rely on that instead.
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

WORKDIR /app

# Copy git directory to include commit hash in build info
COPY .git ./.git

# Copy sources
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY host-contracts/rust_bindings ./host-contracts/rust_bindings
COPY shared ./shared
COPY kms-connector ./kms-connector

# Build with improved caching
WORKDIR /app/kms-connector
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/kms-connector/target,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then \
      export RUSTC_WRAPPER=sccache; \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    git config --global --add safe.directory /app && \
    cargo fetch && \
    cargo build --profile=${CARGO_PROFILE} -p kms-worker && \
    cp target/${CARGO_PROFILE}/kms-worker /tmp/kms-worker

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/kms-worker /app/kms-connector/bin/kms-worker

USER fhevm:fhevm

ENTRYPOINT ["/app/kms-connector/bin/kms-worker", "start"]

HEALTHCHECK --start-period=5s --interval=1m --timeout=3s --retries=3 \
    CMD ["/app/kms-connector/bin/kms-worker", "health", "--endpoint", "http://127.0.0.1:9100/healthz"]

FROM prod AS dev
