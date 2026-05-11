# How to build and run tests

This guide covers how to build the SDK and run the test suites against different target chains.

# Requirements

1. Copy `sdk/js-sdk/test/.env.example` to `sdk/js-sdk/test/.env`
2. Fill in your `MNEMONIC` and `ZAMA_KEY` values

In `sdk/js-sdk/test/.env.devnet` and `.env.testnet`, set:

```
RPC_URL="https://ethereum-sepolia-rpc.publicnode.com"
```

In `sdk/js-sdk/test/.env.mainnet`, set:

```
RPC_URL="https://ethereum-rpc.publicnode.com"
```

In `sdk/js-sdk/test/.env.localhostFhevm`, set:

```
RPC_URL="http://localhost:8545"
```

# Setup

```sh
cd <path/to/fhevm>/sdk/js-sdk/contracts

# Install Forge to run FHE tests
forge soldeer install

# Run the initialization script (dryrun)
export CHAIN=devnet && source ../test/.env && export MNEMONIC && forge script script/InitFHETest.s.sol --rpc-url https://ethereum-sepolia-rpc.publicnode.com

# Run the initialization script ()
export CHAIN=devnet && source ../test/.env && export MNEMONIC && forge script script/InitFHETest.s.sol --rpc-url https://ethereum-sepolia-rpc.publicnode.com --broadcast
```

# Build

```sh
cd <path/to/fhevm>/sdk/js-sdk

# Rebuild the whole project (clean)
# This also runs linting (eslint + tsc) and extensive prettier/formatting checks
# (code style, file casing, import order, leading comments) before compiling.
npm run build
```

# Tests

## Browser

```sh
cd <path/to/fhevm>/sdk/js-sdk

# Optional: install playwright engines
npx playwright install

# Test wasm load in browser using playwright
npm run test:browser
```

## Fast Devnet (no encryption)

```sh
# Test fast tests on devnet
npm run test:fast:devnet
```

## Fast Testnet (no encryption)

```sh
# Test fast tests on testnet
npm run test:fast:testnet
```

## Full Devnet (with encryption)

```sh
# Test all tests on devnet (including slow ones)
npm run test:full:devnet
```

## Full Testnet (no encryption)

```sh
# Test all tests on testnet (including slow ones)
npm run test:full:testnet
```
