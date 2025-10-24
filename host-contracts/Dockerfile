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

RUN mkdir /app && \
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

WORKDIR /install/host-contracts

# Copy the application files
COPY --chown=fhevm:fhevm host-contracts/*.ts host-contracts/tsconfig.json ./
COPY --chown=fhevm:fhevm host-contracts/contracts ./contracts/
COPY --chown=fhevm:fhevm host-contracts/tasks ./tasks/
COPY --chown=fhevm:fhevm host-contracts/lib ./lib/
COPY --chown=fhevm:fhevm host-contracts/decryptionOracle ./decryptionOracle/

# Create addresses directory for storing proxy contract addresses
RUN mkdir -p ./addresses && chown fhevm:fhevm ./addresses

# Pre-compile proxy contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxyACL

WORKDIR /app

RUN mv /install/host-contracts /app &&\
    mv /install/node_modules /app/node_modules

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
