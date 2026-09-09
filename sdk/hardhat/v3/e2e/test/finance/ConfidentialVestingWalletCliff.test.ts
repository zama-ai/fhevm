/* eslint-disable no-unexpected-multiline */
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { expect } from 'chai';
import { network } from 'hardhat';

import type {
  TestConfidentialERC20Mintable,
  TestConfidentialVestingWalletCliff,
} from '../../types/ethers-contracts/index.ts';
import { deployConfidentialERC20Fixture, userDecryptBalance } from '../confidentialERC20/ConfidentialERC20.fixture.ts';
import { type Signers, getSigners } from '../utils/signers.ts';
import { userDecryptReleased } from './ConfidentialVestingWallet.fixture.ts';
import { deployConfidentialVestingWalletCliffFixture } from './ConfidentialVestingWalletCliff.fixture.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

describe('ConfidentialVestingWalletCliff', function () {
  let signers: Signers;
  let beneficiary: HardhatEthersSigner;
  let beneficiaryAddress: string;
  let confidentialERC20Address: Hex;
  let confidentialERC20: TestConfidentialERC20Mintable;
  let startTimestamp: bigint;
  let duration: bigint;
  let cliffSeconds: bigint;
  let confidentialVestingWallet: TestConfidentialVestingWalletCliff;
  let confidentialVestingWalletAddress: Hex;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async function () {
    const latestBlockNumber = await ethers.provider.getBlockNumber();
    const block = await ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${String(latestBlockNumber)} not found`);
    }

    beneficiary = signers.bob;
    beneficiaryAddress = signers.bob.address;

    const contractConfidentialERC20 = await deployConfidentialERC20Fixture(
      signers.alice,
      'Naraggara',
      'NARA',
      signers.alice.address,
    );
    confidentialERC20Address = (await contractConfidentialERC20.getAddress()) as Hex;
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
    confidentialVestingWalletAddress = (await contractConfidentialVestingWallet.getAddress()) as Hex;
  });

  it('post-deployment state', async function () {
    expect(await confidentialVestingWallet.BENEFICIARY()).to.equal(beneficiaryAddress);
    expect(await confidentialVestingWallet.DURATION()).to.equal(duration);
    expect(await confidentialVestingWallet.END_TIMESTAMP()).to.be.eq(startTimestamp + duration);
    expect(await confidentialVestingWallet.START_TIMESTAMP()).to.be.eq(startTimestamp);
    expect(await confidentialVestingWallet.CLIFF()).to.be.eq(cliffSeconds + startTimestamp);
  });

  it('can release', async function () {
    // 10M
    const amount = ethers.parseUnits('10000000', 6);

    let tx = await confidentialERC20.connect(signers.alice).mint(signers.alice, amount);
    await tx.wait();

    const input = fhevm.createEncryptedInput(confidentialERC20Address, signers.alice.address as Hex);
    input.add64(amount);
    const encryptedTransferAmount = await input.encrypt();
    const [transferHandle] = encryptedTransferAmount.handles;
    if (transferHandle === undefined) throw new Error('encrypt() returned no handle');

    tx = await confidentialERC20
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](
        confidentialVestingWalletAddress,
        transferHandle,
        encryptedTransferAmount.inputProof,
      );

    await tx.wait();

    let nextTimestamp = startTimestamp;
    await ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

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
    await ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

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
    ).to.be.eq(0n);

    expect(await userDecryptBalance(beneficiary, confidentialERC20, confidentialERC20Address)).to.be.eq(0n);

    // Bump to the end of the cliff
    nextTimestamp = startTimestamp + cliffSeconds;
    await ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

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
    await ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

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
    await ethers.provider.send('evm_setNextBlockTimestamp', [nextTimestamp.toString()]);

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
    const latestBlockNumber = await ethers.provider.getBlockNumber();
    const block = await ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${String(latestBlockNumber)} not found`);
    }
    startTimestamp = BigInt(block.timestamp + 3600);
    duration = 100n;
    const cliff = duration + 1n;

    const contractFactory = await ethers.getContractFactory('TestConfidentialVestingWalletCliff');
    await expect(contractFactory.connect(signers.alice).deploy(signers.alice.address, startTimestamp, duration, cliff))
      .to.be.revertedWithCustomError(confidentialVestingWallet, 'InvalidCliffDuration')
      .withArgs(cliff, duration);
  });
});
