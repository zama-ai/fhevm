# syntax=docker/dockerfile:1
# Mirrors relayer/docker/relayer/Dockerfile. Builds the indexer + the migrate
# sidecar from the standalone solana-indexer workspace (its own Cargo.lock), so
# Carbon's Solana 3.x graph stays isolated. Build context = the monorepo root (so
# both solana-indexer/ and the sibling solana/crates/zama-solana-acl/ path dep are
# reachable); this matches the `context: ../..` in docker/docker-compose.yaml.
ARG RUST_VERSION=1.91

FROM cgr.dev/zama.ai/rust:${RUST_VERSION}-dev AS build
WORKDIR /app
USER root

# The crate depends on ../solana/crates/zama-solana-acl by path. Copy both the
# workspace and that sibling crate so the relative path resolves inside the image.
COPY solana-indexer/ /app/
COPY solana/crates/zama-solana-acl/ /solana/crates/zama-solana-acl/

# SQLX_OFFLINE so the build needs no live DB (uses the committed .sqlx cache).
ENV SQLX_OFFLINE=true
RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/usr/local/cargo/git/ \
    cargo build --locked --release -p indexer -p indexer-migrate && \
    cp ./target/release/indexer /bin/indexer && \
    cp ./target/release/indexer-migrate /bin/indexer-migrate

FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0-dev AS prod
USER root
# wget backs the compose healthcheck probe against /liveness.
RUN apk add --no-cache ca-certificates=20241010-r0 wget
ARG UID=10001
RUN adduser --disabled-password --no-create-home --uid "${UID}" appuser
USER appuser

WORKDIR /app
COPY --from=build /bin/indexer /bin/
COPY --from=build /bin/indexer-migrate /bin/
COPY solana-indexer/config/ /app/config/

# The indexer reads ./config/default.toml; APP_… env vars override every field.
CMD ["/bin/indexer"]
