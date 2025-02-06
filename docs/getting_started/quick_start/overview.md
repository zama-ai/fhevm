# Quick Start

This tutorial guides you to start quickly with Zama’s **Fully Homomorphic Encryption (FHE)** technology for building confidential smart contracts.

## What you'll learn

In around 20 minutes, you will build and deploy your first confidential ERC20 contract, mint tokens and perform transactions on it.

This tutorial will cover:

1. **Set up Remix** development environment using the Zama Plugin.
2. **Connect your wallet** for deployment and interaction.
3. **Write and deploy** `ConfidentialERC20Mintable` contract on the Sepolia testnet
4. **Interact with your contract** to mint, transfer, and manage confidential tokens.

## What you'll build

The contract that you will build with this tutorial is called `ConfidentialERC20Mintable` — a privacy-preserving ERC20 implementation that leverages **FHE** to keep balances and transactions confidential. To understand this contract, let’s first introduce the foundational concepts.

{% hint style="info" %} Feel free to skip this part if you already have a basic knowledge of confidential ERC20 tokens.{% endhint %}

**ERC20**

ERC20 is a widely used token standard on Ethereum that defines a set of rules for creating and managing fungible tokens.

These tokens are efficient but lack privacy — balances and transactions are visible to anyone on the blockchain.

**Confidential ERC20**

Zama’s `ConfidentialERC20` introduces privacy to ERC20 tokens by storing balances and transactions in an encrypted format using FHE.

The `ConfidentialERC20` contract still supports standard ERC20 functions such as `transfer`, `approve`, `transferFrom`, `balanceOf`, and `totalSupply` but ensures these operations are processed securely with encrypted data.

To explore the implementation details of ConfidentialERC20, check out the [Zama blog post](https://www.zama.ai/post/confidential-erc-20-tokens-using-homomorphic-encryption).

**Confidential ERC-20 Mintable**

The contract that we will build in this tutorial is  `ConfidentialERC20Mintable` . It's built on top of `ConfidentialERC20` by adding secure minting capabilities. This allows authorized accounts to create new tokens, while maintaining the privacy guarantees of encrypted balances and transactions.

The `ConfidentialERC20Mintable` contract ensures:

- **Enhanced privacy**: Balances are stored as encrypted values (`euint64`), preventing public inspection of account balances.
- **Secure transactions**: Token transfers are processed securely, maintaining confidentiality of amounts.
- **Owner visibility**: Only account owners can decrypt and view their balances.

Let’s get started on your journey to building confidential blockchain applications! 
