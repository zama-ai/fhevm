# Stage 1: Build
FROM rust:1.84.1-bullseye AS builder

WORKDIR /app

# Copy app source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install CA certificates for HTTPS
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/fhevm-relayer /usr/local/bin/fhevm-relayer
COPY --from=builder /app/config /app/config

RUN groupadd -g 10001 zama && \
    useradd -u 10000 -g zama zama && \
    chown -R zama:zama /usr/local/bin /app

USER zama:zama

# Add healthcheck
# HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
#     CMD curl -f http://localhost:PORT/health || exit 1

CMD ["/usr/local/bin/fhevm-relayer"]