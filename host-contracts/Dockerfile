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

USER fhevm:fhevm

WORKDIR /app

RUN chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /app

# Copy only necessary files
COPY --chown=fhevm:fhevm package.json package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY --chown=fhevm:fhevm *.ts ./tsconfig.json ./
COPY --chown=fhevm:fhevm contracts ./contracts/
COPY --chown=fhevm:fhevm tasks ./tasks/
COPY --chown=fhevm:fhevm lib ./lib/
COPY --chown=fhevm:fhevm decryptionOracle ./decryptionOracle/

# Pre-compile proxy contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxy

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev