ARG RUST_IMAGE_VERSION

# Stage 1: install DB Migration tooling
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

ARG SCCACHE_BUCKET
ARG SCCACHE_REGION=eu-west-3
ARG SCCACHE_S3_PREFIX
ENV RUSTC_WRAPPER=${SCCACHE_BUCKET:+sccache}
ENV SCCACHE_IDLE_TIMEOUT=0
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET}
ENV SCCACHE_REGION=${SCCACHE_REGION}
ENV SCCACHE_S3_KEY_PREFIX=${SCCACHE_S3_PREFIX}
ENV AWS_REGION=${SCCACHE_REGION}

WORKDIR /app

USER root

# Install required runtime dependencies only
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ]; then \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo install sqlx-cli --version 0.8.6 --no-default-features --features postgres --locked

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
