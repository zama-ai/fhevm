import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHECounterPublicDecrypt, FHECounterPublicDecrypt__factory } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

// One connection per run, shared with every other test file (`getOrCreate`), as v2 had one network
// per run; `--network` selects it.
const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

async function deployFixture(): Promise<{
  readonly fheCounterContract: FHECounterPublicDecrypt;
  readonly fheCounterContractAddress: Hex;
}> {
  const factory: FHECounterPublicDecrypt__factory = await ethers.getContractFactory('FHECounterPublicDecrypt');
  const fheCounterContract = (await factory.deploy()) as FHECounterPublicDecrypt;
  const fheCounterContractAddress = (await fheCounterContract.getAddress()) as Hex;

  return { fheCounterContract, fheCounterContractAddress };
}

describe('FHECounterPublicDecrypt', function () {
  let signers: Signers;
  let fheCounterContract: FHECounterPublicDecrypt;
  let fheCounterContractAddress: Hex;

  before(async function () {
    signers = await getSigners(connection);
  });

  beforeEach(async () => {
    // Check whether the tests are running against an FHEVM mock environment
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }
    ({ fheCounterContract, fheCounterContractAddress } = await deployFixture());
  });

  // Encrypts one euint32 for alice; `handles` is `Hex[]`, so the single handle is narrowed here once.
  async function encryptOne32(value: number): Promise<{ handle: Hex; inputProof: Hex }> {
    const encrypted = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address as Hex)
      .add32(value)
      .encrypt();
    const [handle] = encrypted.handles;
    if (handle === undefined) throw new Error('encrypt() returned no handle');
    return { handle, inputProof: encrypted.inputProof };
  }

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
    const encryptedOneTwoThree = await encryptOne32(clearOneTwoThree);

    const tx = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedOneTwoThree.handle, encryptedOneTwoThree.inputProof);
    await tx.wait();

    const encryptedCountAfterInc = (await fheCounterContract.getCount()) as Hex;
    const publicDecryptResults = await fhevm.publicDecrypt([encryptedCountAfterInc]);

    expect(publicDecryptResults.clearValues[encryptedCountAfterInc]).to.eq(
      BigInt(clearCountBeforeInc + clearOneTwoThree),
    );

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
    const encryptedOne = await encryptOne32(clearOne);

    const tx = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handle, encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc = (await fheCounterContract.getCount()) as Hex;
    const clearCountAfterInc = await fhevm.publicDecryptEuint(FhevmType.euint32, encryptedCountAfterInc);

    expect(clearCountAfterInc).to.eq(BigInt(clearCountBeforeInc + clearOne));
  });

  it('increment the counter by 1 multiple times', async function () {
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);
    const clearCountBeforeInc = 0;

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await encryptOne32(clearOne);

    // First Tx (increment by 1)
    const tx1 = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handle, encryptedOne.inputProof);
    await tx1.wait();
    const encryptedCountAfterInc1 = (await fheCounterContract.getCount()) as Hex;

    // Second Tx (increment by one again)
    const tx2 = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handle, encryptedOne.inputProof);
    await tx2.wait();
    const encryptedCountAfterInc2 = (await fheCounterContract.getCount()) as Hex;

    // Multiple public decrypt
    const decryptedResults = await fhevm.publicDecrypt([encryptedCountAfterInc1, encryptedCountAfterInc2]);

    // Result should contain 2 values
    expect(Object.keys(decryptedResults.clearValues).length).to.eq(2);
    expect(decryptedResults.clearValues[encryptedCountAfterInc1]).to.eq(BigInt(clearCountBeforeInc + clearOne));
    expect(decryptedResults.clearValues[encryptedCountAfterInc2]).to.eq(
      BigInt(clearCountBeforeInc + clearOne + clearOne),
    );
  });

  it('decrement the counter by 1', async function () {
    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await encryptOne32(clearOne);

    // First increment by 1, count becomes 1
    let tx = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handle, encryptedOne.inputProof);
    await tx.wait();

    // Then decrement by 1, count goes back to 0
    tx = await fheCounterContract.connect(signers.alice).decrement(encryptedOne.handle, encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterDec = (await fheCounterContract.getCount()) as Hex;
    const clearCountAfterDec = await fhevm.publicDecryptEuint(FhevmType.euint32, encryptedCountAfterDec);

    expect(clearCountAfterDec).to.eq(0n);
  });

  it('increment the counter by 1 not decryptable', async function () {
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await encryptOne32(clearOne);

    // First Tx (increment by 1)
    const tx = await fheCounterContract
      .connect(signers.alice)
      .incrementNotPubliclyDecryptable(encryptedOne.handle, encryptedOne.inputProof);
    await tx.wait();
    const encryptedCountAfterInc = (await fheCounterContract.getCount()) as Hex;

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
