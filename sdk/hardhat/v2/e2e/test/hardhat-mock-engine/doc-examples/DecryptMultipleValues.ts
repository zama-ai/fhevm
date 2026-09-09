import { timestampNow } from '@fhevm/hardhat-plugin';
import type { HardhatFhevmRuntimeEnvironment } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import * as hre from 'hardhat';

import type { DecryptMultipleValues, DecryptMultipleValues__factory } from '../../../typechain-types';
import type { Signers } from '../signers';

async function deployFixture(): Promise<{
  readonly decryptMultipleValues: DecryptMultipleValues;
  readonly decryptMultipleValuesAddress: string;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: DecryptMultipleValues__factory = await ethers.getContractFactory('DecryptMultipleValues');
  const decryptMultipleValues = (await factory.deploy()) as DecryptMultipleValues;
  const decryptMultipleValuesAddress = await decryptMultipleValues.getAddress();

  return { decryptMultipleValues, decryptMultipleValuesAddress };
}

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe('DecryptMultipleValues', function () {
  let contract: DecryptMultipleValues;
  let contractAddress: string;
  let signers: Signers;

  before(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!hre.fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1] };
  });

  beforeEach(async function () {
    // Deploy a new contract each time we run a new test
    const deployment = await deployFixture();
    contractAddress = deployment.decryptMultipleValuesAddress;
    contract = deployment.decryptMultipleValues;
  });

  // ✅ Test should succeed
  it('decryption should succeed', async function () {
    const tx = await contract.connect(signers.alice).initialize(true, 123456, 78901234567);
    await tx.wait();

    const encryptedBool = (await contract.encryptedBool()) as `0x${string}`;
    const encryptedUint32 = (await contract.encryptedUint32()) as `0x${string}`;
    const encryptedUint64 = (await contract.encryptedUint64()) as `0x${string}`;

    // The FHEVM Hardhat plugin provides a set of convenient helper functions
    // that make it easy to perform FHEVM operations within your Hardhat environment.
    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    // A transport key pair plus a signed decryption permit replace the old
    // generateKeypair + createEIP712 + signTypedData handshake.
    const aliceTransportKeyPair = await fhevm.client.generateTransportKeyPair();

    const startTimestamp = timestampNow();
    const durationDays = 365;

    const aliceSignedPermit = await fhevm.client.signLegacyDecryptionPermit({
      contractAddresses: [contractAddress],
      startTimestamp,
      // The legacy API measured validity in days; `@fhevm/sdk` takes seconds.
      durationSeconds: durationDays * 24 * 60 * 60,
      signerAddress: signers.alice.address,
      signer: signers.alice,
      transportKeyPair: aliceTransportKeyPair,
    });

    // Results come back positionally, in the order the pairs were given — the old API keyed them
    // by handle.
    const [clearBool, clearUint32, clearUint64] = await fhevm.client.decryptValuesFromPairs({
      pairs: [
        { encryptedValue: encryptedBool, contractAddress },
        { encryptedValue: encryptedUint32, contractAddress },
        { encryptedValue: encryptedUint64, contractAddress },
      ],
      transportKeyPair: aliceTransportKeyPair,
      signedPermit: aliceSignedPermit,
    });

    expect(clearBool.value).to.equal(true);
    expect(clearUint32.value).to.equal(123456 + 1);
    expect(clearUint64.value).to.equal(78901234567 + 1);
  });
});
