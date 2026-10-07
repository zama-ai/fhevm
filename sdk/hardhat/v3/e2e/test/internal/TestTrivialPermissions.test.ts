import { expect } from 'chai';
import { network } from 'hardhat';

import type { TestTrivialPermissions, TestTrivialPermissions__factory } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

describe('TestTrivialPermissions', function () {
  let signers: Signers;
  let testTrivialPermissions: TestTrivialPermissions;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async function () {
    const factory: TestTrivialPermissions__factory = await ethers.getContractFactory('TestTrivialPermissions');
    testTrivialPermissions = await factory.connect(signers.alice).deploy();
    await testTrivialPermissions.waitForDeployment();
  });

  it('should fail because missing ACL permission', async function () {
    await expect(testTrivialPermissions.connect(signers.carol).computeFheAdd()).to.be.revertedWithCustomError(
      ...fhevm.revertedWithCustomErrorArgs('FHEVMExecutor', 'ACLNotAllowed'),
    );
  });
});
