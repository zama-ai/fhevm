# Builder
FROM node:23.10.0-alpine3.20 AS builder

USER root

RUN apk add --no-cache \
    kubectl \
    bash && \
    rm -rf /var/cache/apk/*

RUN mkdir -p /app && \
    addgroup -g 10001 httpz && \
    adduser -D -u 10000 -G httpz httpz && \
    mkdir -p /app /home/httpz

# Runtime
FROM node:23.10.0-alpine3.20 AS runtime

COPY --from=builder /lib/ /lib/
COPY --from=builder /bin/bash /bin/bash
COPY --from=builder /usr/lib/ /usr/lib/
COPY --from=builder /usr/bin/kubectl /usr/bin/
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder --chown=httpz:httpz /home/httpz /home/httpz
COPY --from=builder --chown=httpz:httpz /app /app

USER httpz:httpz

ENTRYPOINT ["/bin/bash", "-c"]
