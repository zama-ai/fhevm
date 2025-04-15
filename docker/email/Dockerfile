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
COPY ./apps/email apps/email
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm --filter "./packages/**" build
# RUN pnpx prisma generate --schema apps/email/prisma/schema.prisma
RUN pnpm --filter=email build
# RUN pnpm --filter=email run copy:generated
RUN pnpm deploy --filter=email --prod /prod/email
RUN cp -r apps/email/dist /prod/email/

# --- Runtime stage ---
FROM base AS runtime
WORKDIR /prod-wrkspc
COPY --from=builder /prod/email ./
EXPOSE 3300

RUN addgroup -g 10002 zwsgrp && \
    adduser -D -u 10001 -G zwsgrp zwsusr
RUN chown -R zwsusr:zwsgrp /prod-wrkspc
USER zwsusr:zwsgrp

# TODO: update endpoint if healthcheck is implemented
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -f "dist/src/main" > /dev/null || exit 1

CMD ["pnpm", "start:prod"]