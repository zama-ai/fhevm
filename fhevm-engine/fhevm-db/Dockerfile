# Use the Rust image as the base
FROM rust:1.74

# Install dependencies and tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev postgresql-client xxd && \
    cargo install sqlx-cli --no-default-features --features postgres --locked && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy migrations and initialization script
COPY fhevm-engine/fhevm-db/initialize_db.sh /initialize_db.sh
COPY fhevm-engine/fhevm-db/migrations /migrations
COPY fhevm-engine/fhevm-keys /fhevm-keys

# Make the script executable
RUN chmod +x /initialize_db.sh

# Run the initialization script as the entrypoint
ENTRYPOINT ["/bin/bash", "/initialize_db.sh"]
