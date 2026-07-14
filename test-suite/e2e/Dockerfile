ARG NODE_VERSION=22.22.2-alpine3.22
ARG RELAYER_SDK_VERSION=""
ARG E2E_SDK_FAMILY=""
ARG E2E_SDK_SOURCE=""
ARG E2E_SDK_VERSION=""

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder
ARG RELAYER_SDK_VERSION=""
ARG E2E_SDK_FAMILY
ARG E2E_SDK_SOURCE
ARG E2E_SDK_VERSION

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

# Install the repository graph from its lockfile. Published SDK cases pin the
# selected top-level package while retaining the other workspace dependencies.
RUN npm ci && npm prune && \
    npm run -w test-suite/e2e validate:sdk-selection && \
    sdk_family="$E2E_SDK_FAMILY" && sdk_source="$E2E_SDK_SOURCE" && sdk_version="$E2E_SDK_VERSION" && \
    if [ -z "$sdk_family$sdk_source$sdk_version" ] && [ -n "$RELAYER_SDK_VERSION" ]; then \
      sdk_family="relayer-sdk"; sdk_source="npm"; sdk_version="$RELAYER_SDK_VERSION"; \
    elif [ -z "$sdk_family$sdk_source$sdk_version" ]; then \
      sdk_family="fhevm-sdk"; sdk_source="workspace"; sdk_version="workspace"; \
    fi && \
    if [ "$sdk_source" = "npm" ]; then \
      if [ "$sdk_family" = "fhevm-sdk" ]; then package="@fhevm/sdk"; \
      elif [ "$sdk_family" = "relayer-sdk" ]; then package="@zama-fhe/relayer-sdk"; \
      else echo "Unsupported E2E_SDK_FAMILY: $sdk_family" >&2; exit 1; fi; \
      npm pkg set -w test-suite/e2e "dependencies.${package}=${sdk_version}" && \
      npm install && \
      npm prune; \
    elif [ "$sdk_family" != "fhevm-sdk" ] || [ "$sdk_version" != "workspace" ]; then \
      echo "workspace selection requires fhevm-sdk@workspace" >&2; exit 1; \
    fi

# Compile js-sdk
WORKDIR /app/sdk/js-sdk

RUN npm run build

# Compile test-suite
WORKDIR /app/test-suite/e2e

RUN npm run test:sdk-selection && npm run tsc:sdk-selection
RUN npx hardhat compile

# --- Runtime Stage ---
FROM node:${NODE_VERSION} AS prod
ARG RELAYER_SDK_VERSION=""
ARG E2E_SDK_FAMILY
ARG E2E_SDK_SOURCE
ARG E2E_SDK_VERSION

# RELAYER_SDK_VERSION remains available only for images built through the
# legacy interface. New callers select family, source, and version explicitly.
ENV RELAYER_SDK_VERSION=${RELAYER_SDK_VERSION}
ENV E2E_SDK_FAMILY=${E2E_SDK_FAMILY}
ENV E2E_SDK_SOURCE=${E2E_SDK_SOURCE}
ENV E2E_SDK_VERSION=${E2E_SDK_VERSION}

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
