# Stage 1: Build DB Migration
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

USER root

WORKDIR /app

# Install sqlx-cli
# NOTE: the `rustls` feature is mandatory. Without a TLS backend compiled in,
# `sqlx database create` / `sqlx migrate run` fail against any RDS that requires
# TLS ("TLS upgrade required by connect options but SQLx was built without TLS
# support enabled"), e.g. when sslmode=require or rds.force_ssl=1.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo install sqlx-cli --version 0.7.2 \
    --no-default-features --features postgres,rustls --locked

# Guard: fail the build early if the sqlx CLI was compiled without a TLS
# backend. rustls (and its webpki dependency) leave their crate names in the
# binary; their absence means TLS support was dropped and migrations would
# break against TLS-requiring databases.
RUN if grep -aqE 'rustls|webpki' /usr/local/cargo/bin/sqlx; then \
        echo "sqlx TLS (rustls) support: OK"; \
    else \
        echo "ERROR: sqlx built without rustls TLS support" >&2; \
        exit 1; \
    fi

# Copy migrations and initialization script
COPY listener ./listener
COPY coprocessor/fhevm-engine/ ./coprocessor/fhevm-engine
COPY coprocessor/proto/ ./coprocessor/proto/
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation
COPY coprocessor/fhevm-engine/db-migration/initialize_db.sh ./initialize_db.sh
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
