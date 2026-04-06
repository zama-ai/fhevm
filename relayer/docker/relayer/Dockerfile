# syntax=docker/dockerfile:1
# Base from https://docs.docker.com/guides/rust/develop/
ARG RUST_VERSION=1.93
ARG APP_NAME=fhevm-relayer

FROM cgr.dev/zama.ai/rust:${RUST_VERSION}-dev AS build
ARG APP_NAME
WORKDIR /app/relayer

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
USER root
RUN apk add --no-cache openssl-dev=3.6.1-r4

RUN --mount=type=bind,source=relayer/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=relayer/Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=relayer/build.rs,target=build.rs \
    --mount=type=bind,source=relayer/src,target=src \
    --mount=type=bind,source=relayer/test-support,target=test-support \
#    --mount=type=bind,source=.git/HEAD,target=.git/HEAD \
#    --mount=type=bind,source=.git/index,target=.git/index \
#    --mount=type=bind,source=.git/objects,target=.git/objects \
#    --mount=type=bind,source=.git/refs,target=.git/refs \
    --mount=type=bind,source=relayer/.sqlx,target=.sqlx \
    --mount=type=bind,source=solana-host-contracts/crates/core,target=/app/solana-host-contracts/crates/core \
    --mount=type=cache,target=/app/relayer/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/usr/local/cargo/git/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server


################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0-dev AS prod
USER root
RUN apk add --no-cache ca-certificates=20241010-r0

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser

WORKDIR /app/config

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# What the container should run when it is started.
CMD ["/bin/server"]

FROM prod AS dev
