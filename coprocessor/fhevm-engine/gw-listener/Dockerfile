# Stage 0: Build contracts
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS contract_builder

USER root

WORKDIR /app

COPY gateway-contracts ./gateway-contracts

# Compiled gateway-contracts for gw-listener
WORKDIR /app/gateway-contracts
RUN npm install && \
    DOTENV_CONFIG_PATH=.env.example npx hardhat task:deployAllGatewayContracts

# Stage 1: Build GW Listener
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.91.0 AS builder

ARG CARGO_PROFILE=release

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY gateway-contracts/contracts/ ./gateway-contracts/contracts/
COPY gateway-contracts/rust_bindings/ ./gateway-contracts/rust_bindings
COPY --from=contract_builder /app/gateway-contracts/artifacts/contracts /app/gateway-contracts/artifacts/contracts
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID


WORKDIR /app/coprocessor/fhevm-engine

# Build gw_listener binary
# NOTE: We use a cache mount for the target directory to enable incremental compilation.
# Because cache mounts are NOT committed to the image layer, we must copy the binary
# to a non-mounted path (/tmp) during the same RUN instruction for COPY --from to work.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/coprocessor/fhevm-engine/target,sharing=locked \
    cargo fetch && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --profile=${CARGO_PROFILE} -p gw-listener && \
    cp target/${CARGO_PROFILE}/gw_listener /tmp/gw_listener

# Stage 2: Runtime image
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0 AS prod

COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /tmp/gw_listener /usr/local/bin/gw_listener

USER fhevm:fhevm

CMD ["/usr/local/bin/gw_listener"]

FROM prod AS dev
