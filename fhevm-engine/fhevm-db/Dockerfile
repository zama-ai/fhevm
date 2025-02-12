# Use the Rust image as the base
FROM rust:1.84.1-slim

# Install dependencies and tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev postgresql-client xxd && \
    cargo install sqlx-cli --version 0.7.2 --no-default-features --features postgres --locked && \
    groupadd -r zama && useradd -r -g zama zama && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy migrations and initialization script
COPY fhevm-engine/fhevm-db/initialize_db.sh /initialize_db.sh
COPY fhevm-engine/fhevm-db/migrations /migrations

# Change ownership of the copied files to the non-root user
RUN mkdir /fhevm-keys && \
    chown -R zama:zama /initialize_db.sh /migrations /fhevm-keys && \
    chmod +x /initialize_db.sh

# Switch to the non-root user
USER zama

# Run the initialization script as the entrypoint
ENTRYPOINT ["/bin/bash", "/initialize_db.sh"]
