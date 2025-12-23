// Ported from https://github.com/OpenZeppelin/openzeppelin-confidential-contracts/blob/f0914b66f9f3766915403587b1ef1432d53054d3/test/token/ERC7984/extensions/ERC7984Wrapper.test.ts
// (0.3.0 version)

import { ConfidentialWrapper } from '../types';
import { FhevmType } from '@fhevm/hardhat-plugin';
import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { time } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import { ethers, fhevm, upgrades } from 'hardhat';
import { getRequiredEnvVar } from '../tasks/utils/loadVariables';
import { Addressable } from 'ethers';
import { CONTRACT_NAME } from '../tasks/deploy';
import { createRandomAddress } from './utils/inputs';

// Get values of the first confidential wrapper from the environment variables
const name = getRequiredEnvVar('CONFIDENTIAL_WRAPPER_NAME_0');
const symbol = getRequiredEnvVar('CONFIDENTIAL_WRAPPER_SYMBOL_0');
const uri = getRequiredEnvVar('CONFIDENTIAL_WRAPPER_CONTRACT_URI_0');
const owner = getRequiredEnvVar('CONFIDENTIAL_WRAPPER_OWNER_ADDRESS_0');

// Define ERC20mock values
const erc20contractName = '$ERC20Mock';
const erc20mockName = 'ERC20Mock';
const erc20mockSymbol = 'MOCK';
const erc20mockDecimals = 18;

/* eslint-disable no-unexpected-multiline */
describe('ERC7984Wrapper', function () {
  async function deployConfidentialWrapper(token: string | Addressable) {
    const confidentialWrapperFactory = await ethers.getContractFactory(CONTRACT_NAME);
    const proxy = await upgrades.deployProxy(confidentialWrapperFactory, [name, symbol, uri, token, owner], {
      initializer: 'initialize',
      kind: 'uups',
    });
    await proxy.waitForDeployment();
    return proxy;
  }

  beforeEach(async function () {
    const accounts = await ethers.getSigners();
    const [holder, recipient, operator, anyone] = accounts;

    const token = await ethers.deployContract(erc20contractName, [erc20mockName, erc20mockSymbol, erc20mockDecimals]);
    const confidentialWrapperProxy = await deployConfidentialWrapper(token.target);

    this.accounts = accounts.slice(3);
    this.holder = holder;
    this.recipient = recipient;
    this.token = token;
    this.operator = operator;
    this.wrapper = confidentialWrapperProxy;
    this.anyone = anyone;

    await this.token.$_mint(this.holder.address, ethers.parseUnits('1000', 18));
    await this.token.connect(this.holder).approve(this.wrapper, ethers.MaxUint256);
  });

  describe('Access Control', function () {
    it('should not upgrade if not authorized', async function () {
      const fakeContractAddress = createRandomAddress();
      await expect(
        this.wrapper.connect(this.anyone).upgradeToAndCall(fakeContractAddress, '0x'),
      ).to.be.revertedWithCustomError(this.wrapper, 'OwnableUnauthorizedAccount');
    });
  });

  describe('Wrap', async function () {
    for (const viaCallback of [false, true]) {
      describe(`via ${viaCallback ? 'callback' : 'transfer from'}`, function () {
        it('with multiple of rate', async function () {
          const amountToWrap = ethers.parseUnits('100', 18);

          if (viaCallback) {
            await this.token.connect(this.holder).transferAndCall(this.wrapper, amountToWrap);
          } else {
            await this.wrapper.connect(this.holder).wrap(this.holder.address, amountToWrap);
          }

          await expect(this.token.balanceOf(this.holder)).to.eventually.equal(ethers.parseUnits('900', 18));
          const wrappedBalanceHandle = await this.wrapper.confidentialBalanceOf(this.holder.address);
          await expect(
            fhevm.userDecryptEuint(FhevmType.euint64, wrappedBalanceHandle, this.wrapper.target, this.holder),
          ).to.eventually.equal(ethers.parseUnits('100', 6));
        });

        it('with value less than rate', async function () {
          const amountToWrap = ethers.parseUnits('100', 8);

          if (viaCallback) {
            await this.token.connect(this.holder).transferAndCall(this.wrapper, amountToWrap);
          } else {
            await this.wrapper.connect(this.holder).wrap(this.holder.address, amountToWrap);
          }

          await expect(this.token.balanceOf(this.holder)).to.eventually.equal(ethers.parseUnits('1000', 18));
          const wrappedBalanceHandle = await this.wrapper.confidentialBalanceOf(this.holder.address);
          await expect(
            fhevm.userDecryptEuint(FhevmType.euint64, wrappedBalanceHandle, this.wrapper.target, this.holder),
          ).to.eventually.equal(0);
        });

        it('with non-multiple of rate', async function () {
          const amountToWrap = ethers.parseUnits('101', 11);

          if (viaCallback) {
            await this.token.connect(this.holder).transferAndCall(this.wrapper, amountToWrap);
          } else {
            await this.wrapper.connect(this.holder).wrap(this.holder.address, amountToWrap);
          }

          await expect(this.token.balanceOf(this.holder)).to.eventually.equal(
            ethers.parseUnits('1000', 18) - ethers.parseUnits('10', 12),
          );
          const wrappedBalanceHandle = await this.wrapper.confidentialBalanceOf(this.holder.address);
          await expect(
            fhevm.userDecryptEuint(FhevmType.euint64, wrappedBalanceHandle, this.wrapper.target, this.holder),
          ).to.eventually.equal(10);
        });

        it('max amount works', async function () {
          await this.token.$_mint(this.holder.address, ethers.MaxUint256 / 2n); // mint a lot of tokens

          const rate = await this.wrapper.rate();
          const maxConfidentialSupply = await this.wrapper.maxTotalSupply();
          const maxUnderlyingBalance = maxConfidentialSupply * rate;

          if (viaCallback) {
            await this.token.connect(this.holder).transferAndCall(this.wrapper, maxUnderlyingBalance);
          } else {
            await this.wrapper.connect(this.holder).wrap(this.holder.address, maxUnderlyingBalance);
          }

          await expect(
            fhevm.userDecryptEuint(
              FhevmType.euint64,
              await this.wrapper.confidentialBalanceOf(this.holder.address),
              this.wrapper.target,
              this.holder,
            ),
          ).to.eventually.equal(maxConfidentialSupply);
        });

        it('amount exceeding max fails', async function () {
          await this.token.$_mint(this.holder.address, ethers.MaxUint256 / 2n); // mint a lot of tokens

          const rate = await this.wrapper.rate();
          const maxConfidentialSupply = await this.wrapper.maxTotalSupply();
          const maxUnderlyingBalance = maxConfidentialSupply * rate;

          // first deposit close to the max
          await this.wrapper.connect(this.holder).wrap(this.holder.address, maxUnderlyingBalance);

          // try to deposit more, causing the total supply to exceed the max supported amount
          await expect(
            viaCallback
              ? this.token.connect(this.holder).transferAndCall(this.wrapper, rate)
              : this.wrapper.connect(this.holder).wrap(this.holder.address, rate),
          ).to.be.revertedWithCustomError(this.wrapper, 'ERC7984TotalSupplyOverflow');
        });

        if (viaCallback) {
          it('to another address', async function () {
            const amountToWrap = ethers.parseUnits('100', 18);

            await this.token
              .connect(this.holder)
              [
                'transferAndCall(address,uint256,bytes)'
              ](this.wrapper, amountToWrap, ethers.solidityPacked(['address'], [this.recipient.address]));

            await expect(this.token.balanceOf(this.holder)).to.eventually.equal(ethers.parseUnits('900', 18));
            const wrappedBalanceHandle = await this.wrapper.confidentialBalanceOf(this.recipient.address);
            await expect(
              fhevm.userDecryptEuint(FhevmType.euint64, wrappedBalanceHandle, this.wrapper.target, this.recipient),
            ).to.eventually.equal(ethers.parseUnits('100', 6));
          });

          it('from unauthorized caller', async function () {
            await expect(this.wrapper.connect(this.holder).onTransferReceived(this.holder, this.holder, 100, '0x'))
              .to.be.revertedWithCustomError(this.wrapper, 'ERC7984UnauthorizedCaller')
              .withArgs(this.holder.address);
          });
        }
      });
    }
  });

  describe('Unwrap', async function () {
    beforeEach(async function () {
      const amountToWrap = ethers.parseUnits('100', 18);
      await this.token.connect(this.holder).transferAndCall(this.wrapper, amountToWrap);
    });

    it('less than balance', async function () {
      const withdrawalAmount = ethers.parseUnits('10', 6);
      const encryptedInput = await fhevm
        .createEncryptedInput(this.wrapper.target, this.holder.address)
        .add64(withdrawalAmount)
        .encrypt();

      await this.wrapper
        .connect(this.holder)
        [
          'unwrap(address,address,bytes32,bytes)'
        ](this.holder, this.holder, encryptedInput.handles[0], encryptedInput.inputProof);

      await publicDecryptAndFinalizeUnwrap(this.wrapper, this.holder);

      await expect(this.token.balanceOf(this.holder)).to.eventually.equal(
        withdrawalAmount * 10n ** 12n + ethers.parseUnits('900', 18),
      );
    });

    it('unwrap full balance', async function () {
      await this.wrapper
        .connect(this.holder)
        .unwrap(this.holder, this.holder, await this.wrapper.confidentialBalanceOf(this.holder.address));
      await publicDecryptAndFinalizeUnwrap(this.wrapper, this.holder);

      await expect(this.token.balanceOf(this.holder)).to.eventually.equal(ethers.parseUnits('1000', 18));
    });

    it('more than balance', async function () {
      const withdrawalAmount = ethers.parseUnits('101', 9);
      const input = fhevm.createEncryptedInput(this.wrapper.target, this.holder.address);
      input.add64(withdrawalAmount);
      const encryptedInput = await input.encrypt();

      await this.wrapper
        .connect(this.holder)
        [
          'unwrap(address,address,bytes32,bytes)'
        ](this.holder, this.holder, encryptedInput.handles[0], encryptedInput.inputProof);

      await publicDecryptAndFinalizeUnwrap(this.wrapper, this.holder);
      await expect(this.token.balanceOf(this.holder)).to.eventually.equal(ethers.parseUnits('900', 18));
    });

    it('to invalid recipient', async function () {
      const withdrawalAmount = ethers.parseUnits('10', 9);
      const input = fhevm.createEncryptedInput(this.wrapper.target, this.holder.address);
      input.add64(withdrawalAmount);
      const encryptedInput = await input.encrypt();

      await expect(
        this.wrapper
          .connect(this.holder)
          [
            'unwrap(address,address,bytes32,bytes)'
          ](this.holder, ethers.ZeroAddress, encryptedInput.handles[0], encryptedInput.inputProof),
      )
        .to.be.revertedWithCustomError(this.wrapper, 'ERC7984InvalidReceiver')
        .withArgs(ethers.ZeroAddress);
    });

    it('via an approved operator', async function () {
      const withdrawalAmount = ethers.parseUnits('100', 6);
      const encryptedInput = await fhevm
        .createEncryptedInput(this.wrapper.target, this.operator.address)
        .add64(withdrawalAmount)
        .encrypt();

      await this.wrapper.connect(this.holder).setOperator(this.operator.address, (await time.latest()) + 1000);

      await this.wrapper
        .connect(this.operator)
        [
          'unwrap(address,address,bytes32,bytes)'
        ](this.holder, this.holder, encryptedInput.handles[0], encryptedInput.inputProof);

      await publicDecryptAndFinalizeUnwrap(this.wrapper, this.operator);

      await expect(this.token.balanceOf(this.holder)).to.eventually.equal(ethers.parseUnits('1000', 18));
    });

    it('via an unapproved operator', async function () {
      const withdrawalAmount = ethers.parseUnits('100', 9);
      const input = fhevm.createEncryptedInput(this.wrapper.target, this.operator.address);
      input.add64(withdrawalAmount);
      const encryptedInput = await input.encrypt();

      await expect(
        this.wrapper
          .connect(this.operator)
          [
            'unwrap(address,address,bytes32,bytes)'
          ](this.holder, this.holder, encryptedInput.handles[0], encryptedInput.inputProof),
      )
        .to.be.revertedWithCustomError(this.wrapper, 'ERC7984UnauthorizedSpender')
        .withArgs(this.holder, this.operator);
    });

    it('with a value not allowed to sender', async function () {
      const totalSupplyHandle = await this.wrapper.confidentialTotalSupply();

      await expect(this.wrapper.connect(this.holder).unwrap(this.holder, this.holder, totalSupplyHandle))
        .to.be.revertedWithCustomError(this.wrapper, 'ERC7984UnauthorizedUseOfEncryptedAmount')
        .withArgs(totalSupplyHandle, this.holder);
    });

    it('finalized with invalid signature', async function () {
      const withdrawalAmount = ethers.parseUnits('10', 6);
      const encryptedInput = await fhevm
        .createEncryptedInput(this.wrapper.target, this.holder.address)
        .add64(withdrawalAmount)
        .encrypt();

      await this.wrapper
        .connect(this.holder)
        [
          'unwrap(address,address,bytes32,bytes)'
        ](this.holder, this.holder, encryptedInput.handles[0], encryptedInput.inputProof);

      const event = (await this.wrapper.queryFilter(this.wrapper.filters.UnwrapRequested()))[0];
      const unwrapAmount = event.args[1];
      const publicDecryptResults = await fhevm.publicDecrypt([unwrapAmount]);

      await expect(
        this.wrapper
          .connect(this.holder)
          .finalizeUnwrap(
            unwrapAmount,
            publicDecryptResults.abiEncodedClearValues,
            publicDecryptResults.decryptionProof.slice(0, publicDecryptResults.decryptionProof.length - 2),
          ),
      ).to.be.reverted;
    });

    it('finalize invalid unwrap request', async function () {
      await expect(
        this.wrapper.connect(this.holder).finalizeUnwrap(ethers.ZeroHash, 0, '0x'),
      ).to.be.revertedWithCustomError(this.wrapper, 'InvalidUnwrapRequest');
    });
  });

  describe('Initialization', function () {
    describe('decimals', function () {
      it('when underlying has 6 decimals', async function () {
        const token = (await ethers.deployContract(erc20contractName, [erc20mockName, erc20mockSymbol, 6])).target;
        const wrapper = await deployConfidentialWrapper(token);

        await expect(wrapper.decimals()).to.eventually.equal(6);
        await expect(wrapper.rate()).to.eventually.equal(1);
      });

      it('when underlying has more than 9 decimals', async function () {
        const token = (await ethers.deployContract(erc20contractName, [erc20mockName, erc20mockSymbol, 18])).target;
        const wrapper = await deployConfidentialWrapper(token);

        await expect(wrapper.decimals()).to.eventually.equal(6);
        await expect(wrapper.rate()).to.eventually.equal(10n ** 12n);
      });

      it('when underlying has less than 6 decimals', async function () {
        const token = (await ethers.deployContract(erc20contractName, [erc20mockName, erc20mockSymbol, 4])).target;
        const wrapper = await deployConfidentialWrapper(token);

        await expect(wrapper.decimals()).to.eventually.equal(4);
        await expect(wrapper.rate()).to.eventually.equal(1);
      });

      it('when underlying decimals are not available', async function () {
        const token = (await ethers.deployContract('ERC20RevertDecimalsMock')).target;
        const wrapper = await deployConfidentialWrapper(token);

        await expect(wrapper.decimals()).to.eventually.equal(6);
        await expect(wrapper.rate()).to.eventually.equal(10n ** 12n);
      });

      it('when decimals are over `type(uint8).max`', async function () {
        const token = (await ethers.deployContract('ERC20ExcessDecimalsMock')).target;
        await expect(deployConfidentialWrapper(token)).to.be.reverted;
      });
    });
  });
});
/* eslint-disable no-unexpected-multiline */

async function publicDecryptAndFinalizeUnwrap(wrapper: ConfidentialWrapper, caller: HardhatEthersSigner) {
  const [to, amount] = (await wrapper.queryFilter(wrapper.filters.UnwrapRequested()))[0].args;
  const { abiEncodedClearValues, decryptionProof } = await fhevm.publicDecrypt([amount]);
  await expect(wrapper.connect(caller).finalizeUnwrap(amount, abiEncodedClearValues, decryptionProof))
    .to.emit(wrapper, 'UnwrapFinalized')
    .withArgs(to, amount, abiEncodedClearValues);
}
