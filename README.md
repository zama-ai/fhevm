<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/b07e7e65-12b2-4048-b5de-35e169ed96e4">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/c0fab5b1-adef-4db4-9607-fa0a793acaf8">
  <img width=600 alt="Zama fhEVM">
</picture>
</p>

<hr/>

<p align="center">
  <a href="fhevm-whitepaper-v2.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
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

### What is fhEVM

**fhEVM** is a technology that enables confidential smart contracts on the EVM using fully homomorphic encryption (FHE).

Thanks to a breakthrough in homomorphic encryption, Zama’s fhEVM makes it possible to run confidential smart contracts on encrypted data, guaranteeing both confidentiality and composability with:

- **End-to-end encryption of transactions and state:** Data included in transactions is encrypted and never visible to anyone.
- **Composability and data availability on-chain:** States are updated while remaining encrypted at all times.
- **No impact on existing dapps and state:** Encrypted state co-exists alongside public one, and doesn't impact existing dapps.
  <br></br>

### Main features

- **Solidity integration:** fhEVM contracts are simple solidity contracts that are built using traditional solidity toolchains.
- **Simple developer experience:** Developers can use the `euint` data types to mark which part of their contracts should be private.
- **Programmable privacy:** All the logic for access control of encrypted states is defined by developers in their smart contracts.
- **High precision encrypted integers :** Up to 256 bits of precision for integers
- **Full range of operators :** All typical operators are available: `+`, `-`, `*`, `/`, `<`, `>`, `==`, …
- **Encrypted if-else conditionals :** Check conditions on encrypted states
- **On-chain PRNG :** Generate secure randomness without using oracles
- **Configurable decryption :** Threshold, centralized or KMS decryption
- **Unbounded compute Depth :** Unlimited consecutive FHE operations

_Learn more about fhEVM features in the [documentation](https://docs.zama.ai/fhevm)._
<br></br>

### Use cases

fhEVM is built for developers to write confidential smart contracts without learning cryptography. Leveraging fhEVM, you can unlock a myriad of new use cases such as DeFI, gaming, and more. For instance:

- **Tokenization**: Swap tokens and RWAs on-chain without others seeing the amounts.
- **Blind auctions**: Bid on items without revealing the amount or the winner.
- **On-chain games**: Keep moves, selections, cards, or items hidden until ready to reveal.
- **Confidential voting**: Prevents bribery and blackmailing by keeping votes private.
- **Encrypted DIDs**: Store identities on-chain and generate attestations without ZK.
- **Private transfers**: Keep balances and amounts private, without using mixers.

_Learn more use cases in the [list of examples](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)._
<br></br>

## Table of Contents

- **[Getting Started](#getting-started)**
  - [Installation](#installation)
  - [A Simple Example](#a-simple-example)
- **[Resources](#resources)**
  - [White paper](#white-paper)
    [Demos](#demos-and-tutorials)
  - [Documentation](#documentation)
  - [Blockchain Implementation](#blockchain-implementation)
- **[Working with fhEVM](#working-with-fhevm)**
  - [Developer guide](#developer-guide)
  - [Citations](#citations)
  - [Contributing](#contributing)
  - [License](#license)
- **[Support](#support)**
  <br></br>

## Getting Started

### Installation

```bash
# Using npm
npm install fhevm

# Using Yarn
yarn add fhevm

# Using pnpm
pnpm add fhevm
```

### A Simple Example

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import "fhevm/config/ZamaFHEVMConfig.sol";

contract Counter is SepoliaZamaFHEVMConfig {
  euint8 internal counter;

  constructor() {
    counter = TFHE.asEuint8(0);
    TFHE.allowThis(counter);
  }

  function add(einput valueInput, bytes calldata inputProof) public {
    euint8 value = TFHE.asEuint8(valueInput, inputProof);
    counter = TFHE.add(counter, value);
    TFHE.allowThis(counter);
  }
}
```

_More examples are available [here](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)._

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

> [!Note] > **Zama 5-Question Developer Survey**
>
> We want to hear from you! Take 1 minute to share your thoughts and helping us enhance our documentation and libraries. 👉 **[Click here](https://www.zama.ai/developer-survey)** to participate.

## Resources

### **White Paper**

Gain insights into the technology powering fhEVM with our in-depth white paper:  
👉 [**Confidential EVM Smart Contracts using Fully Homomorphic Encryption**](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper-v2.pdf)

---

### **Demos and Tutorials**

Access a curated collection of demos and step-by-step tutorials to guide your development journey:
🔗 [**Visit the Tutorials Page**](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)

---

### **Documentation**

Master `fhEVM` and build smart contracts using these resources:

- 📘 [**Comprehensive fhEVM Documentation**](https://docs.zama.ai/fhevm)  
  Dive deep into Zama's detailed guide for utilizing the full potential of fhEVM.

- 🤖 [**ZAMA Solidity Developer (Modified ChatGPT Model)**](https://chatgpt.com/g/g-67518aee3c708191b9f08d077a7d6fa1-zama-solidity-developer)  
  Accelerate your smart contract development with AI-powered assistance.

### **Development templates**

Start building faster with pre-configured templates tailored for various frameworks:

#### **Smart Contracts**

- 🔧 [**Hardhat Template**](https://github.com/zama-ai/fhevm-hardhat-template)  
  Smart contracts testing and development <- **_primary entry point for developers wanting to develop smart contracts on fhEVM_**
- 💻 [**fhEVM Contracts**](https://github.com/zama-ai/fhevm-contracts)  
  Library of standardized fhEVM contracts.

#### **Frontend frameworks**

- 🌐 [**React.js Template**](https://github.com/zama-ai/fhevm-react-template)  
  Simplify your FHE dApp development with a clean and optimized React.js template.
- ⚡ [**Next.js Template**](https://github.com/zama-ai/fhevm-next-template)  
  Build server-rendered, scalable dApps with FHE support using this Next.js template.
- 🖼️ [**Vue.js Template**](https://github.com/zama-ai/fhevm-vue-template)  
  Create modular, responsive dApps with FHE capabilities using Vue.js.

---

### 🚀 **Kickstart Your Project Today!**

Leverage these templates to accelerate your development process and bring your ideas to life faster.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

## Blockchain Implementation

To integrate fhevm-go into any EVM-based blockchain, you can follow the [Integration Guide](https://docs.zama.ai/fhevm-go/getting_started/integration).

## Working with fhEVM

### Developer Guide

Install dependencies (Solidity libraries and dev tools)

```bash
npm install
```

> [!Note]
> Solidity files are formatted with prettier.

#### Generate TFHE lib

```
npm run codegen
```

> [!Warning]
> Use this command to generate Solidity code and prettier result automatically!

Files that are generated now (can be seen inside `codegen/main.ts`)

```
lib/Impl.sol
lib/TFHE.sol
contracts/tests/TFHETestSuiteX.sol
test/tfheOperations/tfheOperations.ts
```

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

#### Adding new operators

Operators can be defined as data inside `codegen/common.ts` file and code automatically generates solidity overloads.
Test for overloads must be added (or the build doesn't pass) inside `codegen/overloadsTests.ts` file.

### Citations

To cite fhEVM or the whitepaper in academic papers, please use the following entries:

```text
@Misc{fhEVM,
title={{Confidential EVM Smart Contracts using Fully Homomorphic Encryption}},
author={Zama},
year={2024},
note={\url{https://github.com/zama-ai/fhevm}},
}
```

```text
@techreport{fhEVM,
author = "Zama",
title = "Confidential EVM Smart Contracts using Fully Homomorphic Encryption",
institution = "Zama",
year = "2024"
}
```

### Contributing

There are two ways to contribute to the Zama fhEVM:

- [Open issues](https://github.com/zama-ai/fhevm/issues/new/choose) to report bugs and typos, or to suggest new ideas
- Request to become an official contributor by emailing hello@zama.ai.

Becoming an approved contributor involves signing our Contributor License Agreement (CLA)). Only approved contributors can send pull requests, so please make sure to get in touch before you do!
<br></br>

### License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE) for more details.

#### FAQ

**Is Zama’s technology free to use?**

> Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
> Everything we do is open source and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**

> To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us at hello@zama.ai for more information.

**Do you file IP on your technology?**

> Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**

> We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.

<p align="right">
  <a href="#table-of-contents" > ↑ Back to top </a>
</p>

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/e249e1a8-d724-478c-afa8-e4fe01c1a0fd">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/a72200cc-d93e-44c7-81a8-557901d8798d">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>
