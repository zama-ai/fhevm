# Stage 1: Build ZK Proof Worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

ARG CARGO_PROFILE=release
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

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation

WORKDIR /app/coprocessor/fhevm-engine

# Build zkproof_worker binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ]; then \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --profile=${CARGO_PROFILE} -p zkproof-worker && \
    cp target/${CARGO_PROFILE}/zkproof_worker /tmp/zkproof_worker

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/zkproof_worker /usr/local/bin/zkproof_worker

USER fhevm:fhevm

CMD ["/usr/local/bin/zkproof_worker"]

FROM prod AS dev
