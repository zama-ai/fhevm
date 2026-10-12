import { mine } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers } from 'hardhat';

import { readHostAddress } from '../tasks/taskHelpers';

describe('Cross-transaction HCU depth', function () {
  beforeEach(async function () {
    this.acl = await ethers.getContractAt('ACL', readHostAddress('ACL_CONTRACT_ADDRESS'));
    this.owner = await ethers.getSigner(await this.acl.owner());
    this.limit = await ethers.getContractAt('HCULimit', readHostAddress('HCU_LIMIT_CONTRACT_ADDRESS'), this.owner);
    await this.limit.setLineageDepthLimit(266_032, 1);
    const factory = await ethers.getContractFactory('HCULimitTest');
    this.parent = await factory.deploy();
    this.recipient = await factory.deploy();
    await this.parent.startLineage();
  });

  afterEach(async function () {
    await this.limit.setLineageDepthLimit(0, 0);
    await this.limit.setMaxHCUDepthPerTx(5_000_000);
  });

  it('accumulates small chained transactions while independent transactions remain allowed', async function () {
    for (let i = 0; i < 2; i++) await this.parent.advanceLineage(await this.parent.lineage());
    await expect(this.parent.advanceLineage(await this.parent.lineage())).revertedWithCustomError(
      this.limit,
      'HCULineageDepthLimitExceeded',
    );
    await this.recipient.startLineage();
    await this.recipient.advanceLineage(await this.recipient.lineage());
  });

  it('preserves depth through a cold transient forward to another contract', async function () {
    for (let i = 0; i < 2; i++) await this.parent.advanceLineage(await this.parent.lineage());
    const handle = await this.parent.lineage();
    expect(await this.acl.persistAllowed(handle, await this.recipient.getAddress())).eq(false);
    await expect(this.parent.forwardLineage(await this.recipient.getAddress())).revertedWithCustomError(
      this.limit,
      'HCULineageDepthLimitExceeded',
    );
  });

  it('preserves depth when copying persistent permission to another contract', async function () {
    for (let i = 0; i < 2; i++) await this.parent.advanceLineage(await this.parent.lineage());
    const handle = await this.parent.lineage();
    await this.parent.allowLineage(await this.recipient.getAddress());
    expect(await this.acl.persistAllowed(handle, await this.recipient.getAddress())).eq(true);
    const [, depth] = await this.acl.getPermissionDepth(handle, await this.recipient.getAddress());
    expect(depth).greaterThan(266_000);
    await expect(this.recipient.advanceLineage(handle)).revertedWithCustomError(
      this.limit,
      'HCULineageDepthLimitExceeded',
    );
  });

  it('discounts old depth according to block height rather than timestamp', async function () {
    await this.parent.advanceLineage(await this.parent.lineage());
    await this.limit.setLineageDepthLimit(266_032, 100_000);
    await mine(3);
    await this.parent.advanceLineage(await this.parent.lineage());
    const [allowed, depth, savedBlock] = await this.acl.getPermissionDepth(
      await this.parent.lineage(),
      await this.parent.getAddress(),
    );
    expect(allowed).eq(true);
    expect(depth).eq(133_000);
    expect(savedBlock).eq(await ethers.provider.getBlockNumber());
  });

  it('retains the separate transaction-local depth limit when branches have different inherited depths', async function () {
    await this.limit.setLineageDepthLimit(1_000_000, 1);
    for (let i = 0; i < 4; i++) await this.parent.advanceLineage(await this.parent.lineage());
    await this.limit.setMaxHCUDepthPerTx(300_000);
    await expect(this.parent.combineWithFreshBranch(await this.parent.lineage())).revertedWithCustomError(
      this.limit,
      'HCUTransactionDepthLimitExceeded',
    );
  });

  it('allows disabling lineage throttling without changing token permission calls', async function () {
    await this.limit.setLineageDepthLimit(0, 0);
    for (let i = 0; i < 4; i++) await this.parent.advanceLineage(await this.parent.lineage());
    expect(await this.acl.persistAllowed(await this.parent.lineage(), await this.parent.getAddress())).eq(true);
  });
});
