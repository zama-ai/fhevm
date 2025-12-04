import { anyValue } from '@nomicfoundation/hardhat-chai-matchers/withArgs';
import { time } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers, upgrades } from 'hardhat';

const timeIncreaseNoMine = (duration: number) =>
  time.latest().then(clock => time.setNextBlockTimestamp(clock + duration));

describe('OperatorStaking', function () {
  beforeEach(async function () {
    const [staker1, staker2, admin, ...accounts] = await ethers.getSigners();

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
        0n, // reward rate
      ]),
    );
    const mock = await ethers.deployContract('$OperatorStaking', ['OPStake', 'OP', protocolStaking, admin.address]);

    await Promise.all(
      [staker1, staker2].flatMap(account => [
        token.mint(account, ethers.parseEther('1000')),
        token.$_approve(account, mock, ethers.MaxUint256),
      ]),
    );

    Object.assign(this, { staker1, staker2, admin, accounts, token, protocolStaking, mock });
  });

  describe('deposit', async function () {
    it('should stake into protocol staking', async function () {
      await expect(this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.protocolStaking, ethers.parseEther('1'));
    });

    it('should mint shares', async function () {
      await expect(this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1))
        .to.emit(this.mock, 'Transfer')
        .withArgs(ethers.ZeroAddress, this.staker1, ethers.parseEther('1'));
    });

    it('should pull tokens', async function () {
      await expect(this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.staker1, this.mock, ethers.parseEther('1'));
    });
  });

  describe('redeem', async function () {
    it('simple redemption', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock
        .connect(this.staker1)
        .requestRedeem(await this.mock.balanceOf(this.staker1), this.staker1, this.staker1);

      await expect(this.mock.pendingRedeemRequest(0, this.staker1)).to.eventually.eq(ethers.parseEther('1'));
      await expect(this.mock.claimableRedeemRequest(0, this.staker1)).to.eventually.eq(0);

      await time.increase(60);

      await expect(this.mock.pendingRedeemRequest(0, this.staker1)).to.eventually.eq(0);
      await expect(this.mock.claimableRedeemRequest(0, this.staker1)).to.eventually.eq(ethers.parseEther('1'));

      await expect(this.mock.connect(this.staker1).redeem(ethers.parseEther('1'), this.staker1, this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('1'));
      await expect(this.token.balanceOf(this.mock)).to.eventually.be.eq(0);
    });

    it('zero redemption should terminate early', async function () {
      await expect(this.mock.connect(this.staker1).requestRedeem(0, this.staker1, this.staker1)).to.not.emit(
        this.mock,
        'RedeemRequest',
      );
    });

    it('should not redeem twice', async function () {
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('5'), this.staker2);
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('10'), this.staker1);
      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);
      await this.mock.connect(this.staker2).requestRedeem(ethers.parseEther('1'), this.staker2, this.staker2);

      await timeIncreaseNoMine(60);

      await expect(this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('1'));
      await expect(this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1)).to.not.emit(
        this.token,
        'Transfer',
      );
    });

    it('should revert on redeem more than available', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('10'), this.staker1);
      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);

      await timeIncreaseNoMine(10);
      await expect(this.mock.connect(this.staker1).redeem(ethers.parseEther('1'), this.staker1, this.staker1))
        .to.be.revertedWithCustomError(this.mock, 'ERC4626ExceededMaxRedeem')
        .withArgs(this.staker1, ethers.parseEther('1'), 0);
    });

    it('should be able to redeem a second time', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('10'), this.staker1);
      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);

      await timeIncreaseNoMine(60);

      await expect(this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('1'));

      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('2'), this.staker1, this.staker1);

      await timeIncreaseNoMine(60);

      await expect(this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.staker1, ethers.parseEther('2'));
    });

    it('via separate controller', async function () {
      const controller = this.accounts[0];
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('10'), this.staker1);
      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), controller, this.staker1);

      await timeIncreaseNoMine(60);

      await expect(this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1)).to.not.emit(
        this.token,
        'Transfer',
      );
      await expect(
        this.mock.connect(this.staker1).redeem(ethers.MaxUint256, controller, controller),
      ).to.be.revertedWithCustomError(this.mock, 'Unauthorized');
      await expect(this.mock.connect(controller).redeem(ethers.MaxUint256, controller, controller))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, controller, ethers.parseEther('1'));
    });

    it('should fail if controller is zero address', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);

      await expect(
        this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), ethers.ZeroAddress, this.staker1),
      ).to.be.revertedWithCustomError(this.mock, 'InvalidController');
    });

    it('via approved contract', async function () {
      const approvedActor = this.accounts[0];

      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock.connect(this.staker1).approve(approvedActor, ethers.parseEther('1'));

      await this.mock.connect(approvedActor).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);
    });

    it('should fail via unapproved actor', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);

      await expect(
        this.mock.connect(this.accounts[0]).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1),
      ).to.be.reverted;
    });

    it('should handle reduction in cooldown period correctly', async function () {
      const staker3 = this.accounts[0];
      await this.token.connect(staker3).approve(this.mock, ethers.MaxUint256);
      await this.token.connect(this.staker1).transfer(staker3, ethers.parseEther('1'));

      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('1'), this.staker2);
      await this.mock.connect(staker3).deposit(ethers.parseEther('1'), staker3);

      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);
      await timeIncreaseNoMine(30);

      await this.mock.connect(this.staker2).requestRedeem(ethers.parseEther('1'), this.staker2, this.staker2);

      await this.protocolStaking.connect(this.admin).setUnstakeCooldownPeriod(30);
      await this.mock.connect(staker3).requestRedeem(ethers.parseEther('1'), staker3, staker3);

      // Staker 3 will need to wait 59 seconds

      await timeIncreaseNoMine(30);
      await this.protocolStaking.release(this.mock);

      await expect(this.mock.connect(staker3).redeem(ethers.MaxUint256, staker3, staker3)).to.not.emit(
        this.token,
        'Transfer',
      );

      await timeIncreaseNoMine(29);

      await expect(this.mock.connect(staker3).redeem(ethers.MaxUint256, staker3, staker3))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, staker3, ethers.parseEther('1'));
    });

    describe('with operator', async function () {
      beforeEach(async function () {
        this.operator = this.accounts[0];
        await this.mock.connect(this.staker1).setOperator(this.operator, true);
        await this.mock.connect(this.staker1).deposit(ethers.parseEther('10'), this.staker1);
      });

      it('should be allowed to redeem on behalf of authorized controller', async function () {
        await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);

        await timeIncreaseNoMine(60);

        await expect(this.mock.connect(this.operator).redeem(ethers.MaxUint256, this.operator, this.staker1))
          .to.emit(this.token, 'Transfer')
          .withArgs(this.mock, this.operator, ethers.parseEther('1'));
      });

      it('should not be allowed to redeem on behalf of other controller', async function () {
        await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker2, this.staker1);

        await timeIncreaseNoMine(60);

        await expect(
          this.mock.connect(this.operator).redeem(ethers.MaxUint256, this.operator, this.staker2),
        ).to.be.revertedWithCustomError(this.mock, 'Unauthorized');
      });
    });
  });

  describe('restake', async function () {
    it('should restake in protocol staking', async function () {
      await this.token.connect(this.staker1).transfer(this.mock, ethers.parseEther('10'));
      await expect(this.mock.restake())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.protocolStaking, ethers.parseEther('10'));
    });

    it('should not transfer required tokens', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('10'), this.staker1);
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('1'), this.staker2);
      await this.mock.connect(this.staker2).requestRedeem(ethers.parseEther('1'), this.staker2, this.staker2);

      // Increase the value of each share by 10%
      await this.token.connect(this.staker1).transfer(this.mock, ethers.parseEther('1.1'));

      await timeIncreaseNoMine(60);
      await this.protocolStaking.release(this.mock);

      const restakeAmount = BigInt(ethers.parseEther('1')) + 1n;
      await expect(this.mock.restake())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.protocolStaking, restakeAmount);
    });
  });

  describe('slashing', async function () {
    it('symmetrically passes on losses from staked balance without pending withdrawal', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('2'), this.staker2);

      await this.protocolStaking.slash(this.mock, ethers.parseEther('1.5'));

      // Request redemption of all shares and verify actual withdrawal amounts
      await this.mock
        .connect(this.staker1)
        .requestRedeem(await this.mock.balanceOf(this.staker1), this.staker1, this.staker1);
      await this.mock
        .connect(this.staker2)
        .requestRedeem(await this.mock.balanceOf(this.staker2), this.staker2, this.staker2);

      await timeIncreaseNoMine(60);

      await expect(
        this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1),
      ).to.changeTokenBalance(this.token, this.staker1, ethers.parseEther('0.5'));
      await expect(
        this.mock.connect(this.staker2).redeem(ethers.MaxUint256, this.staker2, this.staker2),
      ).to.changeTokenBalance(this.token, this.staker2, ethers.parseEther('1'));
    });

    it('symmetrically passes on losses from staked balance with pending withdrawal', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('2'), this.staker2);

      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('0.5'), this.staker1, this.staker1);
      // 50% slashing
      await this.protocolStaking.slash(this.mock, ethers.parseEther('1.5'));

      await timeIncreaseNoMine(60);

      await expect(
        this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1),
      ).to.changeTokenBalance(this.token, this.staker1, ethers.parseEther('0.25'));
    });

    it('take excess into account on requestRedeem after slashing partially covered', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('2'), this.staker2);

      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);
      await this.protocolStaking.slash(this.mock, ethers.parseEther('1.5'));

      await expect(this.mock.connect(this.staker2).requestRedeem(ethers.parseEther('2'), this.staker2, this.staker2))
        .to.emit(this.protocolStaking, 'TokensUnstaked')
        .withArgs(this.mock, ethers.parseEther('0.5'), anyValue);
    });

    it('take excess into account on requestRedeem after slashing fully covered', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('1'), this.staker2);

      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);
      await this.protocolStaking.slash(this.mock, ethers.parseEther('1'));

      await timeIncreaseNoMine(30);

      await expect(this.mock.connect(this.staker2).requestRedeem(ethers.parseEther('1'), this.staker2, this.staker2))
        .to.emit(this.protocolStaking, 'TokensUnstaked')
        .withArgs(this.mock, 0, anyValue);

      await time.increase(30);
      await expect(this.mock.maxRedeem(this.staker2)).to.eventually.eq(0);
      await expect(this.mock.maxRedeem(this.staker1)).to.eventually.eq(ethers.parseEther('1'));

      await time.increase(30);
      await expect(this.mock.maxRedeem(this.staker2)).to.eventually.eq(ethers.parseEther('1'));
    });

    it('symmetrically passes on losses from withdrawal balance', async function () {
      await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
      await this.mock.connect(this.staker2).deposit(ethers.parseEther('2'), this.staker2);

      await this.mock.connect(this.staker1).requestRedeem(ethers.parseEther('1'), this.staker1, this.staker1);
      await this.mock.connect(this.staker2).requestRedeem(ethers.parseEther('2'), this.staker2, this.staker2);

      await this.protocolStaking.slashWithdrawal(this.mock, ethers.parseEther('1.5'));

      await timeIncreaseNoMine(60);

      await expect(
        this.mock.connect(this.staker1).redeem(ethers.MaxUint256, this.staker1, this.staker1),
      ).to.changeTokenBalance(this.token, this.staker1, ethers.parseEther('0.5'));
      await expect(
        this.mock.connect(this.staker2).redeem(ethers.MaxUint256, this.staker2, this.staker2),
      ).to.changeTokenBalance(this.token, this.staker2, ethers.parseEther('1'));
    });
  });

  describe('setRewarder', async function () {
    it('only owner can set rewarder', async function () {
      await expect(this.mock.connect(this.staker1).setRewarder(ethers.ZeroAddress)).to.be.revertedWithCustomError(
        this.mock,
        'OwnableUnauthorizedAccount',
      );
    });

    it('should revert same rewarder', async function () {
      const currentRewarder = await this.mock.rewarder();
      await expect(this.mock.connect(this.admin).setRewarder(currentRewarder))
        .to.be.revertedWithCustomError(this.mock, 'InvalidRewarder')
        .withArgs(currentRewarder);
    });

    it('should revert with no code rewarder', async function () {
      await expect(this.mock.connect(this.admin).setRewarder(this.staker1.address))
        .to.be.revertedWithCustomError(this.mock, 'InvalidRewarder')
        .withArgs(this.staker1);
    });

    describe('with new rewarder', async function () {
      beforeEach(async function () {
        const newRewarder = await ethers.deployContract('OperatorRewarder', [
          this.admin,
          this.protocolStaking,
          this.mock,
        ]);
        const oldRewarder = await ethers.getContractAt('OperatorRewarder', await this.mock.rewarder());

        await this.protocolStaking.connect(this.admin).addEligibleAccount(this.mock);
        await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0.5'));

        await this.mock.connect(this.staker1).deposit(ethers.parseEther('1'), this.staker1);
        await this.mock.connect(this.staker2).deposit(ethers.parseEther('3'), this.staker2);
        await timeIncreaseNoMine(10);

        await this.mock.connect(this.admin).setRewarder(newRewarder);
        Object.assign(this, { oldRewarder, newRewarder });
      });

      it('old rewards should remain on old rewarder', async function () {
        await expect(this.oldRewarder.earned(this.staker1)).to.eventually.eq(ethers.parseEther('1.75'));
        await expect(this.newRewarder.earned(this.staker1)).to.eventually.eq(0);
        await expect(this.token.balanceOf(this.oldRewarder)).to.eventually.eq(ethers.parseEther('5.5'));
      });

      it('new rewarder should start accruing rewards properly', async function () {
        await time.increase(10);

        await expect(this.newRewarder.earned(this.staker1)).to.eventually.eq(ethers.parseEther('1.25'));
        await expect(this.newRewarder.earned(this.staker2)).to.eventually.eq(ethers.parseEther('3.75'));
        await expect(this.newRewarder.unpaidOwnerFee()).to.eventually.eq(0);

        await expect(this.newRewarder.claimRewards(this.staker1))
          .to.emit(this.token, 'Transfer')
          .withArgs(this.newRewarder, this.staker1, ethers.parseEther('1.375'));
      });
    });
  });

  describe('setOperator', function () {
    beforeEach(async function () {
      this.tx = this.mock.connect(this.staker1).setOperator(this.staker2, true);
    });

    it('emits event', async function () {
      await expect(this.tx).to.emit(this.mock, 'OperatorSet').withArgs(this.staker1, this.staker2, true);
    });

    it('sets operator', async function () {
      await this.tx;
      await expect(this.mock.isOperator(this.staker1, this.staker2)).to.eventually.eq(true);
    });
  });
});
