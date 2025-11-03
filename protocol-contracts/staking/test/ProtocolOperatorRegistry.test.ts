import { expect } from 'chai';
import { ethers } from 'hardhat';

describe('ProtocolOperatorRegistry', function () {
  beforeEach(async function () {
    const [owner, receiver, thief, ...accounts] = await ethers.getSigners();

    const ownable = await ethers.deployContract('$OwnableMock', [owner]);
    const mock = await ethers.deployContract('ProtocolOperatorRegistry');

    Object.assign(this, { accounts, owner, receiver, thief, ownable, mock });
  });

  it('should accept legitimate claim', async function () {
    await expect(this.mock.connect(this.owner).setStakingAccount(this.ownable))
      .to.emit(this.mock, 'StakingAccountSet')
      .withArgs(this.owner, ethers.ZeroAddress, this.ownable);

    await expect(this.mock.operator(this.ownable)).to.eventually.eq(this.owner);
    await expect(this.mock.stakingAccount(this.owner)).to.eventually.eq(this.ownable);
  });

  it('should reject claim of unowned account', async function () {
    await expect(this.mock.connect(this.thief).setStakingAccount(this.ownable)).to.be.revertedWithCustomError(
      this.mock,
      'StakingAccountNotOwnedByCaller',
    );
  });

  it('should unregister claim on zero claim', async function () {
    await this.mock.connect(this.owner).setStakingAccount(this.ownable);
    await expect(this.mock.connect(this.owner).setStakingAccount(ethers.ZeroAddress))
      .to.emit(this.mock, 'StakingAccountSet')
      .withArgs(this.owner, this.ownable, ethers.ZeroAddress);

    await expect(this.mock.operator(this.ownable)).to.eventually.eq(ethers.ZeroAddress);
    await expect(this.mock.stakingAccount(this.owner)).to.eventually.eq(ethers.ZeroAddress);
  });

  it('should register as operator of staking account by new owner', async function () {
    await this.mock.connect(this.owner).setStakingAccount(this.ownable);
    await this.ownable.$_transferOwnership(this.receiver);

    await expect(this.mock.connect(this.receiver).setStakingAccount(this.ownable))
      .to.emit(this.mock, 'StakingAccountSet')
      .withArgs(this.owner, this.ownable, ethers.ZeroAddress)
      .to.emit(this.mock, 'StakingAccountSet')
      .withArgs(this.receiver, ethers.ZeroAddress, this.ownable);

    await expect(this.mock.operator(this.ownable)).to.eventually.eq(this.receiver);
    await expect(this.mock.stakingAccount(this.receiver)).to.eventually.eq(this.ownable);
    await expect(this.mock.stakingAccount(this.owner)).to.eventually.eq(ethers.ZeroAddress);
  });

  it('should be able to transfer staking account', async function () {
    await this.mock.connect(this.owner).setStakingAccount(this.ownable);
    await this.ownable.$_transferOwnership(this.receiver);
    await this.mock.connect(this.owner).setStakingAccount(ethers.ZeroAddress);
    await this.mock.connect(this.receiver).setStakingAccount(this.ownable);
  });
});
