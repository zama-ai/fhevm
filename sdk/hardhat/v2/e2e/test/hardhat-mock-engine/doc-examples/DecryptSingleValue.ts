import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatFhevmRuntimeEnvironment } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import * as hre from 'hardhat';

import type { DecryptSingleValue, DecryptSingleValue__factory } from '../../../typechain-types';
import type { Signers } from '../signers';

async function deployFixture(): Promise<{
  readonly decryptSingleValue: DecryptSingleValue;
  readonly decryptSingleValueAddress: string;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: DecryptSingleValue__factory = await ethers.getContractFactory('DecryptSingleValue');
  const decryptSingleValue = (await factory.deploy()) as DecryptSingleValue;
  const decryptSingleValueAddress = await decryptSingleValue.getAddress();

  return { decryptSingleValue, decryptSingleValueAddress };
}

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe('DecryptSingleValue', function () {
  let contract: DecryptSingleValue;
  let contractAddress: string;
  let signers: Signers;

  before(async function () {
    // Check whether the tests are running against an FHEVM cleartext environment
    if (!hre.fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1] };
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

    const encryptedUint32 = await contract.encryptedUint32();

    // The FHEVM Hardhat plugin provides a set of convenient helper functions
    // that make it easy to perform FHEVM operations within your Hardhat environment.
    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    const clearUint32 = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedUint32,
      contractAddress, // The contract address
      signers.alice, // The user wallet
    );

    expect(clearUint32).to.equal(123456 + 1);
  });

  // ❌ Test should fail
  it('decryption should fail', async function () {
    const tx = await contract.connect(signers.alice).initializeUint32Wrong(123456);
    await tx.wait();

    const encryptedUint32 = await contract.encryptedUint32();

    await expect(
      hre.fhevm.userDecryptEuint(FhevmType.euint32, encryptedUint32, contractAddress, signers.alice),
    ).to.be.rejectedWith(new RegExp('is not authorized to user decrypt handle', 'i'));
  });
});
