# Other development environment

## Foundry

The fhEVM does not work with Foundry as Foundry employs its own EVM, preventing us from incorporating a mock for our precompiled contract. An [ongoing discussion](https://github.com/foundry-rs/foundry/issues/5576) is exploring the possibility of incorporating a plugin system for precompiles, which could potentially pave the way for the utilization of Foundry at a later stage.

However, you could still use Foundry with the mocked version of the fhEVM, but please be aware that this approach is **NOT** recommended, since the mocked version is not fully equivalent to the real fhEVM node's implementation (see warning in [hardhat](hardhat.md)). In order to do this, you will need to rename your `TFHE.sol` imports from `fhevm/lib/TFHE.sol` to `fhevm/mocks/TFHE.sol` in your solidity source files.
