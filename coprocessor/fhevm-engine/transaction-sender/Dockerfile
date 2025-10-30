# Stage 1: Build Transaction Sender
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.90.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings

WORKDIR /app/coprocessor/fhevm-engine

# Build transaction_sender binary
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p transaction-sender

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/transaction_sender /usr/local/bin/transaction_sender

USER fhevm:fhevm

CMD ["/usr/local/bin/transaction_sender"]

FROM prod AS dev
