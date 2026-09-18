/* eslint-disable no-unexpected-multiline */
import { expect } from 'chai';
import * as hre from 'hardhat';

import { deployConfidentialERC20Fixture, userDecryptBalance } from '../confidentialERC20/ConfidentialERC20.fixture';
import { getSigners, initSigners, type Signers } from '../signers';
import { userDecryptReleased } from './ConfidentialVestingWallet.fixture';
import { deployConfidentialVestingWalletCliffFixture } from './ConfidentialVestingWalletCliff.fixture';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import type { TestConfidentialERC20Mintable, TestConfidentialVestingWalletCliff } from '../../../typechain-types';

describe('ConfidentialVestingWalletCliff', function () {
  let signers: Signers;
  let beneficiary: HardhatEthersSigner;
  let beneficiaryAddress: string;
  let confidentialERC20Address: string;
  let confidentialERC20: TestConfidentialERC20Mintable;
  let startTimestamp: bigint;
  let duration: bigint;
  let cliffSeconds: bigint;
  let confidentialVestingWallet: TestConfidentialVestingWalletCliff;
  let confidentialVestingWalletAddress: string;

  before(async function () {
    await initSigners();
    signers = await getSigners();
  });

  beforeEach(async function () {
    const latestBlockNumber = await hre.ethers.provider.getBlockNumber();
    const block = await hre.ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${latestBlockNumber} not found`);
    }

    beneficiary = signers.bob;
    beneficiaryAddress = signers.bob.address;

    const contractConfidentialERC20 = await deployConfidentialERC20Fixture(
      signers.alice,
      'Naraggara',
      'NARA',
      signers.alice.address,
    );
    confidentialERC20Address = await contractConfidentialERC20.getAddress();
    confidentialERC20 = contractConfidentialERC20;
    startTimestamp = BigInt(block.timestamp + 3600);
    duration = BigInt(36_000); // 36,000 seconds
    cliffSeconds = duration / 4n;

    const contractConfidentialVestingWallet = await deployConfidentialVestingWalletCliffFixture(
      signers.alice,
      beneficiaryAddress,
      startTimestamp,
      duration,
      cliffSeconds,
    );

    confidentialVestingWallet = contractConfidentialVestingWallet;
    confidentialVestingWalletAddress = await contractConfidentialVestingWallet.getAddress();
  });

  it('post-deployment state', async function () {
    expect(await confidentialVestingWallet.BENEFICIARY()).to.equal(beneficiaryAddress);
    expect(await confidentialVestingWallet.DURATION()).to.equal(duration);
    expect(await confidentialVestingWallet.END_TIMESTAMP()).to.be.eq(startTimestamp + duration);
    expect(await confidentialVestingWallet.START_TIMESTAMP()).to.be.eq(startTimestamp);
    expect(await confidentialVestingWallet.START_TIMESTAMP()).to.be.eq(startTimestamp);
    expect(await confidentialVestingWallet.CLIFF()).to.be.eq(cliffSeconds + startTimestamp);
  });

  it('can release', async function () {
    // 10M
    const amount = hre.ethers.parseUnits('10000000', 6);

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, amount);
    await tx.wait();

    const input = hre.fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address);
    input.add64(amount);
    const encryptedTransferAmount = await input.encrypt();

    tx = await confidentialERC20
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](
        confidentialVestingWalletAddress,
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
      );

    await tx.wait();

    let nextTimestamp = startTimestamp;
    await hre.ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

    tx = await confidentialVestingWallet.connect(beneficiary).release(confidentialERC20Address);
    await tx.wait();
    await expect(tx).to.emit(confidentialVestingWallet, 'ConfidentialERC20Released');

    // It should be equal to 0 because the vesting has not started.
    expect(
      await userDecryptReleased(
        beneficiary,
        confidentialERC20Address,
        confidentialVestingWallet,
        confidentialVestingWalletAddress,
      ),
    ).to.be.eq(0n);

    // Move to the cliff - 1 second
    nextTimestamp = startTimestamp + cliffSeconds - 1n;
    await hre.ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

    tx = await confidentialVestingWallet.connect(beneficiary).release(confidentialERC20Address);
    await tx.wait();

    // It should be equal to 0 because of the cliff.
    expect(
      await userDecryptReleased(
        beneficiary,
        confidentialERC20Address,
        confidentialVestingWallet,
        confidentialVestingWalletAddress,
      ),
    ).to.be.eq(0);

    expect(await userDecryptBalance(beneficiary, confidentialERC20, confidentialERC20Address)).to.be.eq(0);

    // Bump to the end of the cliff
    nextTimestamp = startTimestamp + cliffSeconds;
    await hre.ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

    tx = await confidentialVestingWallet.connect(beneficiary).release(confidentialERC20Address);
    await tx.wait();

    // It should be equal to 1/4 since the cliff was reached so everything that was pending is releasable at once.
    expect(
      await userDecryptReleased(
        beneficiary,
        confidentialERC20Address,
        confidentialVestingWallet,
        confidentialVestingWalletAddress,
      ),
    ).to.be.eq(amount / 4n);

    expect(await userDecryptBalance(beneficiary, confidentialERC20, confidentialERC20Address)).to.be.eq(amount / 4n);

    nextTimestamp = startTimestamp + duration / BigInt(2);
    await hre.ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

    tx = await confidentialVestingWallet.connect(beneficiary).release(confidentialERC20Address);
    await tx.wait();

    // It should be equal to 1/4 of the amount vested since 1/4 was already collected.
    expect(
      await userDecryptReleased(
        beneficiary,
        confidentialERC20Address,
        confidentialVestingWallet,
        confidentialVestingWalletAddress,
      ),
    ).to.be.eq(amount / 2n);

    expect(await userDecryptBalance(beneficiary, confidentialERC20, confidentialERC20Address)).to.be.eq(amount / 2n);

    nextTimestamp = startTimestamp + duration;
    await hre.ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

    tx = await confidentialVestingWallet.connect(beneficiary).release(confidentialERC20Address);
    await tx.wait();

    // It should be equal to 1/2 of the amount vested since 2/4 was already collected.
    expect(
      await userDecryptReleased(
        beneficiary,
        confidentialERC20Address,
        confidentialVestingWallet,
        confidentialVestingWalletAddress,
      ),
    ).to.be.eq(amount);

    expect(await userDecryptBalance(beneficiary, confidentialERC20, confidentialERC20Address)).to.be.eq(amount);
  });

  it('cannot deploy if cliff > duration', async function () {
    const latestBlockNumber = await hre.ethers.provider.getBlockNumber();
    const block = await hre.ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${latestBlockNumber} not found`);
    }
    startTimestamp = BigInt(block.timestamp + 3600);
    duration = 100n;
    const cliff = duration + 1n;

    const contractFactory = await hre.ethers.getContractFactory('TestConfidentialVestingWalletCliff');
    await expect(contractFactory.connect(signers.alice).deploy(signers.alice.address, startTimestamp, duration, cliff))
      .to.be.revertedWithCustomError(confidentialVestingWallet, 'InvalidCliffDuration')
      .withArgs(cliff, duration);
  });
});
