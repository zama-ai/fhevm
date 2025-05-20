ARG NODE_VERSION=22.14.0-alpine3.21

# Builder
FROM node:${NODE_VERSION} AS builder

USER root

RUN apk add --no-cache \
    kubectl \
    bash && \
    rm -rf /var/cache/apk/*

RUN mkdir -p /app && \
    addgroup -g 10001 fhevm && \
    adduser -D -u 10000 -G fhevm fhevm && \
    mkdir -p /app /home/fhevm

# Runtime
FROM node:${NODE_VERSION} AS runtime

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/bash /bin/bash
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /usr/bin/kubectl /usr/bin/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=fhevm:fhevm /home/fhevm /home/fhevm
COPY --from=builder --chown=fhevm:fhevm /app /app

USER fhevm:fhevm

ENTRYPOINT ["/bin/bash", "-c"]
