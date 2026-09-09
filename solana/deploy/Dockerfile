# syntax=docker/dockerfile:1
# zama-host / confidential-token deployer image. Same role as host-contracts: CLI + artifacts
# in the image, Helm Job (`charts/contracts` → `/app/deploy-contracts.sh`) invokes
# `node /app/cli.mjs host deploy`. linux/amd64 only — there is no arm64 Agave CLI (anza-xyz/agave#1734).
#
# Build from repo root:
#   docker build --platform=linux/amd64 -f solana/deploy/Dockerfile -t ghcr.io/zama-ai/fhevm/solana-programs:local .

ARG PROGRAM_PROFILE=preview-env
ARG SOLANA_VERSION=4.1.2
ARG ANCHOR_VERSION=1.1.2
ARG KUBECTL_VERSION=v1.32.3

FROM --platform=linux/amd64 ubuntu:24.04 AS sbf
ARG PROGRAM_PROFILE
ARG SOLANA_VERSION
ARG ANCHOR_VERSION
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl git bzip2 build-essential pkg-config libudev-dev libssl-dev python3 \
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
COPY solana/Anchor.toml solana/Cargo.toml solana/Cargo.lock solana/rust-toolchain.toml ./
COPY solana/programs/ programs/
COPY solana/crates/ crates/
COPY solana/runtime-tests/ runtime-tests/
COPY solana/test-kit/ test-kit/
COPY solana/scripts/build-programs.sh scripts/build-programs.sh
# Preview-env profile: compile `declare_id!` behind `--features preview-env`.
# Program private keys are first-deploy runtime secrets (k8s / AWS), not a build input.
RUN rustup show && bash scripts/build-programs.sh "$PROGRAM_PROFILE" \
      zama_host confidential_token demo_vault confidential_batcher

FROM --platform=linux/amd64 oven/bun:1.3.14-debian AS bundle
WORKDIR /src/solana/deploy
COPY solana/deploy/package.json solana/deploy/bun.lock ./
RUN bun install --frozen-lockfile --production
COPY test-suite/fhevm/src/solana/internal/generated /src/test-suite/fhevm/src/solana/internal/generated
COPY solana/demo-dapp/src/vault/internal/generated/demoVault/programAddress.ts /src/solana/demo-dapp/src/vault/internal/generated/demoVault/programAddress.ts
COPY solana/demo-dapp/src/vault/internal/generated/confidentialBatcher/programAddress.ts /src/solana/demo-dapp/src/vault/internal/generated/confidentialBatcher/programAddress.ts
COPY solana/deploy/src ./src
COPY solana/deploy/profiles/preview-env/program-ids.json /src/solana/deploy/profiles/preview-env/program-ids.json
# Bundle to one file so the runtime image does not need bun or a node_modules tree.
RUN ln -s /src/solana/deploy/node_modules /src/test-suite/fhevm/node_modules && bun build src/cli.ts --target=node --outfile=/out/cli.mjs

FROM --platform=linux/amd64 node:22-bookworm-slim AS prod
ARG PROGRAM_PROFILE
ARG KUBECTL_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends \
      bash ca-certificates curl libssl3 libudev1 postgresql-client \
    && curl -fsSL -o /usr/local/bin/kubectl "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl" \
    && chmod +x /usr/local/bin/kubectl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 fhevm \
    && useradd --uid 10000 --gid 10001 --create-home --home-dir /home/fhevm fhevm

COPY --from=sbf /root/.local/share/solana/install/active_release/bin/solana /usr/local/bin/solana
COPY --from=sbf /src/solana/target/deploy/zama_host.so /app/programs/zama_host.so
COPY --from=sbf /src/solana/target/deploy/confidential_token.so /app/programs/confidential_token.so
COPY --from=sbf /src/solana/target/deploy/demo_vault.so /app/programs/demo_vault.so
COPY --from=sbf /src/solana/target/deploy/confidential_batcher.so /app/programs/confidential_batcher.so
COPY --from=bundle /out/cli.mjs /app/cli.mjs

RUN mkdir -p /app/addresses && chown -R fhevm:fhevm /app /home/fhevm \
    && solana --version && kubectl version --client --output=yaml >/dev/null
USER fhevm:fhevm
WORKDIR /app
ENV HOME=/home/fhevm
ENV SOLANA_ARTIFACTS_DIR=/app/programs
ENV ADDRESSES_DIR=/app/addresses
ENV SOLANA_PROGRAM_PROFILE=$PROGRAM_PROFILE
# Compose uses this ENTRYPOINT (same as host-contracts). The Helm Job overrides `command` to
# `/app/deploy-contracts.sh`, which then runs `deployCommands`.
ENTRYPOINT ["node", "/app/cli.mjs"]

FROM prod AS dev
