import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { userDecryptSingleHandle } from '../utils';

describe('Paused gateway', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);

    // Initialize TestInput contract.
    const testInputContractFactory = await ethers.getContractFactory('TestInput');
    this.testInputContract = await testInputContractFactory.connect(this.signers.alice).deploy();
    this.testInputContractAddress = await this.testInputContract.getAddress();
    await this.testInputContract.waitForDeployment();

    // Initialize UserDecrypt contract.
    const userDecryptContractFactory = await ethers.getContractFactory('UserDecrypt');
    this.userDecryptContract = await userDecryptContractFactory.connect(this.signers.alice).deploy();
    await this.userDecryptContract.waitForDeployment();
    this.userDecryptContractAddress = await this.userDecryptContract.getAddress();

    // Initialize HTTPPublicDecrypt contract.
    const httpPublicDecryptContractFactory = await ethers.getContractFactory('HTTPPublicDecrypt');
    this.httpPublicDecryptContract = await httpPublicDecryptContractFactory.connect(this.signers.alice).deploy();
    await this.httpPublicDecryptContract.waitForDeployment();
  });

  // The following test case should cover the InputVerification.verifyProofRequest method calling.
  it('test paused gateway user input (input verification request)', async function () {
    const inputAlice = this.instances.alice.createEncryptedInput(
      this.testInputContractAddress,
      this.signers.alice.address,
    );
    inputAlice.add64(18446744073709550042n);

    await expect(inputAlice.encrypt()).to.be.rejectedWith(new RegExp('Input request failed'));
  });

  // The following test case should cover the Decryption.userDecryptionRequest method calling.
  it('test paused gateway user decrypt (user decryption request)', async function () {
    const handle = await this.userDecryptContract.xBool();
    const { publicKey, privateKey } = this.instances.alice.generateKeypair();
    await expect(
      userDecryptSingleHandle(
        handle,
        this.userDecryptContractAddress,
        this.instances.alice,
        this.signers.alice,
        privateKey,
        publicKey,
      ),
    ).to.be.rejectedWith(new RegExp('User decrypt failed'));
  });

  // The following test case should cover the Decryption.publicDecryptionRequest method calling.
  it('test paused gateway HTTP public decrypt (public decryption request)', async function () {
    const handleBool = await this.httpPublicDecryptContract.xBool();
    const handleAddress = await this.httpPublicDecryptContract.xAddress();
    const handle32 = await this.httpPublicDecryptContract.xUint32();
    await expect(this.instances.alice.publicDecrypt([handleAddress, handle32, handleBool])).to.be.rejectedWith(
      new RegExp('Public decrypt failed'),
    );
  });
});
