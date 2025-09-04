# Stage 1: Build SNS Worker
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.89.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto

WORKDIR /app/coprocessor/fhevm-engine

# Build sns_executor binary
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p sns-worker

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/sns_worker /usr/local/bin/sns_worker

USER fhevm:fhevm

CMD ["/usr/local/bin/sns_worker"]

FROM prod AS dev
