# Stage 1: Build Transaction Sender
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.85.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto

WORKDIR /app/coprocessor/fhevm-engine

# Build transaction_sender binary
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p transaction-sender

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/transaction_sender /usr/local/bin/transaction_sender

USER fhevm:fhevm

CMD ["/usr/local/bin/transaction_sender"]

FROM prod AS dev
