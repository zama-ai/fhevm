FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS prod

USER root

RUN apk add --no-cache \
    bash \
    curl \
    git \
    jq \
    kubectl \
    python3 \
    python3-dev \
    make \
    g++ \
    gcc \
    nodejs-dev && \
    ln -sf /usr/bin/gcc /usr/bin/cc && \
    rm -rf /var/cache/apk/*

# pnpm (used by host-contracts/lz-wiring/) and Foundry `cast` (used by
# host-contracts/lz_wiring_from_eoa_owner.sh) — baked in so LayerZero
# bridge wiring can run without any runtime installs.
RUN npm install -g pnpm@9

# Foundry publishes native musl/alpine builds; grab the archive directly
# instead of running foundryup (which downloads the glibc build and then
# fails to load on Alpine with `Error relocating cast: fcntl64: symbol
# not found`). Pin FOUNDRY_VERSION explicitly so image builds are
# reproducible.
ARG FOUNDRY_VERSION=v1.7.1
RUN curl -fsSL "https://github.com/foundry-rs/foundry/releases/download/${FOUNDRY_VERSION}/foundry_${FOUNDRY_VERSION}_alpine_amd64.tar.gz" \
      | tar -xz -C /usr/local/bin forge cast anvil chisel

RUN mkdir -p /app && \
    chown -R fhevm:fhevm /app &&\
    mkdir -p /install/host-contracts && \
    chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /install

USER fhevm:fhevm

WORKDIR /install

# Copy only necessary files
COPY --chown=fhevm:fhevm package.json package-lock.json ./

COPY --chown=fhevm:fhevm host-contracts/package.json host-contracts/

# Install dependencies
RUN npm ci --workspace=host-contracts  --include-workspace-root=false && \
    npm prune

# Create addresses directory for storing proxy contract addresses

WORKDIR /app

RUN mv /install/host-contracts/* . &&\
    mv /install/node_modules ./node_modules

COPY --chown=fhevm:fhevm host-contracts/*.ts host-contracts/tsconfig.json ./
COPY --chown=fhevm:fhevm host-contracts/contracts ./contracts/
COPY --chown=fhevm:fhevm host-contracts/tasks ./tasks/
COPY --chown=fhevm:fhevm host-contracts/lib ./lib/
COPY --chown=fhevm:fhevm host-contracts/examples/bridge/mocks ./examples/bridge/mocks/
COPY --chown=fhevm:fhevm host-contracts/lz-wiring ./lz-wiring/
COPY --chown=fhevm:fhevm host-contracts/lz_wiring_from_eoa_owner.sh ./lz_wiring_from_eoa_owner.sh

RUN mkdir -p ./addresses && chown fhevm:fhevm ./addresses

# Pre-compile proxy contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxyACL

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
