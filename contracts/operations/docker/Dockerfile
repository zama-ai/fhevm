FROM ghcr.io/zama-ai/httpz-node-golden-image:v23.10.0-alpine3.20

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

WORKDIR /app

# Copy only necessary files
COPY contracts/package.json contracts/package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY contracts/*.ts contracts/tsconfig.json ./
COPY contracts/contracts ./contracts/
COPY contracts/addresses ./addresses/
COPY contracts/tasks ./tasks/
COPY contracts/lib ./lib/
COPY contracts/decryptionOracle ./decryptionOracle/

# Pre-download Solidity compiler
RUN npx hardhat compile --force && \
    chown -R httpz:httpz /app

USER httpz:httpz

ENTRYPOINT ["/bin/bash", "-c"]