<p align="center">
<!-- product name logo -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/Zama-KMS-fhEVM-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="./assets/Zama-KMS-fhEVM-light.png">
  <img width=600 alt="Zama fhEVM & KMS">
</picture>
</p>

---

<p align="center">
  <a href="./fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program"><img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
</p>

## Table of Contents

- **[About](#about)**
  - [Purpose](#purpose)
  - [What is the Zama KMS for fhEVM](#what-is-the-zama-kms-for-fhevm)
  - [Implementation](#implementation)
- **[Running the demo](#running-the-demo)**
  - [fhEVM Native](#fhevm-native)
  - [fhEVM Coprocessor](#fhevm-coprocessor)
- **[Working with KMS](#working-with-kms)**
  - [Disclaimers](#disclaimers)
  - [Citations](#citations)
  - [License](#license)
  - [FAQ](#faq)
- **[Support](#support)**

## About

> [!Warning]
> This demo is an early beta version.

### Purpose

The purpose of this repository is to demonstrate the integration between fhEVM and fully dockerized KMS.

The KMS encompasses all sub-components: gateway, KMS blockchain and KMS core (that can run in centralized or threshold configuration). This is stil an early version with support for (asynchronous) decryption, and reencryption.

### What is the Zama KMS for fhEVM

The Zama KMS is a full key management solution for TFHE, more specifically [TFHE-rs](https://github.com/zama-ai/tfhe-rs), based on a maliciously secure and robust [MPC protocol](https://eprint.iacr.org/2023/815).

The system facilitates this through a the use of a blockchain which provides a means of fulfilling payments to the MPC parties, along with providing an immutable audit log.

Interaction with the same KMS will happen either through an external Ethereum blockchain (fhEVM), providing an API via a smart contract, or through a gateway service.

### Implementation

The KMS is implemented as a gRPC service using the [tonic](https://github.com/hyperium/tonic) crate.
Communication between full nodes and the KMS service is defined by [protobuf](/proto/kms.proto) messages.
The rest of the communication is defined by existing standards and uses JSON-RPC.
For the light client, we currently use CometBFT's [light](https://pkg.go.dev/github.com/cometbft/cometbft/light) package, which provides a service that connects to any CometBFT full node to serve trusted state roots on-demand.
The light client package handles the logic of sequentially verifying block headers.

<br></br>

## Running the demo

There are two versions of fhEVM:

- fhEVM native
- fhEVM coprocessor

### fhEVM Native

To run the demo for fhEVM native, switch [native directory](native).

### fhEVM Coprocessor

To run the demo for fhEVM native, switch [coprocessor directory](coprocessor).




## Working with KMS

### Disclaimers

#### Audits

The Zama KMS is not yet audited and should be considered in an early alpha stage. Known bugs and security issues are present as reflected by issue tracking.

#### Parameters

The default parameters for the Zama KMS are chosen to ensure a failure probability of 2^-64 and symmetric equivalent security of 132 bits.

#### Side-channel attacks

Mitigations for side-channel attacks have not been implemented directly in the Zama KMS. The smart contract of the blockchain from which calls originate is responsible to ensure the validity of calls. In particular that new ciphertexts are correctly constructed (through a proof-of-knowledge).

### Citations

To cite the KMS in academic papers, please use the following entry:

```
@Misc{zama-kms,
  title={{Zama KMS: A Pure Rust Implementation of a Threshold Key Management System for TFHE}},
  author={Zama},
  year={2024},
  note={\url{https://github.com/zama-ai/kms-core}},
}
```

### License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE.txt) for more details.

#### FAQ

**Is Zama’s technology free to use?**

> Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
> Everything we do is open source and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**

> To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us hello@zama.ai for more information.

**Do you file IP on your technology?**

> Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**

> We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.

<br></br>

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/08656d0a-3f44-4126-b8b6-8c601dff5380">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/1c9c9308-50ac-4aab-a4b9-469bb8c536a4">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.
