# Stage 1: Build DB Migration
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.89.0 AS builder

USER root

WORKDIR /app

# Install sqlx-cli
RUN cargo install sqlx-cli --version 0.7.2 \
    --no-default-features --features postgres --locked

# Copy migrations and initialization script
COPY coprocessor/fhevm-engine/ ./coprocessor/fhevm-engine
COPY coprocessor/proto/ ./coprocessor/proto/
COPY coprocessor/fhevm-engine/db-migration/initialize_db.sh ./initialize_db.sh
COPY coprocessor/fhevm-engine/db-migration/migrations ./migrations

WORKDIR /app/coprocessor/fhevm-engine
# Build utils binary
# Currently, this utils is used only for keys converting.
# Later on, it will be extended to replace initialize_db.sh completely
RUN mkdir /fhevm-keys && \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p tfhe-worker --bin utils

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/postgres:17 AS prod

COPY  --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/utils /usr/local/bin/utils
COPY  --from=builder --chown=fhevm:fhevm /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY  --from=builder --chown=fhevm:fhevm /app/initialize_db.sh /initialize_db.sh
COPY  --from=builder --chown=fhevm:fhevm /app/migrations /migrations
COPY  --from=builder --chown=fhevm:fhevm /fhevm-keys /fhevm-keys
COPY  --from=builder /etc/group /etc/group
COPY  --from=builder /etc/passwd /etc/passwd

USER fhevm:fhevm

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD psql --version || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
