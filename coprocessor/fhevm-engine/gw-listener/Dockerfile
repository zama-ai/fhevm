# Stage 1: Build GW Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:latest AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine

WORKDIR /app/coprocessor/fhevm-engine

# Build gw_listener binary
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p gw-listener

# Stage 2: Runtime image
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:latest

COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/gw_listener /usr/local/bin/gw_listener

USER fhevm:fhevm

CMD ["/usr/local/bin/gw_listener"]