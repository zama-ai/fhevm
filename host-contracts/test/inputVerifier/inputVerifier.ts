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

      const origIVAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
      const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
      const inputVerifier = await this.inputVerifierFactory.attach(origIVAdd);
      expect(await inputVerifier.getVersion()).to.equal('InputVerifier v0.2.0');

      const addressSigner = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;
      let setSigners = await inputVerifier.getCoprocessorSigners();
      setSigners = [...setSigners, addressSigner];
      const tx1 = await inputVerifier.connect(deployer).defineNewContext(setSigners, 1);
      await tx1.wait();

      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2); // one signer has been added

      const contractFactory = await ethers.getContractFactory('TestInput');
      const contract = await contractFactory.connect(this.signers.alice).deploy();
      await contract.waitForDeployment();
      const contractAddress = await contract.getAddress();
      const inputAlice = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice.add64(18446744073709550042n);

      const encryptedAmount = await inputAlice.encrypt();
      const tx2 = await contract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
      await tx2.wait();

      await awaitAllDecryptionResults();
      const y2 = await contract.yUint64();
      expect(y2).to.equal(18446744073709550042n); // in this case, one signature still suffices to pass the decrypt (threshold is still 1)

      setSigners = [...setSigners, addressSigner];
      await expect(inputVerifier.connect(deployer).defineNewContext(setSigners, 1)).to.revertedWithCustomError(
        inputVerifier,
        'CoprocessorAlreadySigner',
      ); // cannot add duplicated signer
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2);

      const coprocessorAddressSigner2 = process.env['COPROCESSOR_SIGNER_ADDRESS_2']!;
      const coprocessorAddressSigner3 = process.env['COPROCESSOR_SIGNER_ADDRESS_3']!;
      let setSigners2 = await inputVerifier.getCoprocessorSigners();
      setSigners2 = [...setSigners2, coprocessorAddressSigner2, coprocessorAddressSigner3];
      const tx3 = await inputVerifier.connect(deployer).defineNewContext(setSigners2, 1);
      await tx3.wait();
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(4); // 3rd and 4th signer has been added successfully

      const tx4 = await inputVerifier.connect(deployer).setThreshold(2n);
      await tx4.wait();
      expect(await inputVerifier.getThreshold()).to.equal(2);

      const inputAlice3 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice3.add64(19);
      const encryptedAmount4 = await inputAlice3.encrypt();
      await expect(contract.requestUint64NonTrivial(encryptedAmount4.handles[0], encryptedAmount4.inputProof))
        .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
        .withArgs(1n); // now we need at least 2 signatures (threshold is 2) but we only have 1

      process.env.NUM_COPROCESSORS = '4';
      const inputAlice4 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice4.add64(1992);
      const encryptedAmount5 = await inputAlice4.encrypt();
      const tx6 = await contract.requestUint64NonTrivial(encryptedAmount5.handles[0], encryptedAmount5.inputProof);
      await tx6.wait();
      await awaitAllDecryptionResults();
      const y5 = await contract.yUint64();
      expect(y5).to.equal(1992n); // even with more than 2 signatures decryption should still succeed

      process.env.NUM_COPROCESSORS = '3';
      const inputAlice5 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice5.add64(873);
      const encryptedAmount6 = await inputAlice5.encrypt();
      const tx7 = await contract.requestUint64NonTrivial(encryptedAmount6.handles[0], encryptedAmount6.inputProof);
      await tx7.wait();
      await awaitAllDecryptionResults();
      const y6 = await contract.yUint64();
      expect(y6).to.equal(873n); // 3 signatures should still work

      const initial_coprocessor_signer_address_1 = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;
      process.env.NUM_COPROCESSORS = '2';
      process.env.COPROCESSOR_SIGNER_ADDRESS_1 = process.env.COPROCESSOR_SIGNER_ADDRESS_0; // WARNING: this makes both addresses identical in env
      const inputAlice6 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice6.add64(999);
      const encryptedAmount7 = await inputAlice6.encrypt();
      await expect(
        contract.requestUint64NonTrivial(encryptedAmount7.handles[0], encryptedAmount7.inputProof),
      ).to.revertedWithCustomError(inputVerifier, 'SignaturesVerificationFailed');

      // Put back the original addresses for future tests
      process.env.COPROCESSOR_SIGNER_ADDRESS_1 = initial_coprocessor_signer_address_1;

      process.env.NUM_COPROCESSORS = '1';
      let setSigners3 = [...(await inputVerifier.getCoprocessorSigners())];
      setSigners3.pop();

      const tx9 = await inputVerifier.connect(deployer).defineNewContext(setSigners3, 1);
      await tx9.wait();
      expect(await inputVerifier.getThreshold()).to.equal(1);

      const inputAlice7 = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
      inputAlice7.add64(1001);
      const encryptedAmount8 = await inputAlice7.encrypt();
      const tx10 = await contract.requestUint64NonTrivial(encryptedAmount8.handles[0], encryptedAmount8.inputProof);
      await tx10.wait();
      await awaitAllDecryptionResults();
      expect(await contract.yUint64()).to.equal(1001n); // after removing one of the 4 signers, one signature is enough for decryption
    }
  });

  it('input tests with several non-trivial inputs', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing

      const origIVAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
      const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
      const inputVerifier = await this.inputVerifierFactory.attach(origIVAdd);

      const addressSigner1 = process.env['COPROCESSOR_SIGNER_ADDRESS_0']!;
      const addressSigner2 = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;
      console.log('addressSigner1', addressSigner1);
      console.log('addressSigner2', addressSigner2);
      const tx1 = await inputVerifier.connect(deployer).defineNewContext([addressSigner1, addressSigner2], 2);
      await tx1.wait();

      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2); // 2 signers have been registered
      expect(await inputVerifier.getThreshold()).to.equal(2); // threshold is 2

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

      process.env.NUM_COPROCESSORS = '1';
    }
  });

  it('cannot add/remove signers if not the owner', async function () {
    const origCoprocessorAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    const inputVerifier = await this.inputVerifierFactory.attach(origCoprocessorAdd);
    let setSigners = await inputVerifier.getCoprocessorSigners();
    const randomAccount = this.signers.carol;
    setSigners = [...setSigners, randomAccount];
    await expect(inputVerifier.connect(randomAccount).defineNewContext(setSigners, 2)).to.be.revertedWithCustomError(
      inputVerifier,
      'NotHostOwner',
    );
  });
});
