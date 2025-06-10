## Introduction

**FHEVM Solidity** is a Solidity library that allows developers to write confidential smart contracts with the standard Solidity development workflow.

## Main features

The FHEVM Solidity library offers the following core features:

- **Encrypted types**: Use encrypted integers like `euint8`, `euint16`, ..., `euint256` directly in Solidity contracts.
- **Encrypted operations**: Perform operations on encrypted data using FHE-compatible functions like `add`, `sub`, `eq`, `lt`, `ternary`, etc.
- **Decryption functions**: Use `requestDecryption` to trigger decryption asynchronously via an oracle. The plaintext result is returned to the smart contract through a callback, eliminating the need for client-side processing.
- **Access control**: Restrict which accounts can decrypt or update values with programmable access logic.
- **Symbolic execution**: All encrypted operations are executed symbolically on the chain, with actual computation performed off-chain by the [coprocessor](../coprocessor/).

_See full details in the [Key concepts](https://docs.zama.ai/fhevm/smart-contract/key_concepts) section of the documentation._

## Get started

To start writing confidential smart contracts using FHEVM Solidity, follow the Hardhat setup guide here: [Getting Started with Hardhat](https://docs.zama.ai/fhevm/getting-started/overview-1/hardhat).

## Resources

- [Documentation](https://docs.zama.ai/fhevm/)
- [Contract examples](./examples/)

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../docs/.gitbook/assets/support-banner-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="../docs/.gitbook/assets/support-banner-light.png">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

[![GitHub stars](https://img.shields.io/github/stars/zama-ai/fhevm?style=social)](https://github.com/zama-ai/fhevm/)
