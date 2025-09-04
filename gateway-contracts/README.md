## Introduction

The **FHEVM Gateway** is a set of smart contracts that enables decrypting FHE ciphertexts from different host chains. It acts as an intermediary between the chains, the Key Management Service (KMS) and the coprocessors found within the FHEVM protocol. These contracts are responsible for:

- Verifying the legitimacy of decryption requests
- Centralizing multi-chain ciphertexts access
- Centralizing multi-chain ciphertexts commitments
- Orchestrating KMS materials

## Main features

| Contract            | Description                                                 | Features                                                                                                                          |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `Decryption`        | Decrypt FHE ciphertexts                                     | - Request a public decryption<br>- Request a user decryption<br>- Request a delegated user decryption                             |
| `InputVerification` | Verify an input's zero-knowledge proof of knowledge (ZKPoK) | - Verify a ZKPoK<br>- Reject a ZKPoK                                                                                              |
| `MultichainAcl`     | Centralize Access Control Lists (ACL) from all host chains  | - Grant account access to ciphertexts<br>- Authorize public decryption of ciphertexts<br>- Delegate account access to ciphertexts |
| `CiphertextCommits` | Store ciphertext commitments from all host chains           | - Store regular ciphertext commitments<br>- Store Switch and Squash (SNS) ciphertext commitments                                  |
| `KmsManagement`     | Orchestrate KMS-related materials                           | 🚧 _Not in use yet_ 🚧                                                                                                            |
| `GatewayConfig`     | Administer configuration settings                           | - Register KMS nodes, coprocessors and host chains. <br> - Update KMS nodes, coprocessors and host chains.                        |

## Getting started

Documentation can be found [here](./docs/SUMMARY.md).

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
