import type { FhevmInstance, PublicDecryptResults } from '@zama-fhe/relayer-sdk/node';
import { expect } from 'chai';
import dotenv from 'dotenv';
import type { ethers as EthersT } from 'ethers';
import fs from 'fs';
import { ethers } from 'hardhat';

import { KMSVerifier, KMSVerifier__factory, TestInput } from '../../typechain-types';
import { createInstance } from '../instance';
import { Signers, getSigners, initSigners } from '../signers';

describe('KmsVerifier', function () {
  let signers: Signers;
  let instance: FhevmInstance;
  let kmsVerifierFactory: KMSVerifier__factory;
  let kmsVerifier: KMSVerifier;
  let deployer: EthersT.Wallet;
  let testInputContract: TestInput;
  let testInputContractAddress: string;

  // This pass is necessary to restore the original KMSVerifier state
  // If this pass is omitted, future tests may fail
  afterEach(async function () {
    process.env.NUM_KMS_NODES = '1';
    const kmsAddressSigner0 = process.env['KMS_SIGNER_ADDRESS_0']!;
    const tx = await kmsVerifier.connect(deployer).defineNewContext([kmsAddressSigner0], 1);
    await tx.wait();
  });

  before(async function () {
    await initSigners(2);
    signers = await getSigners();
    kmsVerifierFactory = await ethers.getContractFactory('KMSVerifier');
  });

  beforeEach(async function () {
    process.env.NUM_KMS_NODES = '1';
    const origIVAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).KMS_VERIFIER_CONTRACT_ADDRESS;
    deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    kmsVerifier = kmsVerifierFactory.attach(origIVAdd) as KMSVerifier;
    expect(await kmsVerifier.getVersion()).to.equal('KMSVerifier v0.1.0');
    await resetInstance();
  });

  async function addSigners(params: { list: string[]; threshold: number }) {
    let signersList = await kmsVerifier.getKmsSigners();
    signersList = [...signersList, ...params.list];
    const tx = await kmsVerifier.connect(deployer).defineNewContext(signersList, params.threshold);
    await tx.wait();
  }

  async function removeLastSigner(params: { threshold: number }) {
    let signersList = [...(await kmsVerifier.getKmsSigners())];
    signersList.pop();
    const tx = await kmsVerifier.connect(deployer).defineNewContext(signersList, params.threshold);
    await tx.wait();
  }

  async function resetInstance() {
    instance = await createInstance();
  }

  async function testInputSetPublicUint64(value: bigint) {
    let inputAlice = instance.createEncryptedInput(testInputContractAddress, signers.alice.address);
    inputAlice.add64(value);
    let encryptedAmount = await inputAlice.encrypt();

    let tx = await testInputContract.setPublicUint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
  }

  async function testInputCheckPublicUint64(results: PublicDecryptResults) {
    let tx = await testInputContract.checkPublicUint64(results.abiEncodedClearValues, results.decryptionProof);
    await tx.wait();
  }

  async function publicDecryptUint64(): Promise<{ results: PublicDecryptResults; clearUint64: bigint }> {
    const encUint64 = (await testInputContract.getEuint64()) as `0x${string}`;
    if (typeof encUint64 !== 'string') {
      throw new Error(`Unexpected getEuint64() return type`);
    }
    const res = await instance.publicDecrypt([encUint64]);
    const clearUint64 = res.clearValues[encUint64];
    if (typeof clearUint64 !== 'bigint') {
      throw new Error(
        `Unexpected user decryption result type. Expected 'bigint', got '${typeof clearUint64}' instead.`,
      );
    }
    return { results: res, clearUint64 };
  }

  async function deployTestInput() {
    const testInputContractFactory = await ethers.getContractFactory('TestInput');
    testInputContract = await testInputContractFactory.connect(signers.alice).deploy();
    await testInputContract.waitForDeployment();
    testInputContractAddress = await testInputContract.getAddress();
  }

  async function expectKmsSigners(params: {
    numberOfKmsNodes: number;
    numberOfSigners: number;
    threshold: number;
    signers: string[];
  }) {
    const kmsSignersList = await kmsVerifier.getKmsSigners();
    expect(process.env.NUM_KMS_NODES).to.equal(params.numberOfKmsNodes.toString());
    expect(kmsSignersList.length).to.eq(params.numberOfSigners);
    for (let i = 0; i < params.signers.length; ++i) {
      expect(kmsSignersList.includes(params.signers[i]));
    }
    expect(await kmsVerifier.getThreshold()).to.equal(params.threshold);
  }

  it('original owner adds one signer, then adds one more signers, then adds one more, then removes one signer', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing
      const kmsAddressSigner0 = process.env['KMS_SIGNER_ADDRESS_0']!;
      const kmsAddressSigner1 = process.env['KMS_SIGNER_ADDRESS_1']!;
      const kmsAddressSigner2 = process.env['KMS_SIGNER_ADDRESS_2']!;
      const kmsAddressSigner3 = process.env['KMS_SIGNER_ADDRESS_3']!;

      // - 1 active kms node
      // - 1 registered kms nodes
      // - threshold 1
      await expectKmsSigners({
        numberOfKmsNodes: 1,
        numberOfSigners: 1,
        threshold: 1,
        signers: [kmsAddressSigner0],
      });

      // Register a new kms signer
      await addSigners({ list: [kmsAddressSigner1], threshold: 1 });
      await resetInstance();

      // - 1 active kms signer
      // - 2 registered kms signers
      // - threshold 1
      await expectKmsSigners({
        numberOfKmsNodes: 1,
        numberOfSigners: 2,
        threshold: 1,
        signers: [kmsAddressSigner0, kmsAddressSigner1],
      });

      // Deploy TestInput
      await deployTestInput();

      await testInputSetPublicUint64(18446744073709550042n);
      let pubDec = await publicDecryptUint64();

      // in this case, one signature still suffices to pass the decrypt (threshold is still 1)
      expect(pubDec.clearUint64).to.equal(18446744073709550042n);
      // check signatures should succeed
      await testInputCheckPublicUint64(pubDec.results);

      // Try add a kms signer already registered
      await expect(addSigners({ list: [kmsAddressSigner1], threshold: 1 })).to.revertedWithCustomError(
        kmsVerifier,
        'KMSAlreadySigner',
      ); // cannot add duplicated signer
      expect((await kmsVerifier.getKmsSigners()).length).to.equal(2);

      // Add 2 new kms signers (total 4)
      await addSigners({ list: [kmsAddressSigner2, kmsAddressSigner3], threshold: 1 });
      let tx = await kmsVerifier.connect(deployer).setThreshold(2n);
      await tx.wait();
      await resetInstance();

      // - 1 active kms node
      // - 4 registered kms nodes
      // - threshold 2
      await expectKmsSigners({
        numberOfKmsNodes: 1,
        numberOfSigners: 4,
        threshold: 2,
        signers: [kmsAddressSigner0, kmsAddressSigner1, kmsAddressSigner2, kmsAddressSigner3],
      });

      // Let's disable KMS threshold assertion in the FhevmInstance.publicDecrypt()
      process.env.DISABLE_TEST_KMS_THRESHOLD = '1';
      try {
        // Let's set a new euint64 value
        await testInputSetPublicUint64(19n);
        // get the KMS decryption and signatures (only 1 here)
        pubDec = await publicDecryptUint64();
        // decrypted value is correct, but the number of KMS signatures is not enough
        expect(pubDec.clearUint64).to.equal(19n);

        // now we need at least 2 signatures (threshold is 2) but we only have 1
        await expect(testInputCheckPublicUint64(pubDec.results))
          .to.revertedWithCustomError(kmsVerifier, 'KMSSignatureThresholdNotReached')
          .withArgs(1n);
      } finally {
        process.env.DISABLE_TEST_KMS_THRESHOLD = '0';
      }

      process.env.NUM_KMS_NODES = '4';

      // - 4 active kms nodes
      // - 4 registered kms nodes
      // - threshold 2
      await expectKmsSigners({
        numberOfKmsNodes: 4,
        numberOfSigners: 4,
        threshold: 2,
        signers: [kmsAddressSigner0, kmsAddressSigner1, kmsAddressSigner2, kmsAddressSigner3],
      });

      // even with more than 2 signatures decryption should still succeed
      await testInputSetPublicUint64(1992n);
      pubDec = await publicDecryptUint64();
      expect(pubDec.clearUint64).to.equal(1992n);
      await testInputCheckPublicUint64(pubDec.results);

      process.env.NUM_KMS_NODES = '3';

      // - 3 active kms nodes
      // - 4 registered kms nodes
      // - threshold 2
      await expectKmsSigners({
        numberOfKmsNodes: 3,
        numberOfSigners: 4,
        threshold: 2,
        signers: [kmsAddressSigner0, kmsAddressSigner1, kmsAddressSigner2, kmsAddressSigner3],
      });

      // 3 signatures should still work
      await testInputSetPublicUint64(873n);
      pubDec = await publicDecryptUint64();
      expect(pubDec.clearUint64).to.equal(873n);
      await testInputCheckPublicUint64(pubDec.results);

      const initial_kms_signer_address_1 = process.env['KMS_SIGNER_ADDRESS_1']!;
      process.env.NUM_KMS_NODES = '2';
      // WARNING: this makes both addresses identical in env
      // Force having actually 1 single registered kms signer
      process.env.KMS_SIGNER_ADDRESS_1 = process.env.KMS_SIGNER_ADDRESS_0;

      // Let's disable KMS threshold assertion in the FhevmInstance.publicDecrypt()
      process.env.DISABLE_TEST_KMS_THRESHOLD = '1';
      try {
        await testInputSetPublicUint64(999n);
        pubDec = await publicDecryptUint64();
        expect(pubDec.clearUint64).to.equal(999n);
        // FHE.checkSignatures should revert with FHE.InvalidKMSSignatures error
        await expect(testInputCheckPublicUint64(pubDec.results)).to.revertedWithCustomError(
          testInputContract,
          'InvalidKMSSignatures',
        );
      } finally {
        process.env.DISABLE_TEST_KMS_THRESHOLD = '0';
      }

      // Put back the original addresses for future tests
      process.env.KMS_SIGNER_ADDRESS_1 = initial_kms_signer_address_1;

      // Remove last kms signer
      process.env.NUM_KMS_NODES = '1';
      await removeLastSigner({ threshold: 1 });
      await resetInstance();

      await expectKmsSigners({
        numberOfKmsNodes: 1,
        numberOfSigners: 3,
        threshold: 1,
        signers: [kmsAddressSigner0, kmsAddressSigner1, kmsAddressSigner2],
      });

      // after removing one of the 4 signers, one signature is enough for decryption
      await testInputSetPublicUint64(1001n);
      pubDec = await publicDecryptUint64();
      expect(pubDec.clearUint64).to.equal(1001n);
      await testInputCheckPublicUint64(pubDec.results);
    }
  });

  it('cannot add/remove signers if not the owner', async function () {
    let kmsSignersList = await kmsVerifier.getKmsSigners();
    const randomAccount = signers.carol;
    kmsSignersList = [...kmsSignersList, randomAccount.address];
    await expect(kmsVerifier.connect(randomAccount).defineNewContext(kmsSignersList, 2)).to.be.revertedWithCustomError(
      kmsVerifier,
      'NotHostOwner',
    );
  });
});
