# Quick start tutorial: Build your first FHE smart contract

Welcome to the world of **Fully Homomorphic Encryption (FHE)** and privacy-preserving smart contracts! This tutorial will guide you step-by-step through deploying your first **confidential smart contract** using Zama’s groundbreaking FHE technology.

## What you'll build

You will build a confidentialerc20mintable <- improve this

### Encrypted balances

ConfidentialERC20 revolutionizes the way balances are stored by encrypting them using FHE.

- **Enhanced Privacy**: Balances are stored as encrypted values (`euint64`), ensuring no one can view account balances by inspecting the blockchain.
- **Exclusive Access**: Only the account owner can decrypt and view their own balance.
- **Encrypted Transactions**: Transaction amounts are also encrypted, maintaining confidentiality.

### Standard ERC20 functions with encryption

ConfidentialERC20 supports all the standard ERC20 functions, adapted for encrypted values. For example:

- `transfer`: Securely transfers encrypted tokens.
- `approve`: Approves encrypted amounts for spending.
- `transferFrom`: Transfers tokens on behalf of another address.
- `balanceOf`: Returns the encrypted balance of an account.
- `totalSupply`: Returns the encrypted total token supply.

To dive deeper into the workings of ConfidentialERC20, check out the [Zama blog post](https://www.zama.ai/post/confidential-erc-20-tokens-using-homomorphic-encryption).

## What you'll achieve

In just **~20 minutes**, you’ll:

1. [**Set up Remix**](./remix.md) – Configure your development environment to support FHE contracts.
2. [**Connect Your Wallet**](./connect_wallet.md) – Prepare for deployment by linking your crypto wallet.
3. [**Deploy ConfidentialERC20**](./deploying_cerc20.md) – Launch your first FHE-enabled token on the blockchain.
4. [**Interact with Your Contract**](./interact.md) – Use your deployed contract to mint, transfer, and manage confidential tokens.

### Why FHE smart contracts?

**FHE** enables computations on encrypted data without exposing sensitive information. With Zama’s **fhEVM**, developers can create privacy-first decentralized applications while preserving the integrity and confidentiality of data.

---

### Prerequisites

Before you begin, make sure you have:

- **Basic familiarity with Ethereum and smart contracts**
- **A web browser** (e.g., Chrome, Firefox)
- **A crypto wallet** (like MetaMask) configured for the Sepolia testnet

---

Let’s get started on your journey to building the future of confidential blockchain applications! 🚀
