FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.23.4 AS prod

WORKDIR /app

# Copy only necessary files
COPY --chown=fhevm:fhevm package.json package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY --chown=fhevm:fhevm hardhat.config.ts tsconfig.json ./
COPY --chown=fhevm:fhevm contracts ./contracts/
COPY --chown=fhevm:fhevm tasks ./tasks/

# Create addresses directory for storing proxy contract addresses
RUN mkdir -p ./addresses && chown fhevm:fhevm ./addresses

# Pre-compile proxy and mock contracts
# Implementation contracts and regular proxy contracts cannot be pre-compiled as they depend on the
# GatewayConfig proxy contract's address, which is only available after it is deployed
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxyGatewayConfig && \
    npx hardhat compile:specific --contract contracts/mocks

USER fhevm:fhevm

# Check that some pre-compiled contracts are present
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD test -d /app/artifacts && find /app/artifacts -type f -name "*.json" | grep -q . || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
