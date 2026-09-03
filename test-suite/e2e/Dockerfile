ARG NODE_VERSION=22.22.2-alpine3.22

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder

WORKDIR /app

RUN apk add --no-cache \
    bash \
    curl \
    docker-cli \
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

# Install dependencies
RUN npm ci && \
    npm prune

# Copy sources after deps are cached
COPY library-solidity ./library-solidity
COPY test-suite/e2e ./test-suite/e2e
COPY sdk/js-sdk ./sdk/js-sdk
# The sdk's codegen:solana:check step reads the Anchor IDLs from here.
COPY coprocessor/fhevm-engine/host-listener/idl ./coprocessor/fhevm-engine/host-listener/idl
# The SDK build runs `codegen:solana:check`, which regenerates the Solana program clients from
# these IDLs and compares them with the committed clients in the dapp and the test-suite CLI.
COPY solana/demo-dapp/idl ./solana/demo-dapp/idl
COPY solana/demo-dapp/src/vault/internal/generated ./solana/demo-dapp/src/vault/internal/generated
COPY test-suite/fhevm/src/solana/internal/generated ./test-suite/fhevm/src/solana/internal/generated

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

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/bin /usr/bin
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd

# Fault/recovery consensus tests run from this container against the
# explicitly mounted Docker socket. Install the client in the runtime stage as
# well: copied builder binaries alone are not a stable Docker CLI contract.
RUN apk add --no-cache bash docker-cli

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
