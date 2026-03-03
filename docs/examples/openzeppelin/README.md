This section contains comprehensive guides and examples for using [OpenZeppelin's confidential smart contracts library](https://github.com/OpenZeppelin/openzeppelin-confidential-contracts) with FHEVM. The library provides contracts and utilities that utilize the FHE (Fully Homomorphic Encryption) capabilities of the Zama fhEVM coprocessor to perform confidential transactions.

The library includes the ERC7984 confidential fungible token standard, an ERC20-to-ERC7984 wrapper, confidential vesting wallets, and encrypted voting utilities. See the [official OpenZeppelin documentation](https://docs.openzeppelin.com/confidential-contracts) for more details.

{% hint style="warning" %}
This is an **experimental library**. Developing contracts for confidentiality requires extreme care — many functions do not revert on failure as they would in normal contracts.
{% endhint %}

## Getting Started

This guide will help you set up a development environment for working with OpenZeppelin's confidential contracts and FHEVM.

### Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** >= 20
- **Hardhat** ^2.24
- **Access to an FHEVM-enabled network** and the Zama gateway/relayer

### Project Setup

1. **Clone the FHEVM Hardhat template repository:**

   ```bash
   git clone https://github.com/zama-ai/fhevm-hardhat-template conf-token
   cd conf-token
   ```

2. **Install project dependencies:**

   ```bash
   npm ci
   ```

3. **Install OpenZeppelin's confidential contracts library:**

   ```bash
   npm i @openzeppelin/confidential-contracts
   ```

4. **Compile the contracts:**

   ```bash
   npm run compile
   ```

5. **Run the test suite:**

   ```bash
   npm test
   ```

## Available Guides

Explore the following guides to learn how to implement confidential contracts using OpenZeppelin's library:

- **[ERC7984 Standard](erc7984.md)** - Learn about the ERC7984 standard for confidential tokens
- **[ERC7984 Tutorial](erc7984-tutorial.md)** - Step-by-step tutorial for implementing ERC7984 tokens
- **[ERC7984 to ERC20 Wrapper](ERC7984ERC20WrapperMock.md)** - Convert between confidential and public token standards
- **[Swap ERC7984 to ERC20](swapERC7984ToERC20.md)** - Implement cross-standard token swapping
- **[Swap ERC7984 to ERC7984](swapERC7984ToERC7984.md)** - Confidential token-to-token swapping
- **[Vesting Wallet](vesting-wallet.md)** - Implement confidential token vesting mechanisms
