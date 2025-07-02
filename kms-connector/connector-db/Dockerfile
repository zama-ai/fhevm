# Use the Rust image as the base
FROM rust:1.87.0-bookworm

# Install required runtime dependencies only
RUN cargo install sqlx-cli --version 0.8.6 --no-default-features --features postgres --locked && \
    groupadd -r zama && useradd -r -g zama zama

COPY connector-db/init_db.sh /init_db.sh
COPY connector-db/migrations /migrations

# Change ownership of the copied files to the non-root user
RUN chown -R zama:zama /init_db.sh /migrations && \
    chmod +x /init_db.sh

# Switch to the non-root user
USER zama:zama

# Run the initialization script as the entrypoint
ENTRYPOINT ["/bin/bash", "/init_db.sh"]
