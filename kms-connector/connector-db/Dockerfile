# RUST_IMAGE_VERSION arg can be used to override the default version
ARG RUST_IMAGE_VERSION=1.87.0

# Stage 1: install DB Migration tooling
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

WORKDIR /app

USER root

# Install required runtime dependencies only
RUN cargo install sqlx-cli --version 0.8.6 --no-default-features --features postgres --locked

COPY kms-connector/connector-db/init_db.sh ./init_db.sh
COPY kms-connector/connector-db/migrations ./migrations


# Stage 2: Runtime image
FROM cgr.dev/zama.ai/postgres:17 AS prod

COPY  --from=builder --chown=fhevm:fhevm /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY  --from=builder --chown=fhevm:fhevm /app/init_db.sh /init_db.sh
COPY  --from=builder --chown=fhevm:fhevm /app/migrations /migrations
COPY  --from=builder /etc/group /etc/group
COPY  --from=builder /etc/passwd /etc/passwd

USER fhevm:fhevm

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD psql --version || exit 1

# Run the initialization script as the entrypoint
ENTRYPOINT ["/bin/bash", "/init_db.sh"]


FROM prod AS dev
