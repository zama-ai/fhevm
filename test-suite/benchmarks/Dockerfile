ARG NODE_VERSION=20.19.1-alpine3.21

# --- Builder Stage ---
FROM node:${NODE_VERSION} AS builder

WORKDIR /app

RUN apk add --no-cache \
    bash \
    curl \
    wget \
    vim \
    jq && \
    rm -rf /var/cache/apk/*

RUN addgroup -g 10001 fhevm && \
    adduser -D -u 10000 -G fhevm fhevm && \
    mkdir -p /app /home/fhevm

# Copy only necessary files
COPY benchmarks/package.json benchmarks/package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY benchmarks/*.sh e2e/*.ts e2e/tsconfig.json ./
COPY benchmarks/contracts ./contracts/
COPY benchmarks/tasks ./tasks/

RUN npx hardhat compile && \
    npx hardhat compile:specific --contract decryptionOracle

# --- Runtime Stage ---
FROM node:${NODE_VERSION} AS prod

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/ /bin/
COPY --from=builder /usr/bin /usr/bin
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

USER fhevm:fhevm

WORKDIR /app

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
