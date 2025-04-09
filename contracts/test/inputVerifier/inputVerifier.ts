import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

import { awaitAllDecryptionResults, initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('InputVerifier', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    this.inputVerifierFactory = await ethers.getContractFactory('InputVerifier');
    await initDecryptionOracle();
  });

  it('original owner adds one signer, then adds one more signers, then adds one more, then removes one signer', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing

      const origIVAdd = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier')).INPUT_VERIFIER_CONTRACT_ADDRESS;
      const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
      const inputVerifier = await this.inputVerifierFactory.attach(origIVAdd);
      expect(await inputVerifier.getVersion()).to.equal('InputVerifier v0.1.0');

      const addressSigner = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;
      const tx = await inputVerifier.connect(deployer).addSigner(addressSigner);
      await tx.wait();

      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2); // one signer has been added

      const contractFactory = await ethers.getContractFactory('TestInput');
      const contract = await contractFactory.connect(this.signers.alice).deploy();
      await contract.waitForDeployment();
      const contractAddress = await contract.getAddress();
      const inputAlice = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice.add64(18446744073709550042n);
      const encryptedAmount = await inputAlice.encrypt();

      await expect(contract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof))
        .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
        .withArgs(1n); // should revert because now we are below the threshold! (we receive only 1 signature but threshold is 2)

      await awaitAllDecryptionResults();
      const y = await contract.yUint64();
      expect(y).to.equal(0n);

      process.env.NUM_COPROCESSORS = '2';
      const encryptedAmount2 = await inputAlice.encrypt();
      const tx2 = await contract.requestUint64NonTrivial(encryptedAmount2.handles[0], encryptedAmount2.inputProof);
      await tx2.wait();

      await awaitAllDecryptionResults();
      const y2 = await contract.yUint64();
      expect(y2).to.equal(18446744073709550042n); // in this case, one signature still suffices to pass the decrypt (threshold is still 1)

      const addressSigner2 = process.env['COPROCESSOR_SIGNER_ADDRESS_2']!;
      const tx3 = await inputVerifier.connect(deployer).addSigner(addressSigner2);
      await tx3.wait();
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(3);

      const inputAlice2 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice2.add64(42);
      const encryptedAmount3 = await inputAlice2.encrypt();
      const tx4 = await contract.requestUint64NonTrivial(encryptedAmount3.handles[0], encryptedAmount3.inputProof);
      await tx4.wait();

      await awaitAllDecryptionResults();
      const y4 = await contract.yUint64();
      expect(y4).to.equal(42n); // in this case, two signatures still suffice to pass the decrypt (threshold is still 2)

      const addressSigner3 = process.env['COPROCESSOR_SIGNER_ADDRESS_3']!;
      const tx5 = await inputVerifier.connect(deployer).addSigner(addressSigner3);
      await tx5.wait();
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(4);

      const inputAlice3 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice3.add64(19);
      const encryptedAmount4 = await inputAlice3.encrypt();
      await expect(contract.requestUint64NonTrivial(encryptedAmount4.handles[0], encryptedAmount4.inputProof))
        .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
        .withArgs(2n); // now we need at least 3 signatures

      process.env.NUM_COPROCESSORS = '4';
      const inputAlice4 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice4.add64(1992);
      const encryptedAmount5 = await inputAlice4.encrypt();
      const tx6 = await contract.requestUint64NonTrivial(encryptedAmount5.handles[0], encryptedAmount5.inputProof);
      await tx6.wait();
      await awaitAllDecryptionResults();
      const y5 = await contract.yUint64();
      expect(y5).to.equal(1992n);

      process.env.NUM_COPROCESSORS = '3';
      const inputAlice5 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice5.add64(873);
      const encryptedAmount6 = await inputAlice5.encrypt();
      const tx7 = await contract.requestUint64NonTrivial(encryptedAmount6.handles[0], encryptedAmount6.inputProof);
      await tx7.wait();
      await awaitAllDecryptionResults();
      const y6 = await contract.yUint64();
      expect(y6).to.equal(873n); // 3 signatures should still work

      const tx8 = await inputVerifier.connect(deployer).removeSigner(addressSigner3);
      await tx8.wait();
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(3);
      const tx9 = await inputVerifier.connect(deployer).removeSigner(addressSigner2);
      await tx9.wait();
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2);
      const tx10 = await inputVerifier.connect(deployer).removeSigner(addressSigner);
      await tx10.wait();
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(1);
      process.env.NUM_COPROCESSORS = '1';
    }
  });

  it('input tests with several non-trivial inputs', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing

      const origIVAdd = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier')).INPUT_VERIFIER_CONTRACT_ADDRESS;
      const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
      const inputVerifier = await this.inputVerifierFactory.attach(origIVAdd);
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(1);

      const addressSigner = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;
      const tx = await inputVerifier.connect(deployer).addSigner(addressSigner);
      await tx.wait();

      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2); // one signer has been added

      const contractFactory = await ethers.getContractFactory('TestInput');
      const contract = await contractFactory.connect(this.signers.alice).deploy();
      await contract.waitForDeployment();
      const contractAddress = await contract.getAddress();
      const inputAlice = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice.addBool(true);
      inputAlice.add8(42);
      inputAlice.addAddress('0x1E69D5aa8750Ff56c556C164fE6feaE71BBA9a09');
      const encryptedAmount = await inputAlice.encrypt();

      await expect(
        contract.requestMixedNonTrivial(
          encryptedAmount.handles[0],
          encryptedAmount.handles[1],
          encryptedAmount.handles[2],
          encryptedAmount.inputProof,
        ),
      )
        .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
        .withArgs(1n); // should revert because now we are below the threshold! (we receive only 1 signature but threshold is 2)

      await awaitAllDecryptionResults();
      const y = await contract.yBool();
      expect(y).to.equal(false);

      process.env.NUM_COPROCESSORS = '2';
      const encryptedAmount2 = await inputAlice.encrypt();
      const tx2 = await contract.requestMixedNonTrivial(
        encryptedAmount2.handles[0],
        encryptedAmount2.handles[1],
        encryptedAmount2.handles[2],
        encryptedAmount2.inputProof,
      );
      await tx2.wait();

      await awaitAllDecryptionResults();
      const y2 = await contract.yBool();
      expect(y2).to.equal(true); // in this case, one signature still suffices to pass the decrypt (threshold is still 1)
      const y_8 = await contract.yUint8();
      const y_Add = await contract.yAddress();
      expect(y_8).to.equal(42);
      expect(y_Add).to.equal('0x1E69D5aa8750Ff56c556C164fE6feaE71BBA9a09');

      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2);
      const tx10 = await inputVerifier.connect(deployer).removeSigner(addressSigner);
      await tx10.wait();
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(1);
      process.env.NUM_COPROCESSORS = '1';
    }
  });

  it('cannot add/remove signers if not the owner', async function () {
    const origInputAdd = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    const inputVerifier = await this.inputVerifierFactory.attach(origInputAdd);
    const randomAccount = this.signers.carol;

    await expect(inputVerifier.connect(randomAccount).addSigner(randomAccount)).to.be.revertedWithCustomError(
      inputVerifier,
      'OwnableUnauthorizedAccount',
    );

    await expect(inputVerifier.connect(randomAccount).removeSigner(randomAccount)).to.be.revertedWithCustomError(
      inputVerifier,
      'OwnableUnauthorizedAccount',
    );
  });
});
