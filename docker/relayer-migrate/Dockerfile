# syntax=docker/dockerfile:1
# TODO: move this image to use a safer base
# Base from https://docs.docker.com/guides/rust/develop/
ARG RUST_VERSION=1.83.0
FROM rust:${RUST_VERSION}-slim-bullseye AS runtime
WORKDIR /app

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
# NOTE: We need to mount ssh to be able to fetch the `fhevm-relayer` git repo.
COPY ./lib/diesel.toml diesel.toml
RUN mkdir -p src
COPY ./lib/src/schema.rs src/schema.rs
COPY ./lib/migrations/ migrations/
# libsqlite3-dev default-libmysqlclient-dev 
RUN set -e && apt update && apt install libssl-dev git musl-dev pkg-config libpq-dev ca-certificates netcat-traditional -y && cargo install diesel_cli --no-default-features --features postgres
