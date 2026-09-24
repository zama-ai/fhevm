# RUST_IMAGE_VERSION is injected by CI from rust-toolchain.toml (see coprocessor-docker-build.yml)
ARG RUST_IMAGE_VERSION

# Stage 0: Build contracts
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS contract_builder

USER root

WORKDIR /app

COPY gateway-contracts ./gateway-contracts

# Compiled gateway-contracts for gw-listener
WORKDIR /app/gateway-contracts
RUN npm ci && \
    DOTENV_CONFIG_PATH=.env.example npx hardhat task:deployAllGatewayContracts

# Stage 1: Build GW Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION} AS builder

ARG CARGO_PROFILE=release
ARG SCCACHE_BUCKET
ARG SCCACHE_REGION=eu-west-3
ARG SCCACHE_S3_PREFIX
ENV SCCACHE_IDLE_TIMEOUT=0
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET}
ENV SCCACHE_REGION=${SCCACHE_REGION}
ENV SCCACHE_S3_KEY_PREFIX=${SCCACHE_S3_PREFIX}
ENV AWS_REGION=${SCCACHE_REGION}

USER root

WORKDIR /app

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/contracts/ ./gateway-contracts/contracts/
COPY gateway-contracts/rust_bindings/ ./gateway-contracts/rust_bindings
COPY host-contracts/rust_bindings ./host-contracts/rust_bindings
COPY shared/ciphertext-attestation ./shared/ciphertext-attestation
COPY --from=contract_builder /app/gateway-contracts/artifacts/contracts /app/gateway-contracts/artifacts/contracts
# Solana path-dependencies. `cargo build -p <crate>` still loads EVERY
# fhevm-engine workspace member manifest, so the solana path-deps of ALL
# members must be present, not just this image's own:
#   host-listener/Cargo.toml -> solana/programs/zama-host,
#     solana/crates/zama-solana-acl, solana/crates/zama-solana-transaction
#   tfhe-worker/Cargo.toml (dev-deps) -> solana/programs/confidential-token,
#     solana/programs/zama-host
# zama-host and confidential-token inherit package fields from solana/Cargo.toml,
# and loading that workspace root requires every member it lists, hence the full
# crates/ + programs/ + runtime-tests/ member set. Non-Rust solana trees (docs,
# scripts, geyser, test-fixtures) are deliberately NOT copied so their churn
# never invalidates these layers.
COPY solana/Cargo.toml ./solana/Cargo.toml
COPY solana/crates ./solana/crates
COPY solana/programs ./solana/programs
COPY solana/environments ./solana/environments
COPY solana/runtime-tests ./solana/runtime-tests
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID

WORKDIR /app/coprocessor/fhevm-engine

# Build gw_listener binary
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
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --profile=${CARGO_PROFILE} -p gw-listener && \
    if [ -n "${SCCACHE_BUCKET}" ] && command -v sccache >/dev/null 2>&1; then sccache --show-stats; fi && \
    cp target/${CARGO_PROFILE}/gw_listener /tmp/gw_listener

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/gw_listener /usr/local/bin/gw_listener

USER fhevm:fhevm

CMD ["/usr/local/bin/gw_listener"]

FROM prod AS dev
