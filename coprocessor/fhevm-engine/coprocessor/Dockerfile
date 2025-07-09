# Stage 1: Build TFHE Worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.85.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto

WORKDIR /app/coprocessor/fhevm-engine

# Build coprocessor binary
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p coprocessor

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/coprocessor /usr/local/bin/coprocessor

USER fhevm:fhevm

CMD ["/usr/local/bin/coprocessor"]

FROM prod AS dev
