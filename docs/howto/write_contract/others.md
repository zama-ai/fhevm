# Other development environment

> Our library TFHE requires Solidity version **0.8.19** specifically, as we rely on features exclusive to this version and do not currently provide support for versions beyond it.

Our library compiles seamlessly with the traditional Solidity compiler and is generally compatible with traditional Solidity tools. However, it's important to note that the execution is designed to function exclusively on a fhEVM. As a result, this library is not intended for deployment on a classic EVM, like the one used in tools like anvil, Ganache or testnet like Sepolia.

## Foundry

The fhEVM does not work with Foundry as Foundry employs its own EVM, preventing us from incorporating a mock for our precompiled contract. An [ongoing discussion](https://github.com/foundry-rs/foundry/issues/5576) is exploring the possibility of incorporating a plugin system for precompiles, which could potentially pave the way for the utilization of Foundry at a later stage.
