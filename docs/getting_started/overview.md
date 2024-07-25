# What is fhEVM

<figure><img src="../.gitbook/assets/doc_header_fhevm.png" alt=""><figcaption></figcaption></figure>

## Introduction

There used to be a dilemma in blockchain: keep your application and user data on-chain, allowing everyone to see it, or keep it privately off-chain and lose contract composability.

**fhEVM** is a technology that enables confidential smart contracts on the EVM using Fully Homomorphic Encryption (FHE).

Thanks to a breakthrough in FHE, Zama’s fhEVM makes it possible to run confidential smart contracts on encrypted data, guaranteeing both confidentiality and composability with:

- **End-to-end encryption of transactions and state**: Data included in transactions is encrypted and never visible to anyone.
- **Composability and data availability on-chain**: States are updated while remaining encrypted at all times.
- **No impact on existing dapps and state**: Encrypted state co-exists alongside public one, and doesn't impact existing dapps.

## Main features

- **Solidity integration**: fhEVM contracts are simple solidity contracts that are built using traditional solidity toolchains.
- **Simple developer experience**: Developers can use the euint data types to mark which part of their contracts should be private.
- **Programmable privacy**: All the logic for access control of encrypted states is defined by developers in their smart contracts.
- **High precision encrypted integers** : Up to 256 bits of precision for integers
  -Full range of Operators : All typical operators are available: +, -, \*, /, <, >, ==, …
- **Encrypted `if-else` conditionals**: Check conditions on encrypted states
- **On-chain PRNG**: Generate secure randomness without using oracles
- **Configurable decryption**: Threshold, centralized or KMS decryption
- **Unbounded compute depth**: Unlimited consecutive FHE operations

## Use cases

- **Tokenization**: Swap tokens and RWAs on-chain without others seeing the amounts.
- **Blind auctions**: Bid on items without revealing the amount or the winner.
- **On-chain games**: Keep moves, selections, cards, or items hidden until ready to reveal.
- **Confidential voting**: Prevents bribery and blackmailing by keeping votes private.
- **Encrypted DIDs**: Store identities on-chain and generate attestations without ZK.
- **Private transfers**: Keep balances and amounts private, without using mixers.
