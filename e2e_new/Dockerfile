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
COPY e2e_new/package.json e2e_new/package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY e2e_new/*.sh e2e_new/*.ts e2e_new/tsconfig.json ./
COPY e2e_new/contracts ./contracts/
COPY e2e_new/test ./test/

RUN npx hardhat compile && \
    npx hardhat compile:specific --contract decryptionOracle

# --- Runtime Stage ---
FROM node:${NODE_VERSION} AS runtime

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/bash /bin/bash
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

USER fhevm:fhevm

WORKDIR /app

ENTRYPOINT ["/bin/bash", "-c"]