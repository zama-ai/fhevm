# Stage 1: Build
FROM rust:1.84.1-bullseye AS build

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends protobuf-compiler=3.12.4* && \
    rm -rf /var/lib/apt/lists/*

COPY ./fhevm-engine/ ./fhevm-engine/
COPY ./proto/ ./proto/

WORKDIR /app/fhevm-engine

# Start the database service
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release

# Stage 2: Final image
FROM debian:bullseye-slim

COPY --from=build /app/fhevm-engine/target/release/coprocessor /usr/local/bin/coprocessor
COPY --from=build /app/fhevm-engine/target/release/cli /usr/local/bin/cli
# This binary will be renamed to fhevm_listener
COPY --from=build /app/fhevm-engine/target/release/listen /usr/local/bin/listen
COPY --from=build /app/fhevm-engine/target/release/gw_listener /usr/local/bin/gw_listener
COPY --from=build /app/fhevm-engine/target/release/zkproof_worker /usr/local/bin/zkproof_worker
COPY --from=build /app/fhevm-engine/target/release/sns_worker /usr/local/bin/sns_worker
COPY --from=build /app/fhevm-engine/target/release/transaction_sender /usr/local/bin/transaction_sender

RUN groupadd -r zama && useradd -r -g zama zama && \
    chown -R zama:zama /usr/local/bin && \
    chmod 500 /usr/local/bin/coprocessor && \
    chmod 500 /usr/local/bin/cli && \
    chmod 500 /usr/local/bin/listen && \
    chmod 500 /usr/local/bin/gw_listener && \
    chmod 500 /usr/local/bin/zkproof_worker && \
    chmod 500 /usr/local/bin/sns_worker && \
    chmod 500 /usr/local/bin/transaction_sender

USER zama

CMD ["/usr/local/bin/coprocessor --help"]
