# Using the CLI

The `fhevmjs` Command-Line Interface (CLI) tool provides a simple and efficient way to encrypt data for use with the blockchain's Fully Homomorphic Encryption (FHE) system. This guide explains how to install and use the CLI to encrypt integers and booleans for confidential smart contracts.

## Installation

Ensure you have [Node.js](https://nodejs.org/) installed on your system before proceeding. Then, globally install the `fhevmjs` package to enable the CLI tool:

```bash
npm install -g fhevmjs
```

Once installed, you can access the CLI using the `fhevm` command. Verify the installation and explore available commands using:

```bash
fhevm help
```

To see specific options for encryption, run:

```bash
fhevm encrypt help
```

## Encrypting Data

The CLI allows you to encrypt integers and booleans for use in smart contracts. Encryption is performed using the blockchain's FHE public key, ensuring the confidentiality of your data.

### Syntax

```bash
fhevm encrypt --node <NODE_URL> <CONTRACT_ADDRESS> <USER_ADDRESS> <DATA:TYPE>...
```

- **`--node`**: Specifies the RPC URL of the blockchain node (e.g., `http://localhost:8545`).
- **`<CONTRACT_ADDRESS>`**: The address of the contract interacting with the encrypted data.
- **`<USER_ADDRESS>`**: The address of the user associated with the encrypted data.
- **`<DATA:TYPE>`**: The data to encrypt, followed by its type:
  - `:64` for 64-bit integers
  - `:1` for booleans

### Example Usage

Encrypt the integer `71721075` (64-bit) and the boolean `1` for the contract at `0x8Fdb26641d14a80FCCBE87BF455338Dd9C539a50` and the user at `0xa5e1defb98EFe38EBb2D958CEe052410247F4c80`:

```bash
fhevm encrypt --node http://localhost:8545 0x8Fdb26641d14a80FCCBE87BF455338Dd9C539a50 0xa5e1defb98EFe38EBb2D958CEe052410247F4c80 71721075:64 1:1
```

## Additional Resources

For more advanced features and examples, refer to the [fhevmjs documentation](../../references/fhevmjs.md).
