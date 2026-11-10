# syntax=docker/dockerfile:1
#
# Image of the relayer database rollback tool: sqlx-cli, the migration files
# and scripts/migrate.sh. It carries no relayer code.
#
# Build context: this directory (relayer-db-rollback/).
#
# RUST_IMAGE_VERSION is intentionally left without a default so the Rust
# toolchain version has a single source of truth: relayer/rust-toolchain.toml.
ARG RUST_IMAGE_VERSION

################################################################################
# Stage 1: build sqlx-cli.
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS sqlx-cli-build

# Must match the sqlx version relayer-migrate applies the migrations with
# (relayer/relayer-migrate/Cargo.toml), so both read `_sqlx_migrations` the
# same way.
ARG SQLX_CLI_VERSION=0.8.6

ARG SCCACHE_BUCKET
ARG SCCACHE_REGION=eu-west-3
ARG SCCACHE_S3_PREFIX
ENV SCCACHE_IDLE_TIMEOUT=0
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET}
ENV SCCACHE_REGION=${SCCACHE_REGION}
ENV SCCACHE_S3_KEY_PREFIX=${SCCACHE_S3_PREFIX}
ENV AWS_REGION=${SCCACHE_REGION}

USER root

# The `rustls` feature is mandatory: without a TLS backend sqlx cannot connect
# to a database that requires TLS (sslmode=require, rds.force_ssl=1).
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then \
      export RUSTC_WRAPPER=sccache; \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo install sqlx-cli \
        --version "${SQLX_CLI_VERSION}" \
        --locked \
        --root /opt/sqlx \
        --no-default-features \
        --features rustls,postgres && \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then sccache --show-stats; fi

################################################################################
# Stage 2: runtime image.
FROM cgr.dev/zama.ai/glibc-dynamic:16-dev AS prod
USER root
RUN apk add --no-cache ca-certificates=20241010-r0

# Create a non-privileged user that the tool will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --no-create-home \
    --uid "${UID}" \
    appuser

COPY --from=sqlx-cli-build /opt/sqlx/bin/sqlx /usr/local/bin/sqlx
COPY --chown=appuser:appuser migrations /app/migrations
COPY --chown=appuser:appuser scripts/migrate.sh /app/scripts/migrate.sh

USER appuser
WORKDIR /app

# Fail the build if any migration lacks a down file: an image that cannot roll
# everything back must not exist.
RUN /app/scripts/migrate.sh check

# Read-only by default: validates the image against the database and prints the
# migration state. A rollback has to be requested explicitly:
#   /app/scripts/migrate.sh rollback <target-version>
ENTRYPOINT ["/app/scripts/migrate.sh"]
CMD ["info"]

FROM prod AS dev
