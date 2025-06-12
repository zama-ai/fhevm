import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

import { TestAsyncDecrypt } from '../../types';
import { awaitAllDecryptionResults, initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes256 } from '../utils';

describe('KMSVerifier', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    this.kmsFactory = await ethers.getContractFactory('KMSVerifier');
    await initDecryptionOracle();
  });

  it('original owner adds one signer, then adds two more signers, then removes one signer', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing

      const origKMSAdd = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
      const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
      const kmsVerifier = await this.kmsFactory.attach(origKMSAdd);
      expect(await kmsVerifier.getVersion()).to.equal('KMSVerifier v0.1.0');

      const addressSigner = process.env['KMS_SIGNER_ADDRESS_1']!;
      let setSigners = await kmsVerifier.getKmsSigners();
      setSigners = [...setSigners, addressSigner];
      const tx1 = await kmsVerifier.connect(deployer).defineNewContext(setSigners, 1);
      await tx1.wait();

      expect((await kmsVerifier.getKmsSigners()).length).to.equal(2); // one signer has been added

      const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
      const contract = (await contractFactory.connect(this.signers.alice).deploy()) as TestAsyncDecrypt;
      const tx2 = await contract.requestBool();
      await tx2.wait();
      await awaitAllDecryptionResults();
      expect(await contract.yBool()).to.equal(true); // in this case, one signature still suffices to pass the decrypt (threshold is still 1)

      setSigners = [...setSigners, addressSigner];
      await expect(kmsVerifier.connect(deployer).defineNewContext(setSigners, 1)).to.revertedWithCustomError(
        kmsVerifier,
        'KMSAlreadySigner',
      ); // cannot add duplicated signer
      expect((await kmsVerifier.getKmsSigners()).length).to.equal(2);

      const kmsSigner2Address = process.env['KMS_SIGNER_ADDRESS_2']!;
      const kmsSigner3Address = process.env['KMS_SIGNER_ADDRESS_3']!;
      let setSigners2 = await kmsVerifier.getKmsSigners();
      setSigners2 = [...setSigners2, kmsSigner2Address, kmsSigner3Address];
      const tx3 = await kmsVerifier.connect(deployer).defineNewContext(setSigners2, 1);
      await tx3.wait();
      expect((await kmsVerifier.getKmsSigners()).length).to.equal(4); // 3rd and 4th signer has been added successfully

      const tx4 = await kmsVerifier.connect(deployer).setThreshold(2n);
      await tx4.wait();
      expect(await kmsVerifier.getThreshold()).to.equal(2);

      const tx5 = await contract.requestUint16();
      await tx5.wait();

      await expect(awaitAllDecryptionResults())
        .to.revertedWithCustomError(kmsVerifier, 'KMSSignatureThresholdNotReached')
        .withArgs(1n); // should revert because now we are below the threshold! (we receive only 1 signature but threshold is 2)

      process.env.NUM_KMS_NODES = '4';

      const tx6 = await contract.requestUint8();
      await tx6.wait();
      await awaitAllDecryptionResults();
      expect(await contract.yUint8()).to.equal(42); // even with more than 2 signatures decryption should still succeed

      process.env.NUM_KMS_NODES = '2';
      process.env.KMS_SIGNER_ADDRESS_1 = process.env.KMS_SIGNER_ADDRESS_0;
      const tx8 = await contract.requestUint16();
      await tx8.wait();
      await expect(awaitAllDecryptionResults()).to.revertedWithCustomError(contract, 'InvalidKMSSignatures'); // cannot use duplicated signatures if threshold is 2
      expect(await contract.yUint16()).to.equal(0);

      process.env.NUM_KMS_NODES = '1';
      let setSigners3 = [...(await kmsVerifier.getKmsSigners())];
      setSigners3.pop();

      const tx9 = await kmsVerifier.connect(deployer).defineNewContext(setSigners3, 1);
      await tx9.wait();
      expect(await kmsVerifier.getThreshold()).to.equal(1);

      const tx10 = await contract.requestUint16();
      await tx10.wait();
      await awaitAllDecryptionResults();
      expect(await contract.yUint16()).to.equal(16); // after removing one of the 4 signers, one signature is enough for decryption
    }
  });

  it('cannot add/remove signers if not the owner', async function () {
    const origKMSAdd = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
    const kmsVerifier = await this.kmsFactory.attach(origKMSAdd);
    let setSigners = await kmsVerifier.getKmsSigners();
    const randomAccount = this.signers.carol;
    setSigners = [...setSigners, randomAccount];
    await expect(kmsVerifier.connect(randomAccount).defineNewContext(setSigners, 2)).to.be.revertedWithCustomError(
      kmsVerifier,
      'OwnableUnauthorizedAccount',
    );
  });
});
