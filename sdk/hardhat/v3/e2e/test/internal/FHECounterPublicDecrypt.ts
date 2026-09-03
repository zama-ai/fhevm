import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHECounterPublicDecrypt, FHECounterPublicDecrypt__factory } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

// One connection per run, shared with every other test file (`getOrCreate`), as v2 had one network
// per run; `--network` selects it.
const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

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
    signers = await getSigners(connection);
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

  // The public-decrypt half of v2's test joins at D2; until then the handle changing is the proof.
  it('increment the counter by 1', async function () {
    const encryptedCountBeforeInc = await fheCounterContract.getCount();
    expect(encryptedCountBeforeInc).to.eq(ethers.ZeroHash);

    // Encrypt constant 1 as a euint32
    const clearOne = 1;
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress as `0x`, signers.alice.address as `0x`)
      .add32(clearOne)
      .encrypt();

    const [encryptedOneHandle] = encryptedOne.handles;
    if (encryptedOneHandle === undefined) throw new Error('encrypt() returned no handle');

    const tx = await fheCounterContract.connect(signers.alice).increment(encryptedOneHandle, encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc = await fheCounterContract.getCount();
    expect(encryptedCountAfterInc).to.not.eq(ethers.ZeroHash);
  });
});
