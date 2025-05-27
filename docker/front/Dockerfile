# --- Base stage ---
FROM node:24.0.2-alpine3.21 AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
RUN apk add --no-cache gettext

# --- Builder stage ---
FROM base AS builder
WORKDIR /wrkspc
COPY package.json package.json
COPY pnpm-workspace.yaml pnpm-workspace.yaml
COPY pnpm-lock.yaml pnpm-lock.yaml
COPY ./packages packages
COPY ./apps/front apps/front
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm --filter "./packages/**" build
# RUN pnpx prisma generate --schema apps/front/prisma/schema.prisma
RUN pnpm --filter=front build
# RUN pnpm --filter=front run copy:generated
RUN pnpm deploy --filter=front --prod /prod/front
RUN cp -r apps/front/dist /prod/front/
RUN mkdir -p /prod/front/dist/scripts
COPY ./apps/front/src/public/scripts /prod/front/dist/scripts

# --- Runtime stage ---
FROM nginx:1.28-alpine3.21 AS runtime
WORKDIR /prod-wrkspc
COPY --from=builder /prod/front/dist /usr/share/nginx/html/
COPY /docker/front/nginx.conf /etc/nginx/nginx.conf
EXPOSE 8080
