FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS prod

USER root

RUN apk add --no-cache \
    bash \
    kubectl \
    python3 \
    python3-dev \
    make \
    g++ \
    gcc \
    nodejs-dev && \
    ln -sf /usr/bin/gcc /usr/bin/cc && \
    rm -rf /var/cache/apk/*

RUN mkdir -p /app && \
    chown -R fhevm:fhevm /app &&\
    mkdir -p /install/host-contracts && \
    chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /install

USER fhevm:fhevm

WORKDIR /install

# Copy only necessary files
COPY --chown=fhevm:fhevm package.json package-lock.json ./

COPY --chown=fhevm:fhevm host-contracts/package.json host-contracts/

# Install dependencies
RUN npm ci --workspace=host-contracts  --include-workspace-root=false && \
    npm prune

# Create addresses directory for storing proxy contract addresses

WORKDIR /app

RUN mv /install/host-contracts/* . &&\
    mv /install/node_modules ./node_modules

COPY --chown=fhevm:fhevm host-contracts/*.ts host-contracts/tsconfig.json ./
COPY --chown=fhevm:fhevm host-contracts/contracts ./contracts/
COPY --chown=fhevm:fhevm host-contracts/tasks ./tasks/
COPY --chown=fhevm:fhevm host-contracts/lib ./lib/
COPY --chown=fhevm:fhevm host-contracts/decryptionOracle ./decryptionOracle/

RUN mkdir -p ./addresses && chown fhevm:fhevm ./addresses

# Pre-compile proxy contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxyACL

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
