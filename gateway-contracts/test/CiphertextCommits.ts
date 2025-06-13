import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture, mine } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";

import { CiphertextCommits, CoprocessorContexts, GatewayConfig, InputVerification } from "../typechain-types";
import { CoprocessorContextBlockPeriodsStruct } from "../typechain-types/contracts/interfaces/ICoprocessorContexts";
import {
  ContextStatus,
  createBytes32,
  createCoprocessors,
  createCtHandle,
  createCtHandles,
  createRandomAddress,
  createRandomWallet,
  loadHostChainIds,
  loadTestVariablesFixture,
} from "./utils";

describe("CiphertextCommits", function () {
  // Define the host chains' chain IDs
  const hostChainIds = loadHostChainIds();
  const hostChainId = hostChainIds[0];

  // Create a ctHandle with the host chain ID (it will be added by default)
  const ctHandle = createCtHandle(hostChainId);

  // Define new valid ctHandles (they won't be added by default)
  const newCtHandles = createCtHandles(3, hostChainId);
  const newCtHandle = newCtHandles[0];

  // Define input values
  const keyId = 0;
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Define the first context ID
  const contextId = 1;

  // Define fake values
  const fakeHostChainId = 123;
  const ctHandleFakeChainId = createCtHandle(fakeHostChainId);
  const fakeTxSender = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let coprocessorContexts: CoprocessorContexts;
  let inputVerification: InputVerification;
  let ciphertextCommits: CiphertextCommits;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let coprocessorSigners: HardhatEthersSigner[];
  let owner: Wallet;
  let pauser: HardhatEthersSigner;
  let contractChainId: number;

  async function prepareViewTestFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextCommits, coprocessorTxSenders } = fixtureData;

    // Setup the CiphertextCommits contract state with a ciphertext used during tests
    for (let txSender of coprocessorTxSenders) {
      await ciphertextCommits
        .connect(txSender)
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);
    }
    return fixtureData;
  }

  beforeEach(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixture.gatewayConfig;
    coprocessorContexts = fixture.coprocessorContexts;
    inputVerification = fixture.inputVerification;
    coprocessorTxSenders = fixture.coprocessorTxSenders;
    coprocessorSigners = fixture.coprocessorSigners;
    ciphertextCommits = fixture.ciphertextCommits;
    owner = fixture.owner;
    pauser = fixture.pauser;
    contractChainId = fixture.chainIds[0];
  });

  describe("Add ciphertext material", async function () {
    it("Should revert because the chain ID does not correspond to a registered host chain", async function () {
      // Check that adding a ciphertext material on a fake chain ID reverts
      await expect(
        ciphertextCommits
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandleFakeChainId, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should add a ciphertext material", async function () {
      // Add the ciphertext material with the first coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // The second transaction should reach consensus and thus emit the expected event
      const result = ciphertextCommits
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      await expect(result)
        .to.emit(ciphertextCommits, "AddCiphertextMaterial")
        .withArgs(ctHandle, contextId, ciphertextDigest, snsCiphertextDigest, [
          coprocessorTxSenders[0].address,
          coprocessorTxSenders[1].address,
        ]);

      // Then check that no other events get triggered
      await ciphertextCommits
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const events = await ciphertextCommits.queryFilter(ciphertextCommits.filters.AddCiphertextMaterial(ctHandle));

      // It should emit only the event once consensus is reached which means only the second transaction emits the event
      expect(events.length).to.equal(1);
    });

    it("Should revert because the transaction sender is not a coprocessor from the active context", async function () {
      await expect(
        ciphertextCommits
          .connect(fakeTxSender)
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
        .withArgs(contextId, fakeTxSender.address);
    });

    it("Should revert because the coprocessor transaction sender has already added the ciphertext handle", async function () {
      // Add the ciphertext with the first coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Check that trying to add the same ciphertext with the same coprocessor transaction sender reverts
      await expect(
        ciphertextCommits
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(ciphertextCommits, "CoprocessorAlreadyAdded")
        .withArgs(ctHandle, coprocessorTxSenders[0]);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await ciphertextCommits.connect(owner).pause();

      // Try calling paused add ciphertext material
      await expect(
        ciphertextCommits
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
      ).to.be.revertedWithCustomError(ciphertextCommits, "EnforcedPause");
    });

    // TODO: Add test checking `checkCurrentKeyId` once keys are generated through the Gateway

    describe("Context changes", async function () {
      // Define infos for a new context
      const newContextId = 2;
      const featureSet = 2030;
      const newCoprocessorTxSender = createRandomWallet();
      const newCoprocessorSigner = createRandomWallet();

      // Define block periods
      const coprocessorsPreActivationBlockPeriod = 100;
      const coprocessorsSuspendedBlockPeriod = 100;
      const blockPeriods: CoprocessorContextBlockPeriodsStruct = {
        preActivationBlockPeriod: coprocessorsPreActivationBlockPeriod,
        suspendedBlockPeriod: coprocessorsSuspendedBlockPeriod,
      };

      async function waitBlockPeriod(blockPeriod: number, contractChainId: number) {
        // Mine the specified number of blocks
        await mine(blockPeriod);

        // Define input values
        const contractAddress = createRandomAddress();
        const userAddress = createRandomAddress();
        const ciphertextWithZKProof = createBytes32();

        // Send a proof request to trigger the status refresh of contexts
        await inputVerification.verifyProofRequest(
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithZKProof,
        );
      }

      beforeEach(async function () {
        // Add the ciphertext material with the first coprocessor transaction sender. This should
        // register the request under the first active context (ID 1)
        await ciphertextCommits
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

        // Create a small set of coprocessors with different tx sender and signer addresses
        const coprocessors = createCoprocessors(1, [newCoprocessorTxSender], [newCoprocessorSigner]);

        // Add a new coprocessor context to suspend the first one. The old context will be deactivated
        // after `coprocessorsSuspensionBlockPeriod` blocks
        await coprocessorContexts.connect(owner).addCoprocessorContext(featureSet, blockPeriods, coprocessors);
      });

      it("Should add a ciphertext material with suspended context", async function () {
        // The second transaction should reach consensus and thus emit the expected event
        // This is because the consensus is reached amongst the suspended contexts (3 coprocessors)
        // and not the new one (1 coprocessor)
        const result = ciphertextCommits
          .connect(coprocessorTxSenders[1])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

        await expect(result)
          .to.emit(ciphertextCommits, "AddCiphertextMaterial")
          .withArgs(ctHandle, contextId, ciphertextDigest, snsCiphertextDigest, [
            coprocessorTxSenders[0].address,
            coprocessorTxSenders[1].address,
          ]);
      });

      it("Should revert because the context is no longer valid", async function () {
        // Wait for the pre activation period to pass
        await waitBlockPeriod(coprocessorsPreActivationBlockPeriod, contractChainId);

        // Wait for the suspended period to pass
        await waitBlockPeriod(coprocessorsSuspendedBlockPeriod, contractChainId);

        // Check that adding a ciphertext material that has already been registered under an active context
        // reverts because this context is no longer valid
        await expect(
          ciphertextCommits
            .connect(coprocessorTxSenders[1])
            .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
        )
          .revertedWithCustomError(ciphertextCommits, "InvalidCoprocessorContextAddCiphertext")
          .withArgs(ctHandle, contextId, ContextStatus.Deactivated);
      });

      it("Should revert because the transaction sender is a coprocessor from the suspended context", async function () {
        // Wait for the pre activation period to pass
        await waitBlockPeriod(coprocessorsPreActivationBlockPeriod, contractChainId);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure that a new ciphertext material can't be added by a coprocessor from the suspended context
        await expect(
          ciphertextCommits
            .connect(coprocessorTxSenders[0])
            .addCiphertextMaterial(newCtHandle, keyId, ciphertextDigest, snsCiphertextDigest),
        )
          .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
          .withArgs(newContextId, coprocessorTxSenders[0].address);
      });
    });
  });

  describe("Get ciphertext materials", async function () {
    beforeEach(async function () {
      await loadFixture(prepareViewTestFixture);
    });

    it("Should get regular ciphertext materials", async function () {
      // When
      const result = await ciphertextCommits.getCiphertextMaterials([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([[ctHandle, keyId, ciphertextDigest, coprocessorTxSenders.map((s) => s.address)]]);
    });

    it("Should revert with CiphertextMaterialNotFound (regular)", async function () {
      await expect(ciphertextCommits.getCiphertextMaterials([newCtHandle]))
        .revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandle);
    });

    it("Should get SNS ciphertext materials", async function () {
      // When
      const result = await ciphertextCommits.getSnsCiphertextMaterials([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([
        [ctHandle, keyId, snsCiphertextDigest, coprocessorTxSenders.map((s) => s.address)],
      ]);
    });

    it("Should revert with CiphertextMaterialNotFound (SNS)", async function () {
      await expect(ciphertextCommits.getSnsCiphertextMaterials([newCtHandle]))
        .revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandle);
    });
  });

  describe("Check ciphertext material", async function () {
    beforeEach(async function () {
      await loadFixture(prepareViewTestFixture);
    });

    it("Should not revert as the ciphertext material have been added", async function () {
      await expect(ciphertextCommits.checkCiphertextMaterial(ctHandle)).not.to.be.reverted;
    });

    it("Should revert as the ciphertext material has not been added", async function () {
      await expect(ciphertextCommits.checkCiphertextMaterial(newCtHandle))
        .to.be.revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandle);
    });
  });

  describe("Pause", async function () {
    it("Should pause and unpause contract with owner address", async function () {
      // Check that the contract is not paused
      expect(await ciphertextCommits.paused()).to.be.false;

      // Pause the contract with the owner address
      await expect(ciphertextCommits.connect(owner).pause()).to.emit(ciphertextCommits, "Paused").withArgs(owner);
      expect(await ciphertextCommits.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(ciphertextCommits.connect(owner).unpause()).to.emit(ciphertextCommits, "Unpaused").withArgs(owner);
      expect(await ciphertextCommits.paused()).to.be.false;
    });

    it("Should pause contract with pauser address", async function () {
      // Check that the contract is not paused
      expect(await ciphertextCommits.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(ciphertextCommits.connect(pauser).pause()).to.emit(ciphertextCommits, "Paused").withArgs(pauser);
      expect(await ciphertextCommits.paused()).to.be.true;
    });

    it("Should revert on pause because sender is not owner or pauser address", async function () {
      const notOwnerOrPauser = createRandomWallet();

      await expect(ciphertextCommits.connect(notOwnerOrPauser).pause())
        .to.be.revertedWithCustomError(ciphertextCommits, "NotOwnerOrPauser")
        .withArgs(notOwnerOrPauser.address);
    });
  });
});
