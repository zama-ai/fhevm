# How to build and run tests

This guide covers how to build the SDK and run the test suites against different target chains.

# Requirements

1. Copy `sdk/js-sdk/test/.env.example` to `sdk/js-sdk/test/.env`
2. Fill in your `MNEMONIC` and `ZAMA_KEY` values

`sdk/js-sdk/test/.env.devnet` should contain

# Build

```sh
cd <path/to/fhevm>/sdk/js-sdk

# rebuild the whole project (clean)
# This also runs linting (eslint + tsc) and extensive prettier/formatting checks
# (code style, file casing, import order, leading comments) before compiling.
npm run build
```

# Tests

## Browser

```sh
cd <path/to/fhevm>/sdk/js-sdk

# test wasm load in browser using playwright
npm run test:browser
```

## Fast Devnet (no encryption)

```sh
cd <path/to/fhevm>/sdk/js-sdk

# test fast tests on devnet
npm run test:fast:devnet
```

## Fast Testnet (no encryption)

```sh
cd <path/to/fhevm>/sdk/js-sdk

# test fast tests on testnet
npm run test:fast:testnet
```

## Full Devnet (with encryption)

```sh
cd <path/to/fhevm>/sdk/js-sdk

# test all tests on devnet (including slow ones)
npm run test:full:devnet
```

## Full Testnet (no encryption)

```sh
cd <path/to/fhevm>/sdk/js-sdk

# test all tests on testnet (including slow ones)
npm run test:full:testnet
```
