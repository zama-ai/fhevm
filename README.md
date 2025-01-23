# console

A user-friendly web app to manage your FHE dApps

- apps:

  - [@zws/front](apps/front/README.md) react
  - [@zws/back](apps/back/README.md) nestjs + prisma + graphql
  - [@zws/orchestrator](apps/orchestrator/README.md) nestjs + xstate
  - [@zws/web3](apps/web3/README.md) nestjs

- packages:

  - [messages](packages/messages/README.md) queues messages definition
  - [sqs](packages/sqs/README.md) provides modules to use SQS
  - [utils](packages/utils/README.md) nodejs-related stuff

## installation

```bash
# build & run your local environment
docker compose up

# set up environment
cp apps/front/.env.template apps/front/.env
cp apps/back/.env.template apps/back/.env
cp apps/orchestrator/.env.template apps/orchestrator/.env

# install npm packages
pnpm install

# build our own packages
pnpm --filter messages build
pnpm --filter utils build
pnpm --filter sqs build
```

## develop

### run services locally

```bash
# pick what you want to run
pnpm --filter front start
pnpm --filter back start
pnpm --filter orchestrator start
pnpm --filter web3 start
```

### run a blockchain locally

```bash
make blockchain-install

# create some noise on the blockchain
make blockchain-test

# listen to the blockchain activity
make blockchain-listen
```
