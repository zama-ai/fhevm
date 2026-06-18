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
# Extra cargo features for `-p host-listener` (empty for the mainline EVM build;
# the Solana stack passes `solana-grpc` to compile the Yellowstone transport).
ARG CARGO_FEATURES=""

USER root

WORKDIR /app

COPY listener ./listener
COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY host-contracts/contracts/ ./host-contracts/contracts/
COPY --from=contract_builder /app/host-contracts/artifacts/contracts /app/host-contracts/artifacts/contracts
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID

WORKDIR /app/coprocessor/fhevm-engine

# Build host_listener binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    if echo "${CARGO_FEATURES}" | grep -q solana-grpc; then \
        apt-get update && apt-get install -y --no-install-recommends cmake protobuf-compiler && rm -rf /var/lib/apt/lists/*; \
    fi && \
    cargo fetch && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --profile=${CARGO_PROFILE} -p host-listener ${CARGO_FEATURES:+--features "${CARGO_FEATURES}"} && \
    cp target/${CARGO_PROFILE}/host_listener /tmp/host_listener && \
    cp target/${CARGO_PROFILE}/host_listener_poller /tmp/host_listener_poller && \
    cp target/${CARGO_PROFILE}/host_listener_consumer /tmp/host_listener_consumer && \
    cp target/${CARGO_PROFILE}/solana_host_listener /tmp/solana_host_listener && \
    cp target/${CARGO_PROFILE}/solana_finalized_account_fetcher /tmp/solana_finalized_account_fetcher

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener /usr/local/bin/host_listener
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener_poller /usr/local/bin/host_listener_poller
COPY --from=builder --chown=fhevm:fhevm /tmp/host_listener_consumer /usr/local/bin/host_listener_consumer
COPY --from=builder --chown=fhevm:fhevm /tmp/solana_host_listener /usr/local/bin/solana_host_listener
COPY --from=builder --chown=fhevm:fhevm /tmp/solana_finalized_account_fetcher /usr/local/bin/solana_finalized_account_fetcher

USER fhevm:fhevm

# No ENTRYPOINT - consumers specify the full command (binary + args) in docker-compose

FROM prod AS dev
