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
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

ARG CARGO_PROFILE=release

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/rust_bindings ./gateway-contracts/rust_bindings
COPY host-contracts/contracts/ ./host-contracts/contracts/
COPY --from=contract_builder /app/host-contracts/artifacts/contracts /app/host-contracts/artifacts/contracts

WORKDIR /app/coprocessor/fhevm-engine

# Build stress_generator binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --profile=${CARGO_PROFILE} -p stress-test-generator && \
    cp target/${CARGO_PROFILE}/stress_generator /tmp/stress_generator

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/stress_generator /usr/local/bin/stress_generator
USER fhevm:fhevm

CMD ["/usr/local/bin/stress_generator"]

FROM prod AS dev
