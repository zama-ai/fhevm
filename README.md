# fhEVM

A Solidity library for interacting with an fhEVM blockchain.

## Install

```bash
npm install fhevm
```

or

```bash
yarn install fhevm
```

## Test




<br />
<details>
  <summary>Fast start</summary>
<br />

```bash
# in one terminal
npm run fhevm:start
# in another terminal
npm i
cp .env.example .env
./scripts/faucet.sh
npm test
```
</details>
<br />


### Docker

We provide a docker image to spin up a fhEVM node for local development.

```bash
npm run fhevm:start
# stop
npm run fhevm:stop
```

### Faucet

To use a ready to use test (only for dev) wallet first, prepare the .env file that contains the mnemonic.

```bash
cp .env.example .env
```

This allows the developer to use a few accounts, each account can get coins:

```bash
npm run fhevm:faucet:alice
npm run fhevm:faucet:bob
npm run fhevm:faucet:carol
```


### Run test

```bash
npm test
```

<br />
<details>
  <summary>Error: insufficient funds</summary>
<br />

Ensure the faucet command is succesfull.

</details>
<br />


## Usage

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "fhevm/lib/TFHE.sol";

contract Counter {
  euint32 counter;

  function add(bytes calldata encryptedValue) public {
    euint32 value = TFHE.asEuint32(encryptedValue);
    counter = TFHE.add(counter, value);
  }

  function getCounter(bytes32 publicKey) returns (bytes memory) {
    return TFHE.reencrypt(counter, publicKey);
  }
}
```

See our documentation on [https://docs.zama.ai/homepage/](https://docs.zama.ai/homepage/) for more examples.

## Development Guide

Install dependencies (Solidity libraries and dev tools)

```bash
npm install
```

Note: Solidity files are formatted with prettier.

### Generate TFHE lib and tests

```
npm run codegen
```

WARNING: Use this command to generate Solidity code and prettier result automatically!

Files that are generated now (can be seen inside `codegen/main.ts`)

```
lib/Common.sol
lib/Precompiles.sol
lib/Impl.sol
lib/TFHE.sol
contracts/tests/TFHETestSuiteX.sol
test/tfheOperations/tfheOperations.ts
```

### Adding new operators

Operators can be defined as data inside `codegen/common.ts` file and code automatically generates solidity overloads.
Test for overloads must be added (or the build doesn't pass) inside `codegen/overloadsTests.ts` file.

### Test

```
npm test
```
