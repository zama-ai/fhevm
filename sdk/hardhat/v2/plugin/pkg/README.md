<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zama-ai/fhevm/main/docs/.gitbook/assets/fhevm-header-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/zama-ai/fhevm/main/docs/.gitbook/assets/fhevm-header-light.png">
  <img src="https://raw.githubusercontent.com/zama-ai/fhevm/main/docs/.gitbook/assets/fhevm-header-light.png" width="600" alt="FHEVM">
</picture>
</p>

<hr/>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper.pdf">📃 Read white paper</a> | <a href="https://docs.zama.ai/"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm-mocks/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm-mocks?style=flat-square"></a>
  <a href="https://github.com/zama-ai/fhevm-mocks/blob/main/LICENSE">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
  <a href="https://slsa.dev"><img alt="SLSA 3" src="https://slsa.dev/images/gh-badge-level3.svg" /></a>
</p>

# FHEVM Hardhat Plugin

[![NPM Version](https://img.shields.io/npm/v/%40fhevm%2Fhardhat-plugin)](https://www.npmjs.com/package/@fhevm/hardhat-plugin)
[![hardhat](https://hardhat.org/buidler-plugin-badge.svg?1)](https://hardhat.org)

Hardhat plugin for developing and testing FHEVM contracts.

# Quickstart & Tutorial

- [The FHEVM Hardhat Template](https://github.com/zama-ai/fhevm-hardhat-template)
- [Quickstart Tutorial](https://docs.zama.ai/protocol/solidity-guides/getting-started/quick-start-tutorial)

For additional documentation and resources:

- [Setting up Hardhat](https://docs.zama.ai/protocol/solidity-guides/getting-started/setup)
- [FHEVM Hardhat Plugin Development Guide](https://docs.zama.ai/protocol/solidity-guides/development-guide/hardhat)
- [FHEVM Examples](https://docs.zama.ai/protocol/examples)

## List of peer dependencies

This Hardhat plugin relies on the following peer dependencies to function correctly.

- `@fhevm/hardhat-plugin`
- `@fhevm/solidity`
- `@fhevm/sdk`
- `@nomicfoundation/hardhat-ethers`
- `ethers`
- `hardhat`

### Automatic Installation (npm v7+)

If you're using npm version 7 or later, these peer dependencies will be automatically installed when you install this
plugin — no additional action is required.

### Manual peer dependencies installation

If you're using a package manager that does not automatically install peer dependencies (like pnpm or npm <7) then
you'll need to add them manually to your package.

## Installation

```sh
npm install --save-dev @fhevm/hardhat-plugin
```

And register the plugin in your [`hardhat.config.js`](https://hardhat.org/config/):

```ts
// Typescript
import '@fhevm/hardhat-plugin';
```

## Zama API Key (Mainnet)

To generate encrypted inputs or decrypt FHE data on Ethereum mainnet, you need a Zama API key. To configure it in the
FHEVM Hardhat Plugin, run:

```sh
npx hardhat vars set ZAMA_FHEVM_API_KEY <your-api-key>
```

To verify the Zama API key is properly set up:

```sh
npx hardhat vars get ZAMA_FHEVM_API_KEY
```

# 🚀 Quick Start: Hello World with FHEVM

In this guide, we'll walk through a simple Hello World example using FHEVM (Fully Homomorphic Encryption). The objective
is to build a Solidity smart contract that:

- Validates and stores an encrypted input value a
- Validates and stores a second encrypted input value b
- Computes the FHE sum of a and b, storing the result as aplusb
- Reads and decrypts the encrypted result aplusb

Let’s get started!

### 1. Set Up Your Project Directory

Create and navigate into a new project folder:

```sh
mkdir hello_fhevm
cd ./hello_fhevm
```

### 2. Install Hardhat & FHEVM Hardhat plugin

Install [Hardhat](https://www.npmjs.com/package/hardhat) and the FHEVM Hardhat plugin as development dependencies:

```sh
npm install --save-dev hardhat
npm install --save-dev ethers
npm install --save-dev @nomicfoundation/hardhat-chai-matchers
npm install --save-dev @nomicfoundation/hardhat-ethers
npm install --save-dev @nomicfoundation/hardhat-network-helpers
# Warning: @nomicfoundation/hardhat-chai-matchers requires v4
npm install --save-dev chai@^4.2.0
# Install Typescript related packages required to run Hardhat using Typescript
npm install --save-dev typescript
npm install --save-dev ts-node
npm install --save-dev @types/mocha
```

Install FHEVM related packaged as development dependencies:

```sh
npm install --save-dev @fhevm/hardhat-plugin
```

### 3. Create the Smart Contract `APlusB.sol`

First, create a `contracts` folder to hold your Solidity code:

```sh
mkdir contracts
cd ./contracts
```

Then, inside the `contracts` directory, create a new Solidity file named `APlusB.sol` with the following content:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, euint8, externalEuint8 } from "@fhevm/solidity/lib/FHE.sol";
import { ZamaEthereumConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract APlusB is ZamaEthereumConfig {
  euint8 private _a;
  euint8 private _b;
  euint8 private _aplusb;

  constructor() {}

  function setA(externalEuint8 inputA, bytes calldata inputProof) external {
    _a = FHE.fromExternal(inputA, inputProof);
    FHE.allowThis(_a);
  }

  function setB(externalEuint8 inputB, bytes calldata inputProof) external {
    _b = FHE.fromExternal(inputB, inputProof);
    FHE.allowThis(_b);
  }

  function computeAPlusB() external {
    _aplusb = FHE.add(_a, _b);
    FHE.allowThis(_aplusb);
    FHE.allow(_aplusb, msg.sender);
  }

  function aplusb() public view returns (euint8) {
    return _aplusb;
  }
}
```

### 4. Create the Hardhat config file

In the project root directory `hello_fhevm`, create a new Typescript file named `hardhat.config.ts` with the following
content:

```ts
// Add the FHEVM Hardhat plugin here!
import '@fhevm/hardhat-plugin';
import '@nomicfoundation/hardhat-chai-matchers';
import '@nomicfoundation/hardhat-ethers';
import { HardhatUserConfig } from 'hardhat/config';

const config: HardhatUserConfig = {
  solidity: {
    version: '0.8.28',
    settings: {
      // ⚠️ FHEVM requires at least the "cancun" EVM version
      evmVersion: 'cancun',
    },
  },
};

export default config;
```

### 5. Compile the Solidity contract

```sh
npx hardhat compile
```

### 6. Test the FHEVM Solidity Contract using the FHEVM mock environment

First, from the project root directory, create a `test` folder to hold your test code:

```sh
mkdir test
cd ./test
```

Then, inside the `test` directory, create a new Solidity file named `APlusB.ts` with the following content:

```ts
import { FhevmType } from '@fhevm/hardhat-plugin';
import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers } from 'ethers';
import * as hre from 'hardhat';

async function deployAPlusBFixture(account: HardhatEthersSigner) {
  const contractFactory = await hre.ethers.getContractFactory('APlusB');
  const contract = await contractFactory.connect(account).deploy();
  await contract.waitForDeployment();
  return contract as any as ethers.Contract;
}

describe('APlusB', function () {
  let aplusbContract: ethers.Contract;
  let aplusbContractAddress: string;

  before(async function () {
    const [alice] = await hre.ethers.getSigners();

    aplusbContract = await deployAPlusBFixture(alice);
    aplusbContractAddress = await aplusbContract.getAddress();

    await hre.fhevm.assertCoprocessorInitialized(aplusbContract, 'APlusB');
  });

  it('uint8: add 80 to 123 should equal 203', async function () {
    const [alice] = await hre.ethers.getSigners();

    // 1. Validates and Stores value 'a'

    // Create the encrypted input
    const inputA = hre.fhevm.createEncryptedInput(aplusbContractAddress, alice.address);
    inputA.add8(80);
    const encryptedInputA = await inputA.encrypt();

    // Call the contract with the encrypted value `a`
    const encryptedA = encryptedInputA.handles[0];
    const proofA = encryptedInputA.inputProof;

    let tx = await aplusbContract.setA(encryptedA, proofA);
    await tx.wait();

    // 2. Validates and Stores value 'b'

    // Create the encrypted input
    const inputB = hre.fhevm.createEncryptedInput(aplusbContractAddress, alice.address);
    inputB.add8(123);
    const encryptedInputB = await inputB.encrypt();

    // Call the contract with the encrypted value `b`
    const encryptedB = encryptedInputB.handles[0];
    const proofB = encryptedInputB.inputProof;

    tx = await aplusbContract.setB(encryptedB, proofB);
    await tx.wait();

    // 3. Computes the FHE sum of `a` and `b`, storing the result as `aplusb` on chain
    tx = await aplusbContract.computeAPlusB();
    await tx.wait();

    // 4. Reads the encrypted result `aplusb` = `a` + `b`
    const encryptedAPlusB = await aplusbContract.aplusb();

    // 5. Decrypts `aplusb`
    const clearAPlusB = await hre.fhevm.userDecryptEuint(
      FhevmType.euint8,
      encryptedAPlusB,
      aplusbContractAddress,
      alice as unknown as ethers.Signer,
    );

    expect(clearAPlusB).to.eq(80 + 123);
  });
});
```

From the project root directory, run the test by executing the following hardhat command:

```sh
npx hardhat test
```

# 📘 FHEVM Documentation and Examples

For more FHEVM examples and detailed documentation please go [here](https://docs.zama.ai/protocol)
