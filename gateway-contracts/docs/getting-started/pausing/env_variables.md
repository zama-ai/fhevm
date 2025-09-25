# Environment variables

This section describes the environment variables used for pausing the FHEVM gateway.

In the following, the values are given as examples. Most of them are from the `.env.example` file and are used for local testing. The accounts found in it are already-funded hardhat accounts generated with the following command:

```bash
make get-accounts
```

## Summary

Here's the complete list of environment variables used for pausing the FHEVM gateway. More detailed information can be found in [this section](#in-details) below. Solidity types are defined in [Solidity's documentation](https://docs.soliditylang.org/en/latest/types.html).

| Environment Variable   | Description                       | Solidity Type | Default | Comment |
| ---------------------- | --------------------------------- | ------------- | ------- | ------- |
| `PAUSER_PRIVATE_KEY`   | Private key of one of the pausers | bytes32       | -       | -       |
| `DEPLOYER_PRIVATE_KEY` | Private key of the deployer       | bytes32       | -       | -       |

## In details

- Pauser private key:

```bash
PAUSER_PRIVATE_KEY="0x3588ffb4f4d9bea785a012b895543fe68f2d580a9d449decc91a25878064079a" # (bytes32)
```

The pauser private key is the private key of one of the pausers registered in the `PauserSet` contract. It is used to pause the contracts using the pausing hardhat tasks, including the `pauseAllGatewayContracts`. Each operator are expected to set this environment variable to the private key associated to their hot wallet used as pauser.

- Deployer private key

```bash
DEPLOYER_PRIVATE_KEY="0x7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f" # (bytes32)
```

This is the private key of the deployer account, used to deploy the contracts (see [Deployment](../deployment/env_variables.md)). If the ownership of the `GatewayConfig` contract is handled by the deployer, it can be used to unpause the contracts (see [Pausing](./pausing.md)).
