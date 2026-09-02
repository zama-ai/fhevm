import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHECounterPublicDecrypt, FHECounterPublicDecrypt__factory } from '../../types/ethers-contracts/index.ts';

// One connection per run, shared with every other test file (`getOrCreate`), as v2 had one network
// per run; `--network` selects it.
const { ethers } = await network.getOrCreate();

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
  let fheCounterContract: FHECounterPublicDecrypt;

  beforeEach(async () => {
    // The `fhevm.isCleartext` guard of the v2 suite returns here once network detection lands.
    ({ fheCounterContract } = await deployFixture());
  });

  it('encrypted count should be uninitialized after deployment', async function () {
    const encryptedCount = await fheCounterContract.getCount();
    // Expect initial count to be bytes32(0) after deployment,
    // (meaning the encrypted count value is uninitialized)
    expect(encryptedCount).to.eq(ethers.ZeroHash);
  });
});
