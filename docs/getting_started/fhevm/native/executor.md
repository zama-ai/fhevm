# Executor

An fhEVM-native node consists of the following components:
 * full node/validator node
 * Executor service

More detailed description of the architecture and FHE execution can be found in [Architecture](../../../fundamentals/fhevm/native/architecture.md) and [FHE Computation](../../../fundamentals/fhevm/native/fhe_computation.md).

The Executor service is a gRPC server that accepts FHE computation requests from the full node/validator node and executes them. It is implemented in the [executor](../../../../fhevm-engine/executor/README.md) directory of `fhevm-engine`.

At the time of writing, the [geth](geth.md) implementation is not yet implemented.

The Executor is almost fully functional. We don't yet provide Docker images for it, but it can be built as a normal Rust project.

