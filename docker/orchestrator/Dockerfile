FROM node:22-slim AS final
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN apt-get update -y && apt-get install -y openssl
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --lockfile
RUN pnpm run --filter "./packages/**" build
RUN pnpx prisma generate --schema apps/orchestrator/prisma/schema.prisma
RUN pnpm --filter=orchestrator build
RUN pnpm deploy --filter=orchestrator --prod /prod/orchestrator
ENV NODE_ENV=production
EXPOSE 8003
ENV PORT=8003
CMD [ "pnpm", "start" ]
