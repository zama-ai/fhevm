import { type HardhatFhevmRuntimeEnvironment, timestampNow } from '@fhevm/hardhat-plugin-v3';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { expect } from 'chai';
import { network } from 'hardhat';
import type { LocalAccount } from 'viem';

import type { DecryptMultipleValues, DecryptMultipleValues__factory } from '../../types/ethers-contracts/index.ts';
import { getAccounts, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers } = connection;

type Hex = `0x${string}`;

// The doc example's cast: `owner` deploys, `alice` is the user (accounts #0 and #1); the user's viem
// account signs the decryption permit.
type Signers = { owner: HardhatEthersSigner; alice: HardhatEthersSigner };
type Accounts = { alice: LocalAccount };

async function deployFixture(): Promise<{
  readonly decryptMultipleValues: DecryptMultipleValues;
  readonly decryptMultipleValuesAddress: Hex;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: DecryptMultipleValues__factory = await ethers.getContractFactory('DecryptMultipleValues');
  const decryptMultipleValues = await factory.deploy();
  const decryptMultipleValuesAddress = (await decryptMultipleValues.getAddress()) as Hex;

  return { decryptMultipleValues, decryptMultipleValuesAddress };
}

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe('DecryptMultipleValues', function () {
  let contract: DecryptMultipleValues;
  let contractAddress: Hex;
  let signers: Signers;
  let accounts: Accounts;

  before(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!connection.fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }

    const suiteSigners = await getSigners(connection);
    const suiteAccounts = getAccounts();
    signers = { owner: suiteSigners.alice, alice: suiteSigners.bob };
    accounts = { alice: suiteAccounts.bob };
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

    const encryptedBool = (await contract.encryptedBool()) as Hex;
    const encryptedUint32 = (await contract.encryptedUint32()) as Hex;
    const encryptedUint64 = (await contract.encryptedUint64()) as Hex;

    // The FHEVM Hardhat plugin provides a set of convenient helper functions
    // that make it easy to perform FHEVM operations within your Hardhat environment.
    const fhevm: HardhatFhevmRuntimeEnvironment = connection.fhevm;

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
      signerAddress: accounts.alice.address,
      signer: accounts.alice,
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

    expect(clearBool?.value).to.equal(true);
    expect(clearUint32?.value).to.equal(BigInt(123456 + 1));
    expect(clearUint64?.value).to.equal(BigInt(78901234567 + 1));
  });
});
