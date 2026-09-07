# syntax=docker/dockerfile:1
# zama-host / confidential-token deployer image. Same role as host-contracts: CLI + artifacts
# in the image, Helm Job (`charts/contracts` → `/app/deploy-contracts.sh`) invokes
# `node /app/cli.mjs all`. linux/amd64 only — there is no arm64 Agave CLI (anza-xyz/agave#1734).
#
# Build from repo root:
#   docker build --platform=linux/amd64 -f solana/deploy/Dockerfile -t ghcr.io/zama-ai/fhevm/solana-programs:local .

ARG SOLANA_VERSION=4.1.2
ARG ANCHOR_VERSION=1.1.2
ARG KUBECTL_VERSION=v1.32.3

FROM --platform=linux/amd64 node:22-bookworm AS sbf
ARG SOLANA_VERSION
ARG ANCHOR_VERSION
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl build-essential pkg-config libudev-dev libssl-dev python3 \
    && rm -rf /var/lib/apt/lists/*

# Pin matches solana/rust-toolchain.toml. cargo-build-sbf needs a rustc on PATH.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.91.1
ENV PATH="/root/.cargo/bin:${PATH}"

RUN curl -sSfL "https://release.anza.xyz/v${SOLANA_VERSION}/install" -o /tmp/anza-install.sh \
    && sh /tmp/anza-install.sh \
    && rm /tmp/anza-install.sh
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

RUN curl -sSfL "https://github.com/solana-foundation/anchor/releases/download/v${ANCHOR_VERSION}/anchor-${ANCHOR_VERSION}-x86_64-unknown-linux-gnu" \
      -o /usr/local/bin/anchor \
    && chmod +x /usr/local/bin/anchor \
    && solana --version && anchor --version

WORKDIR /src/solana
COPY solana/ /src/solana/
# Preview-env profile: compile `declare_id!` behind `--features preview-env`.
# Program private keys are first-deploy runtime secrets (k8s / AWS), not a build input.
RUN rustup show \
    && anchor build --ignore-keys --no-idl -p zama_host -- --features preview-env \
    && anchor build --ignore-keys --no-idl -p confidential_token -- --features preview-env

FROM --platform=linux/amd64 oven/bun:1.3.14-debian AS bundle
WORKDIR /app
COPY solana/deploy/package.json solana/deploy/bun.lock /app/
RUN bun install --frozen-lockfile --production
COPY test-suite/fhevm/src/solana/internal/generated /app/src/solana/internal/generated
COPY test-suite/fhevm/src/solana/host-deploy /app/src/solana/host-deploy
# Bundle to one file so the runtime image does not need bun or a node_modules tree.
RUN bun build src/solana/host-deploy/cli.ts --target=node --outfile=/out/cli.mjs

FROM --platform=linux/amd64 node:22-bookworm-slim AS prod
ARG KUBECTL_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends \
      bash ca-certificates curl libssl3 libudev1 \
    && curl -fsSL -o /usr/local/bin/kubectl "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl" \
    && chmod +x /usr/local/bin/kubectl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 fhevm \
    && useradd --uid 10000 --gid 10001 --create-home --home-dir /home/fhevm fhevm

COPY --from=sbf /root/.local/share/solana/install/active_release/bin/solana /usr/local/bin/solana
COPY --from=sbf /src/solana/target/deploy/zama_host.so /app/programs/zama_host.so
COPY --from=sbf /src/solana/target/deploy/confidential_token.so /app/programs/confidential_token.so
COPY --from=bundle /out/cli.mjs /app/cli.mjs

RUN mkdir -p /app/addresses && chown -R fhevm:fhevm /app /home/fhevm \
    && solana --version && kubectl version --client --output=yaml >/dev/null
USER fhevm:fhevm
WORKDIR /app
ENV HOME=/home/fhevm
ENV SOLANA_ARTIFACTS_DIR=/app/programs
ENV ADDRESSES_DIR=/app/addresses
ENV SOLANA_PROGRAM_PROFILE=preview-env
# Compose uses this ENTRYPOINT (same as host-contracts). The Helm Job overrides `command` to
# `/app/deploy-contracts.sh`, which then runs `deployCommands`.
ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
