FROM ghcr.io/zama-ai/fhevm-node:latest

WORKDIR /app

RUN chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /app

# Copy only necessary files
COPY --chown=fhevm:fhevm package.json package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY --chown=fhevm:fhevm hardhat.config.ts tsconfig.json ./
COPY --chown=fhevm:fhevm contracts ./contracts/
COPY --chown=fhevm:fhevm addresses ./addresses/
COPY --chown=fhevm:fhevm tasks ./tasks/

# Pre-compile proxy and mock contracts
# Implementation contracts cannot be pre-compiled as they depend on the proxy contracts' addresses
# which are only available after they are deployed
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxy && \
    npx hardhat compile:specific --contract contracts/mocks 

USER fhevm:fhevm

ENTRYPOINT ["/bin/bash", "-c"]