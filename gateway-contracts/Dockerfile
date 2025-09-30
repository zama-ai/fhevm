# syntax=docker/dockerfile:1

# Build stage
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS builder

# Install build dependencies
RUN apk add --no-cache \
    bash \
    curl \
    && rm -rf /var/cache/apk/*

WORKDIR /app

# Copy package files first for better layer caching
COPY package.json package-lock.json ./

# Install dependencies with cache mount for faster builds
RUN --mount=type=cache,target=/root/.npm \
    npm ci --only=production && \
    npm cache clean --force

# Copy source files
COPY hardhat.config.ts tsconfig.json ./
COPY contracts ./contracts/
COPY tasks ./tasks/

# Create addresses directory
RUN mkdir -p ./addresses

# Pre-compile contracts
RUN npx hardhat clean && \
    npx hardhat compile:specific --contract contracts/emptyProxyGatewayConfig && \
    npx hardhat compile:specific --contract contracts/mocks

# Production stage
FROM ghcr.io/zama-ai/fhevm/gci/nodejs:22.14.0-alpine3.21 AS prod

# Install runtime dependencies
RUN apk add --no-cache \
    bash \
    curl \
    && rm -rf /var/cache/apk/*

WORKDIR /app

# Create non-root user
RUN addgroup -g 1001 -S fhevm && \
    adduser -S fhevm -u 1001 -G fhevm

# Copy built application from builder stage
COPY --from=builder --chown=fhevm:fhevm /app/node_modules ./node_modules
COPY --from=builder --chown=fhevm:fhevm /app/package.json ./package.json
COPY --from=builder --chown=fhevm:fhevm /app/hardhat.config.ts ./hardhat.config.ts
COPY --from=builder --chown=fhevm:fhevm /app/tsconfig.json ./tsconfig.json
COPY --from=builder --chown=fhevm:fhevm /app/contracts ./contracts
COPY --from=builder --chown=fhevm:fhevm /app/tasks ./tasks
COPY --from=builder --chown=fhevm:fhevm /app/artifacts ./artifacts
COPY --from=builder --chown=fhevm:fhevm /app/cache ./cache
COPY --from=builder --chown=fhevm:fhevm /app/addresses ./addresses

# Set proper permissions
RUN chown -R fhevm:fhevm /home/fhevm && \
    chown -R fhevm:fhevm /app

USER fhevm:fhevm

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD test -d /app/artifacts && find /app/artifacts -type f -name "*.json" | grep -q . || exit 1

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

ENTRYPOINT ["/bin/bash", "-c"]