# syntax=docker/dockerfile:1

# RUST_IMAGE_VERSION arg can be used to override the default version
ARG RUST_IMAGE_VERSION=latest

### Multistage build to reduce image size
## First stage sets up basic Rust build environment
FROM rust:${RUST_IMAGE_VERSION}-slim-bookworm AS base

ARG LTO_RELEASE=release

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    gcc \
    git \
    libprotobuf-dev \
    libssl-dev \
    pkg-config \
    protobuf-compiler \
    ssh \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sccache --version ^0.7
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

WORKDIR /app
COPY . .

# Setup SSH and git
RUN mkdir -p -m 0600 /root/.ssh && \
    ssh-keyscan -H github.com >> ~/.ssh/known_hosts && \
    mkdir -p /app/kms-connector/bin

# Configure git with secure token handling
RUN --mount=type=secret,id=BLOCKCHAIN_ACTIONS_TOKEN,env=BLOCKCHAIN_ACTIONS_TOKEN \
    git config --global url."https://$BLOCKCHAIN_ACTIONS_TOKEN@github.com".insteadOf ssh://git@github.com

# Build with improved caching
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo install --locked --profile=${LTO_RELEASE} \
    --path ./kms-connector/simple-connector --root ./kms-connector/simple-connector --bins


# Final runtime stage
FROM debian:stable-slim AS prod

# Install minimal runtime dependencies
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/kms-connector
ENV PATH="$PATH:/app/kms-connector/bin"

# Copy binaries and config
COPY --from=base /app/kms-connector/simple-connector/bin/ /app/kms-connector/bin/
COPY ./kms-connector/config/environments/config-base.toml /app/kms-connector/config/environments/config-base.toml

# Change user to limit root access
RUN groupadd -g 10002 kms && \
    useradd -m -u 10003 -g kms kms
RUN chown -R kms:kms /app/kms-connector
USER kms

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -f "kms-connector" > /dev/null || exit 1

CMD ["/app/kms-connector/bin/kms-connector", "start"]

FROM prod AS dev