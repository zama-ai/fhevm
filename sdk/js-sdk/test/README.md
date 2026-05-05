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

In `sdk/js-sdk/test/.env.localhost` and `sdk/js-sdk/test/.env.localhostFhevm`, set:

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

Add to `/etc/hosts`:

```sh
127.0.0.1	minio
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

## Run tests in cleartext mode

```sh
npm run test
```

## Addresses

```sh
# FHEVM host-contracts mnemonic
MNEMONIC="adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"
MNEMONIC_DERIVATION_PREFIX="m/44'/60'/0'/0/"
MNEMONIC_DERIVATION_INDEX="5"

# cast wallet private-key --mnemonic "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" --mnemonic-derivation-path "m/44'/60'/0'/0/5"
PRIVATE_KEY=0x7697c90f7863e6057fbe25674464e14b57f2c670b1a8ee0f60fb87eb9b615c4d
ADDRESS=0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4
```

```sh
# MNEMONIC="adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"
# Account 5

# nonce=0
# cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 0
PAUDER_SET=0x34e3eD8472e409dbF8FDf933cA996DC75e4Be126

# nonce=1
# cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 1
ACL=0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D

# nonce=3
# cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 3
FHEVM_EXECUTOR=0xe3a9105a3a932253A70F126eb1E3b589C643dD24

# nonce=4
# cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 4
KMS_VERIFIER=0x901F8942346f7AB3a01F6D7613119Bca447Bb030

# nonce=5
# cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 5
INPUT_VERIFIER=0x36772142b74871f255CbD7A3e89B401d3e45825f

# nonce=6
# cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 6
HCU_LIMIT=0x233ff88A48c172d29F675403e6A8e302b0F032D9
```

# fhevm-cli

```sh
cd <root>/test-suite/e2e
```

`./fhevm-cli up --help`
`./fhevm-cli up --override test-suite` : use local test suite
`./fhevm-cli upgrade test-suite`: rebuild test suite
`./fhevm-cli test --grep BBB`
`./fhevm-cli up --target latest-supported`

```json
{
  "target": "latest-supported",
  "lockName": "latest-supported.json",
  "sources": ["profile=latest-supported"],
  "env": {
    "GATEWAY_VERSION": "b2d8a6c",
    "HOST_VERSION": "b2d8a6c",
    "COPROCESSOR_DB_MIGRATION_VERSION": "b2d8a6c",
    "COPROCESSOR_HOST_LISTENER_VERSION": "b2d8a6c",
    "COPROCESSOR_GW_LISTENER_VERSION": "b2d8a6c",
    "COPROCESSOR_TX_SENDER_VERSION": "b2d8a6c",
    "COPROCESSOR_TFHE_WORKER_VERSION": "b2d8a6c",
    "COPROCESSOR_ZKPROOF_WORKER_VERSION": "b2d8a6c",
    "COPROCESSOR_SNS_WORKER_VERSION": "b2d8a6c",
    "CONNECTOR_DB_MIGRATION_VERSION": "b2d8a6c",
    "CONNECTOR_GW_LISTENER_VERSION": "b2d8a6c",
    "CONNECTOR_KMS_WORKER_VERSION": "b2d8a6c",
    "CONNECTOR_TX_SENDER_VERSION": "b2d8a6c",
    "CORE_VERSION": "c57f52f", // 13.20
    "RELAYER_VERSION": "b2d8a6c",
    "RELAYER_MIGRATE_VERSION": "b2d8a6c",
    "TEST_SUITE_VERSION": "v0.12.1"
  }
}
```
