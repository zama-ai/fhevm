import { timestampNow } from '@fhevm/hardhat-plugin';
import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers, fhevm } from 'hardhat';

import type { FHECounterUserDecrypt, FHECounterUserDecrypt__factory } from '../../../typechain-types';

type Signers = {
  alice: HardhatEthersSigner;
};

async function deployFixture(): Promise<{
  readonly fheCounterContract: FHECounterUserDecrypt;
  readonly fheCounterContractAddress: string;
}> {
  const factory: FHECounterUserDecrypt__factory = await ethers.getContractFactory('FHECounterUserDecrypt');
  const fheCounterContract = (await factory.deploy()) as FHECounterUserDecrypt;
  const fheCounterContractAddress = await fheCounterContract.getAddress();

  return { fheCounterContract, fheCounterContractAddress };
}

describe('FHECounterUserDecrypt', function () {
  let signers: Signers;
  let fheCounterContract: FHECounterUserDecrypt;
  let fheCounterContractAddress: string;

  before(async function () {
    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { alice: ethSigners[0] };
  });

  beforeEach(async () => {
    // Check whether the tests are running against an FHEVM mock environment
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }

    ({ fheCounterContract, fheCounterContractAddress } = await deployFixture());
  });

  it('increment the counter by 1 multiple times - userDecrypt multiple values', async function () {
    const encryptedOne = await fhevm
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address)
      .add32(1)
      .encrypt();

    let tx = await fheCounterContract
      .connect(signers.alice)
      .increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc1 = await fheCounterContract.getCount();

    tx = await fheCounterContract.connect(signers.alice).increment(encryptedOne.handles[0], encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc2 = await fheCounterContract.getCount();

    const clearCountAfterInc1 = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountAfterInc1,
      fheCounterContractAddress,
      signers.alice,
    );
    const clearCountAfterInc2 = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountAfterInc2,
      fheCounterContractAddress,
      signers.alice,
    );

    const transportKeyPairAlice = await fhevm.client.generateTransportKeyPair();

    const startTimestamp = timestampNow();
    const durationDays = 365;

    const signedPermitAlice = await fhevm.client.signLegacyDecryptionPermit({
      contractAddresses: [fheCounterContractAddress],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress: signers.alice.address,
      signer: signers.alice,
      transportKeyPair: transportKeyPairAlice,
    });

    // Test multiple decryptions — results come back positionally, in pair order.
    const [decrypted1, decrypted2] = await fhevm.client.decryptValuesFromPairs({
      pairs: [
        { encryptedValue: encryptedCountAfterInc1, contractAddress: fheCounterContractAddress },
        { encryptedValue: encryptedCountAfterInc2, contractAddress: fheCounterContractAddress },
      ],
      transportKeyPair: transportKeyPairAlice,
      signedPermit: signedPermitAlice,
    });

    expect(clearCountAfterInc1).to.eq(1);
    expect(clearCountAfterInc2).to.eq(2);
    expect(decrypted1.value).to.eq(1);
    expect(decrypted2.value).to.eq(2);
  });
});
