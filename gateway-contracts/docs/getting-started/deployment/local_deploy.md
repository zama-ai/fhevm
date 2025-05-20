# Deploy contracts locally

Here is an example of how to deploy contracts on a local network using hardhat.

## Prerequisites

First, git clone `fhevm-gateway` repo and install dependencies with

```bash
npm install
```

## Environment variables

All needed environment variables are defined in the [Environment variables](./env_variables.md) documentation.

For deploying the fhevm gateway contracts, these variables need to be set in an `.env` file. However, the following
`make` commands automatically copy the `.env.example` file to `.env` and update it with the correct values:

**Important**: By default, the accounts used are already funded. If other addresses are used, make sure they are funded
as well.

## Hardhat node

Launch a hardhat node:

```bash
make start-local-node
```

This runs a local Ethereum network on port 8757.

## Deploy contracts

Deploy the contracts using the `localGateway` network:

```bash
make deploy-contracts-local
```

This:

- deploys the contracts through the `deployAllGatewayContracts` task found in [deploy.ts](../../../tasks/deploy.ts)
- adds the host chains to the `GatewayConfig` contract through the `addHostChainsToGatewayConfig` task found in
  [addHostChains.ts](../../../tasks/addHostChains.ts)

## Run tests

Run all tests on the local network:

```bash
make test-local
```

A `--skip-setup` flag is used to avoid re-deploying the contracts before running the tests.
