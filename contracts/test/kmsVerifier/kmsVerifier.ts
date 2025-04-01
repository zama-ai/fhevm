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
      let setSigners = await kmsVerifier.getSigners();
      setSigners = [...setSigners, addressSigner];
      let tx = await kmsVerifier.connect(deployer).defineNewContext(setSigners, 1);
      await tx.wait();

      expect((await kmsVerifier.getSigners()).length).to.equal(2); // one signer has been added

      const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
      const contract = (await contractFactory.connect(this.signers.alice).deploy()) as TestAsyncDecrypt;
      tx = await contract.requestBool();
      await tx.wait();
      await awaitAllDecryptionResults();
      expect(await contract.yBool()).to.equal(true); // in this case, one signature still suffices to pass the decrypt (threshold is still 1)

      setSigners = [...setSigners, addressSigner];
      await expect(kmsVerifier.connect(deployer).defineNewContext(setSigners, 1)).to.revertedWithCustomError(
        kmsVerifier,
        'KMSAlreadySigner',
      ); // cannot add duplicated signer
      expect((await kmsVerifier.getSigners()).length).to.equal(2);

      const kmsSigner2Address = process.env['KMS_SIGNER_ADDRESS_2']!;
      const kmsSigner3Address = process.env['KMS_SIGNER_ADDRESS_3']!;
      let setSigners2 = await kmsVerifier.getSigners();
      setSigners2 = [...setSigners2, kmsSigner2Address, kmsSigner3Address];
      tx = await kmsVerifier.connect(deployer).defineNewContext(setSigners2, 1);
      await tx.wait();
      expect((await kmsVerifier.getSigners()).length).to.equal(4); // 3rd and 4th signer has been added successfully

      tx = await kmsVerifier.connect(deployer).setThreshold(2n);
      await tx.wait();
      expect(await kmsVerifier.getThreshold()).to.equal(2);

      process.env.NUM_KMS_NODES = '4';
      tx = await contract.requestUint8();
      await tx.wait();
      await awaitAllDecryptionResults();
      expect(await contract.yUint8()).to.equal(42); // even with more than 2 signatures decryption should still succeed

      const contract2 = await contractFactory.connect(this.signers.alice).deploy();
      const inputAlice = this.instances.alice.createEncryptedInput(
        await contract2.getAddress(),
        this.signers.alice.address,
      );
      inputAlice.addBytes256(bigIntToBytes256(18446744073709550032n));

      const encryptedAmount = await inputAlice.encrypt();
      tx = await contract2.requestMixedBytes256(encryptedAmount.handles[0], encryptedAmount.inputProof);
      await tx.wait();
      await awaitAllDecryptionResults();

      expect(await contract2.yBytes256()).to.equal(ethers.toBeHex(18446744073709550032n, 256));
      expect(await contract2.yBool()).to.equal(true);
      expect(await contract2.yAddress()).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72'); // testing trustless mixed with ebytes256, in case of several signatures

      process.env.NUM_KMS_NODES = '2';
      process.env.KMS_SIGNER_ADDRESS_1 = process.env.KMS_SIGNER_ADDRESS_0;
      tx = await contract.requestUint16();
      await tx.wait();
      await expect(awaitAllDecryptionResults()).to.revertedWithCustomError(contract, 'InvalidKMSSignatures'); // cannot use duplicated signatures if threshold is 2
      expect(await contract.yUint16()).to.equal(0);

      process.env.NUM_KMS_NODES = '1';
      let setSigners3 = [...(await kmsVerifier.getSigners())];
      setSigners3.pop();
      tx = await kmsVerifier.connect(deployer).defineNewContext(setSigners3, 1);
      await tx.wait();
      expect(await kmsVerifier.getThreshold()).to.equal(1);

      tx = await contract.requestUint16();
      await tx.wait();
      await awaitAllDecryptionResults();
      expect(await contract.yUint16()).to.equal(16); // after removing one of the 4 signers, one signature is enough for decryption
    }
  });

  it('cannot add/remove signers if not the owner', async function () {
    const origKMSAdd = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
    const kmsVerifier = await this.kmsFactory.attach(origKMSAdd);
    let setSigners = await kmsVerifier.getSigners();
    const randomAccount = this.signers.carol;
    setSigners = [...setSigners, randomAccount];
    await expect(kmsVerifier.connect(randomAccount).defineNewContext(setSigners, 2)).to.be.revertedWithCustomError(
      kmsVerifier,
      'OwnableUnauthorizedAccount',
    );
  });
});
