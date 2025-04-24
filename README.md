# console

A user-friendly web app to manage your FHE dApps

- apps:

  - [apps/front](apps/front/README.md) react
  - [apps/back](apps/back/README.md) nestjs + prisma + graphql
  - [apps/orchestrator](apps/orchestrator/README.md) nestjs + xstate
  - [apps/web3](apps/web3/README.md) nestjs

- packages:

  - [messages](packages/messages/README.md) queues messages definition
  - [sqs](packages/sqs/README.md) provides modules to use SQS
  - [utils](packages/utils/README.md) nodejs-related stuff

## installation

```bash
# build & run local aws services and databases
make console-infra-up

# set up environment
cp apps/front/.env.template apps/front/.env
cp apps/back/.env.template apps/back/.env
cp apps/orchestrator/.env.template apps/orchestrator/.env

# install npm packages
pnpm install

# build our own packages
pnpm --filter messages build
pnpm --filter utils build
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

### run fhevm contracts with events using hardhat

An alternative to running the full stack is to use hardhat.
This also allows for the use of the latest version of the FHEVM set of smart contracts.
To do so one can do the following:

1. `make hardhat-run` in a first terminal, that will launch the hardhat node
2. `make hardhat-listen` in a second terminal will deploy the contracts and launch the event listener
3. `make hardhat-test` in a third terminal will then launch some tests that will trigger events

## run local env into containers

There are more complicated and fun ways to have a local env, check [tech-specs/local_env](tech-specs/local_env.md)
