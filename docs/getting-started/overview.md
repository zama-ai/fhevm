# Overview

<figure><img src="../.gitbook/assets/doc_header_fhevm.png" alt=""><figcaption></figcaption></figure>

fhEVM is a suite of solutions that enables confidential smart contracts on the EVM using **Fully Homomorphic Encryption (FHE)**. This document provides a high-level overview of the fhEVM suite along with onboarding guidance tailored to specific audiences.

### For dApp developers

The fhEVM Protocol provides a **`TFHE` Solidity library** for building confidential smart contracts, a **`fhevm.js` Javascript library** to enable front‐end FHE interactions, and a range of developer tools, examples, and templates to streamline the usage for developers.&#x20;

#### Smart contract development

<table><thead><tr><th width="245">Repository</th><th width="580">Description</th></tr></thead><tbody><tr><td><a href="https://github.com/zama-ai/fhevm/">fhevm</a></td><td>Solidity library for FHE operations (e.g., encryption/decryption, arithmetic) within smart contracts.</td></tr><tr><td><a href="https://github.com/zama-ai/fhevm-hardhat-template">fhevm-hardhat-template</a></td><td>Hardhat template with scripts for compiling, deploying, and testing FHE‐enabled contracts.</td></tr><tr><td>fhevm-foundry-template - <em>coming soon</em></td><td>Foundry template for building FHE smart contracts.</td></tr><tr><td><a href="https://github.com/zama-ai/fhevm-contracts">fhevm-contracts</a></td><td>Ready‐to‐use FHE smart contract example covering finance, governance, and ERC‐20 tokens use cases.</td></tr></tbody></table>

#### Frontend development

<table><thead><tr><th width="252">Repository</th><th>Description</th></tr></thead><tbody><tr><td><a href="https://github.com/zama-ai/fhevmjs/">fhevmjs</a></td><td>JavaScript library for client‐side FHE, enabling encryption, decryption, and data handling.</td></tr><tr><td><a href="https://github.com/zama-ai/fhevm-react-template">fhevm-react-template</a></td><td>React.js template to quickly spin up FHE‐enabled dApps.</td></tr><tr><td><a href="https://github.com/zama-ai/fhevm-next-template">fhevm-next-template</a></td><td>Next.js template for integrating FHE in server‐side rendered or hybrid web apps.</td></tr><tr><td><a href="https://github.com/zama-ai/fhevm-vue-template">fhevm-vue-template</a></td><td>Vue.js template for creating privacy‐preserving dApps with encrypted data</td></tr></tbody></table>

#### Examples & Resources

<table><thead><tr><th width="258">Repository</th><th>Description</th></tr></thead><tbody><tr><td><a href="https://github.com/zama-ai/dapps">dapps</a></td><td>Sample decentralized applications demonstrating FHE with real‐world code.</td></tr><tr><td><a href="https://github.com/zama-ai/bounty-program">Zama Bounty Program</a></td><td>Explore open challenges and submit contributions to earn rewards.</td></tr><tr><td><a href="https://github.com/zama-ai/awesome-zama">Awesome Zama</a> </td><td>A curated list by the team at Zama of blog posts, libraries, research papers, and tutorials on Fully Homomorphic Encryption (FHE).</td></tr></tbody></table>

### For network builders

To **integrate FHE at the protocol level** or operate an **FHE‐enabled network**, fhEVM offers the fhevm backend modules. These repositories include the foundational implementations that enables FHE in blockchain systems, ensuring that privacy remains at the core of your network architecture.

<table><thead><tr><th width="260">Repository</th><th>Description</th></tr></thead><tbody><tr><td><a href="https://github.com/zama-ai/fhevm-backend">fhevm-backend</a></td><td>Rust backend &#x26; Go‐Ethereum modules, enabling native or coprocessor‐based FHE.</td></tr><tr><td><a href="https://github.com/zama-ai/fhevm-go/">fhevm-go</a></td><td>Go implementation of the FHE Virtual Machine</td></tr><tr><td><a href="https://github.com/zama-ai/zbc-go-ethereum/">zbc-go-ethereum</a></td><td>Modified go-ethereum with enhanced FHE support</td></tr></tbody></table>
