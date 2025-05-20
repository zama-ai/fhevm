<p align="center">
  <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program"><img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
</p>

## About

### What is it?

The fhevm Gateway is a set of smart contracts that enables decrypting FHE ciphertexts from different host chains. It
acts as an intermediary between the chains, the Key Management Service (KMS) and the coprocessors found within the fhevm
protocol. These contracts are responsible for:

- Verifying the legitimacy of decryption requests
- Centralizing multi-chain ciphertexts access
- Centralizing multi-chain ciphertexts commitments
- Orchestrating KMS materials

### Main features

| Contract            | Description                                                 | Features                                                                                                                          |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `Decryption`        | Decrypt FHE ciphertexts                                     | - Request a public decryption<br>- Request a user decryption<br>- Request a delegated user decryption                             |
| `InputVerification` | Verify an input's zero-knowledge proof of knowledge (ZKPoK) | - Verify a ZKPoK<br>- Reject a ZKPoK                                                                                              |
| `MultichainAcl`     | Centralize Access Control Lists (ACL) from all host chains  | - Grant account access to ciphertexts<br>- Authorize public decryption of ciphertexts<br>- Delegate account access to ciphertexts |
| `CiphertextCommits` | Store ciphertext commitments from all host chains           | - Store regular ciphertext commitments<br>- Store Switch and Squash (SNS) ciphertext commitments                                  |
| `KmsManagement`     | Orchestrate KMS-related materials                           | 🚧 _Not in use yet_ 🚧                                                                                                            |
| `GatewayConfig`     | Administer configuration settings                           | - Register KMS nodes, coprocessors and host chains. <br> - Update KMS nodes, coprocessors and host chains.                        |

## Getting Started

Documentation can be found [here](./docs/SUMMARY.md).

## Working with fhevm-gateway

### Citations

### Contributing

There are two ways to contribute to fhevm-gateway:

- [Open issues](https://github.com/zama-ai/fhevm-gateway/issues/new/choose) to report bugs and typos, or to suggest new
  ideas
- Request to become an official contributor by emailing hello@zama.ai.

Becoming an approved contributor involves signing our Contributor License Agreement (CLA). Only approved contributors
can send pull requests, so please make sure to get in touch before you do! <br></br>

### License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE) for more details.

#### FAQ

**Is Zama’s technology free to use?**

> Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and
> experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s
> commercial patent license.
>
> Everything we do is open source, and we are very transparent on what it means for our users, you can read more about
> how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**

> To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us at hello@zama.ai
> for more information.

**Do you file IP on your technology?**

> Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**

> We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email
> us at hello@zama.ai.

## Support
