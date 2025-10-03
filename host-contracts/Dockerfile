# syntax=docker/dockerfile:1

# Build stage
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS builder

# Install build dependencies
USER root
RUN apk add --no-cache \
    bash \
    kubectl \
    python3 \
    python3-dev \
    make \
    g++ \
    gcc \
    nodejs-dev \
    curl && \
    ln -sf /usr/bin/gcc /usr/bin/cc && \
    rm -rf /var/cache/apk/*

WORKDIR /app

# Create host-contracts directory
RUN mkdir -p /app/host-contracts

# Copy package files first for better layer caching
COPY package.json package-lock.json ./
COPY host-contracts/package.json host-contracts/

# Install dependencies with cache mount
RUN --mount=type=cache,target=/root/.npm \
    npm ci --workspace=host-contracts --include-workspace-root=false && \
    npm cache clean --force

WORKDIR /app/host-contracts

# Copy source files
COPY host-contracts/*.ts host-contracts/tsconfig.json ./
COPY host-contracts/contracts ./contracts/
COPY host-contracts/tasks ./tasks/
COPY host-contracts/lib ./lib/
COPY host-contracts/decryptionOracle ./decryptionOracle/

# Pre-compile proxy contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxyACL

# Production stage
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS prod

# Install runtime dependencies
USER root
RUN apk add --no-cache \
    bash \
    kubectl \
    python3 \
    curl && \
    rm -rf /var/cache/apk/*

# Create non-root user
RUN addgroup -g 1001 -S fhevm && \
    adduser -S fhevm -u 1001 -G fhevm

WORKDIR /app

# Create host-contracts directory
RUN mkdir -p /app/host-contracts

# Copy built application from builder stage
COPY --from=builder --chown=fhevm:fhevm /app/node_modules ./node_modules
COPY --from=builder --chown=fhevm:fhevm /app/package.json ./package.json
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/node_modules /app/host-contracts/node_modules
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/package.json /app/host-contracts/package.json
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/*.ts /app/host-contracts/
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/tsconfig.json /app/host-contracts/
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/contracts /app/host-contracts/contracts/
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/tasks /app/host-contracts/tasks/
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/lib /app/host-contracts/lib/
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/decryptionOracle /app/host-contracts/decryptionOracle/
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/artifacts /app/host-contracts/artifacts/
COPY --from=builder --chown=fhevm:fhevm /app/host-contracts/cache /app/host-contracts/cache/

# Set proper permissions
RUN chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /app

USER fhevm:fhevm

WORKDIR /app/host-contracts

ENTRYPOINT ["/bin/bash", "-c"]

# Development stage with dev dependencies
FROM builder AS dev

# Install dev dependencies
RUN --mount=type=cache,target=/root/.npm \
    npm ci && \
    npm cache clean --force

# Create non-root user
RUN addgroup -g 1001 -S fhevm && \
    adduser -S fhevm -u 1001 -G fhevm

# Set proper permissions
RUN chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /app

USER fhevm:fhevm

WORKDIR /app/host-contracts

ENTRYPOINT ["/bin/bash", "-c"]