# Stage 1: Build DB Migration
FROM ghcr.io/zama-ai/fhevm/gci/rust-glibc:1.85.0 AS builder

USER root

WORKDIR /app

# Install sqlx-cli
RUN cargo install sqlx-cli --version 0.7.2 \
    --no-default-features --features postgres --locked

# Copy migrations and initialization script
COPY coprocessor/fhevm-engine/ ./coprocessor/fhevm-engine
COPY coprocessor/proto/ ./coprocessor/proto/
COPY coprocessor/tfhe-rs/ ./coprocessor/tfhe-rs/
COPY coprocessor/fhevm-engine/db-migration/initialize_db.sh ./initialize_db.sh
COPY coprocessor/fhevm-engine/db-migration/migrations ./migrations

WORKDIR /app/coprocessor/fhevm-engine
# Build utils binary
# Currently, this utils is used only for keys converting. 
# Later on, it will be extended to replace initialize_db.sh completely
RUN mkdir /fhevm-keys && \
    cargo fetch && \
    SQLX_OFFLINE=true cargo build --release -p coprocessor --bin utils

# Avoid a conflict with prod stage
RUN rm -rf /lib/apk

# Stage 2: Runtime image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS prod

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /usr/bin/ /usr/bin/
COPY --from=builder /usr/libexec/ /usr/libexec/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

COPY  --from=builder --chown=fhevm:fhevm /app/coprocessor/fhevm-engine/target/release/utils /usr/local/bin/utils
COPY  --from=builder --chown=fhevm:fhevm /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY  --from=builder --chown=fhevm:fhevm /app/initialize_db.sh /initialize_db.sh
COPY  --from=builder --chown=fhevm:fhevm /app/migrations /migrations
COPY  --from=builder --chown=fhevm:fhevm /fhevm-keys /fhevm-keys

USER fhevm:fhevm

ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:${PATH}"

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD psql --version || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev