# Stage 1: Build DB Migration
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

ARG SCCACHE_BUCKET
ARG SCCACHE_REGION=eu-west-3
ARG SCCACHE_S3_PREFIX
ARG SCCACHE_VERSION=v0.15.0
ENV SCCACHE_IDLE_TIMEOUT=0
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET}
ENV SCCACHE_REGION=${SCCACHE_REGION}
ENV SCCACHE_S3_KEY_PREFIX=${SCCACHE_S3_PREFIX}
ENV AWS_REGION=${SCCACHE_REGION}

USER root

# TODO(sccache): temporary inline install for testing; remove once sccache
# ships in the rust-glibc golden image and rely on that instead.
RUN set -eux; \
    arch="$(uname -m)"; \
    case "$arch" in \
        x86_64)  triple=x86_64-unknown-linux-musl ;; \
        aarch64) triple=aarch64-unknown-linux-musl ;; \
        *) echo "unsupported arch: $arch" >&2; exit 1 ;; \
    esac; \
    curl -fsSL "https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/sccache-${SCCACHE_VERSION}-${triple}.tar.gz" \
        | tar -xz --strip-components=1 -C /usr/local/bin "sccache-${SCCACHE_VERSION}-${triple}/sccache"; \
    chmod +x /usr/local/bin/sccache; \
    sccache --version

WORKDIR /app

# Install sqlx-cli
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then \
      export RUSTC_WRAPPER=sccache; \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo install sqlx-cli --version 0.7.2 \
    --no-default-features --features postgres,rustls --locked

# Install AWS CLI v2 (self-contained bundle) for RDS IAM auth token generation.
# prepare_database_url.sh shells out to `aws rds generate-db-auth-token`. The bundle
# ships its own Python/libs and only depends on glibc, so it runs on the glibc-based
# postgres runtime image below. Pinned to a specific version for reproducible builds.
ARG TARGETARCH
ARG AWS_CLI_VERSION="2.34.52"
RUN set -eux; \
    apk add --no-cache unzip; \
    arch="${TARGETARCH:-}"; \
    if [ -z "$arch" ]; then \
      case "$(uname -m)" in \
        x86_64) arch=amd64 ;; \
        aarch64) arch=arm64 ;; \
      esac; \
    fi; \
    case "$arch" in \
      amd64) aws_arch=x86_64 ;; \
      arm64) aws_arch=aarch64 ;; \
      *) echo "unsupported architecture: ${arch}" >&2; exit 1 ;; \
    esac; \
    curl -fsSL "https://awscli.amazonaws.com/awscli-exe-linux-${aws_arch}-${AWS_CLI_VERSION}.zip" -o /tmp/awscliv2.zip; \
    unzip -q /tmp/awscliv2.zip -d /tmp; \
    /tmp/aws/install --install-dir /usr/local/aws-cli --bin-dir /usr/local/aws-cli-bin; \
    rm -rf /tmp/aws /tmp/awscliv2.zip

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
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then \
      export RUSTC_WRAPPER=sccache; \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
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

# AWS CLI v2, used by prepare_database_url.sh to mint RDS IAM auth tokens.
COPY  --from=builder /usr/local/aws-cli /usr/local/aws-cli
COPY  --from=builder /usr/local/aws-cli-bin /usr/local/aws-cli-bin
ENV PATH="/usr/local/aws-cli-bin:${PATH}"

USER fhevm:fhevm

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD psql --version || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
