# Stage 1: Build SNS Worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

ARG CARGO_PROFILE=release

USER root

# sccache, pinned for reproducibility; this layer only rebuilds when the base
# image changes.
RUN cargo install sccache --locked --version 0.8.2

WORKDIR /app

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation

WORKDIR /app/coprocessor/fhevm-engine

# Build sns_worker binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
#
# The cache mounts only help local builds: CI gets a fresh buildkit instance every
# run, so they start empty there. When CI provides S3 credentials (buildkit secrets
# SCCACHE_AWS_ACCESS_KEY_ID / SCCACHE_AWS_SECRET_ACCESS_KEY), compilation goes
# through sccache backed by the runners' S3 cache bucket instead: compiled
# dependency artifacts (tfhe & friends) are then shared across all coprocessor
# images, platforms, and runs. Without the secrets (local builds, or a CI that
# doesn't pass them) this is a plain cargo build. CARGO_INCREMENTAL=0 because
# sccache cannot cache incremental compilation.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    --mount=type=secret,id=SCCACHE_AWS_ACCESS_KEY_ID \
    --mount=type=secret,id=SCCACHE_AWS_SECRET_ACCESS_KEY \
    if [ -s /run/secrets/SCCACHE_AWS_ACCESS_KEY_ID ]; then \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/SCCACHE_AWS_ACCESS_KEY_ID)" \
             AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/SCCACHE_AWS_SECRET_ACCESS_KEY)" \
             RUSTC_WRAPPER=/usr/local/cargo/bin/sccache \
             SCCACHE_BUCKET=gh-actions-cache-eu-west-3 \
             SCCACHE_REGION=eu-west-3 \
             SCCACHE_S3_KEY_PREFIX=sccache/fhevm-coprocessor \
             CARGO_INCREMENTAL=0; \
    fi && \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --profile=${CARGO_PROFILE} -p sns-worker && \
    cp target/${CARGO_PROFILE}/sns_worker /tmp/sns_worker && \
    if [ -n "${RUSTC_WRAPPER:-}" ]; then sccache --show-stats; fi

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/sns_worker /usr/local/bin/sns_worker

USER fhevm:fhevm

CMD ["/usr/local/bin/sns_worker"]

FROM prod AS dev
