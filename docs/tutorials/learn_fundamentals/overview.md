# Tutorial: Learn fhEVM fundamentals using Hardhat

Throughout this tutorial you will build upon a basic Counter example. These examples are designed to guide you step-by-step through the development of confidential smart contracts, introducing new concepts progressively to deepen your understanding of **fhEVM**.

The primary example, `ConfidentialCounter.sol`, is enhanced in stages to demonstrate a variety of features and best practices for encrypted computations.

- [1. Configure the contract](tutorials/learn_fundamentals/configure.md)
- [2. Add encrypted inputs](tutorials/learn_fundamentals/encryption.md)
- [3. Decrypt only for the user](tutorials/learn_fundamentals/reencryption.md)
- [4. Decrypt for everyone](tutorials/learn_fundamentals/decryption.md)

Each iteration of the counter will build upon previous concepts while introducing new functionality, helping you understand how to develop robust confidential smart contracts.

## Step-by-step workflow

### 1. Understand the fundamentals

Before starting development, familiarize yourself with these key concepts:

- [**Architecture Overview**](../../smart_contracts/architecture_overview.md) - Learn how fhEVM enables confidential smart contracts
- [**Encryption & Computation**](../../smart_contracts/d_re_ecrypt_compute.md) - Master the basics of encrypted data handling
- [**Access Control**](../../smart_contracts/acl) - Understand secure data access management

### 2. Set up your development environment

Choose your preferred development tool:

- [**Hardhat Template**](https://github.com/zama-ai/fhevm-hardhat-template) - Recommended for full development workflow
- [**Remix**](../../getting_started/quick_start/remix.md) - Great for quick prototyping and learning

### 3. Configure your contract

Select the appropriate configuration based on your deployment target:

| Environment      | Configuration            |
| ---------------- | ------------------------ |
| Local Testing    | No special config needed |
| Sepolia Testnet  | `SepoliaZamaFHEVMConfig` |
| Ethereum Mainnet | Coming soon              |

### 4. Implement core features

Master these essential encrypted operations:

1. [**Encrypted Inputs**](../../smart_contracts/inputs.md) - Handle encrypted data
2. [**Decryption**](../../smart_contracts/decryption/decrypt.md) - Reveal data securely
3. [**Reencryption**](../../smart_contracts/decryption/reencryption.md) - Share encrypted data

### 5. Follow Best Practices

Optimize your contracts with these guidelines:

- Use appropriate encrypted types (`euint8`, `euint16`, etc.) to minimize gas
- Prefer scalar operands for better performance
- Handle overflows using `TFHE.select`
- Implement proper access control with `TFHE.allow`
- Follow secure reencryption patterns

### 6. Explore resources

Take advantage of these resources:

- [**fhevm-contracts**](https://github.com/zama-ai/fhevm-contracts) - Pre-built contract templates that are extensible
- [**dapps repository**](https://github.com/zama-ai/dapps) - Complete dApp examples
- [**All Tutorials**](../see-all-tutorials.md) - Additional learning materials
