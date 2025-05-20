<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/5d44c888-a30a-47a5-86b1-f1a10b58572d">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/f9f6b6e1-db81-4cb7-b962-181802576398">
  <img width=600 alt="fhevm">
</picture>
</p>

<hr/>

<p align="center">
  <a href="fhevm-whitepaper-v2.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm-solidity/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm?style=flat-square"></a>
  <a href="https://github.com/zama-ai/fhevm-solidity/blob/main/LICENSE">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
  <a href="https://slsa.dev"><img alt="SLSA 3" src="https://slsa.dev/images/gh-badge-level3.svg" /></a>
</p>

## About

### What is fhevm?

**fhevm** is a technology that enables confidential smart contracts on the EVM using fully homomorphic encryption (FHE).

Thanks to a breakthrough in homomorphic encryption, fhevm makes it possible to run confidential smart contracts on encrypted data, guaranteeing both confidentiality and composability with:

- **End-to-end encryption of transactions and state:** Data included in transactions is encrypted and never visible to anyone.
- **Composability and data availability on-chain:** States are updated while remaining encrypted at all times.
- **No impact on existing dApps and state:** Encrypted state co-exists alongside public one, and doesn't impact existing dApps.
  <br></br>

### Main features

- **Solidity integration:** fhevm contracts are simple solidity contracts that are built using traditional solidity tool-chains.
- **Simple developer experience:** Developers can use the `euint` data types to mark which part of their contracts should be private.
- **Programmable privacy:** All the logic for access control of encrypted states is defined by developers in their smart contracts.
- **High precision encrypted integers :** Up to 256 bits of precision for integers
- **Full range of operators :** All typical operators are available: `+`, `-`, `*`, `/`, `<`, `>`, `==`, …
- **Encrypted if-else conditionals :** Check conditions on encrypted states
- **On-chain PRNG :** Generate secure randomness without using oracles
- **Configurable decryption :** Threshold, centralized or KMS decryption
- **Unbounded compute Depth :** Unlimited consecutive FHE operations

_Learn more about fhevm features in the [documentation](https://docs.zama.ai/fhevm)._
<br></br>

### Use cases

FhEVM is built for developers to write confidential smart contracts without the need to learn cryptography. Leveraging fhevm, you can unlock a myriad of new use cases such as DeFi, gaming, and more. For instance:

- **Confidential transfers**: Keep balances and amounts private, without using mixers.
- **Tokenization**: Swap tokens and RWAs on-chain without others seeing the amounts.
- **Blind auctions**: Bid on items without revealing the amount or the winner.
- **On-chain games**: Keep moves, selections, cards, or items hidden until ready to reveal.
- **Confidential voting**: Prevents bribery and blackmailing by keeping votes private.
- **Encrypted DIDs**: Store identities on-chain and generate attestations without ZK.

_Learn more use cases in the [list of examples](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)._
<br></br>

> [!Note] > **Zama 5-Question Developer Survey**
> We want to hear from you! Take 1 minute to share your thoughts and help us enhance our documentation and libraries. 👉 **[Click here](https://www.zama.ai/developer-survey)** to participate.

### Citations

To cite fhevm or the whitepaper in academic papers, please use the following entries:

```text
@Misc{fhevm,
title={{Confidential EVM Smart Contracts using Fully Homomorphic Encryption}},
author={Zama},
year={2024},
note={\url{https://github.com/zama-ai/fhevm-solidity}},
}
```

### Contributing

There are two ways to contribute to fhevm:

- [Open issues](https://github.com/zama-ai/fhevm-solidity/issues/new/choose) to report bugs and typos, or to suggest new ideas
- Request to become an official contributor by emailing hello@zama.ai.

Becoming an approved contributor involves signing our Contributor License Agreement (CLA). Only approved contributors can send pull requests, so please make sure to get in touch before you do!
<br></br>

### License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE) for more details.

#### FAQ

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
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm-solidity/assets/157474013/e249e1a8-d724-478c-afa8-e4fe01c1a0fd">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm-solidity/assets/157474013/a72200cc-d93e-44c7-81a8-557901d8798d">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>
