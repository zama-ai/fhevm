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
# RUN pnpx prisma generate --schema apps/front/prisma/schema.prisma
RUN pnpm --filter=front build
# RUN pnpm --filter=front run copy:generated
RUN pnpm deploy --filter=front --prod /prod/front
RUN cp -r apps/front/dist /prod/front/

# --- Runtime stage ---
FROM base AS runtime
WORKDIR /prod-wrkspc
# COPY --from=builder /prod/front ./
COPY --from=builder /prod/front/dist ./front/dist
RUN pnpm add -g http-server
EXPOSE 8001

# RUN addgroup -g 10002 zwsgrp && \
#     adduser -D -H -u 10001 -G zwsgrp zwsusr && \
#     apk add --no-cache openssl wget
# RUN chown -R zwsusr:zwsgrp /prod/front
# USER zwsusr:zwsgrp

# TODO: update endpoint if healthcheck is implemented
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD pgrep -f "http-server" > /dev/null || exit 1

# Error: sh: vite: not found
# HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
#     CMD wget -qO- http://localhost:8001/ || exit 1
    # CMD wget -qO- http://localhost:8001/healthcheck || exit 1

# Error: sh: vite: not found
# CMD ["pnpm", "start"]
CMD ["http-server", "front/dist", "-p", "8001"]
