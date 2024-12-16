# Quick start

This guide walks you through developing secure and efficient confidential smart contracts using fhEVM. Whether you're new to Fully Homomorphic Encryption (FHE) or an experienced blockchain developer, we'll cover everything you need to know - from setting up your development environment to deploying your first encrypted contract.

## Network information

If you need access to a Sepolia node and aren’t sure how to proceed, consider using a node provider like [Alchemy](https://www.alchemy.com/), Infura, or similar services. These providers offer easy setup and management, allowing you to create an API key to connect to the network seamlessly.

## Getting test ETH

You can get test ETH for Sepolia from these faucets:

- Alchemy Sepolia Faucet - `https://www.alchemy.com/faucets/ethereum-sepolia`
- QuickNode Sepolia Faucet - `https://faucet.quicknode.com/ethereum/sepolia`

## Configuring Sepolia on your wallet

Most Ethereum wallets have built-in support for testnets like Sepolia. You can add Sepolia to your wallet in two ways:

- **Automatic configuration**: Many wallets like MetaMask have Sepolia pre-configured. Simply open your network selector and choose "Show/hide test networks" to enable testnet visibility.
- **Manual configuration**: The exact steps for manual configuration will vary by wallet, but generally involve:

1. Opening network settings
2. Selecting "Add Network" or "Add Network Manually"
3. Filling in the above information
4. Saving the configuration

## Step-by-step workflow

### 1. (Optional) Learn the overall architecture

Before diving into development, we recommend understanding the overall architecture of fhEVM:

- [**Architecture overview**](../fundamentals/architecture_overview.md): Learn how fhEVM enables confidential smart contracts
- [**Encryption & computation**](../fundamentals/d_re_ecrypt_compute.md): Understand how data is encrypted, decrypted and computed
- [**Access control**](../fundamentals/acl/): Learn about managing access to encrypted data

This knowledge will help you make better design decisions when building your confidential smart contracts.

### 2. Use the Hardhat template

Begin with our custom [`fhevm-hardhat-template` repository](https://github.com/zama-ai/fhevm-hardhat-template).

- **Why Hardhat?**: It is a powerful Solidity development environment, offering tools for writing, testing, and deploying contracts to the `fhEVM` using TypeScript.
- **Benefit**: The template provides a pre-configured setup tailored to confidential smart contracts, saving development time and ensuring compatibility.

### 3. Configure the contract

Choose and inherit the correct configuration based on the environment:

- **Mock network**: For local testing and development.
- **Testnets (e.g., Sepolia)**: For deploying to public test networks.
- **Mainnet**: When deploying to production.

Ensure configuration contracts (e.g., `SepoliaZamaFHEVMConfig`, `SepoliaZamaFHEVMConfig`) are inherited correctly to initialize encryption parameters, cryptographic keys, and Gateway addresses. See [configuration](../fundamentals/configure.md) for more details.

### 4. Begin with unencrypted logic

Develop your contract as you would for a traditional EVM chain:

- Use cleartext variables and basic logic to simplify debugging and reasoning about the contract’s behavior.
- Focus on implementing core functionality without adding encryption initially.

For a step-by-step guide on developing your first confidential smart contract, see our [First smart contract](../getting_started/first_smart_contract.md). This guide covers:

- Creating a basic encrypted counter contract
- Understanding the configuration process
- Working with encrypted state variables

Key resources for working with encrypted types:

- [Supported encrypted types](../fundamentals/types.md) - Learn about euint8, euint16, euint32, euint64, ebool and eaddress
- [Encrypted operations](../fundamentals/operations.md) - Understand arithmetic, comparison, and logical operations on encrypted data

### 5. Add encryption

Once the logic is stable, integrate the `TFHE` Solidity library to enable encryption:

- **Convert sensitive variables**: Replace plaintext types like `uintX` with encrypted types such as `euintX`.
- **Enable confidentiality**: Encrypted variables and operations ensure sensitive data remains private while still being processed.

Learn how to implement core encryption operations:

- [Encrypting inputs](../fundamentals/inputs.md) - Create and validate encrypted inputs
- [Decrypting values](../fundamentals/decryption/decrypt.md) - Securely decrypt data for authorized users
- [Reencryption](../fundamentals/decryption/reencryption.md) - Share encrypted data between parties

### 6. Follow best practices

Throughout the documentation, you'll find sections marked with 🔧 that highlight important best practices. These include:

- **Optimized data types:** Use appropriately sized encrypted types (`euint8`, `euint16`, etc.) to minimize gas costs.
- **Scalar operands:** Whenever possible, use scalar operands in operations to reduce computation and gas usage.
- **Overflow handling:** Manage arithmetic overflows in encrypted operations using conditional logic (`TFHE.select`).
- **Secure access control:** Use `TFHE.allow` and `TFHE.isSenderAllowed` to implement robust ACL (Access Control List) mechanisms for encrypted values.
- **Reencryption patterns:** Follow the recommended approaches for reencryption to share or repurpose encrypted data securely.

### 7. Leverage example templates

Use the [`fhevm-contracts repository`](https://github.com/zama-ai/fhevm-contracts) for pre-built examples:

- **Why templates?**: They demonstrate common patterns and best practices for encrypted operations, such as governance, token standards, and utility contracts.
- **How to use**: Extend or customize these templates to suit your application’s needs.

For more details, explore the [fhevm-contracts documentation](../guides/contracts.md).

## Contract examples

Throughout the documentation you will encounter many Counter contract examples. These examples are designed to guide you step-by-step through the development of confidential smart contracts, introducing new concepts progressively to deepen your understanding of **fhEVM**.

The primary example, `Counter.sol`, is enhanced in stages to demonstrate a variety of features and best practices for encrypted computations.

- [Counter1.sol](../getting_started/first_smart_contract.md#your-first-smart-contract) - Introduction to basic encrypted state variables and simple operations.
- [Counter2.sol](../fundamentals/inputs.md#upgrade-of-our-counter-contract) - Incorporating encrypted inputs into the contract for enhanced functionality.
- [Counter3.sol](../fundamentals/decryption/decrypt.md#applying-decryption-to-the-counter-example) - Introduction to decryption and how contracts can interact with the Gateway.
- [Counter4.sol](../fundamentals/decryption/reencryption.md#applying-re-encryption-to-the-counter-example) - Introduction to re-encryption, enabling secure sharing of encrypted data.

Each iteration of the counter will build upon previous concepts while introducing new functionality, helping you understand how to develop robust confidential smart contracts.

## Table of all addresses

Save this in your `.env` file:

| Contract/Service       | Address/Value                              |
| ---------------------- | ------------------------------------------ |
| TFHE_EXECUTOR_CONTRACT | 0x687408aB54661ba0b4aeF3a44156c616c6955E07 |
| ACL_CONTRACT           | 0xFee8407e2f5e3Ee68ad77cAE98c434e637f516e5 |
| PAYMENT_CONTRACT       | 0xFb03BE574d14C256D56F09a198B586bdfc0A9de2 |
| KMS_VERIFIER_CONTRACT  | 0x9D6891A6240D6130c54ae243d8005063D05fE14b |
| GATEWAY_CONTRACT       | 0x33347831500F1e73f0ccCBb95c9f86B94d7b1123 |
| PUBLIC_KEY_ID          | 0301c5dd3e2702992b7c12930b7d4defeaaa52cf   |
| GATEWAY_URL            | `https://gateway.sepolia.zama.ai/`         |
