# Quick Start

This tutorial guides you to start quickly with Zama’s **Fully Homomorphic Encryption (FHE)** technology for building confidential smart contracts.&#x20;

## What You’ll Learn

In about 20 minutes, you will:

- Build your first **confidential ERC20** contract that leverages FHE.
- Deploy the contract on the **Sepolia** Network.
- **Mint tokens** and **perform transactions** in FHE.
- Build a **frontend application** for your contract.

## Prerequisite

- A basic understanding of **Solidity** library and **Ethereum**.
- A certain amount of **Sepolia ETH** available.
  - &#x20;If you don’t have enough ETH, use a Sepolia faucet to request free SepoliaETH for testing such as [Alchemy Faucet](https://www.alchemy.com/faucets/ethereum-sepolia) or [QuickNode Faucet](https://faucet.quicknode.com/ethereum/sepolia).

## What is Confidenetial ERC20

The contract that you will build with this tutorial is called `ConfidentialERC20Mintable` — a privacy-preserving ERC20 implementation that leverages **FHE** to keep balances and transactions confidential. To understand this contract, let’s first introduce the foundational concepts.

**RC20**

ERC20 is a widely used token standard on Ethereum that defines a set of rules for creating and managing fungible tokens. These tokens are efficient but lack privacy — balances and transactions are visible to anyone on the blockchain.

**Confidential ERC20**

Zama’s `ConfidentialERC20` introduces privacy to ERC20 tokens by storing balances and transactions in an encrypted format using FHE.

The `ConfidentialERC20` contract still supports standard ERC20 functions such as `transfer`, `approve`, `transferFrom`, `balanceOf`, and `totalSupply` but ensures these operations are processed securely with encrypted data.

To explore the implementation details of ConfidentialERC20, check out the [Zama blog post](https://www.zama.ai/post/confidential-erc-20-tokens-using-homomorphic-encryption).

**Confidential ERC-20 Mintable**

The contract that we will build in this tutorial is `ConfidentialERC20Mintable` . It's built on top of `ConfidentialERC20` by adding secure minting capabilities. This allows authorized accounts to create new tokens, while maintaining the privacy guarantees of encrypted balances and transactions.

The `ConfidentialERC20Mintable` contract ensures:

- **Enhanced privacy**: Balances are stored as encrypted values (`euint64`), preventing public inspection of account balances.
- **Secure transactions**: Token transfers are processed securely, maintaining confidentiality of amounts.
- **Owner visibility**: Only account owners can decrypt and view their balances.

## Next steps

Choose your path and get started:

- [**Remix Guide**](remix) – Rapid in‐browser setup, great for **learning** and fast **prototyping**.
- [**Hardhat Guide**](hardhat) – Full-fledged development environment, suitable for **production**.
