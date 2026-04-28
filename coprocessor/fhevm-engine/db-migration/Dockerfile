# Stage 1: Build DB Migration
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

USER root

WORKDIR /app

# Install sqlx-cli
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo install sqlx-cli --version 0.7.2 \
    --no-default-features --features postgres,rustls --locked

# Copy migrations and initialization script
COPY coprocessor/fhevm-engine/ ./coprocessor/fhevm-engine
COPY coprocessor/proto/ ./coprocessor/proto/
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY coprocessor/fhevm-engine/db-migration/initialize_db.sh ./initialize_db.sh
COPY coprocessor/fhevm-engine/db-migration/insert_test_host_chain.sh ./insert_test_host_chain.sh
COPY coprocessor/fhevm-engine/db-migration/migrations ./migrations

WORKDIR /app/coprocessor/fhevm-engine

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    cargo build --release -p fhevm-engine-common --bin resolve_database_url && \
    mkdir -p /out && \
    cp target/release/resolve_database_url /out/

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/postgres:17 AS prod

COPY  --from=builder --chown=fhevm:fhevm /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY  --from=builder --chown=fhevm:fhevm /out/resolve_database_url /usr/local/bin/resolve_database_url
COPY  --from=builder --chown=fhevm:fhevm /app/initialize_db.sh /initialize_db.sh
COPY  --from=builder --chown=fhevm:fhevm /app/insert_test_host_chain.sh /insert_test_host_chain.sh
COPY  --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/db-migration/prepare_database_url.sh /prepare_database_url.sh
COPY  --from=builder --chown=fhevm:fhevm /app/migrations /migrations
COPY  --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/db-migration/db-scripts /db-scripts
COPY  --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/db-migration/revert_coprocessor_db_state.sh /revert_coprocessor_db_state.sh
COPY  --from=builder /etc/group /etc/group
COPY  --from=builder /etc/passwd /etc/passwd

USER fhevm:fhevm

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD psql --version || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
