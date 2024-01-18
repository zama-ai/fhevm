<p align="center">
<img width=600 src="https://github.com/zama-ai/fhevm/assets/1384478/265d051c-e177-42b4-b9a2-d2b2e474131b" />
</p>
<hr/>
<p align="center">
  <a href="./fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Read documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a>
</p>
<p align="center">
<!-- Version badge using shields.io -->
  <a href="https://github.com/zama-ai/fhevm/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm?style=flat-square">
  </a>
<!-- Zama Bounty Program -->
  <a href="https://github.com/zama-ai/bounty-program">
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-yellow?style=flat-square">
  </a>
</p>
<hr/>

## Bring confidential smart contracts to your blockchain with fhEVM

There used to be a dilemma in blockchain: keep your application and user data on-chain, allowing everyone to see it, or keep it privately off-chain and lose contract composability.
Thanks to a breakthrough in homomorphic encryption, Zama’s fhEVM makes it possible to run confidential smart contracts on encrypted data, guaranteeing both confidentiality and composability.

### Zama’s fhEVM enables confidential smart contracts using fully homomorphic encryption (FHE)

- **End-to-end encryption of transactions and state:** Data included in transactions is encrypted and never visible to anyone.
- **Composability and data availability on-chain:** States are updated while remaining encrypted at all times.
- **No impact on existing dapps and state:** Encrypted state co-exist alongside public one, and doesn't impact existing dapps.

### Developers can write confidential smart contracts without learning cryptography

- **Solidity Integration:** fhEVM contracts are simple solidity contracts that are built using traditional solidity toolchains.
- **Simple Developer Experience:** Developers can use the `euint` data types to mark which part of their contracts should be private.
- **Programmable Privacy:** All the logic for access control of encrypted states is defined by developers in their smart contracts.

You can take a look at our [list of examples](https://docs.zama.ai/fhevm/resources/examples).

### Powerful features available out of the box

- **High Precision Encrypted Integers -** Up to 256 bits of precision for integers
- **Full range of Operators -** All typical operators are available: `+`, `-`, `*`, `/`, `<`, `>`, `==`, …
- **Encrypted If-Else Conditionals -** Check conditions on encrypted states
- **On-chain PRNG -** Generate secure randomness without using oracles
- **Configurable Decryption -** Threshold, centralized or KMS decryption
- **Unbounded Compute Depth -** Unlimited consecutive FHE operations

### fhEVM implementation

For now, fhEVM is implemented on evmos. You can find all the resources related to this implementation on [this repository](https://github.com/zama-ai/fhevm-evmos).

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

pragma solidity ^0.8.20;

import "fhevm/lib/TFHE.sol";

contract Counter {
  euint32 counter;

  function add(bytes calldata encryptedValue) public {
    euint32 value = TFHE.asEuint32(encryptedValue);
    counter = counter + value;
  }

  function getCounter(bytes32 publicKey) returns (bytes memory) {
    return TFHE.reencrypt(counter, publicKey);
  }
}
```

See our documentation on [https://docs.zama.ai/fhevm/writing-contract/getting_started](https://docs.zama.ai/fhevm/how-to/write_contract) for more details.

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
lib/Impl.sol
lib/TFHE.sol
mocks/Impl.sol
mocks/TFHE.sol
contracts/tests/TFHETestSuiteX.sol
test/tfheOperations/tfheOperations.ts
```

### Tests

The easiest way to understand how to write/dev smart contract and interact with them using **fhevmjs** is to read and explore the available tests in this repository.

#### Fast start

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

For development purposes, we provide a ready to use wallet. In order to use
it, prepare the .env file that contains the mnemonic.

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

Ensure the faucet command was succesful.

</details>
<br />

#### Run tests for network1 network

```bash
# codegen for network1 network
TARGET_NETWORK=Network1 npx ts-node codegen/main.ts && npm run prettier
# run tests for network1 network, assumes network1 rpc already running locally
npx hardhat test --network localNetwork1
```

### Adding new operators

Operators can be defined as data inside `codegen/common.ts` file and code automatically generates solidity overloads.
Test for overloads must be added (or the build doesn't pass) inside `codegen/overloadsTests.ts` file.

### Mocked mode

The mocked mode allows faster testing and the ability to analyze coverage of the tests. In this mocked version, encrypted types are not really encrypted, and the tests are run on the original version of the EVM, on a local hardhat network instance. To run the tests in mocked mode, you can use directly the following command:

```bash
npm run test:mock
```

To analyze the coverage of the tests (in mocked mode necessarily, as this cannot be done on the real fhEVM node), you can use this command :

```bash
npm run coverage:mock
```

Then open the file `coverage/index.html`. You can see there which line or branch for each contract which has been covered or missed by your test suite. This allows increased security by pointing out missing branches not covered yet by the current tests.

Notice that, due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhEVM, the most important change is the `TFHE.isInitialized` method which will always return `true` in the mocked version. Another big difference in mocked mode, compared to the real fhEVM implementation, is that there is no ciphertext verification neither checking that a ciphertext has been honestly obtained (see section 4 of the [whitepaper](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper.pdf)). This means that before deploying to production, developers still need to run the tests with the original fhEVM node, as a final check in non-mocked mode, with `npm run test`.

## Contributing

There are two ways to contribute to the Zama fhEVM:

- you can open issues to report bugs or typos, or to suggest new ideas
- you can ask to become an official contributor by emailing hello@zama.ai. (becoming an approved contributor involves signing our Contributor License Agreement (CLA))
  Only approved contributors can send pull requests, so please make sure to get in touch before you do!

## Credits

This library uses several dependencies and we would like to thank the contributors of those libraries.

## Need support?

- Ask technical questions on the Zama discourse forum: [community.zama.ai](https://community.zama.ai)
- Discuss live with the team on the FHE.org discord server: [discord.fhe.org](https://discord.fhe.org)
<!-- markdown-link-check-disable -->
- Follow Zama on Twitter: [@zama_fhe](https://twitter.com/zama_fhe)
<!-- markdown-link-check-enable -->

## License

This software is distributed under the BSD-3-Clause-Clear license. If you have any questions,
please contact us at `hello@zama.ai`.
