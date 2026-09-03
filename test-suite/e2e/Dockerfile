ARG NODE_VERSION=22.22.2-alpine3.22

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder

# SDK_SOURCE selects how @fhevm/sdk is provided to test-suite/e2e (it isn't
# declared in package.json — install-sdk.sh installs it in place):
#   local    (default) - build + pack sdk/js-sdk and install the tarball,
#                         so e2e tests exercise the local, unreleased SDK.
#   registry            - install a fixed version from the npm registry;
#                         SDK_VERSION must be set, there is no default.
ARG SDK_SOURCE=local
ARG SDK_VERSION=

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

# Install dependencies (@fhevm/sdk isn't among them; it's installed below
# per SDK_SOURCE)
RUN npm ci && \
    npm prune

# Copy sources after deps are cached
COPY library-solidity ./library-solidity
COPY test-suite/e2e ./test-suite/e2e
COPY sdk/js-sdk ./sdk/js-sdk

# sdk/js-sdk is no longer an npm workspace member: install its own deps
# standalone using its own lockfile, then point test-suite/e2e's @fhevm/sdk at
# either a local build (install-sdk.sh builds + packs it) or a fixed registry
# version, per SDK_SOURCE.
RUN if [ "$SDK_SOURCE" = "local" ]; then \
      (cd sdk/js-sdk && npm ci) && \
      (cd test-suite/e2e && ./scripts/install-sdk.sh local); \
    elif [ "$SDK_SOURCE" = "registry" ]; then \
      (cd test-suite/e2e && ./scripts/install-sdk.sh registry "$SDK_VERSION"); \
    else \
      echo "Unknown SDK_SOURCE '$SDK_SOURCE' (use 'local' or 'registry')" >&2 && exit 1; \
    fi

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
