# Pausing mechanism

This section describes the pausing mechanism for the fhevm Gateway protocol.

It is meant to be used in case of emergency, to pause the protocol in order to prevent any new requests (input proof, decryptions) to be processed. The Gateway can only be paused and unpaused manually.

It is handled through the following contracts:

- `PauserSet`: manages the pauser addresses (see [PauserSet](../contracts/pauser_set.md))
- `Pausable` (abstract): provides the pausing and unpausing functionalities

## Pausing contracts

A pauser is an account that can pause the following Gateway contracts:

- `Decryption`
- `InputVerification`

A paused contract means that any transaction sent to trigger its _request_ functions will be reverted. This means responses for already-sent requests will still be accepted and on-going consensus can be reached. Additionally, other view functions will still be callable.

### Decryption contract

When paused, the following functions will be reverted:

- `publicDecryptionRequest`
- `userDecryptionRequest`
- `delegatedUserDecryptionRequest`

### InputVerification contract

When paused, the following functions will be reverted:

- `verifyProofRequest`

## Hardhat tasks

It is possible to pause and unpause the contracts through hardhat tasks defined in [pauseContracts.ts](../../../tasks/pauseContracts.ts).

### Pausing tasks

Pausing tasks are:

- `pauseAllGatewayContracts`: pause all the Gateway contracts
- `pauseInputVerification`: pause the `InputVerification` contract only
- `pauseDecryption`: pause the `Decryption` contract only

These tasks require the `PAUSER_PRIVATE_KEY` environment variable to be set locally. See [Environment variables](./env_variables.md) for more details.

### Unpausing tasks

Unpausing tasks are:

- `unpauseAllGatewayContracts`: unpause all the Gateway contracts
- `unpauseInputVerification`: unpause the `InputVerification` contract only
- `unpauseDecryption`: unpause the `Decryption` contract only

**Important**: These tasks are only possible if the ownership of the `GatewayConfig` contract is handled by the deployer, which requires the `DEPLOYER_PRIVATE_KEY` environment variable to be set locally (see [Environment variables](./env_variables.md)). Once the `GatewayConfig` contract is owned by a multi-sig owner, these tasks will not be able to unpause the contracts. Instead, `GatewayConfig` contract will need to be unpaused by calling its `unpause` function through this multi-sig contract.
