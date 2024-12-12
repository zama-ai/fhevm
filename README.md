# console

- apps:
  - [@zws/front](apps/front/README.md) react
  - [@zws/back](apps/back/README.md) nestjs + prisma + graphql
  - [@zws/orchestrator](apps/orchestrator/README.md) nestjs + xstate

# installation

```bash
# build & run your local environment
$ docker compose up

# set up environment
$  cp apps/front/.env.template apps/front/.env
$  cp apps/back/.env.template apps/back/.env
$  cp apps/orchestrator/.env.template apps/orchestrator/.env


# install packages
$ pnpm install
```

# develop

```bash
# build the necessary packages
pnpm --filter utils build

# run all in dev mode
$ pnpm start
```
