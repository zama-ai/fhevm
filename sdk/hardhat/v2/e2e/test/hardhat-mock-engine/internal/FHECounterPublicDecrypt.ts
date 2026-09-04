import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers, fhevm } from 'hardhat';

import type { FHECounterPublicDecrypt, FHECounterPublicDecrypt__factory } from '../../../typechain-types';

type Signers = {
  deployer: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

async function deployFixture(): Promise<{
  readonly fheCounterContract: FHECounterPublicDecrypt;
  readonly fheCounterContractAddress: string;
}> {
  const factory: FHECounterPublicDecrypt__factory = await ethers.getContractFactory('FHECounterPublicDecrypt');
  const fheCounterContract = (await factory.deploy()) as FHECounterPublicDecrypt;
  const fheCounterContractAddress = await fheCounterContract.getAddress();

  return { fheCounterContract, fheCounterContractAddress };
}

describe('FHECounterPublicDecrypt', function () {
  let signers: Signers;
  let fheCounterContract: FHECounterPublicDecrypt;
  let fheCounterContractAddress: string;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { deployer: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };
  });

  beforeEach(async () => {
    // Check whether the tests are running against an FHEVM mock environment
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }
    ({ fheCounterContract, fheCounterContractAddress } = await deployFixture());
  });

  it('encrypted count should be uninitialized after deployment', async function () {
    const encryptedCount = await fheCounterContract.getCount();
    // Expect initial count to be bytes32(0) after deployment,
    // (meaning the encrypted count value is uninitialized)
    expect(encryptedCount).to.eq(ethers.ZeroHash);
  });

  it('increment the counter by 123 and verify public decrypt', async function () {
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
    const clearCountBeforeInc = 0;

    // Encrypt constant 123 as a euint32
    const clearOneTwoThree = 123;
    const encryptedOneTwoThree = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(clearOneTwoThree)
      .encrypt();

    const tx = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedOneTwoThree.handles[0], encryptedOneTwoThree.inputProof);
    await tx.wait();

    const encryptedCountAfterInc = (await fheCounterContract.getCount()) as `0x${string}`;
    const publicDecryptResults = await fhevm.publicDecrypt([encryptedCountAfterInc]);

    expect(publicDecryptResults.clearValues[encryptedCountAfterInc]).to.eq(clearCountBeforeInc + clearOneTwoThree);

    console.log(publicDecryptResults.abiEncodedClearValues);
    console.log(publicDecryptResults.decryptionProof);

    await fheCounterContract.verify(
      [encryptedCountAfterInc],
      publicDecryptResults.abiEncodedClearValues,
      publicDecryptResults.decryptionProof,
    );
  });

  it('increment the counter by 1', async function () {
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
    const clearCountBeforeInc = 0;

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(clearOne)
      .encrypt();

    const tx = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc = await fheCounterContract.getCount();
    const clearCountAfterInc = await fhevm.publicDecryptEuint(FhevmType.euint32, encryptedCountAfterInc);

    expect(clearCountAfterInc).to.eq(clearCountBeforeInc + clearOne);
  });

  it('increment the counter by 1 multiple times', async function () {
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
    const clearCountBeforeInc = 0;

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(clearOne)
      .encrypt();

    // First Tx (increment by 1)
    const tx1 = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx1.wait();
    const encryptedCountAfterInc1 = await fheCounterContract.getCount();

    // Second Tx (increment by one again)
    const tx2 = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx2.wait();
    const encryptedCountAfterInc2 = await fheCounterContract.getCount();

    // Multiple public decrypt
    const decryptedResults = await fhevm.publicDecrypt([encryptedCountAfterInc1, encryptedCountAfterInc2]);

    // Result should contain 2 values
    expect(Object.keys(decryptedResults.clearValues).length).to.eq(2);
    expect(decryptedResults.clearValues[encryptedCountAfterInc1 as `0x${string}`]).to.eq(
      clearCountBeforeInc + clearOne,
    );
    expect(decryptedResults.clearValues[encryptedCountAfterInc2 as `0x${string}`]).to.eq(
      clearCountBeforeInc + clearOne + clearOne,
    );
  });

  it('decrement the counter by 1', async function () {
    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(clearOne)
      .encrypt();

    // First increment by 1, count becomes 1
    let tx = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    // Then decrement by 1, count goes back to 0
    tx = await fheCounterContract.connect(signers.alice).decrement(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterDec = await fheCounterContract.getCount();
    const clearCountAfterInc = await fhevm.publicDecryptEuint(FhevmType.euint32, encryptedCountAfterDec);

    expect(clearCountAfterInc).to.eq(0);
  });

  it('increment the counter by 1 not decryptable', async function () {
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(clearOne)
      .encrypt();

    // First Tx (increment by 1)
    const tx = await fheCounterContract
      .connect(signers.alice)
      .incrementNotPubliclyDecryptable(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();
    const encryptedCountAfterInc = await fheCounterContract.getCount();

    let failed;
    try {
      await fhevm.publicDecrypt([encryptedCountAfterInc]);
      failed = false;
    } catch {
      failed = true;
    }
    expect(failed).to.eq(true);
  });
});
