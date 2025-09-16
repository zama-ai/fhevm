FROM node:22.14.0-alpine3.21 AS prod

USER root

RUN apk add --no-cache bash kubectl python3 python3-dev make g++ gcc nodejs-dev && ln -sf /usr/bin/gcc /usr/bin/cc && rm -rf /var/cache/apk/* && addgroup -S fhevm && adduser -S -G fhevm fhevm

RUN corepack enable pnpm

USER fhevm:fhevm

WORKDIR /app

RUN chown -R fhevm:fhevm /home/fhevm && chown -R fhevm:fhevm /app

# Copy only necessary files
COPY --chown=fhevm:fhevm apps/oracle/package.json ./apps/oracle/
COPY --chown=fhevm:fhevm package.json pnpm-lock.yaml pnpm-workspace.yaml ./

# Install dependencies
RUN pnpm i --frozen-lockfile && pnpm prune

# Copy the application files
COPY --chown=fhevm:fhevm apps/oracle/*.ts ./apps/oracle/tsconfig.json ./apps/oracle/
COPY --chown=fhevm:fhevm apps/oracle/contracts ./apps/oracle/contracts/
COPY --chown=fhevm:fhevm apps/oracle/tasks ./apps/oracle/tasks/

WORKDIR /app/apps/oracle

# Pre-compile proxy contracts
RUN pnpm hardhat clean && \
    pnpm hardhat compile:specific --contract contracts/emptyProxy

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
