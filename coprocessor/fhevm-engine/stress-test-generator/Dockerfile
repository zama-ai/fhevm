# Stage 0: Build contracts
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS contract_builder

USER root

WORKDIR /app

COPY package.json package-lock.json ./
COPY host-contracts ./host-contracts

# Install dependencies
RUN npm ci --include=dev

# Compiled host-contracts for listeners
WORKDIR /app/host-contracts
RUN cp .env.example .env \
    && npm rebuild @nomicfoundation/slang --unsafe-perm \
    && HARDHAT_NETWORK=hardhat npm run deploy:emptyProxies \
    && npx hardhat compile

# Stage 1: Build Stress-Tool
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.90.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY host-contracts/contracts/ ./host-contracts/contracts/
COPY --from=contract_builder /app/host-contracts/artifacts/contracts /app/host-contracts/artifacts/contracts

WORKDIR /app/coprocessor/fhevm-engine

# Build stress_generator binary
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p stress-test-generator

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/stress_generator /usr/local/bin/stress_generator
USER fhevm:fhevm

CMD ["/usr/local/bin/stress_generator"]

FROM prod AS dev
