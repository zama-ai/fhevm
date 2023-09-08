<p align="center">
<img width=600 src="https://github.com/zama-ai/fhevm-solidity/assets/1384478/265d051c-e177-42b4-b9a2-d2b2e474131b" />
</p>
<hr/>
<p align="center">
  <a href="https://docs.zama.ai/fhevm"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Read documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a>
</p>
<p align="center">
<!-- Version badge using shields.io -->
  <a href="https://github.com/zama-ai/fhevm-solidity/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm-solidity?style=flat-square">
  </a>
<!-- Zama Bounty Program -->
  <a href="https://github.com/zama-ai/bounty-program">
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-yellow?style=flat-square">
  </a>
</p>
<hr/>

A Solidity library for interacting with an fhEVM blockchain.

## Install

```bash
# Using npm
npm install fhevm

# Using Yarn
yarn add fhevm

# Using pnpm
pnpm add fhevm
```

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

See our documentation on [https://docs.zama.ai/fhevm/solidity/getting_started](https://docs.zama.ai/fhevm/solidity/getting_started) for more details.

## Development Guide

Install dependencies (Solidity libraries and dev tools)

```bash
npm install
```

Note: Solidity files are formatted with prettier.

### Generate TFHE lib

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

### Tests

The easiest way to understand how to write/dev smart contract and interact with them using **fhevmjs** is to read and explore the few tests available in this repository.

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

#### Docker

We provide a docker image to spin up a fhEVM node for local development.

```bash
npm run fhevm:start
# stop
npm run fhevm:stop
```

#### Faucet

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

#### Run test

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

### Adding new operators

Operators can be defined as data inside `codegen/common.ts` file and code automatically generates solidity overloads.
Test for overloads must be added (or the build doesn't pass) inside `codegen/overloadsTests.ts` file.

## Contributing

There are two ways to contribute to the Zama fhEVM:

- you can open issues to report bugs or typos, or to suggest new ideas
- you can ask to become an official contributor by emailing hello@zama.ai. (becoming an approved contributor involves signing our Contributor License Agreement (CLA))
Only approved contributors can send pull requests, so please make sure to get in touch before you do!

## Credits

This library uses several dependencies and we would like to thank the contributors of those libraries.

## Need support?
<a target="_blank" href="https://community.zama.ai">
  <img src="https://github.com/zama-ai/fhevm-solidity/assets/1384478/049dfc9b-3caa-4c56-8bee-3d1700664db9">
</a>

## License

This software is distributed under the BSD-3-Clause-Clear license. If you have any questions,
please contact us at `hello@zama.ai`.
