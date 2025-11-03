import { anyValue } from '@nomicfoundation/hardhat-chai-matchers/withArgs';
import { mine, time } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers, upgrades } from 'hardhat';

const timeIncreaseNoMine = (duration: number) =>
  time.latest().then(clock => time.setNextBlockTimestamp(clock + duration));
const eligibleAccountRole = ethers.id('ELIGIBLE_ACCOUNT_ROLE');

/* eslint-disable no-unexpected-multiline */
describe('Protocol Staking', function () {
  beforeEach(async function () {
    const [staker1, staker2, admin, upgrader, manager, anyone, ...accounts] = await ethers.getSigners();
    const token = await ethers.deployContract('$ERC20Mock', ['StakingToken', 'ST', 18]);
    const mock = await ethers
      .getContractFactory('ProtocolStaking')
      .then(factory =>
        upgrades.deployProxy(factory, [
          'StakedToken',
          'SST',
          '1',
          token.target,
          admin.address,
          upgrader.address,
          manager.address,
          1,
        ]),
      );

    await Promise.all(
      [staker1, staker2].flatMap(account => [
        token.mint(account, ethers.parseEther('1001')),
        token.$_approve(account, mock, ethers.MaxUint256),
      ]),
    );

    Object.assign(this, {
      accounts,
      staker1,
      staker2,
      admin,
      upgrader,
      manager,
      anyone,
      token,
      mock,
    });
  });

  it('unstake cooldown period returned correctly', async function () {
    await expect(this.mock.unstakeCooldownPeriod()).to.eventually.eq(1);
    await this.mock.connect(this.manager).setUnstakeCooldownPeriod(100);
    await expect(this.mock.unstakeCooldownPeriod()).to.eventually.eq(100);
  });

  it('should return reward rate', async function () {
    await expect(this.mock.rewardRate()).to.eventually.eq(0);
    await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
    await expect(this.mock.rewardRate()).to.eventually.eq(ethers.parseEther('0.5'));
  });

  describe('Access Control', function () {
    it('should not set unstake cooldown period if not authorized', async function () {
      await expect(this.mock.connect(this.anyone).setUnstakeCooldownPeriod(100)).to.be.revertedWithCustomError(
        this.mock,
        'AccessControlUnauthorizedAccount',
      );
    });

    it('should not set reward rate if not authorized', async function () {
      await expect(this.mock.connect(this.anyone).setRewardRate(100)).to.be.revertedWithCustomError(
        this.mock,
        'AccessControlUnauthorizedAccount',
      );
    });

    it('should not add eligible account if not authorized', async function () {
      await expect(this.mock.connect(this.anyone).addEligibleAccount(this.staker1)).to.be.revertedWithCustomError(
        this.mock,
        'AccessControlUnauthorizedAccount',
      );
    });

    it('should not remove eligible account if not authorized', async function () {
      await expect(this.mock.connect(this.anyone).removeEligibleAccount(this.staker1)).to.be.revertedWithCustomError(
        this.mock,
        'AccessControlUnauthorizedAccount',
      );
    });

    it('should not grant eligible account role if not role admin', async function () {
      await expect(
        this.mock.connect(this.admin).grantRole(eligibleAccountRole, this.anyone.address),
      ).to.be.revertedWithCustomError(this.mock, 'AccessControlUnauthorizedAccount');
      await this.mock.connect(this.manager).grantRole(eligibleAccountRole, this.anyone.address);
    });

    it('should not revoke eligible account role if not role admin', async function () {
      await this.mock.connect(this.manager).grantRole(eligibleAccountRole, this.anyone.address);
      await expect(
        this.mock.connect(this.admin).revokeRole(eligibleAccountRole, this.anyone.address),
      ).to.be.revertedWithCustomError(this.mock, 'AccessControlUnauthorizedAccount');
      await this.mock.connect(this.manager).revokeRole(eligibleAccountRole, this.anyone.address);
    });

    it('should not upgrade if not authorized', async function () {
      await expect(
        this.mock.connect(this.anyone).upgradeToAndCall(this.mock.target, '0x'),
      ).to.be.revertedWithCustomError(this.mock, 'AccessControlUnauthorizedAccount');
    });
  });

  describe('Staking', function () {
    it('should emit event on stake', async function () {
      await expect(this.mock.connect(this.staker1).stake(ethers.parseEther('100')))
        .to.emit(this.mock, 'TokensStaked')
        .withArgs(this.staker1, ethers.parseEther('100'))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.staker1, this.mock, ethers.parseEther('100'));
      await expect(this.mock.balanceOf(this.staker1)).to.eventually.equal(ethers.parseEther('100'));
    });

    it("should not reward accounts that aren't eligible", async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));

      // Reward 0.5 tokens per block in aggregate
      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await timeIncreaseNoMine(10);

      await expect(this.mock.totalStakedWeight()).to.eventually.equal(0);
      await expect(this.mock.connect(this.staker1).earned(this.staker1)).to.eventually.equal(0);
    });

    it('earned should not revert if account staked but not yet earned rewards', async function () {
      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.001'));
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);
      await this.mock.connect(this.staker1).stake(ethers.parseEther('1000'));
      await timeIncreaseNoMine(60 * 60 * 24); // increase time by 1 day
      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0'));
      await this.mock.connect(this.staker2).stake(ethers.parseEther('1000'));
      await this.mock.connect(this.staker1).stake(ethers.parseEther('1'));
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(0);
    });

    it('Single user should get 100% of rewards', async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));

      // Reward 0.5 tokens per block in aggregate
      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.manager).setRewardRate(0);
      await expect(this.mock.totalStakedWeight()).to.eventually.equal(
        await this.mock.weight(await this.mock.balanceOf(this.staker1)),
      );
      expect(await this.mock.earned(this.staker1)).to.be.equal(ethers.parseEther('5'));
    });

    it('Two users should split rewards according to sqrt', async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker2).stake(ethers.parseEther('1000'));

      // Reward 0.5 tokens per block in aggregate
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);
      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.manager).setRewardRate(0);

      const earned1 = await this.mock.earned(this.staker1);
      const earned2 = await this.mock.earned(this.staker2);

      expect(earned1 + earned2).to.be.lessThanOrEqual(ethers.parseEther('5'));
      expect(earned1 + earned2).to.be.closeTo(ethers.parseEther('5'), 1n);

      expect((1000n * earned2) / earned1).to.be.closeTo(Math.round((1000 * Math.sqrt(1000)) / Math.sqrt(100)), 5n);
    });

    it('Second staker should not get reward from previous period', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      // Reward 0.5 tokens per block in aggregate
      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      // staker1 stakes early and stars accumulating rewards
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await timeIncreaseNoMine(10);
      // staker2 stakes late
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));
      await timeIncreaseNoMine(10);
      // stop rewards
      await this.mock.connect(this.manager).setRewardRate(0);

      const earned1 = await this.mock.earned(this.staker1);
      const earned2 = await this.mock.earned(this.staker2);

      expect(earned1 + earned2).to.be.equal(ethers.parseEther('10'));
      expect(earned1).to.be.closeTo(earned2 * 3n, 5n);
    });
  });

  describe('Unstaking', function () {
    beforeEach(async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker2).stake(ethers.parseEther('1000'));
    });

    it('should not transfer instantly', async function () {
      await this.mock.connect(this.manager).setUnstakeCooldownPeriod(60); // 1 minute
      await expect(this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('50')))
        .to.emit(this.mock, 'Transfer')
        .withArgs(this.staker1, ethers.ZeroAddress, ethers.parseEther('50'))
        .to.not.emit(this.token, 'Transfer');
    });

    it('should be able to unstake to someone else', async function () {
      const currentTime = BigInt(await time.latest());
      const cooldownPeriod = BigInt(await this.mock.unstakeCooldownPeriod());
      await expect(this.mock.connect(this.staker1).unstake(this.staker2, ethers.parseEther('50')))
        .to.emit(this.mock, 'TokensUnstaked')
        .withArgs(this.staker1, this.staker2, ethers.parseEther('50'), currentTime + cooldownPeriod + 1n);
      await mine();
      await expect(this.mock.release(this.staker2))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker2, ethers.parseEther('50'));
    });

    it('should not unstake to zero address', async function () {
      await expect(
        this.mock.connect(this.staker1).unstake(ethers.ZeroAddress, ethers.parseEther('50')),
      ).to.be.revertedWithCustomError(this.mock, 'InvalidUnstakeRecipient');
    });

    describe('Release', function () {
      it('should transfer after cooldown complete', async function () {
        await this.mock.connect(this.manager).setUnstakeCooldownPeriod(60); // 1 minute
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('50'));
        await expect(this.mock.awaitingRelease(this.staker1)).to.eventually.eq(ethers.parseEther('50'));

        await timeIncreaseNoMine(60);

        const tx = await this.mock.release(this.staker1);
        await expect(tx).to.changeTokenBalance(this.token, this.staker1, ethers.parseEther('50'));
        await expect(tx).to.emit(this.mock, 'TokensReleased').withArgs(this.staker1, ethers.parseEther('50'));
        await expect(this.mock.awaitingRelease(this.staker1)).to.eventually.eq(ethers.parseEther('0'));
      });

      it('should only release once', async function () {
        await this.mock.connect(this.manager).setUnstakeCooldownPeriod(60); // 1 minute
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('50'));

        await timeIncreaseNoMine(60);

        await expect(this.mock.release(this.staker1)).to.changeTokenBalance(
          this.token,
          this.staker1,
          ethers.parseEther('50'),
        );
        await expect(this.mock.release(this.staker1)).to.not.emit(this.token, 'Transfer');
      });

      it("should not release if cooldown isn't complete", async function () {
        await this.mock.connect(this.manager).setUnstakeCooldownPeriod(60);
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('50'));

        await timeIncreaseNoMine(30);
        await expect(this.mock.release(this.staker1)).to.not.emit(this.token, 'Transfer');
      });

      it('should combine multiple complete withdrawals', async function () {
        await this.mock.connect(this.manager).setUnstakeCooldownPeriod(60); // 1 minute
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('50'));

        await timeIncreaseNoMine(30);
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('50'));
        await expect(this.mock.awaitingRelease(this.staker1)).to.eventually.eq(ethers.parseEther('100'));

        await timeIncreaseNoMine(60);
        await expect(this.mock.release(this.staker1))
          .to.emit(this.token, 'Transfer')
          .withArgs(this.mock, this.staker1, ethers.parseEther('100'));
        await expect(this.mock.awaitingRelease(this.staker1)).to.eventually.eq(ethers.parseEther('0'));
      });

      it('should only release completed cooldowns in batch', async function () {
        await this.mock.connect(this.manager).setUnstakeCooldownPeriod(60); // 1 minute
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('25'));

        await timeIncreaseNoMine(20);
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('25'));

        await timeIncreaseNoMine(20);
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('25'));

        await timeIncreaseNoMine(40);
        await expect(this.mock.release(this.staker1))
          .to.emit(this.token, 'Transfer')
          .withArgs(this.mock, this.staker1, ethers.parseEther('50'));
      });

      it('should handle decrease in cooldown period gracefully', async function () {
        await this.mock.connect(this.manager).setUnstakeCooldownPeriod(120);
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('25'));

        await timeIncreaseNoMine(30);
        await this.mock.connect(this.manager).setUnstakeCooldownPeriod(30);
        await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('25'));

        // advance 30 seconds. Still need to wait another 60 seconds for the original unstake request to complete.
        await timeIncreaseNoMine(30);
        await expect(this.mock.release(this.staker1)).to.not.emit(this.token, 'Transfer');

        await timeIncreaseNoMine(60);
        await expect(this.mock.release(this.staker1))
          .to.emit(this.token, 'Transfer')
          .withArgs(this.mock, this.staker1, ethers.parseEther('50'));
      });
    });

    it('should decrease total staking amount log accordingly', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);

      const beforetotalStakedWeight = await this.mock.totalStakedWeight();
      const beforeStaker1Log = await this.mock.weight(await this.mock.balanceOf(this.staker1));
      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('75'));
      const afterStaker1Log = await this.mock.weight(await this.mock.balanceOf(this.staker1));
      const aftertotalStakedWeight = await this.mock.totalStakedWeight();
      expect(beforetotalStakedWeight - aftertotalStakedWeight).to.equal(beforeStaker1Log - afterStaker1Log);
    });
  });

  describe('Claim Rewards', function () {
    it('should mint from null address', async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));

      // Reward 0.5 tokens per block in aggregate
      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.manager).setRewardRate(0);
      const earned = await this.mock.earned(this.staker1);
      await expect(this.mock.claimRewards(this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(ethers.ZeroAddress, this.staker1, earned)
        .to.emit(this.mock, 'RewardsClaimed')
        .withArgs(this.staker1, this.staker1, earned);
    });

    it('should be able to set recipient', async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker1).setRewardsRecipient(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await timeIncreaseNoMine(10);

      await expect(this.mock.claimRewards(this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(ethers.ZeroAddress, this.staker2, anyValue)
        .to.emit(this.mock, 'RewardsClaimed')
        .withArgs(this.staker1, this.staker2, anyValue);
    });
  });

  describe('all stakers unstake', function () {
    it('then new staker stakes', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));

      // Staker 1 stakes ... wait (10 second) ... and unstakes
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));

      // time passes (10 seconds) while no one is staked
      await timeIncreaseNoMine(10);

      // Staker 2 stakes ... wait (1 second)
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await time.increase(1);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('5.0')); // 0.5 * 10
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('0.5')); // 0.5 * 1
    });

    it('then old staker returns', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      // 3 in rewards for 1 (since 1 block at the beginning alone)
      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));
      // 3 in rewards for 2 (since 1 block at the end alone)
      await this.mock.connect(this.staker2).unstake(this.staker2, ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('3.0'));
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('3.0'));
      await timeIncreaseNoMine(100);

      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await time.increase(1);

      // staker 0.5 gets one more from the extra block
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('3.5'));
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('3.0'));
    });
  });

  describe('all stakers unstake', function () {
    it('then new staker stakes', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));

      // Staker 1 stakes ... wait (10 second) ... and unstakes
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));

      // time passes (10 seconds) while no one is staked
      await timeIncreaseNoMine(10);

      // Staker 2 stakes ... wait (1 second)
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await time.increase(1);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('5.0')); // 0.5 * 10
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('0.5')); // 0.5 * 1
    });

    it('then old staker returns', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      // 3 in rewards for 1 (since 1 block at the beginning alone)
      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));
      // 3 in rewards for 2 (since 1 block at the end alone)
      await this.mock.connect(this.staker2).unstake(this.staker2, ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('3.0'));
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('3.0'));
      await timeIncreaseNoMine(100);

      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await time.increase(1);

      // staker 0.5 gets one more from the extra block
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('3.5'));
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('3.0'));
    });
  });

  describe('all stakers unstake', function () {
    it('then new staker stakes', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));

      // Staker 1 stakes ... wait (10 second) ... and unstakes
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));

      // time passes (10 seconds) while no one is staked
      await timeIncreaseNoMine(10);

      // Staker 2 stakes ... wait (1 second)
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await time.increase(1);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('5.0')); // 0.5 * 10
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('0.5')); // 0.5 * 1
    });

    it('then old staker returns', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      // 3 in rewards for 1 (since 1 block at the beginning alone)
      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));
      // 3 in rewards for 2 (since 1 block at the end alone)
      await this.mock.connect(this.staker2).unstake(this.staker2, ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('3.0'));
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('3.0'));
      await timeIncreaseNoMine(100);

      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await time.increase(1);

      // staker 0.5 gets one more from the extra block
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('3.5'));
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('3.0'));
    });
  });

  describe('all stakers unstake', function () {
    it('then new staker stakes', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));

      // Staker 1 stakes ... wait (10 second) ... and unstakes
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));

      // time passes (10 seconds) while no one is staked
      await timeIncreaseNoMine(10);

      // Staker 2 stakes ... wait (1 second)
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await time.increase(1);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('5.0')); // 0.5 * 10
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('0.5')); // 0.5 * 1
    });

    it('then old staker returns', async function () {
      await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
      await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

      await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker2).stake(ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      // 3 in rewards for 1 (since 1 block at the beginning alone)
      await this.mock.connect(this.staker1).unstake(this.staker1, ethers.parseEther('100'));
      // 3 in rewards for 2 (since 1 block at the end alone)
      await this.mock.connect(this.staker2).unstake(this.staker2, ethers.parseEther('100'));

      await timeIncreaseNoMine(10);

      await expect(this.mock.earned(this.staker1)).to.eventually.equal(ethers.parseEther('3.0'));
      await expect(this.mock.earned(this.staker2)).to.eventually.equal(ethers.parseEther('3.0'));
      await timeIncreaseNoMine(100);

      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await time.increase(1);

      // staker 0.5 gets one more from the extra block
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('3.5'));
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('3.0'));
    });
  });

  describe('Manage Eligible Accounts', function () {
    describe('Add Eligible Account', function () {
      it('should emit event', async function () {
        await expect(this.mock.connect(this.manager).addEligibleAccount(this.staker1))
          .to.emit(this.mock, 'RoleGranted')
          .withArgs(eligibleAccountRole, this.staker1, this.manager);
      });

      it('should reflect in eligible account storage', async function () {
        await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
        await this.mock.connect(this.manager).addEligibleAccount(this.staker2);

        await expect(this.mock.isEligibleAccount(this.staker1)).to.eventually.equal(true);
        await expect(this.mock.isEligibleAccount(this.staker2)).to.eventually.equal(true);
        await expect(this.mock.isEligibleAccount(this.admin)).to.eventually.equal(false);
      });

      it('should add to totalStakedWeight', async function () {
        const weightBefore = await this.mock.totalStakedWeight();
        const staker1Weight = await this.mock.weight(await this.mock.balanceOf(this.staker1));
        await this.mock.connect(this.manager).addEligibleAccount(this.staker1);

        await expect(this.mock.totalStakedWeight()).to.eventually.eq(weightBefore + staker1Weight);
      });

      it("can't add zero address", async function () {
        await expect(this.mock.connect(this.manager).addEligibleAccount(ethers.ZeroAddress))
          .to.be.revertedWithCustomError(this.mock, 'InvalidEligibleAccount')
          .withArgs(ethers.ZeroAddress);
      });
    });

    describe('Remove Eligible Account', function () {
      beforeEach(async function () {
        await this.mock.connect(this.manager).addEligibleAccount(this.staker1);
        await this.mock.connect(this.manager).addEligibleAccount(this.staker2);
      });

      it('should emit event', async function () {
        await expect(this.mock.connect(this.manager).removeEligibleAccount(this.staker1))
          .to.emit(this.mock, 'RoleRevoked')
          .withArgs(eligibleAccountRole, this.staker1, this.manager);
      });

      it('should reflect in eligible account list', async function () {
        await this.mock.connect(this.manager).removeEligibleAccount(this.staker1);

        await expect(this.mock.isEligibleAccount(this.staker1)).to.eventually.equal(false);
        await expect(this.mock.isEligibleAccount(this.staker2)).to.eventually.equal(true);
      });

      it('should deduct from totalStakedWeight', async function () {
        const weightBefore = await this.mock.totalStakedWeight();
        const staker1Weight = await this.mock.weight(await this.mock.balanceOf(this.staker1));
        await this.mock.connect(this.manager).removeEligibleAccount(this.staker1);

        await expect(this.mock.totalStakedWeight()).to.eventually.eq(weightBefore - staker1Weight);
      });

      it('should retain rewards after removed as an eligible account', async function () {
        await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
        await this.mock.connect(this.manager).setRewardRate(ethers.parseEther('0.5'));
        await timeIncreaseNoMine(10);

        await this.mock.connect(this.manager).removeEligibleAccount(this.staker1);
        await time.increase(100);

        expect(await this.mock.earned(this.staker1)).to.be.equal(ethers.parseEther('5'));
      });
    });
  });

  it('set cooldown period should revert for 0', async function () {
    await expect(this.mock.connect(this.manager).setUnstakeCooldownPeriod(0)).to.be.revertedWithCustomError(
      this.mock,
      'InvalidUnstakeCooldownPeriod',
    );
  });

  describe('Transfer', function () {
    it('transfer is disabled', async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await expect(this.mock.connect(this.staker1).transfer(this.staker2, 100)).to.be.revertedWithCustomError(
        this.mock,
        'TransferDisabled',
      );
    });

    it('transferFrom is disabled', async function () {
      await this.mock.connect(this.staker1).stake(ethers.parseEther('100'));
      await this.mock.connect(this.staker1).approve(this.staker2, ethers.MaxUint256);
      await expect(
        this.mock.connect(this.staker2).transferFrom(this.staker1, this.staker2, 100),
      ).to.be.revertedWithCustomError(this.mock, 'TransferDisabled');
    });
  });
});
