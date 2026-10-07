import { FhevmType } from '@fhevm/hardhat-plugin';
import type { HardhatFhevmRuntimeEnvironment } from '@fhevm/hardhat-plugin';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import * as hre from 'hardhat';

import type { EncryptMultipleValues, EncryptMultipleValues__factory } from '../../../typechain-types';
import type { Signers } from '../signers';

async function deployFixture(): Promise<{
  readonly encryptMultipleValues: EncryptMultipleValues;
  readonly encryptMultipleValuesAddress: string;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: EncryptMultipleValues__factory = await ethers.getContractFactory('EncryptMultipleValues');
  const encryptMultipleValues = (await factory.deploy()) as EncryptMultipleValues;
  const encryptMultipleValuesAddress = await encryptMultipleValues.getAddress();

  return { encryptMultipleValues, encryptMultipleValuesAddress };
}

/**
 * This trivial example demonstrates the FHE encryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
describe('EncryptMultipleValues', function () {
  let contract: EncryptMultipleValues;
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
    contractAddress = deployment.encryptMultipleValuesAddress;
    contract = deployment.encryptMultipleValues;
  });

  // ✅ Test should succeed
  it('encryption should succeed', async function () {
    // Use the FHEVM Hardhat plugin runtime environment
    // to perform FHEVM input encryptions.
    const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

    const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address);

    input.addBool(true);
    input.add32(123456);
    input.addAddress(signers.owner.address);

    const enc = await input.encrypt();

    const inputEbool = enc.handles[0];
    const inputEuint32 = enc.handles[1];
    const inputEaddress = enc.handles[2];
    const inputProof = enc.inputProof;

    // Don't forget to call `connect(signers.alice)` to make sure
    // the Solidity `msg.sender` is `signers.alice.address`.
    const tx = await contract.connect(signers.alice).initialize(inputEbool, inputEuint32, inputEaddress, inputProof);
    await tx.wait();

    const encryptedBool = await contract.encryptedBool();
    const encryptedUint32 = await contract.encryptedUint32();
    const encryptedAddress = await contract.encryptedAddress();

    const clearBool = await fhevm.userDecryptEbool(
      encryptedBool,
      contractAddress, // The contract address
      signers.alice, // The user wallet
    );

    const clearUint32 = await fhevm.userDecryptEuint(
      FhevmType.euint32, // Specify the encrypted type
      encryptedUint32,
      contractAddress, // The contract address
      signers.alice, // The user wallet
    );

    const clearAddress = await fhevm.userDecryptEaddress(
      encryptedAddress,
      contractAddress, // The contract address
      signers.alice, // The user wallet
    );

    expect(clearBool).to.equal(true);
    expect(clearUint32).to.equal(123456);
    expect(clearAddress).to.equal(signers.owner.address);
  });
});
