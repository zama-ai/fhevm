<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/b07e7e65-12b2-4048-b5de-35e169ed96e4">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/c0fab5b1-adef-4db4-9607-fa0a793acaf8">
  <img width=600 alt="Zama fhEVM">
</picture>
</p>

<hr/>

<p align="center">
  <a href="./fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
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
  - [Demos](#demos)
  - [Tutorials](#tutorials)
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

For now, fhEVM is implemented on evmos.

```bash
# Using npm
npm install fhevm

# Using Yarn
yarn add fhevm

# Using pnpm
pnpm add fhevm
```

_Find more details on implementation instructions in [this repository](https://github.com/zama-ai/fhevm-evmos)._
<br></br>

### A Simple Example

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";

contract Counter {
  euint32 counter;

  function add(einput valueInput, bytes calldata inputProof) public {
    euint32 value = TFHE.asEuint32(valueInput, inputProof);
    counter = TFHE.add(counter, value);
    TFHE.allow(counter, address(this));
  }
}
```

_More examples are available [here](https://github.com/zama-ai/fhevm/tree/main/examples)._

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

## Resources

### White paper

- [Confidential EVM Smart Contracts using Fully Homomorphic Encryption](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper.pdf)
  <br></br>

### Demos

#### Finance

- [ERC-20](https://github.com/zama-ai/fhevm/blob/main/examples/EncryptedERC20.sol): A variation of the standard ERC20 smart contract that incorporates encrypted balances, providing additional privacy for token holders.
- [Darkpool](https://github.com/omurovec/fhe-darkpools): A smart contract that enables anonymous trading of cryptocurrencies or assets, typically used to execute large orders without affecting the market price. - by [Owen Murovec](https://github.com/omurovec)

#### Games:

- [Cipherbomb](https://github.com/immortal-tofu/cipherbomb): A Hardhat-based template for developing Solidity smart contracts, with sensible defaults. - by Clément Danjou
- [Battleship](https://github.com/battleship-fhevm/battleship-hardhat): A smart contract that replicates the classic Battleship game on a blockchain in a transparent manner. - by [Owen Murovec](https://github.com/omurovec)

#### Others

- [Governor DAO](https://github.com/zama-ai/fhevm/tree/main/examples/Governor): A DAO smart contract that facilitates governance decisions through encrypted voting.
- [Blind auction](https://github.com/zama-ai/fhevm/blob/main/examples/BlindAuction.sol): A smart contract for conducting blind auctions where bids are encrypted and the winning bid remains private.
- [Decentralized ID](https://github.com/zama-ai/fhevm/tree/main/examples/Identity): A blockchain-based identity management system using smart contracts to store and manage encrypted personal data.

_If you have built awesome projects using fhEVM, please let us know and we will be happy to showcase them here!_
<br></br>

### Tutorials

- [[Video tutorial] How to Write Confidential Smart Contracts Using Zama's fhEVM](https://www.zama.ai/post/video-tutorial-how-to-write-confidential-smart-contracts-using-zamas-fhevm)
- [Confidential ERC-20 Tokens Using Homomorphic Encryption and the fhEVM](https://www.zama.ai/post/confidential-erc-20-tokens-using-homomorphic-encryption)
- [On-chain Blind Auctions Using Homomorphic Encryption and the fhEVM](https://www.zama.ai/post/on-chain-blind-auctions-using-homomorphic-encryption)
- [Programmable Privacy and Onchain Compliance using Homomorphic Encryption](https://www.zama.ai/post/programmable-privacy-and-onchain-compliance-using-homomorphic-encryption)

_Explore more useful resources in [fhEVM tutorials](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials) and [Awesome Zama repo](https://github.com/zama-ai/awesome-zama)._
<br></br>

### Documentation

Full, comprehensive documentation is available here: [https://docs.zama.ai/fhevm](https://docs.zama.ai/fhevm).

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

### Blockchain Implementation

To support fhEVM in an EVM-based blockchain, the **fhevm-go** library can be used as it implements all the needed FHE functionalities.
It is available here: [fhevm-go](https://github.com/zama-ai/fhevm-go)

To integrate fhevm-go into any EVM-based blockchain, you can follow the [Integration Guide](https://docs.zama.ai/fhevm-go/getting-started/integration).

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
mocks/Impl.sol
mocks/TFHE.sol
contracts/tests/TFHETestSuiteX.sol
test/tfheOperations/tfheOperations.ts
```

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

#### Tests

The easiest way to understand how to write/dev smart contract and interact with them using **fhevmjs** is to read and explore the available tests in this repository.

##### Fast start

```bash
# in one terminal
npm run fhevm:start
# in another terminal
npm i
cp .env.example .env
./scripts/faucet.sh
npm test
```

</details>

##### Docker

We provide a docker image to spin up a fhEVM node for local development.

```bash
npm run fhevm:start
# stop
npm run fhevm:stop
```

##### Faucet

For development purposes, we provide a ready to use wallet. In order to use
it, prepare the .env file that contains the mnemonic.

```bash
cp .env.example .env
```

This allows the developer to use a few accounts, each account can get coins:

```bash
npm run fhevm:faucet:alice
npm run fhevm:faucet:bob
npm run fhevm:faucet:carol
```

##### Run test

```bash
npm test
```

<details>
  <summary>Error: insufficient funds</summary>

Ensure the faucet command was successful.

</details>

##### Run tests for network1 network

Network1 doesn't support shanghai, so you should update the `evmVersion` [here](https://github.com/zama-ai/fhevm/blob/main/hardhat.config.ts#L170) to use `paris`, and make sure contracts are compiled using that version.

```bash
# codegen for network1 network
TARGET_NETWORK=Network1 npx ts-node codegen/main.ts && npm run prettier
# run tests for network1 network, assumes network1 rpc already running locally
npx hardhat test --network localNetwork1
```

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

#### Adding new operators

Operators can be defined as data inside `codegen/common.ts` file and code automatically generates solidity overloads.
Test for overloads must be added (or the build doesn't pass) inside `codegen/overloadsTests.ts` file.

#### Mocked mode

The mocked mode allows faster testing and the ability to analyze coverage of the tests. In this mocked version, encrypted types are not really encrypted, and the tests are run on the original version of the EVM, on a local hardhat network instance. To run the tests in mocked mode, you can use directly the following command:

```bash
npm run test:mock
```

To analyze the coverage of the tests (in mocked mode necessarily, as this cannot be done on the real fhEVM node), you can use this command :

```bash
npm run coverage:mock
```

Then open the file `coverage/index.html`. You can see there which line or branch for each contract which has been covered or missed by your test suite. This allows increased security by pointing out missing branches not covered yet by the current tests.

> [!Note]
> Due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhEVM, the main difference is the difference in gas prices for the FHE operations. This means that before deploying to production, developers still need to run the tests with the original fhEVM node, as a final check in non-mocked mode, with `npm run test`.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

### Citations

To cite fhEVM or the whitepaper in academic papers, please use the following entries:

```text
@Misc{fhEVM,
title={{Private smart contracts on the EVM using homomorphic encryption}},
author={Zama},
year={2023},
note={\url{https://github.com/zama-ai/fhevm}},
}
```

```text
@techreport{fhEVM,
author = "Morten Dahl, Clément Danjou, Daniel Demmler, Tore Frederiksen, Petar Ivanov,
Marc Joye, Dragos Rotaru, Nigel Smart, Louis Tremblay Thibault
",
title = "Confidential EVM Smart Contracts using Fully Homomorphic Encryption",
institution = "Zama",
year = "2023"
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
