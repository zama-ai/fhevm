# Using Hardhat

The best way to start writing smart contracts with fhEVM is to use our [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).
It allows you to start a fhEVM docker image and run your smart contract on it. Read the [README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md) for more information.
When developing confidential contracts, we recommend to use first the mocked version of fhEVM for faster testing with `pnpm test:mock` and coverage computation via `pnpm coverage:mock`, this will lead to a better developer experience. However, keep in mind that the mocked fhEVM has some limitations and discrepancies compared to the real fhEVM node, as explained in the warning section at the end of this page.
It's essential to run tests of the final contract version using the real fhEVM. You can do this by running `pnpm test` before deployment.

## Mocked mode

For faster testing iterations, instead of launching all the tests on the local fhEVM node via `pnpm test`or `npx hardhat test` which could last several minutes, you could use instead a mocked version of the fhEVM.
The same tests should (almost always) pass, as is, without any modification: neither the javascript files neither the solidity files need to be changed between the mocked and the real version. The mocked mode does not actually real encryption for encrypted types and runs the tests on a local hardhat node which is implementing the original EVM (i.e non-fhEVM). Additionally, the mocked mode will let you use all the hardhat related special testing/debugging methods, such as `evm_mine`, `evm_snapshot`, `evm_revert` etc, which are very helpful for testing.

To run the mocked tests use either:

```
pnpm test:mock
```

Or equivalently:

```
npx hardhat test --network hardhat
```

In mocked mode, all tests should pass in few seconds instead of few minutes, allowing a better developer experience.
Furthermore, getting the coverage of tests is only possible in mocked mode. Just use the following command:

```
pnpm coverage:mock
```

Or equivalently:

```
npx hardhat coverage
```

Then open the file `coverage/index.html` to see the coverage results. This will increase security by pointing out missing branches not covered yet by the current test suite.

**Notice :** Due to limitations in the `solidity-coverage` package, the coverage computation in fhEVM does not support tests involving the `evm_snapshot`hardhat testing method, however, this method is still supported when running tests in mocked mode! In case you are using hardhat snapshots, we recommend you to end your test description by the`[skip-on-coverage]` tag. Here is a concrete example for illustration purpose:

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

In the previous snippet, the first test will be run in every case, whether in "real" non-mocked mode (`pnpm test`), testing mocked mode (`pnpm test:mock`) or coverage (mocked) mode (`pnpm coverage:mock`). On the other hand, the second test will be run **only** in testing mocked mode, i.e only when running `pnpm test:mock`, since snapshots only works in that specific case. Actually, the second test will be skipped if run in coverage mode, since its description string ends with `[skip-on-coverage]` and similarly, we avoid the test to fail in non-mocked mode since we check that the network name is `hardhat`.

⚠️ **Warning :** Due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhEVM, the main difference is the difference in gas prices for the FHE operations. This means that before deploying to production, developers still need to run the tests with the original fhEVM node, as a final check in non-mocked mode, with `pnpm test` or `npx hardhat test`.
