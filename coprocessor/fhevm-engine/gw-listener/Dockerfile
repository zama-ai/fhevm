# Stage 1: Build GW Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.85.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine

WORKDIR /app/coprocessor/fhevm-engine

# Build gw_listener binary
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p gw-listener

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS runtime

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /usr/bin/ /usr/bin/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/ssl/certs/ /etc/ssl/certs/
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/gw_listener /usr/local/bin/gw_listener

USER fhevm:fhevm

CMD ["/usr/local/bin/gw_listener"]