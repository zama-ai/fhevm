FROM ghcr.io/zama-ai/fhevm-node:latest

WORKDIR /app

RUN chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /app

# Copy only necessary files
COPY --chown=fhevm:fhevm package.json ./
COPY --chown=fhevm:fhevm package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY --chown=fhevm:fhevm ./hardhat.config.ts ./tsconfig.json ./
COPY --chown=fhevm:fhevm ./contracts ./contracts/
COPY --chown=fhevm:fhevm ./addresses ./addresses/
COPY --chown=fhevm:fhevm ./tasks ./tasks/

# Pre-compile proxy contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxy

USER fhevm:fhevm

ENTRYPOINT ["/bin/bash", "-c"]