# PauserSet contract

This section describes the `PauserSet` contract. It is used to manage the pausers, addresses that are allowed to pause some of the Gateway contracts.

## Pausers

Pausers are account addresses registered in the `PauserSet` contract. They are expected to be hot wallets controlled by the [operators](./gateway_config.md#operators) of the fhevm Gateway protocol.

They are allowed to pause some of the Gateway contracts. See [Pausing](../pausing/pausing.md) for more details.

## PauserSet

The `PauserSet` contract allows to:

- add a pauser: `addPauser`
- remove a pauser: `removePauser`
- swap a pauser with another address: `swapPauser`
- check if an address is a pauser: `isPauser`

It is deployed as an immutable contract and its address is stored in the `GatewayConfig` contract.
