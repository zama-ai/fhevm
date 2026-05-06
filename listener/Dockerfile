# syntax=docker/dockerfile:1

# ── Planner (generate dependency recipe) ─────────────────────────────
FROM rust:1.92-slim-bookworm AS planner
RUN cargo install cargo-chef
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── Builder (cook deps, then build app) ──────────────────────────────
FROM rust:1.92-slim-bookworm AS builder
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-chef
ENV SQLX_OFFLINE=true
WORKDIR /app

# Cook dependencies (cached unless Cargo.toml/Cargo.lock change)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release -p listener_core

# ── Runtime ──────────────────────────────────────────────────────────
FROM gcr.io/distroless/cc-debian12:nonroot AS prod
COPY --from=builder /app/target/release/listener_core /app/listener_core
COPY --from=builder /app/crates/listener_core/migrations/ /app/migrations/
WORKDIR /app
ENTRYPOINT ["/app/listener_core"]
CMD ["--config", "/config.yaml"]

FROM prod AS dev
