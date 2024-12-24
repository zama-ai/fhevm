# End-to-End (E2E) Tests

This repository contains end-to-end (E2E) tests to ensure that all components work together as intended.

## Installation

Before you begin, ensure that you have [Node.js](https://nodejs.org/) and [npm](https://www.npmjs.com/) installed on your machine.

## Configuration

### Default Configuration: Sepolia

The default configuration is set up for the Sepolia network. To prepare the environment for testing on Sepolia, run the following command:

```bash
make prepare-test-sepolia
```

This will:

1. Copy `.env.example` to `.env`.
2. Update the `contracts/E2EFHEVMConfig.sol` file with the Sepolia-specific configuration.
3. Info: In hardhat.config.ts Sepolia is already set as default network

### Local Testing Configuration

To prepare the environment for testing with a local coprocessor setup, use the following command:

```bash
make prepare-test-local-coprocessor
```

This will:

1. Copy `.env.local` to `.env`.
2. Update the `contracts/E2EFHEVMConfig.sol` file with the local configuration.
3. Update hardhat.config.ts to make localCoprocessor as default network

Note: to print the test accounts addresses:

```bash
make print-accounts
```

## Running the Tests

To run the entire test suite:

```bash
npm run test
```

If you want to focus on a specific test:

```bash
npm run test test/encryptedERC20/EncryptedERC20.ts
```

## Cleaning Up

To clean up temporary files and reset the environment, run:

```bash
make clean
```

---

**Note:** Always double-check your `.env` file and address configurations before running tests to avoid unexpected errors.
