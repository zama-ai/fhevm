# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.90.0
ARG APP_NAME=fhevm-relayer
ARG DATABASE_URL

FROM cgr.dev/zama.ai/rust:${RUST_VERSION}-dev AS build
ARG APP_NAME
ARG DATABASE_URL
WORKDIR /app

USER root
RUN apk add --no-cache openssl-dev

ENV DATABASE_URL=${DATABASE_URL}
ENV MAX_ATTEMPTS=1
# Online mode: verify queries against live database during build
# Requires relayer_db with schema to exist before build (see deploy script)
ENV SQLX_OFFLINE=false

RUN --mount=type=bind,source=console/apps/relayer/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=console/apps/relayer/Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=console/apps/relayer/build.rs,target=build.rs \
    --mount=type=bind,source=console/apps/relayer/src,target=src \
    --mount=type=bind,source=console/apps/relayer/test-support,target=test-support \
    --mount=type=bind,source=console/fhevm/gateway-contracts/rust_bindings,target=/fhevm/gateway-contracts/rust_bindings \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/usr/local/cargo/git/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0-dev AS runtime
USER root
RUN apk add --no-cache ca-certificates

ARG UID=10001
RUN adduser \
    --disabled-password \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser
WORKDIR /app/config

COPY --from=build /bin/server /bin/

CMD ["/bin/server"]
