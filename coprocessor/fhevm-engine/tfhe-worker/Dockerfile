# syntax=docker/dockerfile:1
ARG RUST_IMAGE_VERSION=1.91.0

# Stage 1: Build TFHE Worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

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

USER root

WORKDIR /app

# TODO(sccache): temporary inline install for testing; remove once sccache
# ships in the rust-glibc golden image and rely on that instead.
# Skipped entirely (no network fetch) when SCCACHE_BUCKET is unset, the same
# signal the build hook below uses, so the disabled path stays a plain build.
RUN set -eux; \
    if [ -z "${SCCACHE_BUCKET}" ]; then echo "SCCACHE_BUCKET unset; skipping sccache install"; exit 0; fi; \
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

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation
# Solana path-dependencies. `cargo build -p <crate>` still loads EVERY
# fhevm-engine workspace member manifest, so the solana path-deps of ALL
# members must be present, not just this image's own:
#   host-listener/Cargo.toml -> solana/programs/zama-host,
#     solana/crates/zama-solana-acl, solana/crates/zama-solana-transaction
#   tfhe-worker/Cargo.toml (dev-deps) -> solana/programs/confidential-token,
#     solana/programs/zama-host
# zama-host and confidential-token inherit package fields from solana/Cargo.toml,
# and loading that workspace root requires every member it lists, hence the full
# crates/ + programs/ + runtime-tests/ member set. Non-Rust solana trees (docs,
# scripts, geyser, test-fixtures) are deliberately NOT copied so their churn
# never invalidates these layers.
COPY solana/Cargo.toml ./solana/Cargo.toml
COPY solana/crates ./solana/crates
COPY solana/programs ./solana/programs
COPY solana/runtime-tests ./solana/runtime-tests

WORKDIR /app/coprocessor/fhevm-engine

# Build tfhe_worker binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then \
      export RUSTC_WRAPPER=sccache; \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --profile=${CARGO_PROFILE} -p tfhe-worker && \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then sccache --show-stats; fi && \
    cp target/${CARGO_PROFILE}/tfhe_worker /tmp/tfhe_worker

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/tfhe_worker /usr/local/bin/tfhe_worker

USER fhevm:fhevm

CMD ["/usr/local/bin/tfhe_worker"]

FROM prod AS dev
