# Genesis

## Contracts

For an fhEVM-native blockchain to operate and execute FHE computations, certain contracts need to be available when creating the chain - see [Contracts](../contracts.md). Strictly speaking, these contracts don't have to be available in the genesis block and can be deployed in the second block of the chain, at runtime.

## Keys

FHE-related keys need to available for the chain to operate properly. For example, a public FHE execution key is needed at the Executor to be able to compute on encrypted data.

As a convenience, the FHE public key can also be stored on validators/full nodes.
