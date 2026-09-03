import { FhevmType, timestampNow } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { FHECounterUserDecrypt, FHECounterUserDecrypt__factory } from '../../types/ethers-contracts/index.ts';
import { type Accounts, type Signers, getAccounts, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

async function deployFixture(): Promise<{
  readonly fheCounterContract: FHECounterUserDecrypt;
  readonly fheCounterContractAddress: Hex;
}> {
  const factory: FHECounterUserDecrypt__factory = await ethers.getContractFactory('FHECounterUserDecrypt');
  const fheCounterContract = (await factory.deploy()) as FHECounterUserDecrypt;
  const fheCounterContractAddress = (await fheCounterContract.getAddress()) as Hex;

  return { fheCounterContract, fheCounterContractAddress };
}

describe('FHECounterUserDecrypt', function () {
  let signers: Signers;
  let accounts: Accounts;
  let fheCounterContract: FHECounterUserDecrypt;
  let fheCounterContractAddress: Hex;

  before(async function () {
    signers = await getSigners(connection);
    accounts = getAccounts();
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
      .createEncryptedInput(fheCounterContractAddress, signers.alice.address as Hex)
      .add32(1)
      .encrypt();
    const [encryptedOneHandle] = encryptedOne.handles;
    if (encryptedOneHandle === undefined) throw new Error('encrypt() returned no handle');

    let tx = await fheCounterContract.connect(signers.alice).increment(encryptedOneHandle, encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc1 = (await fheCounterContract.getCount()) as Hex;

    tx = await fheCounterContract.connect(signers.alice).increment(encryptedOneHandle, encryptedOne.inputProof);
    await tx.wait();

    const encryptedCountAfterInc2 = (await fheCounterContract.getCount()) as Hex;

    const clearCountAfterInc1 = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountAfterInc1,
      fheCounterContractAddress,
      accounts.alice,
    );
    const clearCountAfterInc2 = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      encryptedCountAfterInc2,
      fheCounterContractAddress,
      accounts.alice,
    );

    const transportKeyPairAlice = await fhevm.client.generateTransportKeyPair();

    const startTimestamp = timestampNow();
    const durationDays = 365;

    const signedPermitAlice = await fhevm.client.signLegacyDecryptionPermit({
      contractAddresses: [fheCounterContractAddress],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress: accounts.alice.address,
      signer: accounts.alice,
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

    expect(clearCountAfterInc1).to.eq(1n);
    expect(clearCountAfterInc2).to.eq(2n);
    expect(decrypted1?.value).to.eq(1n);
    expect(decrypted2?.value).to.eq(2n);
  });
});
