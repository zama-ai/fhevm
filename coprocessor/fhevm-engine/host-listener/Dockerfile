# Stage 0: Build contracts
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS contract_builder

USER root

WORKDIR /app

# Copy root lockfile for workspace resolution
COPY package.json package-lock.json ./

COPY host-contracts ./host-contracts

# Compiled host-contracts for listeners
RUN cp host-contracts/.env.example host-contracts/.env && \
    npm ci --workspace=host-contracts --include-workspace-root=false && \
    cd host-contracts && \
    HARDHAT_NETWORK=hardhat npm run deploy:emptyProxies && \
    npx hardhat compile

# Stage 1: Build Host Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

ARG CARGO_PROFILE=release
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

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY host-contracts/contracts/ ./host-contracts/contracts/
COPY --from=contract_builder /app/host-contracts/artifacts/contracts /app/host-contracts/artifacts/contracts
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID

WORKDIR /app/coprocessor/fhevm-engine

# Build host_listener binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    --mount=type=secret,id=sccache_aws_access_key_id \
    --mount=type=secret,id=sccache_aws_secret_access_key \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then \
      export RUSTC_WRAPPER=sccache; \
      export AWS_ACCESS_KEY_ID="$(cat /run/secrets/sccache_aws_access_key_id)"; \
      export AWS_SECRET_ACCESS_KEY="$(cat /run/secrets/sccache_aws_secret_access_key)"; \
    fi && \
    cargo fetch && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --profile=${CARGO_PROFILE} -p host-listener && \
    cp target/${CARGO_PROFILE}/host_listener /tmp/host_listener && \
    cp target/${CARGO_PROFILE}/host_listener_poller /tmp/host_listener_poller && \
    cp target/${CARGO_PROFILE}/host_listener_consumer /tmp/host_listener_consumer

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener /usr/local/bin/host_listener
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener_poller /usr/local/bin/host_listener_poller
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener_consumer /usr/local/bin/host_listener_consumer

USER fhevm:fhevm

# No ENTRYPOINT - consumers specify the full command (binary + args) in docker-compose

FROM prod AS dev
