# Foundry

This guide explains how to use Foundry with FHEVM for developing smart contracts.

While a Foundry template is currently in development, we strongly recommend using the [Hardhat template](getting-started/quick-start-tutorial/setup.md) for now, as it provides a fully tested and supported development environment for FHEVM smart contracts.

{% hint style="warning" %}
Foundry does not natively support the FHEVM coprocessor. The mock-based approach described below does **not** replicate the full behavior of a real FHEVM node — encrypted operations run as plaintext stubs, so tests may pass locally but fail on a live network.
{% endhint %}

If you still want to use Foundry, you can compile against a mocked version of the FHE library. To do this, change your `FHE.sol` imports from `@fhevm/solidity/lib/FHE.sol` to `fhevm/mocks/FHE.sol` in your Solidity source files.
