# Deploy contracts locally

Here is an example of how to deploy contracts on a local network using hardhat.

## Prerequisites

First, git clone `FHEVM-gateway` repo and install dependencies with

```bash
npm install
```

## Environment variables

All needed environment variables are defined in the [Environment variables](./env_variables.md) documentation.

For deploying the FHEVM gateway contracts, these variables need to be set in an `.env` file. However, the following `make` commands automatically copy the `.env.example` file to `.env` and update it with the correct values:

**Important**: By default, the accounts used are already funded. If other addresses are used, make sure they are funded as well.

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

- deploys the contracts through the `deployAllGatewayContracts` task found in [deploy.ts](../../../tasks/deployment/contracts.ts)
- adds the host chains to the `GatewayConfig` contract through the `addHostChainsToGatewayConfig` task found in [addHostChains.ts](../../../tasks/addHostChains.ts)

## Manage host chains

Once a host chain is registered, its lifecycle is managed through three owner-only tasks found in
[manageHostChains.ts](../../../tasks/manageHostChains.ts). Each task signs with `DEPLOYER_PRIVATE_KEY`
and resolves the `GatewayConfig` address the same way as `addHostChainsToGatewayConfig` — from the
`GATEWAY_CONFIG_ADDRESS` env var by default, or from the `addresses/` directory when
`--use-internal-proxy-address true` is passed (local / CI flow):

```bash
# Disable a registered host chain (it stays registered but is flagged disabled)
npx hardhat task:disableHostChainOnGatewayConfig --chain-id <id>

# Re-enable a previously disabled host chain
npx hardhat task:enableHostChainOnGatewayConfig --chain-id <id>

# Remove a host chain entirely (it must be disabled first)
npx hardhat task:removeHostChainOnGatewayConfig --chain-id <id>
```

Each task runs a pre-flight check (chain registered / disabled state) so the operator gets a clean
error before submitting a transaction that would otherwise revert on-chain.

## Run tests

Run all tests on the local network:

```bash
make test-local
```

A `--skip-setup` flag is used to avoid re-deploying the contracts before running the tests.
