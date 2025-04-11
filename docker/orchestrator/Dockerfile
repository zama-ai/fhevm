# --- Base stage ---
FROM node:22.14.0-alpine3.21 AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

# --- Builder stage ---
FROM base AS builder
WORKDIR /wrkspc
COPY . .
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm --filter "./packages/**" build
RUN pnpx prisma generate --schema apps/orchestrator/prisma/schema.prisma
RUN pnpm --filter=orchestrator build
RUN pnpm --filter=orchestrator run copy:generated
RUN pnpm deploy --filter=orchestrator --prod /prod/orchestrator
RUN cp -r apps/orchestrator/dist /prod/orchestrator/

# --- Runtime stage ---
FROM base AS runtime
WORKDIR /prod-wrkspc
COPY --from=builder /prod/orchestrator ./
EXPOSE 3001

RUN addgroup -g 10002 zwsgrp && \
    adduser -D -u 10001 -G zwsgrp zwsusr
RUN chown -R zwsusr:zwsgrp /prod-wrkspc
USER zwsusr:zwsgrp

# TODO: update endpoint if healthcheck is implemented
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -f "dist/src/main" > /dev/null || exit 1

CMD ["pnpm", "start:prod"]