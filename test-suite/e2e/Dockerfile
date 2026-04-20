ARG NODE_VERSION=20.19.1-alpine3.21
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

# Install dependencies
RUN npm ci && \
    npm prune && \
    if [ -n "$RELAYER_SDK_VERSION" ]; then \
      npm install --prefix test-suite/e2e --no-save "@zama-fhe/relayer-sdk@${RELAYER_SDK_VERSION}"; \
    fi && \
    RELAYER_SDK_VERSION="$RELAYER_SDK_VERSION" node -e "const normalize=(v)=>typeof v==='string'?v.replace(/^v/,''):v;const sdkPath=require.resolve('@zama-fhe/relayer-sdk/package.json',{paths:['/app/test-suite/e2e']});const sdk=require(sdkPath);const expected=normalize(process.env.RELAYER_SDK_VERSION);if(expected&&normalize(sdk.version)!==expected){throw new Error('resolved @zama-fhe/relayer-sdk '+sdk.version+' instead of '+expected+' from '+sdkPath);}const tkmsPath=require.resolve('node-tkms/package.json',{paths:['/app/test-suite/e2e']});const tkms=require(tkmsPath);const expectedTkms=normalize(sdk.dependencies?.['node-tkms']);if(expectedTkms&&normalize(tkms.version)!==expectedTkms){throw new Error('resolved node-tkms '+tkms.version+' instead of '+expectedTkms+' from '+tkmsPath);}"

# Copy sources after deps are cached
COPY library-solidity ./library-solidity
COPY test-suite/e2e ./test-suite/e2e

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
