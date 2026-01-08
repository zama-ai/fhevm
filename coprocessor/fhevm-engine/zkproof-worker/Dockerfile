# Stage 1: Build ZK Proof Worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

ARG CARGO_PROFILE=release

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings

WORKDIR /app/coprocessor/fhevm-engine

# Build zkproof_worker binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
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
