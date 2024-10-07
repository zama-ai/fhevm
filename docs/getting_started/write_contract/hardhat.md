# Using Hardhat

This document guides you to start with fhEVM by using our [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).

## Using Hardhat with fhEVM

To start writing smart contracts with fhEVM, we recommend using our [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template). This template allows you to launch an fhEVM Docker image and run your smart contract on it. For more information, refer to the [README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md).

When developing confidential contracts, we recommend using the mocked version of fhEVM first for faster testing. You can achieve this by running `pnpm test:mock` for tests and `pnpm coverage:mock` for coverage computation. This approach provides a better developer experience. However, keep in mind that the mocked fhEVM has limitations and discrepancies compared to the real fhEVM node, as detailed in the [limitations](#limitations) section below.

It's essential to test the final contract version with the real fhEVM before deployment. Run `pnpm test` to ensure everything works correctly.

## Mocked mode

For faster iteration testing, you can use the mocked version of the fhEVM instead of launching all tests on a local fhEVM node with `pnpm test` or `npx hardhat test`, which can take several minutes.

The same tests should generally pass without modification—no changes to the JavaScript or Solidity files are required when switching between the mocked and real versions.

In mocked mode, actual encryption for encrypted types is not performed. Instead, the tests run on a local Hardhat node that implements the original EVM (non-fhEVM). Additionally, this mode supports all Hardhat-related testing and debugging methods, such as `evm_mine`, `evm_snapshot`, and `evm_revert`, which are highly useful for testing.

To run the mocked tests use either:

```
pnpm test:mock
```

Or equivalently:

```
npx hardhat test --network hardhat
```

In mocked mode, all tests should pass in few seconds instead of few minutes, allowing a better developer experience.

### Coverage in mocked mode

Coverage computation is only possible in mocked mode. Run the following command to compute coverage:

```
pnpm coverage:mock
```

Or equivalently:

```
npx hardhat coverage
```

After running the command, open the `coverage/index.html` file to view the results. This helps identify any missing branches not covered by the current test suite, increasing the security of your contracts.

#### Important note

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

In this snippet, the first test will always run, whether in "real" non-mocked mode (`pnpm test`), testing mocked mode (`pnpm test:mock`) or coverage (mocked) mode (`pnpm coverage:mock`). On the other hand, the second test will be run **only** in testing mocked mode(`pnpm test:mock`), because snapshots only works in that specific case.
Actually, the second test will be skipped if run in coverage mode, since its description string ends with `[skip-on-coverage]` and similarly, we avoid the test to fail in non-mocked mode since we check that the network name is `hardhat`.

### Limitations

⚠️ **Warning :** Due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhEVM, mainly in gas prices for the FHE operations. This means that before deploying to production, developers still need to run the tests with the original fhEVM node, as a final check in non-mocked mode, with `pnpm test` or `npx hardhat test`.

{% hint style="success" %}
**Zama 5-Question Developer Survey**

We want to hear from you! Take 1 minute to share your thoughts and helping us enhance our documentation and libraries. **👉** [**Click here**](https://www.zama.ai/developer-survey) to participate.
{% endhint %}
