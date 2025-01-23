# Foundry

This guide explains how to use Foundry with fhEVM for developing smart contracts.

While a Foundry template is currently in development, we strongly recommend using the [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template) for now, as it provides a fully tested and supported development environment for fhEVM smart contracts.

However, you could still use Foundry with the mocked version of the fhEVM, but please be aware that this approach is **NOT** recommended, since the mocked version is not fully equivalent to the real fhEVM node's implementation (see warning in [hardhat](hardhat.md)). In order to do this, you will need to rename your `TFHE.sol` imports from `fhevm/lib/TFHE.sol` to `fhevm/mocks/TFHE.sol` in your solidity source files.
