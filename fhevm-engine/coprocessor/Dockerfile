# Stage 1: Build
FROM node:20-slim AS build_contract

# Set the working directory
WORKDIR /app

COPY ./contracts ./contracts

# Compiled contracts for listeners
WORKDIR /app/contracts
RUN npm install && npx hardhat compile

FROM rust:1.85.0-bookworm AS build

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

COPY ./contracts/contracts/ ./contracts/contracts/
COPY --from=build_contract /app/contracts/artifacts/contracts /app/contracts/artifacts/contracts
COPY ./fhevm-engine/ ./fhevm-engine/
COPY ./proto/ ./proto/

WORKDIR /app/fhevm-engine

# Build all binaries
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release

# Stage 2: Final image
FROM debian:bookworm-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /app/fhevm-engine/target/release/coprocessor /usr/local/bin/coprocessor
COPY --from=build /app/fhevm-engine/target/release/cli /usr/local/bin/cli
COPY --from=build /app/fhevm-engine/target/release/fhevm_listener /usr/local/bin/fhevm_listener
COPY --from=build /app/fhevm-engine/target/release/gw_listener /usr/local/bin/gw_listener
COPY --from=build /app/fhevm-engine/target/release/zkproof_worker /usr/local/bin/zkproof_worker
COPY --from=build /app/fhevm-engine/target/release/sns_worker /usr/local/bin/sns_worker
COPY --from=build /app/fhevm-engine/target/release/transaction_sender /usr/local/bin/transaction_sender

RUN groupadd -g 10001 zama && \
    useradd -u 10000 -g zama zama && \
    chown -R zama:zama /usr/local/bin && \
    chmod 500 /usr/local/bin/coprocessor && \
    chmod 500 /usr/local/bin/cli && \
    chmod 500 /usr/local/bin/fhevm_listener && \
    chmod 500 /usr/local/bin/gw_listener && \
    chmod 500 /usr/local/bin/zkproof_worker && \
    chmod 500 /usr/local/bin/sns_worker && \
    chmod 500 /usr/local/bin/transaction_sender

USER zama:zama

CMD ["/usr/local/bin/coprocessor"]
