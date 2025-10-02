import { loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers } from 'hardhat';

import { InputVerifier, TestInput } from '../../types';
import { awaitAllDecryptionResults, initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { Signers, getSigners, initSigners } from '../signers';
import { FhevmInstances } from '../types';

describe('InputVerifier', function () {
  let deployer: Wallet;
  let signers: Signers;
  let instances: FhevmInstances;
  let inputVerifier: InputVerifier;
  let testInput: TestInput;

  async function deployInputVerifierFixture() {
    const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);

    // Initialize signers and FHEVM instances.
    await initSigners(2);
    const signers = await getSigners();
    const instances = await createInstances(signers);

    // Attach to the existing InputVerifier contract.
    const inputVerifierFactory = await ethers.getContractFactory('InputVerifier');
    const origIVAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).INPUT_VERIFIER_CONTRACT_ADDRESS;
    const inputVerifier = inputVerifierFactory.attach(origIVAdd) as InputVerifier;

    // Deploy the TestInput contract.
    const testInputFactory = await ethers.getContractFactory('TestInput');
    const testInput = await testInputFactory.connect(signers.alice).deploy();
    await testInput.waitForDeployment();

    return {
      deployer,
      signers,
      instances,
      inputVerifierFactory,
      inputVerifier,
      testInput,
    };
  }

  beforeEach(async function () {
    const fixtureData = await loadFixture(deployInputVerifierFixture);
    deployer = fixtureData.deployer;
    signers = fixtureData.signers;
    instances = fixtureData.instances;
    inputVerifier = fixtureData.inputVerifier;
    testInput = fixtureData.testInput;

    await initDecryptionOracle();
  });

  describe('Coprocessor context', function () {
    it('Should revert because the sender is not the host owner', async function () {
      const fakeOwner = signers.alice;
      await expect(inputVerifier.connect(fakeOwner).addNewContextAndSuspendOldOne(2, []))
        .to.be.revertedWithCustomError(inputVerifier, 'NotHostOwner')
        .withArgs(fakeOwner);
    });

    it('Should revert because the context ID is null', async function () {
      const nullContextId = 0;
      await expect(
        inputVerifier.connect(deployer).addNewContextAndSuspendOldOne(nullContextId, []),
      ).to.be.revertedWithCustomError(inputVerifier, 'InvalidNullContextId');
    });

    it('Should revert because the context signers is empty', async function () {
      const contextId = 2;
      await expect(inputVerifier.connect(deployer).addNewContextAndSuspendOldOne(contextId, []))
        .to.be.revertedWithCustomError(inputVerifier, 'EmptyCoprocessorSignerAddresses')
        .withArgs(contextId);
    });

    it('Should revert because the context ID is already used', async function () {
      const alreadyUsedContextId = 1;
      const newContextSigners = [signers.alice.address, signers.bob.address];
      await expect(
        inputVerifier.connect(deployer).addNewContextAndSuspendOldOne(alreadyUsedContextId, newContextSigners),
      )
        .to.be.revertedWithCustomError(inputVerifier, 'ContextAlreadyUsed')
        .withArgs(alreadyUsedContextId);
    });

    it('Should add the new coprocessor context', async function () {
      const previousContextId = 1;
      const newContextId = 2;
      const newContextSigners = [signers.alice.address, signers.bob.address];
      await expect(inputVerifier.connect(deployer).addNewContextAndSuspendOldOne(newContextId, newContextSigners)).to.be
        .fulfilled;

      // New context signers should contain the new signers.
      const contextSigners = await inputVerifier.getCoprocessorSigners(newContextId);
      expect(contextSigners.length).to.equal(2);
      expect(contextSigners[0]).to.equal(signers.alice.address);
      expect(contextSigners[1]).to.equal(signers.bob.address);

      // Previous context should be marked as suspended.
      const isSuspended = await inputVerifier.isCoprocessorContextActiveOrSuspended(previousContextId);
      expect(isSuspended).to.equal(true);

      // Threshold should be half + 1.
      const threshold = await inputVerifier.getThreshold(newContextId);
      expect(threshold).to.equal(newContextSigners.length / 2 + 1);

      // The address should be a signer for the context ID.
      expect(await inputVerifier.isSigner(newContextId, signers.alice.address)).to.equal(true);
    });

    it('Should deactivate the suspended coprocessor context', async function () {
      const previousContextId = 1;
      const newContextId = 2;
      const newContextSigners = [signers.alice.address, signers.bob.address];
      await inputVerifier.connect(deployer).addNewContextAndSuspendOldOne(newContextId, newContextSigners);

      // Previous context should be marked as suspended.
      expect(await inputVerifier.isCoprocessorContextActiveOrSuspended(previousContextId)).to.be.equal(true);

      await expect(inputVerifier.connect(deployer).removeSuspendedCoprocessorContext()).to.be.fulfilled;

      // Previous context should be marked as deactivated.
      await expect(inputVerifier.isCoprocessorContextActiveOrSuspended(previousContextId))
        .to.revertedWithCustomError(inputVerifier, 'InvalidContextId')
        .withArgs(previousContextId);
    });
  });

  describe('Non-trivial inputs', function () {
    it('Should handle uint64 non-trivial input correctly', async function () {
      // To avoid messing up other tests if used on the real node, in parallel testing.
      if (process.env.HARDHAT_PARALLEL !== '1') {
        const testInputAddress = await testInput.getAddress();
        const inputAlice = instances.alice.createEncryptedInput(testInputAddress, signers.alice.address);
        inputAlice.add64(18446744073709550042n);

        // Value should not be decrypted yet.
        expect(await testInput.yUint64()).to.equal(0n);

        const encryptedAmount = await inputAlice.encrypt();
        await testInput.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof);
        await awaitAllDecryptionResults();

        // Value should be decrypted now.
        expect(await testInput.yUint64()).to.equal(18446744073709550042n);
      }
    });

    it('Should handle mixed non-trivial inputs correctly', async function () {
      // To avoid messing up other tests if used on the real node, in parallel testing.
      if (process.env.HARDHAT_PARALLEL !== '1') {
        const testInputAddress = await testInput.getAddress();
        const inputAlice = instances.alice.createEncryptedInput(testInputAddress, signers.alice.address);

        inputAlice.addBool(true);
        inputAlice.add8(42);
        inputAlice.addAddress('0x1E69D5aa8750Ff56c556C164fE6feaE71BBA9a09');

        // Values should not be decrypted yet.
        expect(await testInput.yBool()).to.equal(false);
        expect(await testInput.yUint8()).to.equal(0);
        expect(await testInput.yAddress()).to.equal('0x0000000000000000000000000000000000000000');

        const encryptedAmount = await inputAlice.encrypt();
        await testInput.requestMixedNonTrivial(
          encryptedAmount.handles[0],
          encryptedAmount.handles[1],
          encryptedAmount.handles[2],
          encryptedAmount.inputProof,
        );
        await awaitAllDecryptionResults();

        // Values should be decrypted now.
        expect(await testInput.yBool()).to.equal(true);
        expect(await testInput.yUint8()).to.equal(42);
        expect(await testInput.yAddress()).to.equal('0x1E69D5aa8750Ff56c556C164fE6feaE71BBA9a09');
      }
    });

    it('Should revert if not enough signatures are provided', async function () {
      // To avoid messing up other tests if used on the real node, in parallel testing.
      if (process.env.HARDHAT_PARALLEL !== '1') {
        // Add new context with 2 signers, so threshold becomes 2.
        const newContextId = 2;
        const newContextSigners = [signers.alice.address, signers.bob.address];
        await inputVerifier.connect(deployer).addNewContextAndSuspendOldOne(newContextId, newContextSigners);

        const testInputAddress = await testInput.getAddress();

        // Prepare uint64 non-trivial input.
        const inputAlice = instances.alice.createEncryptedInput(testInputAddress, signers.alice.address);
        inputAlice.add64(18446744073709550042n);
        const aliceEncryptedAmount = await inputAlice.encrypt(newContextId);

        // Should revert due to not enough signatures for uint64 non-trivial input.
        await expect(
          testInput.requestUint64NonTrivial(aliceEncryptedAmount.handles[0], aliceEncryptedAmount.inputProof),
        )
          .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
          .withArgs(1n);

        // Prepare mixed non-trivial inputs.
        const inputBob = instances.alice.createEncryptedInput(testInputAddress, signers.bob.address);
        inputBob.addBool(true);
        inputBob.add8(42);
        inputBob.addAddress('0x1E69D5aa8750Ff56c556C164fE6feaE71BBA9a09');
        const bobEncryptedAmount = await inputBob.encrypt(newContextId);

        // Should revert due to not enough signatures for mixed non-trivial input.
        await expect(
          testInput.requestMixedNonTrivial(
            bobEncryptedAmount.handles[0],
            bobEncryptedAmount.handles[1],
            bobEncryptedAmount.handles[2],
            bobEncryptedAmount.inputProof,
          ),
        )
          .to.revertedWithCustomError(inputVerifier, 'SignatureThresholdNotReached')
          .withArgs(1n);
      }
    });
  });
});
