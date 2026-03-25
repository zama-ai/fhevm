<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/.gitbook/assets/fhevm-header-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="docs/.gitbook/assets/fhevm-header-light.png">
  <img width=500 alt="fhevm">
</picture>
</p>

<hr/>

<p align="center">
  <a href="fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/protocol"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm?style=flat-square"></a>
  <a href="https://github.com/zama-ai/fhevm/blob/main/LICENSE">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
  <a href="https://slsa.dev"><img alt="SLSA 3" src="https://slsa.dev/images/gh-badge-level3.svg" /></a>
</p>


## About

### What is FHEVM?

**FHEVM** is the core framework of the *Zama Confidential Blockchain Protocol*. It enables confidential smart contracts on EVM-compatible blockchains by leveraging Fully Homomorphic Encryption (FHE), allowing encrypted data to be processed directly onchain.

FHEVM ensures both confidentiality and composability, with the following guarantees:
- **End-to-end encryption of transactions and state:** Data included in transactions is encrypted and never visible to anyone.
- **Composability and data availability on-chain:** States are updated while remaining encrypted at all times.
- **No impact on existing dApps and state:** Encrypted state co-exists alongside public one, and doesn't impact existing dApps.
<br></br>

### Table of contents

- [About](#about)
  - [What is FHEVM?](#what-is-fhevm)
  - [Project structure](#project-structure)
  - [Main features](#main-features)
  - [Use cases](#use-cases)
- [Resources](#resources)
- [Working with FHEVM](#working-with-fhevm)
  - [Citations](#citations)
  - [Contributing](#contributing)
  - [License](#license)
  - [FAQ](#faq)
- [Support](#support)
  <br></br>
### Project structure
The directories of this repository are organized in the following way:

###### FHEVM Contracts

- **`gateway-contracts/`**: Smart contracts managing the gateway between on-chain and off-chain components.

- **`host-contracts/`**: Smart Contracts deployed on the host chain for orchestrating FHE workflows.

###### FHEVM Compute Engines

- **`coprocessor/`**: Rust-based coprocessor implementation for FHE operations.

- **`kms-connector/`**: Interface for integrating with Key Management Services (KMS) to handle encryption keys securely.

###### FHEVM Utilities
- **`charts/`**: Helm charts and deployment configurations for the stack.

- **`golden-container-images/`**: Docker golden images for Node.js and Rust environments used as base images by the stack.

- **`test-suite/`**: Integration with docker-compose and tests covering end-to-end FHEVM stack behavior.



  <br></br>
### Main features

- **Privacy by design:** Building decentralized apps with full privacy and confidentiality on Ethereum, leveraging FHE.
- **Solidity integration:** Write FHEVM contracts like any standard Solidity contract using Solidity. Compatible with existing toolchains — such as Hardhat and Foundry (*coming soon*).
- **Programmable privacy:**  Define exactly what data is encrypted and write the access control logic directly in your smart contracts.
- **High precision encrypted integers :** Up to 256 bits of precision for integers.
- **Full range of operators:** All typical operators are available: `+`, `-`, `*`, `/`, `<`, `>`, `==`, ternary-if, boolean operations…. Consecutive FHE operations are not limited.
- **Security:** The underlying FHE crypto-scheme of FHEVM is quantum-resistant. Decryption is managed via a key management system (KMS) using multi-party computation (MPC), ensuring security even if some parties are compromised or misbehaving.
- **Symbolic execution of FHE computations:** All FHE operations are executed symbolically on the host chain, significantly reducing execution time. The actual computations on encrypted data are offloaded asynchronously to our coprocessor, allowing for faster, efficient, and scalable processing.

_Learn more about FHEVM features in the [documentation](https://docs.zama.ai/protocol) and in our [whitepaper](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper.pdf)._
<br></br>

### Use cases

FHEVM is built for developers to write confidential smart contracts without the need to learn cryptography. Leveraging FHEVM, you can unlock a myriad of new use cases such as DeFi, gaming, and more. For instance:

- **Confidential transfers**: Keep balances and amounts private, without using mixers.
- **Tokenization**: Swap tokens and RWAs on-chain without others seeing the amounts.
- **Blind auctions**: Bid on items without revealing the amount or the winner.
- **On-chain games**: Keep moves, selections, cards, or items hidden until ready to reveal.
- **Confidential voting**: Prevents bribery and blackmailing by keeping votes private.
- **Encrypted DIDs**: Store identities on-chain and generate attestations without ZK.

_Learn more use cases in the [list of examples](https://docs.zama.ai/protocol/examples)._
<br></br>


## Resources
- [Documentation](https://docs.zama.ai/protocol) — Official documentation of FHEVM.
- [Whitepaper](./fhevm-whitepaper.pdf) — Technical overview of FHEVM's cryptographic design.
- [Examples](https://docs.zama.ai/protocol/examples) — Examples of building confidential smart contracts.
- [Awesome Zama – FHEVM](https://github.com/zama-ai/awesome-zama?tab=readme-ov-file#fhevm) — Curated articles, talks, and ecosystem projects.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

## Working with FHEVM
### Citations

To cite FHEVM or the whitepaper in academic papers, please use the following entries:

```text
@Misc{FHEVM,
title={{FHEVM: A full-stack framework for integrating Fully Homomorphic Encryption (FHE) with blockchain applications},
author={Zama},
year={2025},
note={\url{https://github.com/zama-ai/fhevm}},
}
```

### Contributing

There are two ways to contribute to FHEVM:

- [Open issues](https://github.com/zama-ai/fhevm/issues/new/choose) to report bugs and typos, or to suggest new ideas
- Request to become an official contributor by emailing hello@zama.ai.

Becoming an approved contributor involves signing our Contributor License Agreement (CLA). Only approved contributors can send pull requests, so please make sure to get in touch before you do!
<br></br>

### License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE) for more details.

### FAQ

**Is Zama’s technology free to use?**

> Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
> Everything we do is open source, and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**

> To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us at hello@zama.ai for more information.

**Do you file IP on your technology?**

> Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**

> We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/.gitbook/assets/support-banner-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="docs/.gitbook/assets/support-banner-light.png">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>
