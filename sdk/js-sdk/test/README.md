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

In `sdk/js-sdk/test/.env.localcleartext` and `sdk/js-sdk/test/.env.localstack`, set:

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

## Localstack (per protocol version)

Each command runs the suite against a localstack started with a specific
protocol version (a `--chain` + `--fhevm-cli-profile` pair). The runner
restarts localstack via `fhevm-cli` before the tests and stops it afterwards,
so these share a single stack and must be run **one at a time** (sequentially),
not in parallel.

```sh
# Latest / default
npm run test:localstack

# A specific protocol version
npm run test:localstack:v11   # localstack_v11 + v0.11.0-mainnet.json
npm run test:localstack:v12   # localstack_v12 + v0.12.0-testnet.json
npm run test:localstack:v13   # localstack_v13 + v0.13.0.json

# All versions, sequentially (stops at the first failure)
npm run test:localstack:v11 && \
  npm run test:localstack:v12 && \
  npm run test:localstack:v13 && \
  npm run test:localstack
```

Prerequisites: the Solidity dependencies must be installed once
(`cd contracts && forge soldeer install`, see [Setup](#setup)), and `forge`
must be available on `PATH`.

> Note (macOS): the test scripts require Bash. The default `/bin/bash` (3.2) is
> fine; the scripts avoid Bash 4-only syntax.

## Definition of Done (run everything)

`dod` runs the full Definition-of-Done gate. Use `--help` to print the exact
command list.

```sh
# Standard gate: clean, codegen, prettier, lint, unit tests, dev + prod builds,
# browser test, and cleartext suites (v12, v13, latest).
npm run dod

# Everything above PLUS the long suites: testnet, devnet, and every localstack
# version (v11, v12, v13, latest), run sequentially.
npm run dod:full

# List the exact commands without running them.
node test/scripts/dod.mjs --help
```

`dod` stops at the first failing command. `dod:full` additionally requires
testnet/devnet credentials (`ZAMA_FHEVM_API_KEY`, `RPC_URL`) and a Playwright
browser for the browser test.

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

# @fhevm/solidity

```
    // chainId = 31337
    function _getLocalConfig() private pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D,
                CoprocessorAddress: 0xe3a9105a3a932253A70F126eb1E3b589C643dD24,
                KMSVerifierAddress: 0x901F8942346f7AB3a01F6D7613119Bca447Bb030
            });
    }

    // chainId = 31337
    function _getLocalstackConfig() private pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c,
                CoprocessorAddress: 0xcCAe95fF1d11656358E782570dF0418F59fA40e1,
                KMSVerifierAddress: 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11
            });
    }
```

# fhevm-cli

Run e2e tests using @fhevm/sdk:
edit : `/Users/alex/src/me/zama-ai/fhevm/test-suite/e2e/test/instance.ts`
set `const useFhevmSdk = true;`

Run e2e tests using @zama-fhe/relayer-sdk:
edit : `/Users/alex/src/me/zama-ai/fhevm/test-suite/e2e/test/instance.ts`
set `const useFhevmSdk = false;`

## Init

```
cd <root>/test-suite/fhevm

# install fhevm-cli (if needed)
bun install
```

## Regular start

```
./fhevm-cli up --help

# Use local test suite
./fhevm-cli up --override test-suite

# Rebuild test suite (if needed)
./fhevm-cli upgrade test-suite

# With a specific old relayer-sdk
RELAYER_SDK_VERSION=0.4.2 ./fhevm-cli upgrade test-suite
RELAYER_SDK_VERSION=0.5.0-rc.1 ./fhevm-cli upgrade test-suite

# Run full test suite
./fhevm-cli test standard
./fhevm-cli test erc20

# Run specific test
./fhevm-cli test --grep "test delegated user decrypt"
```

## Test against protocol v0.11

```
# Use local test suite and v0.11 profile
./fhevm-cli up --override test-suite --lock-file profiles/v0.11.json

# Rebuild test suite (if needed)
./fhevm-cli upgrade test-suite

# Run full test suite
./fhevm-cli test standard

# Run specific test
./fhevm-cli test --grep "test delegated user decrypt"
```

## Profile

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
