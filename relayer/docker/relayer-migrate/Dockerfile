# syntax=docker/dockerfile:1
# Base from https://docs.docker.com/guides/rust/develop/
ARG RUST_VERSION=1.93
ARG APP_NAME=relayer-migrate

FROM cgr.dev/zama.ai/rust:${RUST_VERSION}-dev AS build
ARG APP_NAME
WORKDIR /app

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
USER root
RUN apk add --no-cache openssl-dev=3.6.1-r4 clang=22.1.1-r1

RUN --mount=type=bind,source=relayer-migrate/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=relayer-migrate/Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=relayer-migrate/src,target=src \
    --mount=type=bind,source=relayer-migrate/migrations,target=migrations \
    --mount=type=bind,source=relayer-migrate/down,target=down \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/usr/local/cargo/git/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server && \
    cp ./target/release/down-migrate-v0_8_8-to-v0_8_7 /bin/


################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
FROM cgr.dev/zama.ai/glibc-dynamic:15.2.0-dev AS runtime
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
# Down migration binary for v0.8.8 → v0.8.7 rollback
COPY --from=build /bin/down-migrate-v0_8_8-to-v0_8_7 /bin/

# What the container should run when it is started.
CMD ["/bin/server"]
