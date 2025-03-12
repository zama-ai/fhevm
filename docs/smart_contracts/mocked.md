# Mocked mode

This document provides an overview of mocked mode in the fhEVM framework, explaining how it enables faster development and testing of smart contracts that use Fully Homomorphic Encryption (FHE).

## Overview

**Mocked mode** is a development and testing feature provided in the `fhEVM` framework that allows developers to simulate the behavior of Fully Homomorphic Encryption (FHE) without requiring the full encryption and decryption processes to be performed. This makes development and testing cycles faster and more efficient by replacing actual cryptographic operations with mocked values, which behave similarly to encrypted data but without the computational overhead of true encryption.

## How to use mocked mode

### 1. **Hardhat template**

Mocked mode is currently supported in the [Zama Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template). Developers can enable mocked mode to simulate encrypted operations while building and testing smart contracts locally. The Hardhat template includes pre-configured scripts and libraries to simplify the setup process for mocked mode.

Refer to the [[Quick start - Hardhat]](../getting-started/overview-1/hardhat/README.md) guide for instructions on using the Hardhat template.

### 2. **Foundry (coming soon)**

Mocked mode support is planned for [Foundry](./write_contract/foundry.md) in future releases.

## How mocked mode works

For faster testing iterations, instead of launching all the tests on the local fhEVM node, which could last several minutes, you can use a mocked version of the fhEVM by running `pnpm test`. The same tests should (almost always) pass as is, without any modification; neither the JavaScript files nor the Solidity files need to be changed between the mocked and the real version.

The mocked mode does **not** actually perform real encryption for encrypted types and instead runs the tests on a local Hardhat node, which is implementing the original EVM (i.e., non-fhEVM).

Additionally, the mocked mode will let you use all the Hardhat-related special testing and debugging methods, such as `evm_mine`, `evm_snapshot`, `evm_revert`, etc., which are very helpful for testing.

## Development workflow with mocked mode

When developing confidential contracts, we recommend to use first the mocked version of fhEVM for faster testing with `pnpm test` and coverage computation via `pnpm coverage`, this will lead to a better developer experience.

It's essential to run tests of the final contract version using the real fhEVM. You can do this by running `pnpm test` before deployment.

To run the mocked tests use either:

```sh
pnpm test
```

Or equivalently:

```sh
npx hardhat test --network hardhat
```

In mocked mode, all tests should pass in few seconds instead of few minutes, allowing a better developer experience.
Furthermore, getting the coverage of tests is only possible in mocked mode. Just use the following command:

```
pnpm coverage
```

Or equivalently:

```
npx hardhat coverage
```

Then open the file `coverage/index.html` to see the coverage results. This will increase security by pointing out missing branches not covered yet by the current test suite.

{% hint style="info" %}
Due to limitations in the `solidity-coverage` package, test coverage computation does not work with tests that use the `evm_snapshot` Hardhat testing method.
{% endhint %}

If you are using Hardhat snapshots in your tests, we recommend adding the `[skip-on-coverage]` tag at the end of your test description. Here's an example:

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

- **The first test** always runs in both mocked mode (`pnpm test`) and coverage mode (`pnpm coverage`).
- **The second test** runs only while testing mocked mode (`pnpm test`) since it uses snapshots which are only available there. The test is skipped in coverage mode due to the `[skip-on-coverage]` suffix in its description. It also checks that the network is 'hardhat' to avoid failures in non-mocked environments.
