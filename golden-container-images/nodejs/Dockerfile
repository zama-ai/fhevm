ARG NODE_VERSION=22.23.4

FROM cgr.dev/zama.ai/node:${NODE_VERSION}-dev AS prod

USER root

# Contract commands need Bash, npm and kubectl at runtime.
RUN apk add --no-cache bash kubectl-1.34-default && \
    addgroup -g 10001 fhevm && \
    adduser -D -u 10000 -G fhevm fhevm && \
    mkdir -p /app /home/fhevm && \
    chown -R fhevm:fhevm /app /home/fhevm

WORKDIR /app
USER fhevm:fhevm

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD node --version || exit 1

ENTRYPOINT ["/bin/bash", "-c"]

FROM prod AS dev
