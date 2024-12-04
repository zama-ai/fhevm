# Using Hardhat

This document guides you to start with fhEVM by using our [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).&#x20;

This template allows you to launch an fhEVM Docker image and run your smart contract on it. For more information, refer to the [README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md).

## Features of the hardhat template

The Hardhat template comes pre-configured with tools and libraries to streamline the development process:

- **Frameworks and libraries**:
  - [Hardhat](https://github.com/nomiclabs/hardhat): Compile, deploy, and test smart contracts.
  - [TypeChain](https://github.com/ethereum-ts/TypeChain): Generate TypeScript bindings for contracts.
  - [Ethers.js](https://github.com/ethers-io/ethers.js/): Ethereum library for interactions.
  - [Solhint](https://github.com/protofire/solhint): Linter for Solidity code.
  - [Solcover](https://github.com/sc-forks/solidity-coverage): Code coverage analysis.
  - [Prettier Plugin Solidity](https://github.com/prettier-solidity/prettier-plugin-solidity): Solidity code formatter.
- **Default configurations**:\
  Includes sensible default configurations for tools like Prettier, Solhint, and ESLint.
- **GitHub actions**:\
  Pre-configured for CI/CD pipelines to lint and test contracts on every push or pull request.

## Getting started

### Prerequisites

1. **Install** [**pnpm**](https://pnpm.io/installation) for dependency management.
2. **Set up a mnemonic**: Create a `.env` file by copying `.env.example`:

   ```bash
   cp .env.example .env
   ```

   Generate a mnemonic using [this tool](https://iancoleman.io/bip39/) if needed.

3. **Install dependencies**: Ensure you’re using Node.js v20 or later:

   ```bash
   pnpm install
   ```

### Commands overview

Here’s a list of essential commands to work with the Hardhat template:

| **Command**      | **Description**                                               |
| ---------------- | ------------------------------------------------------------- |
| `pnpm compile`   | Compiles the smart contracts.                                 |
| `pnpm typechain` | Compiles contracts and generates TypeChain bindings.          |
| `pnpm test`      | Runs tests locally in mocked mode, simulating FHE operations. |
| `pnpm lint:sol`  | Lints Solidity code.                                          |
| `pnpm lint:ts`   | Lints TypeScript code.                                        |
| `pnpm clean`     | Cleans contract artifacts, cache, and coverage reports.       |
| `pnpm coverage`  | Analyzes test coverage (mocked mode only).                    |

### Mocked mode vs non-mocked mode

#### Mocked mode

Mocked mode allows faster testing by simulating FHE operations. This mode runs tests on a local Hardhat network without requiring a real fhEVM node. Use the following commands:

- **Run tests**:

  ```bash
  pnpm test
  ```

- **Analyze coverage**:

  ```bash
  pnpm coverage
  ```

> **Note**: Mocked mode approximates gas consumption for FHE operations but may slightly differ from actual fhEVM behavior.

#### Non-mocked mode

Non-mocked mode uses a real fhEVM node, such as the coprocessor on the Sepolia test network.

- **Run tests on Sepolia**:

  ```bash
  npx hardhat test [PATH_TO_TEST] --network sepolia
  ```

- **Requirements**:
  1. Fund test accounts on Sepolia.
  2. Pass the correct `FHEVMConfig` struct in your smart contract’s constructor.

> **Note**: Refer to the [fhEVM documentation](https://docs.zama.ai/fhevm) for configuring `FHEVMConfig`.

## Development tips

### Syntax highlighting

For Solidity syntax highlighting, use the [Hardhat Solidity](https://marketplace.visualstudio.com/items?itemName=NomicFoundation.hardhat-solidity) extension for VSCode.

### Important note

Due to limitations in the `solidity-coverage` package, coverage computation in fhEVM does not support tests involving the `evm_snapshot` Hardhat testing method. However, this method is still supported when running tests in mocked mode. If you are using Hardhat snapshots, we recommend to end your your test description with the `[skip-on-coverage]` tag to to avoid coverage issues. Here is an example:

```js
import { expect } from 'chai';
import { ethers, network } from 'hardhat';

import { createInstances, decrypt8, decrypt16, decrypt32, decrypt64 } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployRandFixture } from './Rand.fixture';

describe('Rand', function () {
  before(async function () {
    await initSigners();
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployRandFixture();
    this.contractAddress = await contract.getAddress();
    this.rand = contract;
    this.instances = await createInstances(this.signers);
  });

  it('64 bits generate with upper bound and decrypt', async function () {
    const values: bigint[] = [];
    for (let i = 0; i < 5; i++) {
      const txn = await this.rand.generate64UpperBound(262144);
      await txn.wait();
      const valueHandle = await this.rand.value64();
      const value = await decrypt64(valueHandle);
      expect(value).to.be.lessThanOrEqual(262141);
      values.push(value);
    }
    // Expect at least two different generated values.
    const unique = new Set(values);
    expect(unique.size).to.be.greaterThanOrEqual(2);
  });

  it('8 and 16 bits generate and decrypt with hardhat snapshots [skip-on-coverage]', async function () {
    if (network.name === 'hardhat') {
      // snapshots are only possible in hardhat node, i.e in mocked mode
      this.snapshotId = await ethers.provider.send('evm_snapshot');
      const values: number[] = [];
      for (let i = 0; i < 5; i++) {
        const txn = await this.rand.generate8();
        await txn.wait();
        const valueHandle = await this.rand.value8();
        const value = await decrypt8(valueHandle);
        expect(value).to.be.lessThanOrEqual(0xff);
        values.push(value);
      }
      // Expect at least two different generated values.
      const unique = new Set(values);
      expect(unique.size).to.be.greaterThanOrEqual(2);

      await ethers.provider.send('evm_revert', [this.snapshotId]);
      const values2: number[] = [];
      for (let i = 0; i < 5; i++) {
        const txn = await this.rand.generate8();
        await txn.wait();
        const valueHandle = await this.rand.value8();
        const value = await decrypt8(valueHandle);
        expect(value).to.be.lessThanOrEqual(0xff);
        values2.push(value);
      }
      // Expect at least two different generated values.
      const unique2 = new Set(values2);
      expect(unique2.size).to.be.greaterThanOrEqual(2);
    }
  });
});
```

In this snippet, the first test will always run, whether in "real" non-mocked mode (`pnpm test`), testing mocked mode (`pnpm test`) or coverage (mocked) mode (`pnpm coverage`). On the other hand, the second test will be run **only** in testing mocked mode(`pnpm test`), because snapshots only works in that specific case. Actually, the second test will be skipped if run in coverage mode, since its description string ends with `[skip-on-coverage]` and similarly, we avoid the test to fail in non-mocked mode since we check that the network name is `hardhat`.

## Limitations

{% hint style="danger" %}
Due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhEVM, mainly in gas prices for the FHE operations. This means that before deploying to production, developers still need to run the tests with the original fhEVM node, as a final check in non-mocked mode, with `pnpm test` or `npx hardhat test`.
{% endhint %}

By using this Hardhat template, you can streamline your fhEVM development workflow and focus on building robust, privacy-preserving smart contracts. For additional details, visit the [fhevm-hardhat-template README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md).

{% hint style="success" %}
**Zama 5-Question Developer Survey**

We want to hear from you! Take 1 minute to share your thoughts and helping us enhance our documentation and libraries. **👉** [**Click here**](https://www.zama.ai/developer-survey) to participate.
{% endhint %}
