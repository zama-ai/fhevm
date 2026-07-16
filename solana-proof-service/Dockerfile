# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.91.1
ARG APP_NAME=solana-proof-service

FROM cgr.dev/zama.ai/rust:${RUST_VERSION}-dev AS build
ARG APP_NAME
WORKDIR /app

USER root
RUN apk add --no-cache openssl-dev protobuf-dev

# Workspace members + path deps into ../solana (mounted at /solana).
ENV SQLX_OFFLINE=true
ENV NO_DNA=1
RUN --mount=type=bind,source=solana-proof-service/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=solana-proof-service/Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=solana-proof-service/rust-toolchain.toml,target=rust-toolchain.toml \
    --mount=type=bind,source=solana-proof-service/crates,target=crates \
    --mount=type=bind,source=solana,target=/solana \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/usr/local/cargo/git/ \
    cargo build --locked --release -p solana-proof-service && \
    cp ./target/release/$APP_NAME /bin/server

FROM cgr.dev/zama.ai/glibc-dynamic:16-dev AS prod
USER root
RUN apk add --no-cache ca-certificates

ARG UID=10001
RUN adduser \
    --disabled-password \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser
WORKDIR /app

COPY --from=build /bin/server /bin/
COPY solana-proof-service/config/app.yaml /app/config/app.yaml

ENV SOLANA_PROOF_CONFIG_PATH=/app/config/app.yaml
CMD ["/bin/server"]

FROM prod AS dev
