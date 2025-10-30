# Stage 0: Build contracts
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS contract_builder

USER root

WORKDIR /app

COPY host-contracts ./host-contracts

# Compiled host-contracts for listeners
WORKDIR /app/host-contracts
RUN cp .env.example .env
RUN npm install && HARDHAT_NETWORK=hardhat npm run deploy:emptyProxies && npx hardhat compile

# Stage 1: Build Host Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.90.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY host-contracts/contracts/ ./host-contracts/contracts/
COPY --from=contract_builder /app/host-contracts/artifacts/contracts /app/host-contracts/artifacts/contracts
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID

WORKDIR /app/coprocessor/fhevm-engine

# Build host_listener binary
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --release -p host-listener

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/host_listener /usr/local/bin/host_listener

USER fhevm:fhevm

CMD ["/usr/local/bin/host_listener"]

FROM prod AS dev
