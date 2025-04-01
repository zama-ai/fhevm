FROM node:22-slim AS final
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN apt-get update -y && apt-get install -y openssl
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --lockfile
RUN pnpm run --filter "./packages/**" build
RUN pnpm --filter=email build 
RUN pnpm deploy --filter=email --prod /prod/email
ENV NODE_ENV=production
EXPOSE 8002
ENV PORT=8002
CMD [ "pnpm", "start" ]
