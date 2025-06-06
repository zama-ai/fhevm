# Foundry

This guide explains how to use Foundry with fhevm for developing smart contracts.

While a Foundry template is currently in development, we strongly recommend using the [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template) for now, as it provides a fully tested and supported development environment for fhevm smart contracts.

However, you could still use Foundry with the mocked version of the fhevm, but please be aware that this approach is **NOT** recommended, since the mocked version is not fully equivalent to the real fhevm node's implementation (see warning in hardhat). In order to do this, you will need to rename your `FHE.sol` imports from `fhevm/lib/FHE.sol` to `fhevm/mocks/FHE.sol` in your solidity source files.
