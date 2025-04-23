FROM ghcr.io/zama-ai/fhevm-node:latest

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

USER fhevm:fhevm

WORKDIR /app

RUN chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /app

# Copy only necessary files
COPY --chown=fhevm:fhevm contracts/package.json contracts/package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY --chown=fhevm:fhevm contracts/*.ts contracts/tsconfig.json ./
COPY --chown=fhevm:fhevm contracts/contracts ./contracts/
COPY --chown=fhevm:fhevm contracts/addresses ./addresses/
COPY --chown=fhevm:fhevm contracts/tasks ./tasks/
COPY --chown=fhevm:fhevm contracts/lib ./lib/
COPY --chown=fhevm:fhevm contracts/decryptionOracle ./decryptionOracle/

# Pre-compile proxy contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxy

ENTRYPOINT ["/bin/bash", "-c"]