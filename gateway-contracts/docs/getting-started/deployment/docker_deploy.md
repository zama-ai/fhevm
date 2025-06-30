# Deploy contracts in Docker

Here is an example of how to deploy contracts on a local network using Docker.

## Environment variables

All needed environment variables are defined in the [Environment variables](./env_variables.md) documentation.

For deploying the fhevm gateway contracts, these variables need to be set in an `.env` file. However, the following `make` commands automatically copy the `.env.example` file to `.env` and update it with the correct values:

**Important**: By default, the accounts used are already funded. If other addresses are used, make sure they are funded as well.

## Docker deployment

### Build and start

Build the Docker image:

```sh
make docker-compose-build
```

Then start the containers:

```sh
make docker-compose-up
```

This should create three containers:

- `anvil-node`: A local Ethereum network with chain id `54321` on port `8546`
- `deploy-gateway-contracts`: Deploys the contracts
- `add-host-chains`: Adds the host chains to the `GatewayConfig` contract, which runs after the contracts are deployed

### Debug

To check if the deployment is successful, run the following commands:

```sh
docker logs deploy-gateway-contracts
```

If the logs show `Contract deployment done!`, the contracts were deployed successfully.

Then check:

```sh
docker logs add-host-chains
```

If the logs show `Host chains registration done!`, the host chains were registered successfully.

Both steps are required and should be run in this particular order before interacting with the contracts. In particular, if host chains are not registered properly, several transactions are expected to be reverted.

### Clean up

When finished, clean up the containers:

```sh
make docker-compose-down
```
