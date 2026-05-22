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

# Install dependencies
RUN npm ci && \
    npm prune && \
    if [ -n "$RELAYER_SDK_VERSION" ]; then \
      npm install -w test-suite/e2e --no-save "@zama-fhe/relayer-sdk@${RELAYER_SDK_VERSION}"; \
    fi

# Copy sources after deps are cached
COPY library-solidity ./library-solidity
COPY test-suite/e2e ./test-suite/e2e
COPY sdk/js-sdk ./sdk/js-sdk

# Patch sources: emit requested relayer-sdk version into
# test/_relayer_sdk_version.ts so the e2e tests can branch on it at runtime.
RUN printf '// Auto-generated at Docker build time. Do not edit.\nexport const relayer_sdk_version = "%s";\n' \
      "$RELAYER_SDK_VERSION" > test-suite/e2e/test/_relayer_sdk_version.ts

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
