import { expect } from 'chai';
import dotenv from 'dotenv';
import type { ethers as EthersT } from 'ethers';
import fs from 'fs';
import { ethers } from 'hardhat';

import { InputVerifier, InputVerifier__factory, TestInput } from '../../typechain-types';
import { createInstances } from '../instance';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';
import { userDecryptSingleHandle } from '../utils';

describe('InputVerifier', function () {
  let signers: Signers;
  let instances: FhevmInstances;
  let inputVerifierFactory: InputVerifier__factory;
  let inputVerifier: InputVerifier;
  let deployer: EthersT.Wallet;
  let testInputContract: TestInput;
  let testInputContractAddress: string;
  let aliceKeys: {
    publicKey: string;
    privateKey: string;
  };

  // This pass is necessary to restore the original InputVerifier state
  // If this pass is omitted, future tests may fail
  afterEach(async function () {
    process.env.NUM_COPROCESSORS = '1';
    const coprocessorAddressSigner0 = process.env['COPROCESSOR_SIGNER_ADDRESS_0']!;
    const tx = await inputVerifier.connect(deployer).defineNewContext([coprocessorAddressSigner0], 1);
    await tx.wait();
  });

  before(async function () {
    await initSigners(2);
    signers = await getSigners();
    instances = await createInstances(signers);
    inputVerifierFactory = await ethers.getContractFactory('InputVerifier');
    aliceKeys = instances.alice.generateKeypair();
  });

  beforeEach(async function () {
    process.env.NUM_COPROCESSORS = '1';
    const origIVAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    inputVerifier = inputVerifierFactory.attach(origIVAdd) as InputVerifier;
    expect(await inputVerifier.getVersion()).to.equal('InputVerifier v0.2.0');
  });

  async function addSigners(params: { list: string[]; threshold: number }) {
    let signersList = await inputVerifier.getCoprocessorSigners();
    signersList = [...signersList, ...params.list];
    const tx = await inputVerifier.connect(deployer).defineNewContext(signersList, params.threshold);
    await tx.wait();
  }

  async function removeLastSigner(params: { threshold: number }) {
    let signersList = [...(await inputVerifier.getCoprocessorSigners())];
    signersList.pop();
    const tx = await inputVerifier.connect(deployer).defineNewContext(signersList, params.threshold);
    await tx.wait();
  }

  async function testInputSetUint64(value: bigint) {
    let inputAlice = instances.alice.createEncryptedInput(testInputContractAddress, signers.alice.address);
    inputAlice.add64(value);
    let encryptedAmount = await inputAlice.encrypt();

    let tx = await testInputContract.setUint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
  }

  async function getClearUint64(keypair: { privateKey: string; publicKey: string }): Promise<bigint> {
    const encUint64 = await testInputContract.getEuint64();
    const clearUint64 = await userDecryptSingleHandle(
      encUint64,
      testInputContractAddress,
      instances.alice,
      signers.alice,
      keypair.privateKey,
      keypair.publicKey,
    );
    if (typeof clearUint64 !== 'bigint') {
      throw new Error(
        `Unexpected user decryption result type. Expected 'bigint', got '${typeof clearUint64}' instead.`,
      );
    }
    return clearUint64;
  }

  async function deployTestInput() {
    const testInputContractFactory = await ethers.getContractFactory('TestInput');
    testInputContract = await testInputContractFactory.connect(signers.alice).deploy();
    await testInputContract.waitForDeployment();
    testInputContractAddress = await testInputContract.getAddress();
  }

  async function expectCoprocSigners(params: {
    numberOfCoprocessors: number;
    numberOfSigners: number;
    threshold: number;
    signers: string[];
  }) {
    const coprocSignersList = await inputVerifier.getCoprocessorSigners();
    expect(process.env.NUM_COPROCESSORS).to.equal(params.numberOfCoprocessors.toString());
    expect(coprocSignersList.length).to.eq(params.numberOfSigners);
    for (let i = 0; i < params.signers.length; ++i) {
      expect(coprocSignersList.includes(params.signers[i]));
    }
    expect(await inputVerifier.getThreshold()).to.equal(params.threshold);
  }

  it('original owner adds one signer, then adds one more signers, then adds one more, then removes one signer', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing
      const coprocessorAddressSigner0 = process.env['COPROCESSOR_SIGNER_ADDRESS_0']!;
      const coprocessorAddressSigner1 = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;
      const coprocessorAddressSigner2 = process.env['COPROCESSOR_SIGNER_ADDRESS_2']!;
      const coprocessorAddressSigner3 = process.env['COPROCESSOR_SIGNER_ADDRESS_3']!;

      // - 1 active coprocessor
      // - 1 registered coprocessors
      // - threshold 1
      await expectCoprocSigners({
        numberOfCoprocessors: 1,
        numberOfSigners: 1,
        threshold: 1,
        signers: [coprocessorAddressSigner0],
      });

      // Register a new coproc signer
      await addSigners({ list: [coprocessorAddressSigner1], threshold: 1 });

      // - 1 active coprocessor
      // - 2 registered coprocessors
      // - threshold 1
      await expectCoprocSigners({
        numberOfCoprocessors: 1,
        numberOfSigners: 2,
        threshold: 1,
        signers: [coprocessorAddressSigner0, coprocessorAddressSigner1],
      });

      // Deploy TestInput
      await deployTestInput();

      await testInputSetUint64(18446744073709550042n);
      let clearUint64 = await getClearUint64(aliceKeys);

      // in this case, one signature still suffices to pass the decrypt (threshold is still 1)
      expect(clearUint64).to.equal(18446744073709550042n);

      // Try add a coproc signer already registered
      await expect(addSigners({ list: [coprocessorAddressSigner1], threshold: 1 })).to.revertedWithCustomError(
        inputVerifier,
        'CoprocessorAlreadySigner',
      ); // cannot add duplicated signer
      expect((await inputVerifier.getCoprocessorSigners()).length).to.equal(2);

      // Add 2 new coproc signers (total 4)
      await addSigners({ list: [coprocessorAddressSigner2, coprocessorAddressSigner3], threshold: 1 });
      let tx = await inputVerifier.connect(deployer).setThreshold(2n);
      await tx.wait();

      // - 1 active coprocessor
      // - 4 registered coprocessors
      // - threshold 2
      await expectCoprocSigners({
        numberOfCoprocessors: 1,
        numberOfSigners: 4,
        threshold: 2,
        signers: [
          coprocessorAddressSigner0,
          coprocessorAddressSigner1,
          coprocessorAddressSigner2,
          coprocessorAddressSigner3,
        ],
      });

      // now we need at least 2 signatures (threshold is 2) but we only have 1
      await expect(testInputSetUint64(19n))
        .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
        .withArgs(1n);

      process.env.NUM_COPROCESSORS = '4';

      // - 4 active coprocessor
      // - 4 registered coprocessors
      // - threshold 2
      await expectCoprocSigners({
        numberOfCoprocessors: 4,
        numberOfSigners: 4,
        threshold: 2,
        signers: [
          coprocessorAddressSigner0,
          coprocessorAddressSigner1,
          coprocessorAddressSigner2,
          coprocessorAddressSigner3,
        ],
      });

      // even with more than 2 signatures decryption should still succeed
      await testInputSetUint64(1992n);
      clearUint64 = await getClearUint64(aliceKeys);
      expect(clearUint64).to.equal(1992n);

      process.env.NUM_COPROCESSORS = '3';

      // - 3 active coprocessor
      // - 4 registered coprocessors
      // - threshold 2
      await expectCoprocSigners({
        numberOfCoprocessors: 3,
        numberOfSigners: 4,
        threshold: 2,
        signers: [
          coprocessorAddressSigner0,
          coprocessorAddressSigner1,
          coprocessorAddressSigner2,
          coprocessorAddressSigner3,
        ],
      });

      // 3 signatures should still work
      await testInputSetUint64(873n);
      clearUint64 = await getClearUint64(aliceKeys);
      expect(clearUint64).to.equal(873n);

      const initial_coprocessor_signer_address_1 = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;
      process.env.NUM_COPROCESSORS = '2';
      // WARNING: this makes both addresses identical in env
      // Force having actually 1 single registered coproc signer
      process.env.COPROCESSOR_SIGNER_ADDRESS_1 = process.env.COPROCESSOR_SIGNER_ADDRESS_0;

      await expect(testInputSetUint64(999n)).to.revertedWithCustomError(inputVerifier, 'SignaturesVerificationFailed');

      // Put back the original addresses for future tests
      process.env.COPROCESSOR_SIGNER_ADDRESS_1 = initial_coprocessor_signer_address_1;

      // Remove last coproc signer
      process.env.NUM_COPROCESSORS = '1';
      await removeLastSigner({ threshold: 1 });

      await expectCoprocSigners({
        numberOfCoprocessors: 1,
        numberOfSigners: 3,
        threshold: 1,
        signers: [coprocessorAddressSigner0, coprocessorAddressSigner1, coprocessorAddressSigner2],
      });

      // after removing one of the 4 signers, one signature is enough for decryption
      await testInputSetUint64(1001n);
      clearUint64 = await getClearUint64(aliceKeys);
      expect(clearUint64).to.equal(1001n);
    }
  });

  it('input tests with several non-trivial inputs', async function () {
    if (process.env.HARDHAT_PARALLEL !== '1') {
      // to avoid messing up other tests if used on the real node, in parallel testing

      const coprocessorAddressSigner0 = process.env['COPROCESSOR_SIGNER_ADDRESS_0']!;
      const coprocessorAddressSigner1 = process.env['COPROCESSOR_SIGNER_ADDRESS_1']!;

      await deployTestInput();

      let tx = await inputVerifier
        .connect(deployer)
        .defineNewContext([coprocessorAddressSigner0, coprocessorAddressSigner1], 2);
      await tx.wait();

      // - 1 active coprocessor
      // - 2 registered coprocessors
      // - threshold 1
      await expectCoprocSigners({
        numberOfCoprocessors: 1,
        numberOfSigners: 2,
        threshold: 2,
        signers: [coprocessorAddressSigner0, coprocessorAddressSigner1],
      });

      // should revert because now we are below the threshold! (we receive only 1 signature but threshold is 2)
      await expect(testInputSetUint64(999n))
        .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
        .withArgs(1n);

      process.env.NUM_COPROCESSORS = '2';

      // - 2 active coprocessors
      // - 2 registered coprocessors
      // - threshold 2
      await expectCoprocSigners({
        numberOfCoprocessors: 2,
        numberOfSigners: 2,
        threshold: 2,
        signers: [coprocessorAddressSigner0, coprocessorAddressSigner1],
      });

      // in this case, 2 signatures suffice to pass the decrypt (threshold is 2)
      await testInputSetUint64(998n);
      const clearUint64 = await getClearUint64(aliceKeys);
      expect(clearUint64).to.equal(998n);

      process.env.NUM_COPROCESSORS = '1';
    }
  });

  it('cannot add/remove signers if not the owner', async function () {
    let coprocSignersList = await inputVerifier.getCoprocessorSigners();
    const randomAccount = signers.carol;
    coprocSignersList = [...coprocSignersList, randomAccount.address];
    await expect(
      inputVerifier.connect(randomAccount).defineNewContext(coprocSignersList, 2),
    ).to.be.revertedWithCustomError(inputVerifier, 'NotHostOwner');
  });
});
