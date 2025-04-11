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
RUN pnpx prisma generate --schema apps/back/prisma/schema.prisma
RUN pnpm --filter=back build
RUN pnpm --filter=back run copy:generated
RUN pnpm deploy --filter=back --prod /prod/back
RUN cp -r apps/back/dist /prod/back/

# --- Runtime stage ---
FROM base AS runtime
WORKDIR /prod-wrkspc
COPY --from=builder /prod/back ./
EXPOSE 3000

# RUN addgroup -g 10002 zwsgrp && \
#     adduser -D -H -u 10001 -G zwsgrp zwsusr && \
#     apk add --no-cache openssl wget
# RUN chown -R zwsusr:zwsgrp /prod/back
# USER zwsusr:zwsgrp

# TODO: update endpoint if healthcheck is implemented
# 3000 port returns 404 and EXPOSE=8000 port is not the right port
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -f "dist/src/main" > /dev/null || exit 1
    # CMD wget -qO- http://localhost:3000/healthcheck || exit 1

CMD ["pnpm", "start:prod"]
