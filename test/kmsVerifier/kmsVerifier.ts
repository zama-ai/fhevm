import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

import { awaitAllDecryptionResults, initGateway } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes256 } from '../utils';

describe('KMSVerifier', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    this.kmsFactory = await ethers.getContractFactory('KMSVerifier');
    await initGateway();
  });

  it('original owner adds one signer, then adds two more signers, then removes one signer', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing

      const origKMSAdd = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
      const deployer = new ethers.Wallet(process.env.PRIVATE_KEY_FHEVM_DEPLOYER!).connect(ethers.provider);
      const kmsVerifier = await this.kmsFactory.attach(origKMSAdd);
      expect(await kmsVerifier.getVersion()).to.equal('KMSVerifier v0.1.0');

      const privKeySigner = process.env['PRIVATE_KEY_KMS_SIGNER_1']!;
      const kmsSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
      const tx = await kmsVerifier.connect(deployer).addSigner(kmsSigner.address);
      await tx.wait();

      expect((await kmsVerifier.getSigners()).length).to.equal(2); // one signer has been added

      const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
      const contract = await contractFactory.connect(this.signers.alice).deploy();
      const tx2 = await contract.requestBool({ gasLimit: 5_000_000 });
      await tx2.wait();
      await awaitAllDecryptionResults();
      const y = await contract.yBool();
      expect(y).to.equal(true); // in this case, one signature still suffices to pass the decrypt (threshold is still 1)

      const kmsSignerDup = new ethers.Wallet(privKeySigner).connect(ethers.provider);
      await expect(kmsVerifier.connect(deployer).addSigner(kmsSignerDup.address)).to.revertedWith(
        'KMSVerifier: Address is already a signer',
      ); // cannot add duplicated signer
      expect((await kmsVerifier.getSigners()).length).to.equal(2);

      const privKeySigner2 = process.env['PRIVATE_KEY_KMS_SIGNER_2']!;
      const kmsSigner2 = new ethers.Wallet(privKeySigner2).connect(ethers.provider);
      const tx3 = await kmsVerifier.connect(deployer).addSigner(kmsSigner2.address);
      await tx3.wait();
      const privKeySigner3 = process.env['PRIVATE_KEY_KMS_SIGNER_3']!;
      const kmsSigner3 = new ethers.Wallet(privKeySigner3).connect(ethers.provider);
      const tx4 = await kmsVerifier.connect(deployer).addSigner(kmsSigner3.address);
      await tx4.wait();
      expect((await kmsVerifier.getSigners()).length).to.equal(4); // 3rd and 4th signer has been added successfully

      const tx5 = await contract.requestUint4({ gasLimit: 5_000_000 });
      await tx5.wait();
      await expect(awaitAllDecryptionResults()).to.revertedWith(
        'KmsVerifier: at least threshold number of signatures required',
      ); // should revert because now we are below the threshold! (we receive only 1 signature but threshold is 2)
      const y2 = await contract.yUint4();
      expect(y2).to.equal(0);

      process.env.NUM_KMS_SIGNERS = '2';
      await awaitAllDecryptionResults();
      const y3 = await contract.yUint4();
      expect(y3).to.equal(4); // with 2 signatures decryption should now succeed

      process.env.NUM_KMS_SIGNERS = '4';
      const tx6 = await contract.requestUint8({ gasLimit: 5_000_000 });
      await tx6.wait();
      await awaitAllDecryptionResults();
      const y4 = await contract.yUint8();
      expect(y4).to.equal(42); // even with more than 2 signatures decryption should still succeed

      const contract2 = await contractFactory.connect(this.signers.alice).deploy();
      const inputAlice = this.instances.alice.createEncryptedInput(
        await contract2.getAddress(),
        this.signers.alice.address,
      );
      inputAlice.addBytes256(bigIntToBytes256(18446744073709550032n));

      process.env.NUM_KMS_SIGNERS = '1';
      const encryptedAmount2 = await inputAlice.encrypt();
      await expect(
        contract2.requestMixedBytes256Trustless(encryptedAmount2.handles[0], encryptedAmount2.inputProof, {
          gasLimit: 5_000_000,
        }),
      ).to.revertedWith('KmsVerifier: at least threshold number of signatures required'); // this should fail because in this case the InputVerifier received only one KMS signature (instead of at least 2);

      if (process.env.IS_COPROCESSOR === 'true') {
        // different format of inputProof for native
        const cheatInputProof = encryptedAmount2.inputProof + encryptedAmount2.inputProof.slice(-130); // trying to cheat by repeating the first kms signer signature
        const cheat = cheatInputProof.slice(0, 5) + '2' + cheatInputProof.slice(6);
        await expect(
          contract2.requestMixedBytes256Trustless(encryptedAmount2.handles[0], cheat, {
            gasLimit: 5_000_000,
          }),
        ).to.revertedWith('Not enough unique KMS input signatures'); // this should fail because in this case the InputVerifier received only one KMS signature (instead of at least 2)
      }
      process.env.NUM_KMS_SIGNERS = '4';
      const encryptedAmount = await inputAlice.encrypt();
      const tx6bis = await contract2.requestMixedBytes256Trustless(
        encryptedAmount.handles[0],
        encryptedAmount.inputProof,
        {
          gasLimit: 5_000_000,
        },
      );
      await tx6bis.wait();
      await awaitAllDecryptionResults();
      const ybis = await contract2.yBytes256();
      expect(ybis).to.equal(ethers.toBeHex(18446744073709550032n, 256));
      const yb = await contract2.yBool();
      expect(yb).to.equal(true);
      const yAdd = await contract2.yAddress();
      expect(yAdd).to.equal('0x8ba1f109551bD432803012645Ac136ddd64DBA72'); // testing trustless mixed with ebytes256, in case of several signatures

      process.env.NUM_KMS_SIGNERS = '2';
      process.env.PRIVATE_KEY_KMS_SIGNER_1 = process.env.PRIVATE_KEY_KMS_SIGNER_0;
      const tx7 = await contract.requestUint16({ gasLimit: 5_000_000 });
      await tx7.wait();
      await expect(awaitAllDecryptionResults()).to.revertedWith('KMS signature verification failed'); // cannot use duplicated signatures if threshold is 2
      const y5 = await contract.yUint16();
      expect(y5).to.equal(0);

      process.env.NUM_KMS_SIGNERS = '1';
      const tx8 = await kmsVerifier.connect(deployer).removeSigner(kmsSigner2.address);
      await tx8.wait();
      await awaitAllDecryptionResults();
      const y6 = await contract.yUint16();
      expect(y6).to.equal(16); // after removing one of the 4 signers, one signature is enough for decryption
    }
  });
});
