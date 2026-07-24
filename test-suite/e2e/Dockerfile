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
RUN if [ -n "$RELAYER_SDK_VERSION" ]; then \
      npm pkg set -w test-suite/e2e "dependencies.@zama-fhe/relayer-sdk=${RELAYER_SDK_VERSION}" && \
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

# --- CLI Builder Stage ---
# sdk/cli-js-sdk is a standalone pnpm workspace (not part of the root npm
# workspaces), so it gets its own stage. `pnpm deploy` produces a
# self-contained folder (package + resolved prod node_modules, no store
# symlinks) suitable for copying into the runtime stage.
FROM node:${NODE_VERSION} AS cli-builder

# pnpm prompts before purging an out-of-sync modules dir; CI=true makes it
# non-interactive.
ENV CI=true

RUN npm install -g pnpm@11.9.0

WORKDIR /cli
COPY sdk/cli-js-sdk ./

RUN pnpm install --frozen-lockfile --filter cli-fhevm-sdk... && \
    pnpm --filter cli-fhevm-sdk build && \
    pnpm --filter cli-fhevm-sdk deploy --prod --legacy /out/fhevm-cli

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

# fhevm-sdk CLI: wallet/API key/RPC URL are injected at runtime via env vars
# (PRIVATE_KEY, MNEMONIC, ZAMA_FHEVM_API_KEY, SEPOLIA_RPC_URL, ...).
COPY --from=cli-builder --chown=fhevm:fhevm /out/fhevm-cli /app/fhevm-cli
RUN ln -s /app/fhevm-cli/bin/fhevm-sdk.mjs /usr/local/bin/fhevm-sdk

USER fhevm:fhevm

WORKDIR /app/test-suite/e2e

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
