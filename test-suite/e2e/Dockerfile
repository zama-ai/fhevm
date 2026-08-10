ARG NODE_VERSION=22.22.2-alpine3.22

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder

# SDK_SOURCE selects how @fhevm/sdk is provided to test-suite/e2e:
#   local    (default) - build + pack sdk/js-sdk and install the tarball,
#                         so e2e tests exercise the local, unreleased SDK.
#   registry            - install a fixed version from the npm registry
#                         (defaults to the version pinned in package.json).
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

# Install dependencies (installs test-suite/e2e's registry-pinned @fhevm/sdk
# as a baseline; overridden below when SDK_SOURCE=local)
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

USER fhevm:fhevm

WORKDIR /app/test-suite/e2e

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
