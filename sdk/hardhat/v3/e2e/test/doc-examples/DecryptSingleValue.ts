import { FhevmType, type HardhatFhevmRuntimeEnvironment } from '@fhevm/hardhat-plugin-v3';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { expect } from 'chai';
import type { LocalAccount } from 'viem';
import { network } from 'hardhat';

import type { DecryptSingleValue, DecryptSingleValue__factory } from '../../types/ethers-contracts/index.ts';
import { expectRejectedWith } from '../utils/expect.ts';
import { getAccounts, getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers } = connection;

type Hex = `0x${string}`;

// The doc example's cast: `owner` deploys, `alice` is the user. They are the suite's accounts #0 and #1;
// the user also needs her viem account, which signs the decryption permit.
type Signers = { owner: HardhatEthersSigner; alice: HardhatEthersSigner };
type Accounts = { alice: LocalAccount };

async function deployFixture(): Promise<{
  readonly decryptSingleValue: DecryptSingleValue;
  readonly decryptSingleValueAddress: Hex;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: DecryptSingleValue__factory = await ethers.getContractFactory('DecryptSingleValue');
  const decryptSingleValue = await factory.deploy();
  const decryptSingleValueAddress = (await decryptSingleValue.getAddress()) as Hex;

  return { decryptSingleValue, decryptSingleValueAddress };
}

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe('DecryptSingleValue', function () {
  let contract: DecryptSingleValue;
  let contractAddress: Hex;
  let signers: Signers;
  let accounts: Accounts;

  before(async function () {
    // Check whether the tests are running against an FHEVM cleartext environment
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
    contractAddress = deployment.decryptSingleValueAddress;
    contract = deployment.decryptSingleValue;
  });

  // ✅ Test should succeed
  it('decryption should succeed', async function () {
    const tx = await contract.connect(signers.alice).initializeUint32(123456);
    await tx.wait();

    const encryptedUint32 = (await contract.encryptedUint32()) as Hex;

    // The FHEVM Hardhat plugin provides a set of convenient helper functions
    // that make it easy to perform FHEVM operations within your Hardhat environment.
    const fhevm: HardhatFhevmRuntimeEnvironment = connection.fhevm;

    const clearUint32 = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedUint32,
      contractAddress, // The contract address
      accounts.alice, // The user account
    );

    expect(clearUint32).to.equal(BigInt(123456 + 1));
  });

  // ❌ Test should fail
  it('decryption should fail', async function () {
    const tx = await contract.connect(signers.alice).initializeUint32Wrong(123456);
    await tx.wait();

    const encryptedUint32 = (await contract.encryptedUint32()) as Hex;

    await expectRejectedWith(
      connection.fhevm.userDecryptEuint(FhevmType.euint32, encryptedUint32, contractAddress, accounts.alice),
      /is not authorized to user decrypt handle/i,
    );
  });
});
