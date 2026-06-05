ARG NODE_VERSION=22.22.2-alpine3.22
ARG RELAYER_SDK_VERSION=""

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder
ARG RELAYER_SDK_VERSION=""

WORKDIR /app

RUN apk add --no-cache \
    bash \
    curl \
    wget \
    vim \
    jq \
    python3 \
    build-base && \
    rm -rf /var/cache/apk/*

RUN addgroup -g 10001 fhevm && \
    adduser -D -u 10000 -G fhevm fhevm && \
    mkdir -p /app /home/fhevm

# Copy only dependency manifests first for better cache reuse
COPY package.json package-lock.json ./
COPY library-solidity/package.json ./library-solidity/package.json
COPY test-suite/e2e/package.json ./test-suite/e2e/
COPY sdk/js-sdk/package.json ./sdk/js-sdk/
COPY sdk/js-sdk/src/package.json ./sdk/js-sdk/src/

COPY library-solidity ./library-solidity
COPY test-suite/e2e ./test-suite/e2e
COPY sdk/js-sdk ./sdk/js-sdk

# Install dependencies. Normal builds stay locked; rollout hotswap builds first
# update the e2e workspace manifest in the image, then let npm resolve one
# coherent dependency graph for that requested SDK version.
#
# The e2e workspace defaults to @zama-fhe/relayer-sdk ^0.5.0-alpha.2 and the
# committed lockfile pins that exact nested version, so a plain `npm pkg set` +
# install keeps 0.5.0-alpha as a nested dep that node resolves before any
# hoisted pin. For the hotswap path, force the requested SDK with a root
# `overrides` entry (safe: js-sdk has no relayer-sdk dependency) AND drop the
# lockfile so npm re-resolves one coherent graph instead of restoring the
# pinned nested version. A build-time assertion below fails loudly if the e2e
# workspace still resolves a different SDK.
RUN if [ -n "$RELAYER_SDK_VERSION" ]; then \
      npm pkg set -w test-suite/e2e "dependencies.@zama-fhe/relayer-sdk=${RELAYER_SDK_VERSION}" && \
      npm pkg set "overrides.@zama-fhe/relayer-sdk=${RELAYER_SDK_VERSION}" && \
      rm -f package-lock.json && \
      npm install && \
      npm prune; \
    else \
      npm ci && \
      npm prune; \
    fi

# Compile js-sdk
WORKDIR /app/sdk/js-sdk

RUN npm install
RUN npm run build

# Compile test-suite
WORKDIR /app/test-suite/e2e

RUN npx hardhat compile

# Guard against silently shipping the wrong SDK: when a version was requested,
# fail the build loudly if the e2e workspace resolves a different one. This is
# the check that would have caught the rollout running 0.5.0-alpha under a
# RELAYER_SDK_VERSION=0.4.2 pin.
RUN if [ -n "$RELAYER_SDK_VERSION" ]; then \
      node -e "const p=require.resolve('@zama-fhe/relayer-sdk/package.json',{paths:['/app/test-suite/e2e']});const v=require(p).version;const want=process.env.RELAYER_SDK_VERSION;console.log('[build] e2e @zama-fhe/relayer-sdk resolved to '+v+' (requested '+want+')');if(v!==want){console.error('ERROR: e2e resolved @zama-fhe/relayer-sdk '+v+' but RELAYER_SDK_VERSION='+want);process.exit(1);}"; \
    fi

# --- Runtime Stage ---
FROM node:${NODE_VERSION} AS prod
ARG RELAYER_SDK_VERSION=""

# Propagate the build ARG into the container's runtime env. instance.ts gates
# on this:
#   process.env.RELAYER_SDK_VERSION === ""   → use @fhevm/sdk (default)
#   process.env.RELAYER_SDK_VERSION === "x"  → use @zama-fhe/relayer-sdk@x
ENV RELAYER_SDK_VERSION=${RELAYER_SDK_VERSION}

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/bin /usr/bin
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

USER fhevm:fhevm

WORKDIR /app/test-suite/e2e

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
