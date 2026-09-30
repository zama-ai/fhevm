import { expect } from 'chai';
import { network } from 'hardhat';

import type { TestErrors, TestErrors__factory } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

describe('TestErrors', function () {
  let signers: Signers;
  let testErrors: TestErrors;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async function () {
    const factory: TestErrors__factory = await ethers.getContractFactory('TestErrors');
    testErrors = await factory.connect(signers.alice).deploy();
    await testErrors.waitForDeployment();
  });

  it('Test ACL error permissions', async function () {
    const tx = await testErrors.connect(signers.alice).initCypherTextUint64NoAllow(123);
    await tx.wait();

    await expect(testErrors.connect(signers.alice).add(456)).to.be.revertedWithCustomError(
      ...fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed'),
    );
  });
});
