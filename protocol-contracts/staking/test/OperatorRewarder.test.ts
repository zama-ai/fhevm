import { impersonate } from './helpers/accounts';
import { time } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers, upgrades } from 'hardhat';
import hre from 'hardhat';

const timeIncreaseNoMine = (duration: number) =>
  time.latest().then(clock => time.setNextBlockTimestamp(clock + duration));

// DECIMAL_OFFSET is used in OperatorStaking to mitigate inflation attacks.
// This creates 10^DECIMAL_OFFSET virtual shares per asset unit.
const DECIMAL_OFFSET = 2n;
const SHARES_PER_ASSET_UNIT = 10n ** DECIMAL_OFFSET;

describe('OperatorRewarder', function () {
  beforeEach(async function () {
    const [delegator1, delegator2, claimer, admin, beneficiary, anyone, ...accounts] = await ethers.getSigners();

    const token = await ethers.deployContract('$ERC20Mock', ['StakingToken', 'ST', 18]);
    const protocolStaking = await ethers.getContractFactory('ProtocolStakingSlashingMock').then(factory =>
      upgrades.deployProxy(factory, [
        'StakedToken',
        'SST',
        '1',
        token.target,
        admin.address,
        admin.address,
        60 /* 1 min */, // unstake cooldown period
        ethers.parseEther('0.5'), // reward rate
      ]),
    );
    const operatorStaking = await ethers.getContractFactory('OperatorStaking').then(factory =>
      upgrades.deployProxy(factory, [
        'OPStake',
        'OP',
        protocolStaking.target,
        beneficiary.address,
        10000, // 100% maximum fee
        0,
      ]),
    );
    const mock = await ethers.getContractAt('OperatorRewarder', await operatorStaking.rewarder());
    await expect(mock.token()).to.eventually.eq(token.target);

    await Promise.all(
      [delegator1, delegator2].flatMap(account => [
        token.mint(account, ethers.parseEther('1000')),
        token.$_approve(account, operatorStaking, ethers.MaxUint256),
      ]),
    );

    await protocolStaking.connect(admin).addEligibleAccount(operatorStaking);

    Object.assign(this, {
      delegator1,
      delegator2,
      claimer,
      admin,
      beneficiary,
      anyone,
      accounts,
      token,
      operatorStaking,
      protocolStaking,
      mock,
    });
  });

  describe('deployment', function () {
    it('should not deploy if max fee is over 100%', async function () {
      const maxFeeTooHigh = 10001;
      await expect(
        ethers.deployContract('OperatorRewarder', [
          this.beneficiary,
          this.protocolStaking,
          this.mock,
          maxFeeTooHigh,
          0,
        ]),
      )
        .to.be.revertedWithCustomError(this.mock, 'InvalidBasisPoints')
        .withArgs(maxFeeTooHigh);
    });

    it('should not deploy if fee is over max fee', async function () {
      const maxFee = 1000;
      const feeOverMaxFee = 1234;
      await expect(
        ethers.deployContract('OperatorRewarder', [
          this.beneficiary,
          this.protocolStaking,
          this.mock,
          maxFee,
          feeOverMaxFee,
        ]),
      )
        .to.be.revertedWithCustomError(this.mock, 'MaxBasisPointsExceeded')
        .withArgs(feeOverMaxFee, maxFee);
    });
  });

  describe('transferBeneficiary', function () {
    it('should transfer beneficiary address', async function () {
      await expect(this.mock.beneficiary()).to.eventually.not.eq(this.anyone.address);
      await expect(this.mock.connect(this.admin).transferBeneficiary(this.anyone.address))
        .to.emit(this.mock, 'BeneficiaryTransferred')
        .withArgs(this.beneficiary.address, this.anyone.address);
      await expect(this.mock.beneficiary()).to.eventually.eq(this.anyone.address);
    });

    it('should not transfer beneficiary address to zero address', async function () {
      await expect(this.mock.connect(this.admin).transferBeneficiary(ethers.ZeroAddress))
        .to.be.revertedWithCustomError(this.mock, 'InvalidBeneficiary')
        .withArgs(ethers.ZeroAddress);
    });

    it('should not transfer beneficiary address to same address', async function () {
      await expect(this.mock.connect(this.admin).transferBeneficiary(this.beneficiary.address))
        .to.be.revertedWithCustomError(this.mock, 'BeneficiaryAlreadySet')
        .withArgs(this.beneficiary.address);
    });

    it('should not transfer beneficiary address if not owner', async function () {
      await expect(this.mock.connect(this.anyone).transferBeneficiary(this.anyone.address))
        .to.be.revertedWithCustomError(this.mock, 'CallerNotProtocolStakingOwner')
        .withArgs(this.anyone.address);
    });
  });

  describe('Access Control', function () {
    it('should be same owner as ProtocolStaking owner', async function () {
      const protocolStakingOwner = await this.protocolStaking.owner();
      const operatorRewarderOwner = await this.mock.owner();

      expect(operatorRewarderOwner).to.equal(protocolStakingOwner);
    });

    it('should update ProtocolStaking and OperatorStaking owner if ProtocolStaking owner is changed', async function () {
      await this.protocolStaking.connect(this.admin).beginDefaultAdminTransfer(this.anyone);
      await this.protocolStaking.connect(this.anyone).acceptDefaultAdminTransfer();

      const protocolStakingOwner = await this.protocolStaking.owner();
      const operatorRewarderOwner = await this.mock.owner();
      expect(protocolStakingOwner).to.equal(this.anyone);
      expect(operatorRewarderOwner).to.equal(this.anyone);
    });
  });

  describe('View and claim delegator reward', async function () {
    it('should give all to solo delegator', async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0'));

      await expect(this.mock.unpaidFee()).to.eventually.eq(0);
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('5'));
      await expect(this.mock.connect(this.delegator1).claimRewards(this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('5'))
        .to.emit(this.mock, 'RewardsClaimed')
        .withArgs(this.delegator1, ethers.parseEther('5'));
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(0);

      // Historical reward: 10 (seconds) * 0.5 (reward rate) = 5
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('5'));
    });

    it('should split between two equal delegators', async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.operatorStaking.connect(this.delegator2).deposit(ethers.parseEther('1'), this.delegator2);

      await timeIncreaseNoMine(9);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);

      await expect(this.mock.unpaidFee()).to.eventually.eq(0);
      await expect(this.mock.earned(this.delegator2)).to.eventually.eq(ethers.parseEther('2.25'));
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('2.75'));

      await expect(this.mock.connect(this.delegator1).claimRewards(this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('2.75'))
        .to.emit(this.mock, 'RewardsClaimed')
        .withArgs(this.delegator1, ethers.parseEther('2.75'));
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(0);

      await expect(this.mock.connect(this.delegator2).claimRewards(this.delegator2))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator2, ethers.parseEther('2.25'))
        .to.emit(this.mock, 'RewardsClaimed')
        .withArgs(this.delegator2, ethers.parseEther('2.25'));
      await expect(this.mock.earned(this.delegator2)).to.eventually.eq(0);

      // Historical reward: (1+9) (seconds) * 0.5 (reward rate) = 5
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('5'));
    });

    it('should not claim past reward after receiving new shares on transfer', async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);
      await this.mock.connect(this.delegator1).claimRewards(this.delegator1); // claims past rewards before not being able to
      const sharesToTransfer = ethers.parseEther('1') * SHARES_PER_ASSET_UNIT;
      await this.operatorStaking.connect(this.delegator1).transfer(this.delegator2, sharesToTransfer);
      // delegator1 will be able deposit and claim reward again
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(0);
      // delegator2 cannot claim any reward
      await expect(this.mock.earned(this.delegator2)).to.eventually.eq(0);

      // Historical reward: 10 (seconds) * 0.5 (reward rate) = 5
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('5'));
    });

    it('should decrease rewards appropriately for fee', async function () {
      await this.mock.connect(this.beneficiary).setFee(1000); // 10% fee
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);

      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('4.5'));
      await expect(this.mock.unpaidFee()).to.eventually.eq(ethers.parseEther('0.5'));

      await expect(this.mock.connect(this.delegator1).claimRewards(this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('4.5'))
        .to.emit(this.mock, 'RewardsClaimed')
        .withArgs(this.delegator1, ethers.parseEther('4.5'));
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(0);

      await expect(this.mock.connect(this.beneficiary).claimFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.beneficiary, ethers.parseEther('0.5'))
        .to.emit(this.mock, 'FeeClaimed')
        .withArgs(this.beneficiary, ethers.parseEther('0.5'));
      await expect(this.mock.unpaidFee()).to.eventually.eq(0);

      // Historical reward: 10 (seconds) * 0.5 (reward rate) - 0.5 (10% fee) = 4.5
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('4.5'));
    });

    it('should not trigger payment if no delegator reward', async function () {
      await this.protocolStaking.connect(this.admin).setRewardRate(0);
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await time.increase(9);

      await expect(this.mock.connect(this.delegator1).claimRewards(this.delegator1))
        .to.not.emit(this.token, 'Transfer')
        .to.not.emit(this.mock, 'RewardsClaimed');

      // Historical reward: 0 (no reward rate)
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('0'));
    });

    it('should calculate properly after full removal then delegate again', async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.operatorStaking.connect(this.delegator2).deposit(ethers.parseEther('1'), this.delegator2);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);

      await this.mock.connect(this.delegator1).claimRewards(this.delegator1);
      await this.mock.connect(this.delegator2).claimRewards(this.delegator2);

      const sharesToRedeem = ethers.parseEther('1') * SHARES_PER_ASSET_UNIT;
      await this.operatorStaking
        .connect(this.delegator1)
        .requestRedeem(sharesToRedeem, this.delegator1, this.delegator1);
      await this.operatorStaking
        .connect(this.delegator2)
        .requestRedeem(sharesToRedeem, this.delegator2, this.delegator2);
      await timeIncreaseNoMine(60);

      await this.operatorStaking.connect(this.delegator1).redeem(sharesToRedeem, this.delegator1, this.delegator1);
      await this.operatorStaking.connect(this.delegator2).redeem(sharesToRedeem, this.delegator2, this.delegator2);

      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(0);

      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0.5'));

      await time.increase(10);
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('5'));

      await expect(this.mock.connect(this.delegator1).claimRewards(this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('5.5'));
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(0);

      // Historical reward: (1+10+10+1) (seconds) * 0.5 (reward rate) = 11
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('11'));
    });

    it("should properly count rewards after pending withdrawal that's not yet redeemed", async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('3'), this.delegator1);
      await this.operatorStaking.connect(this.delegator2).deposit(ethers.parseEther('1'), this.delegator2);

      await timeIncreaseNoMine(10);

      const sharesToRedeem = ethers.parseEther('2') * SHARES_PER_ASSET_UNIT;
      await this.operatorStaking
        .connect(this.delegator1)
        .requestRedeem(sharesToRedeem, this.delegator1, this.delegator1);

      await time.increase(10);

      expect(await this.mock.earned(this.delegator1)).to.be.closeTo(ethers.parseEther('6.75'), 1n);
      await expect(this.mock.earned(this.delegator2)).to.eventually.eq(ethers.parseEther('3.75'));

      // Historical reward: (1+10) (seconds) * 0.5 (reward rate) = 10.5
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('10.5'));
    });

    it('should claim rewards with authorized claimer', async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator1).setClaimer(this.claimer);
      await expect(this.mock.claimer(this.delegator1)).to.eventually.eq(this.claimer);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0'));

      await expect(this.mock.connect(this.claimer).claimRewards(this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('5.5'));
    });

    it('should claim rewards with delegator first and then authorized claimer', async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await expect(this.mock.claimer(this.delegator1)).to.eventually.eq(this.delegator1);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0'));

      await expect(this.mock.connect(this.delegator1).claimRewards(this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('5'));

      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0.5'));
      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0'));

      await this.mock.connect(this.delegator1).setClaimer(this.claimer);

      await expect(this.mock.connect(this.claimer).claimRewards(this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('5'));
    });

    it('should not claim rewards if not authorized to claim rewards', async function () {
      await expect(this.mock.claimer(this.delegator1)).to.not.eventually.eq(this.claimer);
      await expect(this.mock.connect(this.claimer).claimRewards(this.delegator1))
        .to.be.revertedWithCustomError(this.mock, 'ClaimerNotAuthorized')
        .withArgs(this.delegator1, this.claimer);
    });
  });

  describe('View and claim owner reward', async function () {
    beforeEach(async function () {
      await this.mock.connect(this.beneficiary).setFee(1000);
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
    });

    it('should send tokens', async function () {
      await time.increase(20);

      await expect(this.mock.unpaidFee()).to.eventually.eq(ethers.parseEther('1'));
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('9'));

      // 1 more second goes by
      await expect(this.mock.connect(this.beneficiary).claimFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.beneficiary, ethers.parseEther('1.05'))
        .to.emit(this.mock, 'FeeClaimed')
        .withArgs(this.beneficiary, ethers.parseEther('1.05'));
      await expect(this.mock.unpaidFee()).to.eventually.eq(0);

      // Historical reward: (20+1) (seconds) * 0.5 (reward rate) - 1.05 (10% fee) = 9.45
      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('9.45'));
    });

    it('should reset pending fee', async function () {
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.beneficiary).claimFee();

      await expect(this.mock.unpaidFee()).to.eventually.eq(0);
    });

    it('should not effect delegator earned amount', async function () {
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.beneficiary).claimFee();

      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('4.5'));
    });

    it('should not effect historical reward', async function () {
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.beneficiary).claimFee();

      await expect(this.mock.historicalReward()).to.eventually.eq(ethers.parseEther('4.5'));
    });

    it('should process second claim accurately', async function () {
      await timeIncreaseNoMine(10);
      await expect(this.mock.connect(this.beneficiary).claimFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.beneficiary, ethers.parseEther('0.5'))
        .to.emit(this.mock, 'FeeClaimed')
        .withArgs(this.beneficiary, ethers.parseEther('0.5'));

      await timeIncreaseNoMine(5);
      await expect(this.mock.connect(this.beneficiary).claimFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.beneficiary, ethers.parseEther('0.25'))
        .to.emit(this.mock, 'FeeClaimed')
        .withArgs(this.beneficiary, ethers.parseEther('0.25'));
    });

    it('should not claim fee if not beneficiary', async function () {
      await expect(this.mock.connect(this.anyone).claimFee())
        .to.be.revertedWithCustomError(this.mock, 'CallerNotBeneficiary')
        .withArgs(this.anyone);
    });
  });

  describe('setMaxFee', async function () {
    it('should update max fee', async function () {
      await expect(this.mock.maxFeeBasisPoints()).to.eventually.eq(10000);
      await expect(this.mock.connect(this.admin).setMaxFee(1234))
        .to.emit(this.mock, 'MaxFeeUpdated')
        .withArgs(10000, 1234);
      await expect(this.mock.maxFeeBasisPoints()).to.eventually.eq(1234);
    });

    it('should set fee to max fee and claim fees if new max fee lower than current fee', async function () {
      await this.mock.connect(this.beneficiary).setFee(1000);
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await timeIncreaseNoMine(10);

      // If the new max fee is lower than the current fee:
      // - the fee is set to the new max fee
      // - the unpaid fees are claimed and transferred to the beneficiary
      await expect(this.mock.connect(this.admin).setMaxFee(500))
        .to.emit(this.mock, 'MaxFeeUpdated')
        .withArgs(10000, 500)
        .to.emit(this.mock, 'FeeUpdated')
        .withArgs(1000, 500)
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.beneficiary, ethers.parseEther('0.5'));
      await expect(this.mock.feeBasisPoints()).to.eventually.eq(500);
    });

    it('should not set max fee if not owner', async function () {
      await expect(this.mock.connect(this.anyone).setMaxFee(1234))
        .to.be.revertedWithCustomError(this.mock, 'CallerNotProtocolStakingOwner')
        .withArgs(this.anyone);
    });

    it('should revert if over 100%', async function () {
      await expect(this.mock.connect(this.admin).setMaxFee(10001))
        .to.be.revertedWithCustomError(this.mock, 'InvalidBasisPoints')
        .withArgs(10001);
    });

    it('should revert if max fee already set', async function () {
      await this.mock.connect(this.admin).setMaxFee(1000);
      await expect(this.mock.connect(this.admin).setMaxFee(1000))
        .to.be.revertedWithCustomError(this.mock, 'MaxFeeAlreadySet')
        .withArgs(1000);
    });
  });

  describe('setFee', async function () {
    it('should update storage', async function () {
      await expect(this.mock.feeBasisPoints()).to.eventually.eq(0);
      await expect(this.mock.connect(this.beneficiary).setFee(1234)).to.emit(this.mock, 'FeeUpdated').withArgs(0, 1234);
      await expect(this.mock.feeBasisPoints()).to.eventually.eq(1234);
    });

    it('should send pending fees', async function () {
      await this.mock.connect(this.beneficiary).setFee(1000);
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);

      await timeIncreaseNoMine(10);
      await expect(this.mock.connect(this.beneficiary).setFee(2000))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.beneficiary, ethers.parseEther('0.5'));
    });

    it('should accrue awards accurately after change', async function () {
      await this.mock.connect(this.beneficiary).setFee(1000);
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);

      await timeIncreaseNoMine(10);
      await this.mock.connect(this.beneficiary).setFee(2000);

      await time.increase(10);
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('8.5'));
      await expect(this.mock.unpaidFee()).to.eventually.eq(ethers.parseEther('1')); // 0.5 already sent
    });

    it('should not take fees from past rewards', async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);

      await timeIncreaseNoMine(10);
      await this.mock.connect(this.beneficiary).setFee(1000);
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('5'));
      await expect(this.mock.unpaidFee()).to.eventually.eq(0);

      await time.increase(10);
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('9.5'));
      await expect(this.mock.unpaidFee()).to.eventually.eq(ethers.parseEther('0.5'));
    });

    it('should not set fee if not beneficiary', async function () {
      await expect(this.mock.connect(this.anyone).setFee(1234))
        .to.be.revertedWithCustomError(this.mock, 'CallerNotBeneficiary')
        .withArgs(this.anyone);
    });

    it('should revert if over 100%', async function () {
      await expect(this.mock.connect(this.beneficiary).setFee(10001))
        .to.be.revertedWithCustomError(this.mock, 'MaxBasisPointsExceeded')
        .withArgs(10001, 10000);
    });

    it('should revert if fee already set', async function () {
      await this.mock.connect(this.beneficiary).setFee(1000);
      await expect(this.mock.connect(this.beneficiary).setFee(1000))
        .to.be.revertedWithCustomError(this.mock, 'FeeAlreadySet')
        .withArgs(1000);
    });

    it('should revert if over max fee', async function () {
      await this.mock.connect(this.admin).setMaxFee(1000);
      await expect(this.mock.connect(this.beneficiary).setFee(1234))
        .to.be.revertedWithCustomError(this.mock, 'MaxBasisPointsExceeded')
        .withArgs(1234, 1000);
    });
  });

  describe('setClaimer', async function () {
    it('should be caller if no claimer set', async function () {
      await expect(this.mock.claimer(this.anyone)).to.eventually.eq(this.anyone);
    });

    it('should set claimer', async function () {
      await expect(this.mock.claimer(this.anyone)).to.eventually.eq(this.anyone);
      await expect(this.mock.connect(this.anyone).setClaimer(this.claimer))
        .to.emit(this.mock, 'ClaimerAuthorized')
        .withArgs(this.anyone, this.claimer);
      await expect(this.mock.claimer(this.anyone)).to.eventually.eq(this.claimer);
    });

    it('should not set claimer to zero address', async function () {
      await expect(this.mock.connect(this.anyone).setClaimer(ethers.ZeroAddress))
        .to.be.revertedWithCustomError(this.mock, 'InvalidClaimer')
        .withArgs(ethers.ZeroAddress);
    });

    it('should not set same claimer', async function () {
      await this.mock.connect(this.anyone).setClaimer(this.claimer);

      await expect(this.mock.connect(this.anyone).setClaimer(this.claimer))
        .to.be.revertedWithCustomError(this.mock, 'ClaimerAlreadySet')
        .withArgs(this.anyone, this.claimer);
    });
  });

  describe('start', function () {
    it('should be started', async function () {
      await expect(this.mock.isStarted()).to.eventually.eq(true);
    });

    it("can't start twice", async function () {
      const signer = await impersonate(hre, this.operatorStaking.target);
      await expect(this.mock.connect(signer).start()).to.be.revertedWithCustomError(this.mock, 'AlreadyStarted');
    });

    describe('with new rewarder', async function () {
      beforeEach(async function () {
        const notStartedRewarder = await ethers.deployContract('OperatorRewarder', [
          this.beneficiary,
          this.protocolStaking,
          this.mock,
          10000, // 100% maximum fee
          0, // 0% fee
        ]);
        Object.assign(this, { notStartedRewarder });
      });

      it('should revert if started not called by OperatorStaking', async function () {
        await expect(this.notStartedRewarder.connect(this.admin).start())
          .to.be.revertedWithCustomError(this.notStartedRewarder, 'CallerNotOperatorStaking')
          .withArgs(this.admin);
      });

      it('should revert if not started for claimFee', async function () {
        await expect(this.notStartedRewarder.connect(this.beneficiary).claimFee()).to.be.revertedWithCustomError(
          this.notStartedRewarder,
          'NotStarted',
        );
      });

      it('should revert if not started for setFee', async function () {
        await expect(this.notStartedRewarder.connect(this.beneficiary).setFee(1000)).to.be.revertedWithCustomError(
          this.notStartedRewarder,
          'NotStarted',
        );
      });

      it('should revert if not started for setMaxFee', async function () {
        await expect(this.notStartedRewarder.connect(this.admin).setMaxFee(1000)).to.be.revertedWithCustomError(
          this.notStartedRewarder,
          'NotStarted',
        );
      });
    });
  });

  describe('shutdown', function () {
    beforeEach(async function () {
      await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await timeIncreaseNoMine(9);

      const newRewarder = await ethers.deployContract('OperatorRewarder', [
        this.beneficiary.address,
        this.protocolStaking,
        this.operatorStaking,
        10000, // 100% maximum fee
        0,
      ]);

      this.tx = this.operatorStaking.connect(this.admin).setRewarder(newRewarder);
    });

    it('should emit event', async function () {
      await expect(this.tx).to.emit(this.mock, 'Shutdown');
    });

    it('should set shutdown flag', async function () {
      await expect(this.mock.isShutdown()).to.eventually.eq(false);
      await this.tx;
      await expect(this.mock.isShutdown()).to.eventually.eq(true);
    });

    it('should stop accruing rewards after shutdown', async function () {
      await this.tx;

      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('5'));

      await timeIncreaseNoMine(10);
      await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('5'));
    });

    it('only callable by protocolStaking', async function () {
      await expect(this.mock.connect(this.admin).shutdown())
        .to.be.revertedWithCustomError(this.mock, 'CallerNotOperatorStaking')
        .withArgs(this.admin);
    });

    it("can't be called twice", async function () {
      await this.tx;
      const signer = await impersonate(hre, this.operatorStaking.target);
      await expect(this.mock.connect(signer).shutdown()).to.be.revertedWithCustomError(this.mock, 'AlreadyShutdown');
    });
  });

  describe('transferHook', function () {
    it('should revert if not called by operatorStaking contract', async function () {
      await expect(
        this.mock.connect(this.anyone).transferHook(this.delegator1, this.delegator1, ethers.parseEther('1')),
      )
        .to.be.revertedWithCustomError(this.mock, 'CallerNotOperatorStaking')
        .withArgs(this.anyone);
    });

    describe('should handle transfers properly', function () {
      it('one delegator to a new delegator full balance', async function () {
        await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
        await timeIncreaseNoMine(10);

        const sharesToTransfer = ethers.parseEther('1') * SHARES_PER_ASSET_UNIT;
        await this.operatorStaking.connect(this.delegator1).transfer(this.delegator2, sharesToTransfer);
        await time.increase(10);

        await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('5'));
        await expect(this.mock.earned(this.delegator2)).to.eventually.eq(ethers.parseEther('5'));
      });

      it('one delegator to a new delegator partial balance', async function () {
        await this.operatorStaking.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
        await timeIncreaseNoMine(10);

        const sharesToTransfer = ethers.parseEther('0.5') * SHARES_PER_ASSET_UNIT;
        await this.operatorStaking.connect(this.delegator1).transfer(this.delegator2, sharesToTransfer);
        await time.increase(10);

        await expect(this.mock.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('7.5'));
        await expect(this.mock.earned(this.delegator2)).to.eventually.eq(ethers.parseEther('2.5'));
      });
    });
  });
});
