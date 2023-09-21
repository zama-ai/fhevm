# Getting Started

Welcome to the documentation for fhEVM Solidity Library! This comprehensive guide provides developers with detailed information on the library's functions, parameters, and usage examples. Explore how to leverage TFHE's powerful capabilities for computing over encrypted data within Solidity smart contracts, enabling secure computations and encrypted data manipulation. Unlock a new level of privacy and confidentiality in your blockchain applications with fhEVM Solidity Library.

## Installation

To get started with fhEVM Solidity Library, you need to install it as a dependency in your JavaScript project. You can do this using npm (Node Package Manager) or Yarn. Open your terminal and navigate to your project's directory, then run one of the following commands:

```bash
# Using npm
npm install fhevm

# Using Yarn
yarn add fhevm

# Using pnpm
pnpm add fhevm
```

This will download and install the fhEVM Solidity Library and its dependencies into your project.

## Quick start

{% tabs %}
{% tab title="Hardhat" %}
The best way to start writing smart contracts with fhEVM is to use our [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).

It allows you to start a fhEVM docker image and run your smart contract on it. Read the [README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md) for more information.
{% endtab %}

{% tab title="Remix IDE" %}
We developed a [version of Remix IDE](https://github.com/zama-ai/remix-project) to interact with a blockchain using fhEVM. You can use it on [https://remix.zama.ai](https://remix.zama.ai)
{% endtab %}

{% tab title="Docker" %}
We provide a docker image to spin up a fhEVM node for local development.

```bash
docker run -i -p 8545:8545 --rm --name fhevm ghcr.io/zama-ai/evmos-dev-node:v0.1.9
```

### Faucet

If you need to get coins for a specific wallet, you can use the faucet as follow:

```bash
docker exec -i fhevm faucet 0xa5e1defb98EFe38EBb2D958CEe052410247F4c80
```
{% endtab %}
{% endtabs %}
