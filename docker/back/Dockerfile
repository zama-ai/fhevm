FROM node:22-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN apt-get update -y && apt-get install -y openssl
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --lockfile
RUN pnpm run --filter "./packages/**" build
RUN pnpx prisma generate --schema apps/back/prisma/schema.prisma
RUN pnpm --filter=back build 
RUN pnpm deploy --filter=back --prod /prod/back
ENV NODE_ENV=production
EXPOSE 8000
ENV PORT=8000
CMD [ "pnpm", "start" ]
