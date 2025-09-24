# --- Base stage ---
FROM node:22-alpine3.22 AS base
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
COPY ./apps/front apps/front
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm --filter "./packages/**" build
RUN pnpm --filter=front build
RUN pnpm deploy --filter=front --prod /prod/front
RUN cp -r apps/front/dist /prod/front/
RUN mkdir -p /prod/front/dist/scripts

# --- Runtime stage ---
FROM nginx:1.29.1-alpine3.22 AS runtime
WORKDIR /prod-wrkspc
COPY --from=builder /prod/front/dist /usr/share/nginx/html/
COPY /docker/front/nginx.conf /etc/nginx/nginx.conf
# Copy the entrypoint script
COPY apps/front/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT [ "/entrypoint.sh" ]

EXPOSE 8080

# The default command to run after the entrypoint script finishes.
# This starts Nginx in the foreground.
CMD ["nginx", "-g", "daemon off;"]