# syntax=docker/dockerfile:1

# Global build args (declared before any FROM so they can be used in FROM lines).
# YELLOWSTONE_REF MUST match the validator's Agave patch (Geyser ABI is patch-coupled).
# SOLANA_VALIDATOR_IMAGE MUST be the SAME Agave patch as YELLOWSTONE_REF.
ARG YELLOWSTONE_REF=v5.0.1+solana.2.1.21
ARG SOLANA_VALIDATOR_IMAGE

# Stage 1 — build the Yellowstone geyser plugin from the pinned upstream tag.
FROM rust:1-bookworm AS plugin-builder
ARG YELLOWSTONE_REF
RUN apt-get update && apt-get install -y --no-install-recommends \
        git protobuf-compiler libprotobuf-dev pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /src
RUN git clone --depth 1 --branch "${YELLOWSTONE_REF}" \
        https://github.com/rpcpool/yellowstone-grpc .
# yellowstone's own rust-toolchain.toml pins the exact rustc; rustup honors it.
RUN cargo build --release -p yellowstone-grpc-geyser \
    && test -f target/release/libyellowstone_grpc_geyser.so

# Stage 2 — bake plugin + config into the matching 2.1.x validator image.
FROM ${SOLANA_VALIDATOR_IMAGE}
COPY --from=plugin-builder \
     /src/target/release/libyellowstone_grpc_geyser.so \
     /plugins/libyellowstone_grpc_geyser.so
COPY yellowstone-config.json /plugins/yellowstone-config.json
# Base image sets ENTRYPOINT=/usr/bin/solana-run.sh; clear it so the compose
# `command:` (solana-test-validator …) runs as the container command.
ENTRYPOINT []
