# Stage 1: Build GW Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.89.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine

WORKDIR /app/coprocessor/fhevm-engine

# Build gw_listener binary
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p gw-listener

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/gw_listener /usr/local/bin/gw_listener

USER fhevm:fhevm

CMD ["/usr/local/bin/gw_listener"]

FROM prod AS dev
