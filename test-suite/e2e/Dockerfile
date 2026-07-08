ARG NODE_VERSION=22.22.2-alpine3.22
ARG RELAYER_SDK_VERSION=""
ARG FHEVM_SDK_VERSION=""

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder
ARG RELAYER_SDK_VERSION=""
ARG FHEVM_SDK_VERSION=""

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
# @fhevm/sdk is also a local workspace (sdk/js-sdk/src): npm always links a
# same-named workspace package over the registry regardless of the version
# range requested, so pinning FHEVM_SDK_VERSION alone would silently keep
# resolving to local source. Dropping "sdk/js-sdk/src" from the root
# workspaces list is what actually makes npm fetch the published version.
RUN if [ -n "$RELAYER_SDK_VERSION" ] || [ -n "$FHEVM_SDK_VERSION" ]; then \
      if [ -n "$RELAYER_SDK_VERSION" ]; then \
        npm pkg set -w test-suite/e2e "dependencies.@zama-fhe/relayer-sdk=${RELAYER_SDK_VERSION}"; \
      fi && \
      if [ -n "$FHEVM_SDK_VERSION" ]; then \
        npm pkg set -w test-suite/e2e "dependencies.@fhevm/sdk=${FHEVM_SDK_VERSION}" && \
        node -e "const fs=require('fs');const p=JSON.parse(fs.readFileSync('package.json','utf8'));p.workspaces=p.workspaces.filter(w=>w!=='sdk/js-sdk/src');fs.writeFileSync('package.json',JSON.stringify(p,null,2)+'\n')"; \
      fi && \
      npm install && \
      npm prune; \
    else \
      npm ci && \
      npm prune; \
    fi && \
    node -e "const p=require.resolve('@fhevm/sdk/package.json',{paths:['test-suite/e2e']});console.log('Resolved @fhevm/sdk -> '+p);console.log('@fhevm/sdk version: '+require(p).version)"

# Compile js-sdk
WORKDIR /app/sdk/js-sdk

RUN npm install
RUN npm run build

# Compile test-suite
WORKDIR /app/test-suite/e2e

RUN npx hardhat compile

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
