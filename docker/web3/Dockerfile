FROM node:22-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN apt-get update -y && apt-get install -y openssl
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --lockfile
RUN pnpm run --filter "./packages/**" build
RUN pnpm --filter=web3 build
RUN pnpm deploy --filter=web3 --prod /prod/web3
ENV NODE_ENV=production
EXPOSE 8004
ENV PORT=8004
CMD [ "pnpm", "start" ]
