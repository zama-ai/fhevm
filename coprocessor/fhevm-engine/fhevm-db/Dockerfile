# Use the Rust image as the base
FROM rust:1.85.0-bookworm AS build

# Install dependencies and tools


# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# Copy migrations and initialization script
COPY fhevm-engine/ /fhevm-engine
COPY proto/ /proto/

WORKDIR /fhevm-engine
# Build utils binary
# Currently, this utils is used only for keys converting. 
# Later on, it will be extended to replace initialize_db.sh completely
RUN cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p coprocessor --bin utils

FROM rust:1.85.0-bookworm AS final

# Install required runtime dependencies only
RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev postgresql-client xxd && \
    cargo install sqlx-cli --version 0.7.2 --no-default-features --features postgres --locked && \
    groupadd -r zama && useradd -r -g zama zama && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

COPY fhevm-engine/fhevm-db/initialize_db.sh /initialize_db.sh
COPY fhevm-engine/fhevm-db/migrations /migrations
COPY  --from=build /fhevm-engine/target/release/utils /usr/local/bin/utils

# Change ownership of the copied files to the non-root user
RUN mkdir /fhevm-keys && \
    chown -R zama:zama /initialize_db.sh /migrations /fhevm-keys && \
    chmod +x /initialize_db.sh


# Switch to the non-root user
USER zama:zama

# Run the initialization script as the entrypoint
ENTRYPOINT ["/bin/bash", "/initialize_db.sh"]
