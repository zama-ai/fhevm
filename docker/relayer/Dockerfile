# syntax=docker/dockerfile:1
# Base from https://docs.docker.com/guides/rust/develop/
ARG RUST_VERSION=1.83.0
ARG CARGO_NET_GIT_FETCH_WITH_CLI=false
ARG APP_NAME=zws-relayer  # binary name specified in apps/relayer/relayer/Cargo.toml

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
ARG CARGO_NET_GIT_FETCH_WITH_CLI
WORKDIR /app

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
# NOTE: We need to mount ssh to be able to fetch the `fhevm-relayer` git repo.
RUN --mount=type=ssh \
    --mount=type=bind,source=lib,target=lib \
    --mount=type=bind,source=relayer,target=relayer \
    --mount=type=bind,source=fhevm-relayer,target=fhevm-relayer \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
apt update
# TODO: investigate why sqlite3 is required to build diesel if only postgres is specified
apt install libssl-dev git musl-dev pkg-config libpq-dev libsqlite3-dev -y
ls -R && cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the debian bullseye image as the foundation for    running the app.
# By specifying the "bullseye-slim" tag, it will also use whatever happens to    be the
# most recent version of that tag when you build your Dockerfile. If
# reproducibility is important, consider using a digest
# (e.g.,    debian@sha256:ac707220fbd7b67fc19b112cee8170b41a9e97f703f588b2cdbbcdcecdd8af57).
FROM debian:bullseye-slim AS final
RUN apt update && apt install libssl-dev libpq-dev libsqlite3-dev -y

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
CMD ["/bin/server"]
