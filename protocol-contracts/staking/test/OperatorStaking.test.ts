import { anyValue } from '@nomicfoundation/hardhat-chai-matchers/withArgs';
import { time } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers, upgrades } from 'hardhat';

const timeIncreaseNoMine = (duration: number) =>
  time.latest().then(clock => time.setNextBlockTimestamp(clock + duration));

describe('OperatorStaking', function () {
  beforeEach(async function () {
    const [delegator1, delegator2, admin, beneficiary, anyone, ...accounts] = await ethers.getSigners();

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
        0n, // reward rate
      ]),
    );
    const mock = await ethers.getContractFactory('OperatorStaking').then(factory =>
      upgrades.deployProxy(factory, [
        'OPStake',
        'OP',
        protocolStaking.target,
        beneficiary.address,
        10000, // 100% maximum fee
        0,
      ]),
    );

    // Mint tokens and approve mock contract
    await Promise.all(
      [delegator1, delegator2].flatMap(account => [
        token.mint(account, ethers.parseEther('1000')),
        token.$_approve(account, mock, ethers.MaxUint256),
      ]),
    );

    Object.assign(this, {
      delegator1,
      delegator2,
      admin,
      beneficiary,
      anyone,
      accounts,
      token,
      protocolStaking,
      mock,
    });
  });

  describe('Access Control', function () {
    it('should be same owner as ProtocolStaking owner', async function () {
      const protocolStakingOwner = await this.protocolStaking.owner();
      const operatorStakingOwner = await this.mock.owner();

      expect(operatorStakingOwner).to.equal(protocolStakingOwner);
    });

    it('should update ProtocolStaking and OperatorStaking owner if ProtocolStaking owner is changed', async function () {
      await this.protocolStaking.connect(this.admin).beginDefaultAdminTransfer(this.anyone);
      await this.protocolStaking.connect(this.anyone).acceptDefaultAdminTransfer();

      const protocolStakingOwner = await this.protocolStaking.owner();
      const operatorStakingOwner = await this.mock.owner();
      expect(protocolStakingOwner).to.equal(this.anyone);
      expect(operatorStakingOwner).to.equal(this.anyone);
    });

    it('should not upgrade if not authorized', async function () {
      await expect(
        this.mock.connect(this.anyone).upgradeToAndCall(this.mock.target, '0x'),
      ).to.be.revertedWithCustomError(this.mock, 'CallerNotProtocolStakingOwner');
    });
  });

  describe('deposit', async function () {
    it('should stake into protocol staking', async function () {
      await expect(this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.protocolStaking, ethers.parseEther('1'));
    });

    it('should mint shares', async function () {
      await expect(this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1))
        .to.emit(this.mock, 'Transfer')
        .withArgs(ethers.ZeroAddress, this.delegator1, ethers.parseEther('1'));
    });

    it('should pull tokens', async function () {
      await expect(this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.delegator1, this.mock, ethers.parseEther('1'));
    });
  });

  describe('depositWithPermit', async function () {
    beforeEach(async function () {
      // Define delegator, mint tokens but don't approve mock contract
      const delegatorNoApproval = this.accounts[0];
      await Promise.all(
        [delegatorNoApproval].flatMap(account => [this.token.mint(account, ethers.parseEther('1000'))]),
      );

      // Get deposit with permit inputs
      const owner = delegatorNoApproval.address;
      const spender = this.mock.target;
      const value = ethers.parseEther('1');
      const deadline = ethers.MaxUint256;

      // Get permit parameters from the token
      const tokenAddress = this.token.target;
      const nonce = await this.token.nonces(owner);
      const name = await this.token.name();

      // Get chain ID
      const { chainId } = await ethers.provider.getNetwork();

      // Define EIP-712 domain
      const domain = {
        name: name,
        version: '1',
        chainId: chainId,
        verifyingContract: tokenAddress,
      };

      // Define EIP-712 Permit type
      const types = {
        Permit: [
          { name: 'owner', type: 'address' },
          { name: 'spender', type: 'address' },
          { name: 'value', type: 'uint256' },
          { name: 'nonce', type: 'uint256' },
          { name: 'deadline', type: 'uint256' },
        ],
      };

      // Define EIP-712 Permit message
      const message = {
        owner: owner,
        spender: spender,
        value: value,
        nonce: nonce,
        deadline: deadline,
      };

      // Sign EIP-712 Permit message
      const flatSig = await delegatorNoApproval.signTypedData(domain, types, message);

      // Split into v, r, s
      const sig = ethers.Signature.from(flatSig);

      // Deposit with permit
      const depositWithPermitTx = this.mock
        .connect(delegatorNoApproval)
        .depositWithPermit(value, delegatorNoApproval, deadline, sig.v, sig.r, sig.s);

      Object.assign(this, {
        depositWithPermitTx,
        permitValue: value,
        delegatorNoApproval: delegatorNoApproval,
        permitDeadline: deadline,
        v: sig.v,
        r: sig.r,
        s: sig.s,
      });
    });

    it('should stake into protocol staking with permit', async function () {
      await expect(this.depositWithPermitTx)
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.protocolStaking, this.permitValue);
    });

    it('should mint shares with permit', async function () {
      await expect(this.depositWithPermitTx)
        .to.emit(this.mock, 'Transfer')
        .withArgs(ethers.ZeroAddress, this.delegatorNoApproval, this.permitValue);
    });

    it('should pull tokens with permit', async function () {
      await expect(this.depositWithPermitTx)
        .to.emit(this.token, 'Transfer')
        .withArgs(this.delegatorNoApproval, this.mock, this.permitValue);
    });

    it('should revert if signature is invalid', async function () {
      await expect(
        this.mock
          .connect(this.delegatorNoApproval)
          .depositWithPermit(this.permitValue, this.delegatorNoApproval, this.permitDeadline, 0, this.r, this.s),
      ).to.be.revertedWithCustomError(this.token, 'ECDSAInvalidSignature');
    });

    it('should revert if signer is invalid', async function () {
      await expect(
        this.mock
          .connect(this.delegator1)
          .depositWithPermit(this.permitValue, this.delegator1, this.permitDeadline, this.v, this.r, this.s),
      ).to.be.revertedWithCustomError(this.token, 'ERC2612InvalidSigner');
    });
  });

  describe('redeem', async function () {
    it('simple redemption', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock
        .connect(this.delegator1)
        .requestRedeem(await this.mock.balanceOf(this.delegator1), this.delegator1, this.delegator1);

      await expect(this.mock.pendingRedeemRequest(0, this.delegator1)).to.eventually.eq(ethers.parseEther('1'));
      await expect(this.mock.claimableRedeemRequest(0, this.delegator1)).to.eventually.eq(0);

      await time.increase(60);

      await expect(this.mock.pendingRedeemRequest(0, this.delegator1)).to.eventually.eq(0);
      await expect(this.mock.claimableRedeemRequest(0, this.delegator1)).to.eventually.eq(ethers.parseEther('1'));

      await expect(this.mock.connect(this.delegator1).redeem(ethers.parseEther('1'), this.delegator1, this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('1'));
      await expect(this.token.balanceOf(this.mock)).to.eventually.be.eq(0);
    });

    it('zero redemption should terminate early', async function () {
      await expect(this.mock.connect(this.delegator1).requestRedeem(0, this.delegator1, this.delegator1)).to.not.emit(
        this.mock,
        'RedeemRequest',
      );
    });

    it('should not redeem twice', async function () {
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('5'), this.delegator2);
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('10'), this.delegator1);
      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);
      await this.mock.connect(this.delegator2).requestRedeem(ethers.parseEther('1'), this.delegator2, this.delegator2);

      await timeIncreaseNoMine(60);

      await expect(this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('1'));
      await expect(
        this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1),
      ).to.not.emit(this.token, 'Transfer');
    });

    it('should revert on redeem more than available', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('10'), this.delegator1);
      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);

      await timeIncreaseNoMine(10);
      await expect(this.mock.connect(this.delegator1).redeem(ethers.parseEther('1'), this.delegator1, this.delegator1))
        .to.be.revertedWithCustomError(this.mock, 'ERC4626ExceededMaxRedeem')
        .withArgs(this.delegator1, ethers.parseEther('1'), 0);
    });

    it('should be able to redeem a second time', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('10'), this.delegator1);
      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);

      await timeIncreaseNoMine(60);

      await expect(this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('1'));

      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('2'), this.delegator1, this.delegator1);

      await timeIncreaseNoMine(60);

      await expect(this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.delegator1, ethers.parseEther('2'));
    });

    it('via separate controller', async function () {
      const controller = this.accounts[0];
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('10'), this.delegator1);
      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), controller, this.delegator1);

      await timeIncreaseNoMine(60);

      await expect(
        this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1),
      ).to.not.emit(this.token, 'Transfer');
      await expect(
        this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, controller, controller),
      ).to.be.revertedWithCustomError(this.mock, 'Unauthorized');
      await expect(this.mock.connect(controller).redeem(ethers.MaxUint256, controller, controller))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, controller, ethers.parseEther('1'));
    });

    it('should fail if controller is zero address', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);

      await expect(
        this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), ethers.ZeroAddress, this.delegator1),
      ).to.be.revertedWithCustomError(this.mock, 'InvalidController');
    });

    it('via approved contract', async function () {
      const approvedActor = this.accounts[0];

      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator1).approve(approvedActor, ethers.parseEther('1'));

      await this.mock.connect(approvedActor).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);
    });

    it('should fail via unapproved actor', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);

      await expect(
        this.mock.connect(this.accounts[0]).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1),
      ).to.be.reverted;
    });

    it('should handle reduction in cooldown period correctly', async function () {
      const delegator3 = this.accounts[0];
      await this.token.connect(delegator3).approve(this.mock, ethers.MaxUint256);
      await this.token.connect(this.delegator1).transfer(delegator3, ethers.parseEther('1'));

      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('1'), this.delegator2);
      await this.mock.connect(delegator3).deposit(ethers.parseEther('1'), delegator3);

      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);
      await timeIncreaseNoMine(30);

      await this.mock.connect(this.delegator2).requestRedeem(ethers.parseEther('1'), this.delegator2, this.delegator2);

      await this.protocolStaking.connect(this.admin).setUnstakeCooldownPeriod(30);
      await this.mock.connect(delegator3).requestRedeem(ethers.parseEther('1'), delegator3, delegator3);

      // delegator 3 will need to wait 59 seconds

      await timeIncreaseNoMine(30);
      await this.protocolStaking.release(this.mock);

      await expect(this.mock.connect(delegator3).redeem(ethers.MaxUint256, delegator3, delegator3)).to.not.emit(
        this.token,
        'Transfer',
      );

      await timeIncreaseNoMine(29);

      await expect(this.mock.connect(delegator3).redeem(ethers.MaxUint256, delegator3, delegator3))
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, delegator3, ethers.parseEther('1'));
    });

    describe('with operator', async function () {
      beforeEach(async function () {
        this.operator = this.accounts[0];
        await this.mock.connect(this.delegator1).setOperator(this.operator, true);
        await this.mock.connect(this.delegator1).deposit(ethers.parseEther('10'), this.delegator1);
      });

      it('should be allowed to redeem on behalf of authorized controller', async function () {
        await this.mock
          .connect(this.delegator1)
          .requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);

        await timeIncreaseNoMine(60);

        await expect(this.mock.connect(this.operator).redeem(ethers.MaxUint256, this.operator, this.delegator1))
          .to.emit(this.token, 'Transfer')
          .withArgs(this.mock, this.operator, ethers.parseEther('1'));
      });

      it('should not be allowed to redeem on behalf of other controller', async function () {
        await this.mock
          .connect(this.delegator1)
          .requestRedeem(ethers.parseEther('1'), this.delegator2, this.delegator1);

        await timeIncreaseNoMine(60);

        await expect(
          this.mock.connect(this.operator).redeem(ethers.MaxUint256, this.operator, this.delegator2),
        ).to.be.revertedWithCustomError(this.mock, 'Unauthorized');
      });
    });
  });

  describe('stakeExcess', async function () {
    it('should restake in protocol staking', async function () {
      await this.token.connect(this.delegator1).transfer(this.mock, ethers.parseEther('10'));
      await expect(this.mock.stakeExcess())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.protocolStaking, ethers.parseEther('10'));
    });

    it('should not transfer required tokens', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('10'), this.delegator1);
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('1'), this.delegator2);
      await this.mock.connect(this.delegator2).requestRedeem(ethers.parseEther('1'), this.delegator2, this.delegator2);

      // Increase the value of each share by 10%
      await this.token.connect(this.delegator1).transfer(this.mock, ethers.parseEther('1.1'));

      await timeIncreaseNoMine(60);
      await this.protocolStaking.release(this.mock);

      const restakeAmount = BigInt(ethers.parseEther('1')) + 1n;
      await expect(this.mock.stakeExcess())
        .to.emit(this.token, 'Transfer')
        .withArgs(this.mock, this.protocolStaking, restakeAmount);
    });
  });

  describe('slashing', async function () {
    it('symmetrically passes on losses from staked balance without pending withdrawal', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('2'), this.delegator2);

      await this.protocolStaking.slash(this.mock, ethers.parseEther('1.5'));

      // Request redemption of all shares and verify actual withdrawal amounts
      await this.mock
        .connect(this.delegator1)
        .requestRedeem(await this.mock.balanceOf(this.delegator1), this.delegator1, this.delegator1);
      await this.mock
        .connect(this.delegator2)
        .requestRedeem(await this.mock.balanceOf(this.delegator2), this.delegator2, this.delegator2);

      await timeIncreaseNoMine(60);

      await expect(
        this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1),
      ).to.changeTokenBalance(this.token, this.delegator1, ethers.parseEther('0.5'));
      await expect(
        this.mock.connect(this.delegator2).redeem(ethers.MaxUint256, this.delegator2, this.delegator2),
      ).to.changeTokenBalance(this.token, this.delegator2, ethers.parseEther('1'));
    });

    it('symmetrically passes on losses from staked balance with pending withdrawal', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('2'), this.delegator2);

      await this.mock
        .connect(this.delegator1)
        .requestRedeem(ethers.parseEther('0.5'), this.delegator1, this.delegator1);
      // 50% slashing
      await this.protocolStaking.slash(this.mock, ethers.parseEther('1.5'));

      await timeIncreaseNoMine(60);

      await expect(
        this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1),
      ).to.changeTokenBalance(this.token, this.delegator1, ethers.parseEther('0.25'));
    });

    it('take excess into account on requestRedeem after slashing partially covered', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('2'), this.delegator2);

      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);
      await this.protocolStaking.slash(this.mock, ethers.parseEther('1.5'));

      await expect(
        this.mock.connect(this.delegator2).requestRedeem(ethers.parseEther('2'), this.delegator2, this.delegator2),
      )
        .to.emit(this.protocolStaking, 'TokensUnstaked')
        .withArgs(this.mock, ethers.parseEther('0.5'), anyValue);
    });

    it('take excess into account on requestRedeem after slashing fully covered', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('1'), this.delegator2);

      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);
      await this.protocolStaking.slash(this.mock, ethers.parseEther('1'));

      await timeIncreaseNoMine(30);

      await expect(
        this.mock.connect(this.delegator2).requestRedeem(ethers.parseEther('1'), this.delegator2, this.delegator2),
      )
        .to.emit(this.protocolStaking, 'TokensUnstaked')
        .withArgs(this.mock, 0, anyValue);

      await time.increase(30);
      await expect(this.mock.maxRedeem(this.delegator2)).to.eventually.eq(0);
      await expect(this.mock.maxRedeem(this.delegator1)).to.eventually.eq(ethers.parseEther('1'));

      await time.increase(30);
      await expect(this.mock.maxRedeem(this.delegator2)).to.eventually.eq(ethers.parseEther('1'));
    });

    it('symmetrically passes on losses from withdrawal balance', async function () {
      await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
      await this.mock.connect(this.delegator2).deposit(ethers.parseEther('2'), this.delegator2);

      await this.mock.connect(this.delegator1).requestRedeem(ethers.parseEther('1'), this.delegator1, this.delegator1);
      await this.mock.connect(this.delegator2).requestRedeem(ethers.parseEther('2'), this.delegator2, this.delegator2);

      await this.protocolStaking.slashWithdrawal(this.mock, ethers.parseEther('1.5'));

      await timeIncreaseNoMine(60);

      await expect(
        this.mock.connect(this.delegator1).redeem(ethers.MaxUint256, this.delegator1, this.delegator1),
      ).to.changeTokenBalance(this.token, this.delegator1, ethers.parseEther('0.5'));
      await expect(
        this.mock.connect(this.delegator2).redeem(ethers.MaxUint256, this.delegator2, this.delegator2),
      ).to.changeTokenBalance(this.token, this.delegator2, ethers.parseEther('1'));
    });
  });

  describe('setRewarder', async function () {
    it('only owner can set rewarder', async function () {
      await expect(this.mock.connect(this.delegator1).setRewarder(ethers.ZeroAddress)).to.be.revertedWithCustomError(
        this.mock,
        'CallerNotProtocolStakingOwner',
      );
    });

    it('should revert same rewarder', async function () {
      const currentRewarder = await this.mock.rewarder();
      await expect(this.mock.connect(this.admin).setRewarder(currentRewarder))
        .to.be.revertedWithCustomError(this.mock, 'InvalidRewarder')
        .withArgs(currentRewarder);
    });

    it('should revert with no code rewarder', async function () {
      await expect(this.mock.connect(this.admin).setRewarder(this.delegator1.address))
        .to.be.revertedWithCustomError(this.mock, 'InvalidRewarder')
        .withArgs(this.delegator1);
    });

    describe('with new rewarder', async function () {
      beforeEach(async function () {
        const newRewarder = await ethers.deployContract('OperatorRewarder', [
          this.beneficiary,
          this.protocolStaking,
          this.mock,
          10000, // 100% maximum fee
          0,
        ]);
        const oldRewarder = await ethers.getContractAt('OperatorRewarder', await this.mock.rewarder());

        await this.protocolStaking.connect(this.admin).addEligibleAccount(this.mock);
        await this.protocolStaking.connect(this.admin).setRewardRate(ethers.parseEther('0.5'));

        await this.mock.connect(this.delegator1).deposit(ethers.parseEther('1'), this.delegator1);
        await this.mock.connect(this.delegator2).deposit(ethers.parseEther('3'), this.delegator2);
        await timeIncreaseNoMine(10);

        await this.mock.connect(this.admin).setRewarder(newRewarder);
        Object.assign(this, { oldRewarder, newRewarder });
      });

      it('old rewards should remain on old rewarder', async function () {
        await expect(this.oldRewarder.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('1.75'));
        await expect(this.newRewarder.earned(this.delegator1)).to.eventually.eq(0);
        await expect(this.token.balanceOf(this.oldRewarder)).to.eventually.eq(ethers.parseEther('5.5'));
      });

      it('new rewarder should start accruing rewards properly', async function () {
        await time.increase(10);

        await expect(this.newRewarder.earned(this.delegator1)).to.eventually.eq(ethers.parseEther('1.25'));
        await expect(this.newRewarder.earned(this.delegator2)).to.eventually.eq(ethers.parseEther('3.75'));
        await expect(this.newRewarder.unpaidFee()).to.eventually.eq(0);

        await expect(this.newRewarder.claimRewards(this.delegator1))
          .to.emit(this.token, 'Transfer')
          .withArgs(this.newRewarder, this.delegator1, ethers.parseEther('1.375'));
      });
    });
  });

  describe('setOperator', function () {
    beforeEach(async function () {
      this.tx = this.mock.connect(this.delegator1).setOperator(this.delegator2, true);
    });

    it('emits event', async function () {
      await expect(this.tx).to.emit(this.mock, 'OperatorSet').withArgs(this.delegator1, this.delegator2, true);
    });

    it('sets operator', async function () {
      await this.tx;
      await expect(this.mock.isOperator(this.delegator1, this.delegator2)).to.eventually.eq(true);
    });
  });
});
