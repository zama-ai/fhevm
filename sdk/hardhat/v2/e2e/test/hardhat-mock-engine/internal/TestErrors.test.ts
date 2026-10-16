import { expect } from 'chai';
import * as hre from 'hardhat';

import type { TestErrors } from '../../../typechain-types';
import { type Signers, getSigners, initSigners } from '../signers';
import { deployTestErrorsFixture } from './TestErrors.fixture';

describe('TestErrors', function () {
  let signers: Signers;
  let testErrors: TestErrors;
  let testErrorsAddress: string;

  before(async function () {
    await initSigners();
    signers = await getSigners();
  });

  beforeEach(async function () {
    testErrors = await deployTestErrorsFixture(signers.alice);
    testErrorsAddress = await testErrors.getAddress();
  });

  it('Test ACL error permissions', async function () {
    console.log(testErrorsAddress);
    const tx = await testErrors.connect(signers.alice).initCypherTextUint64NoAllow(123);
    await tx.wait();

    await expect(testErrors.connect(signers.alice).add(456)).to.be.revertedWithCustomError(
      ...hre.fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed'),
    );
  });
});
