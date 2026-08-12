ARG NODE_VERSION=22.22.2-alpine3.22
ARG RELAYER_SDK_VERSION=""

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder
ARG RELAYER_SDK_VERSION=""

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

COPY library-solidity ./library-solidity
COPY test-suite/e2e ./test-suite/e2e
COPY sdk/js-sdk ./sdk/js-sdk

# Install dependencies. Normal builds stay locked; rollout hotswap builds replace
# the e2e workspace SDK, then verify what Node resolves before producing the image.
# The uninstall is required: installing over the existing dependency leaves the
# lockfile's version in place. RELAYER_SDK_VERSION must be an exact version, since
# the check below compares it to what Node resolves.
RUN if [ -n "$RELAYER_SDK_VERSION" ]; then \
      npm uninstall -w test-suite/e2e "@zama-fhe/relayer-sdk" && \
      npm install -w test-suite/e2e --save-exact "@zama-fhe/relayer-sdk@${RELAYER_SDK_VERSION}" && \
      npm prune && \
      installed_sdk_version="$(cd test-suite/e2e && node -p "require('@zama-fhe/relayer-sdk/package.json').version")" && \
      if [ "$installed_sdk_version" != "$RELAYER_SDK_VERSION" ]; then \
        echo "requested @zama-fhe/relayer-sdk@${RELAYER_SDK_VERSION} but Node resolves ${installed_sdk_version}" >&2; \
        exit 1; \
      fi; \
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

# Fault/recovery consensus tests run from this container against the
# explicitly mounted Docker socket. Install the client in the runtime stage as
# well: copied builder binaries alone are not a stable Docker CLI contract.
RUN apk add --no-cache bash docker-cli

COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

USER fhevm:fhevm

WORKDIR /app/test-suite/e2e

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
