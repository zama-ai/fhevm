# Stage 0: Build contracts
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS contract_builder

USER root

WORKDIR /app

COPY host-contracts ./host-contracts

# Compiled host-contracts for listeners
WORKDIR /app/host-contracts
RUN cp .env.example .env
RUN npm install && HARDHAT_NETWORK=hardhat npm run deploy:emptyProxies && npx hardhat compile

# Stage 1: Build Transaction Sender
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.85.0 AS builder

USER root

WORKDIR /app

COPY coprocessor/fhevm-engine ./coprocessor/fhevm-engine
COPY coprocessor/proto ./coprocessor/proto
COPY host-contracts/contracts/ ./host-contracts/contracts/
COPY --from=contract_builder /app/host-contracts/artifacts/contracts /app/host-contracts/artifacts/contracts
COPY .git/HEAD ./coprocessor/fhevm-engine/BUILD_ID

WORKDIR /app/coprocessor/fhevm-engine

# Build fhevm_listener binary
RUN cargo fetch && \
    SQLX_OFFLINE=true BUILD_ID=$(cat BUILD_ID) cargo build --release -p fhevm-listener

# Avoid a conflict with prod stage
RUN rm -rf /lib/apk

# Stage 3: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /usr/bin/ /usr/bin/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/ssl/certs/ /etc/ssl/certs/
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

COPY --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/fhevm_listener /usr/local/bin/fhevm_listener

USER fhevm:fhevm

CMD ["/usr/local/bin/fhevm_listener"]

FROM prod AS dev