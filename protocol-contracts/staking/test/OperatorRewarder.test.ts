import { impersonate } from './helpers/accounts';
import { time } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers, upgrades } from 'hardhat';
import hre from 'hardhat';

const timeIncreaseNoMine = (duration: number) =>
  time.latest().then(clock => time.setNextBlockTimestamp(clock + duration));

describe('OperatorRewarder', function () {
  beforeEach(async function () {
    const [staker1, staker2, admin, anyone, ...accounts] = await ethers.getSigners();

    const token = await ethers.deployContract('$ERC20Mock', ['StakingToken', 'ST', 18]);
    const protocolStaking = await ethers.getContractFactory('ProtocolStakingSlashingMock').then(factory =>
      upgrades.deployProxy(factory, [
        'StakedToken',
        'SST',
        '1',
        token.target,
        admin.address,
        admin.address,
        admin.address,
        60 /* 1 min */, // unstake cooldown period
        ethers.parseEther('0.5'), // reward rate
      ]),
    );
    const operatorStaking = await ethers.deployContract('$OperatorStaking', [
      'OPStake',
      'OP',
      protocolStaking,
      admin.address,
    ]);
    const mock = await ethers.getContractAt('OperatorRewarder', await operatorStaking.rewarder());
    await expect(mock.token()).to.eventually.eq(token.target);

    await Promise.all(
      [staker1, staker2].flatMap(account => [
        token.mint(account, ethers.parseEther('1000')),
        token.$_approve(account, operatorStaking, ethers.MaxUint256),
      ]),
    );

    await protocolStaking.connect(admin).addEligibleAccount(operatorStaking);

    Object.assign(this, { staker1, staker2, admin, anyone, accounts, token, operatorStaking, protocolStaking, mock });
  });

  describe('View and claim staker reward', async function () {
    it('should give all to solo staker', async function () {
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0'));

      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(0);
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('5'));
      await expect(this.mock.claimRewards(this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('5'));
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(0);
    });

    it('should split between two equal stakers', async function () {
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.operatorStaking.connect(this.staker2).deposit(ethers.parseEther('1'), this.staker2);

      await timeIncreaseNoMine(9);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);

      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(0);
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('2.25'));
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('2.75'));

      await expect(this.mock.claimRewards(this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('2.75'));
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(0);

      await expect(this.mock.claimRewards(this.staker2))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker2, ethers.parseEther('2.25'));
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(0);
    });

    it('should not claim past reward after receiving new shares on transfer', async function () {
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);
      await this.mock.claimRewards(this.staker1); // claims past rewards before not being able to
      await this.operatorStaking.connect(this.staker1).transfer(this.staker2, ethers.parseEther('1'));
      // staker1 will be able deposit and claim reward again
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(0);
      // staker2 cannot claim any reward
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(0);
    });

    it('should decrease rewards appropriately for owner fee', async function () {
      await this.mock.connect(this.admin).setOwnerFee('1000'); // 10% owner fee
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);

      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('4.5'));
      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(ethers.parseEther('0.5'));

      await expect(this.mock.claimRewards(this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('4.5'));
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(0);

      await expect(this.mock.connect(this.admin).claimOwnerFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.admin, ethers.parseEther('0.5'));
      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(0);
    });

    it('should not trigger payment if no staker reward', async function () {
      await this.protocolStaking.connect(this.admin).setRewardRate(0);
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await time.increase(9);

      await expect(this.mock.claimRewards(this.staker1)).to.not.emit(this.token, 'Transfer');
    });

    it('should calculate properly after full removal then restake', async function () {
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.operatorStaking.connect(this.staker2).deposit(ethers.parseEther('1'), this.staker2);

      await timeIncreaseNoMine(10);
      await this.protocolStaking.connect(this.admin).setRewardRate(0);

      await this.mock.claimRewards(this.staker1);
      await this.mock.claimRewards(this.staker2);

      await this.operatorStaking
        .connect(this.staker1)
        .requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);
      await this.operatorStaking
        .connect(this.staker2)
        .requestRedeem(ethers.parseEther('1'), this.staker2, this.staker2);
      await timeIncreaseNoMine(60);

      await this.operatorStaking.connect(this.staker1).redeem(ethers.parseEther('1'), this.staker1, this.staker1);
      await this.operatorStaking.connect(this.staker2).redeem(ethers.parseEther('1'), this.staker2, this.staker2);

      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(0);

      await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0.5'));

      await time.increase(10);
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('5'));

      await expect(this.mock.claimRewards(this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('5.5'));
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(0);
    });

    it("should properly count rewards after pending withdrawal that's not yet redeemed", async function () {
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('3'), this.staker1);
      await this.operatorStaking.connect(this.staker2).deposit(ethers.parseEther('1'), this.staker2);

      await timeIncreaseNoMine(10);

      await this.operatorStaking
        .connect(this.staker1)
        .requestRedeem(ethers.parseEther('2'), this.staker1, this.staker1);

      await time.increase(10);

      expect(await this.mock.earned(this.staker1)).to.be.closeTo(ethers.parseEther('6.75'), 1n);
      await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('3.75'));
    });
  });

  describe('View and claim owner reward', async function () {
    beforeEach(async function () {
      await this.mock.connect(this.admin).setOwnerFee(1000);
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
    });

    it('should send tokens', async function () {
      await time.increase(20);

      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(ethers.parseEther('1'));
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('9'));

      // 1 more second goes by
      await expect(this.mock.connect(this.admin).claimOwnerFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.admin, ethers.parseEther('1.05'));
      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(0);
    });

    it('should reset pending owner fee', async function () {
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.admin).claimOwnerFee();

      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(0);
    });

    it('should not effect staker earned amount', async function () {
      await timeIncreaseNoMine(10);
      await this.mock.connect(this.admin).claimOwnerFee();

      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('4.5'));
    });

    it('should process second claim accurately', async function () {
      await timeIncreaseNoMine(10);
      await expect(this.mock.connect(this.admin).claimOwnerFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.admin, ethers.parseEther('0.5'));

      await timeIncreaseNoMine(5);
      await expect(this.mock.connect(this.admin).claimOwnerFee())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.admin, ethers.parseEther('0.25'));
    });

    it('should not claim owner fee if not owner', async function () {
      await expect(this.mock.connect(this.anyone).claimOwnerFee())
        .to.be.revertedWithCustomError(this.mock, 'OwnableUnauthorizedAccount')
        .withArgs(this.anyone);
    });
  });

  describe('setOwnerFee', async function () {
    it('should update storage', async function () {
      await expect(this.mock.ownerFeeBasisPoints()).to.eventually.eq(0);
      await expect(this.mock.connect(this.admin).setOwnerFee(1234))
        .to.emit(this.mock, 'OwnerFeeUpdated')
        .withArgs(0, 1234);
      await expect(this.mock.ownerFeeBasisPoints()).to.eventually.eq(1234);
    });

    it('should send pending fees', async function () {
      await this.mock.connect(this.admin).setOwnerFee(1000);
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);

      await timeIncreaseNoMine(10);
      await expect(this.mock.connect(this.admin).setOwnerFee(2000))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.admin, ethers.parseEther('0.5'));
    });

    it('should accrue awards accurately after change', async function () {
      await this.mock.connect(this.admin).setOwnerFee(1000);
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);

      await timeIncreaseNoMine(10);
      await this.mock.connect(this.admin).setOwnerFee(2000);

      await time.increase(10);
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('8.5'));
      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(ethers.parseEther('1')); // 0.5 already sent
    });

    it('should not take fees from past rewards', async function () {
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);

      await timeIncreaseNoMine(10);
      await this.mock.connect(this.admin).setOwnerFee(1000);
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('5'));
      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(0);

      await time.increase(10);
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('9.5'));
      await expect(this.mock.unpaidOwnerFee()).to.eventually.eq(ethers.parseEther('0.5'));
    });

    it('should not set owner fee if not owner', async function () {
      await expect(this.mock.connect(this.anyone).setOwnerFee(1234))
        .to.be.revertedWithCustomError(this.mock, 'OwnableUnauthorizedAccount')
        .withArgs(this.anyone);
    });

    it('should revert if over 100%', async function () {
      await expect(this.mock.connect(this.admin).setOwnerFee(10001))
        .to.be.revertedWithCustomError(this.mock, 'InvalidBasisPoints')
        .withArgs(10001);
    });
  });

  describe('shutdown', function () {
    beforeEach(async function () {
      await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await timeIncreaseNoMine(9);

      const newRewarder = await ethers.deployContract('OperatorRewarder', [
        this.admin,
        this.protocolStaking,
        this.operatorStaking,
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

      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('5'));

      await timeIncreaseNoMine(10);
      await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('5'));
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
      await expect(this.mock.connect(this.anyone).transferHook(this.staker1, this.staker1, ethers.parseEther('1')))
        .to.be.revertedWithCustomError(this.mock, 'CallerNotOperatorStaking')
        .withArgs(this.anyone);
    });

    describe('should handle transfers properly', function () {
      it('one staker to a new staker full balance', async function () {
        await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
        await timeIncreaseNoMine(10);

        await this.operatorStaking.connect(this.staker1).transfer(this.staker2, ethers.parseEther('1'));
        await time.increase(10);

        await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('5'));
        await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('5'));
      });

      it('one staker to a new staker partial balance', async function () {
        await this.operatorStaking.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
        await timeIncreaseNoMine(10);

        await this.operatorStaking.connect(this.staker1).transfer(this.staker2, ethers.parseEther('0.5'));
        await time.increase(10);

        await expect(this.mock.earned(this.staker1)).to.eventually.eq(ethers.parseEther('7.5'));
        await expect(this.mock.earned(this.staker2)).to.eventually.eq(ethers.parseEther('2.5'));
      });
    });
  });
});
