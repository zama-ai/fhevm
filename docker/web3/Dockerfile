# --- Base stage ---
FROM node:22.14.0-alpine3.21 AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

# --- Builder stage ---
FROM base AS builder
WORKDIR /wrkspc
COPY package.json package.json
COPY pnpm-workspace.yaml pnpm-workspace.yaml
COPY pnpm-lock.yaml pnpm-lock.yaml
COPY ./packages packages
COPY ./apps/web3 apps/web3
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm --filter "./packages/**" build
RUN pnpx prisma generate --schema apps/web3/prisma/schema.prisma
RUN pnpm --filter=web3 build
RUN pnpm --filter=web3 run copy:generated
RUN pnpm deploy --filter=web3 --prod /prod/web3
RUN cp -r apps/web3/dist /prod/web3/

# --- Runtime stage ---
FROM base AS runtime
WORKDIR /prod-wrkspc
COPY --from=builder /prod/web3 ./
EXPOSE 3002

RUN addgroup -g 10002 zwsgrp && \
    adduser -D -u 10001 -G zwsgrp zwsusr
RUN chown -R zwsusr:zwsgrp /prod-wrkspc
USER zwsusr:zwsgrp

# TODO: update endpoint if healthcheck is implemented
# RUN apk add --no-cache openssl wget
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -f "dist/src/main" > /dev/null || exit 1
    # CMD wget -qO- http://localhost:3002/healthcheck || exit 1

CMD ["pnpm", "start:prod"]
