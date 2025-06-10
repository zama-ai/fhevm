import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture, time } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { HDNodeWallet, Wallet } from "ethers";
import hre from "hardhat";

import {
  CiphertextCommits,
  CiphertextCommits__factory,
  CoprocessorContexts,
  GatewayConfig,
  InputVerification,
} from "../typechain-types";
import {
  CoprocessorContextTimePeriodsStruct,
  CoprocessorV2Struct,
} from "../typechain-types/contracts/interfaces/ICoprocessorContexts";
import {
  ContextStatus,
  addNewCoprocessorContext,
  createBytes32,
  createCtHandle,
  createCtHandles,
  createRandomWallet,
  loadHostChainIds,
  loadTestVariablesFixture,
  refreshCoprocessorContextAfterTimePeriod,
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

  // Define the first coprocessor context ID
  const contextId = 1;

  // Define fake values
  const fakeHostChainId = 123;
  const ctHandleFakeChainId = createCtHandle(fakeHostChainId);
  const fakeTxSender = createRandomWallet();
  const fakeCiphertextDigest = createBytes32();

  let gatewayConfig: GatewayConfig;
  let coprocessorContexts: CoprocessorContexts;
  let inputVerification: InputVerification;
  let ciphertextCommits: CiphertextCommits;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let coprocessorSigners: HardhatEthersSigner[];
  let owner: Wallet;
  let pauser: Wallet;
  let contractChainId: number;
  let coprocessors: CoprocessorV2Struct[];

  async function prepareViewTestFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextCommits, coprocessorTxSenders } = fixtureData;

    const unusedCoprocessorTxSender = coprocessorTxSenders[0];
    const usedCoprocessorTxSender = coprocessorTxSenders.slice(1);

    // Add the ciphertext material using all but the first coprocessor, which is enough to reach
    // consensus
    for (let txSender of usedCoprocessorTxSender) {
      await ciphertextCommits
        .connect(txSender)
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);
    }
    return { ...fixtureData, unusedCoprocessorTxSender, usedCoprocessorTxSender };
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
    coprocessors = fixture.coprocessors;
  });

  describe("Deployment", function () {
    let ciphertextCommitsFactory: CiphertextCommits__factory;

    beforeEach(async function () {
      // Get the CiphertextCommits contract factory
      ciphertextCommitsFactory = await hre.ethers.getContractFactory("CiphertextCommits", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(ciphertextCommits, ciphertextCommitsFactory, {
          call: { fn: "initializeFromEmptyProxy" },
        }),
      ).to.be.revertedWithCustomError(ciphertextCommits, "NotInitializingFromEmptyProxy");
    });
  });

  describe("Add ciphertext material", async function () {
    it("Should revert because the chain ID does not correspond to a registered host chain", async function () {
      // Check that adding a ciphertext material on a fake chain ID reverts
      await expect(
        ciphertextCommits
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandleFakeChainId, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(ciphertextCommits, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should add a ciphertext material with 2 valid calls", async function () {
      // Trigger 2 valid add ciphertext material calls
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const resultTx2 = ciphertextCommits
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Consensus should be reached at the second call
      // Check 2nd call event: it should only contain the 2 coprocessor transaction sender addresses
      await expect(resultTx2)
        .to.emit(ciphertextCommits, "AddCiphertextMaterial")
        .withArgs(ctHandle, ciphertextDigest, snsCiphertextDigest, contextId);
    });

    it("Should add a ciphertext material with 2 valid calls and ignore the other valid one", async function () {
      // Trigger 3 valid add ciphertext material calls
      const resultTx1 = await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      await ciphertextCommits
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const resultTx3 = await ciphertextCommits
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Check that the 1st and 3rd calls do not emit an event:
      // - 1st call is ignored because consensus is not reached yet
      // - 3rd call is ignored (not reverted) even though it is late
      await expect(resultTx1).to.not.emit(ciphertextCommits, "AddCiphertextMaterial");
      await expect(resultTx3).to.not.emit(ciphertextCommits, "AddCiphertextMaterial");
    });

    it("Should add a ciphertext material with 2 valid and 1 malicious calls ", async function () {
      // Trigger 1 valid add ciphertext material call
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Trigger 1 malicious add ciphertext material call
      // By "malicious", here we mean a call that would try to provide different infos (keyId, digests)
      // with respect to handle with on-going consensus
      const fakeResultTx2 = await ciphertextCommits
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, fakeCiphertextDigest, snsCiphertextDigest);

      // Make sure that the consensus has not been reached yet
      await expect(fakeResultTx2).to.not.emit(ciphertextCommits, "AddCiphertextMaterial");

      // Trigger a 2nd valid add ciphertext material call: consensus should then be reached for this
      // handle and the associated infos
      const resultTx3 = ciphertextCommits
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Check 2nd call event: it should only contain 2 coprocessor transaction sender addresses, the
      // 1st and 3rd one
      await expect(resultTx3)
        .to.emit(ciphertextCommits, "AddCiphertextMaterial")
        .withArgs(ctHandle, ciphertextDigest, snsCiphertextDigest, contextId);
    });

    it("Should get all valid coprocessor transaction senders from add ciphertext material consensus", async function () {
      // Trigger a valid add ciphertext material call using the first coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Check that the coprocessor transaction senders list is empty because consensus is not reached yet
      const addCiphertextMaterialConsensusTxSenders1 = await ciphertextCommits.getConsensusTxSenders(ctHandle);
      expect(addCiphertextMaterialConsensusTxSenders1).to.deep.equal([]);

      // Trigger a valid add ciphertext material call using the second coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // 2 coprocessor transaction senders, at the moment the consensus is reached
      const addCiphertextMaterialConsensusTxSenders2 = await ciphertextCommits.getConsensusTxSenders(ctHandle);
      expect(addCiphertextMaterialConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger a valid add ciphertext material call using the third coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // 3 coprocessor transaction senders, after the consensus is reached
      const addCiphertextMaterialConsensusTxSenders3 = await ciphertextCommits.getConsensusTxSenders(ctHandle);
      expect(addCiphertextMaterialConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should get all valid coprocessor storage URLs from add ciphertext material at and after consensus", async function () {
      // Trigger a valid add ciphertext material call using the first and second coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);
      await ciphertextCommits
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const expectedConsensusStorageUrls1 = coprocessors.slice(0, 2).map((coprocessor) => coprocessor.storageUrl);

      // Check that we get the expected list of storage URLs at consensus
      const consensusSStorageUrls1 = await ciphertextCommits.getConsensusStorageUrls([ctHandle]);
      expect(consensusSStorageUrls1).to.deep.equal([expectedConsensusStorageUrls1]);

      // Trigger a valid add ciphertext material call using the third coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const expectedConsensusStorageUrls2 = coprocessors.slice(0, 3).map((coprocessor) => coprocessor.storageUrl);

      // Check that we get the expected list of storage URLs after consensus
      const consensusStorageUrls2 = await ciphertextCommits.getConsensusStorageUrls([ctHandle]);
      expect(consensusStorageUrls2).to.deep.equal([expectedConsensusStorageUrls2]);
    });

    it("Should revert because add ciphertext material consensus has not been reached", async function () {
      // Trigger a valid add ciphertext material call using the first coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      await expect(ciphertextCommits.getConsensusStorageUrls([ctHandle]))
        .revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(ctHandle);
    });

    it("Should revert because the transaction sender is not a coprocessor from the active context", async function () {
      await expect(
        ciphertextCommits
          .connect(fakeTxSender)
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(ciphertextCommits, "NotCoprocessorTxSenderFromContext")
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

    describe("Context changes", async function () {
      let timePeriods: CoprocessorContextTimePeriodsStruct;
      let newCoprocessorTxSenders: HDNodeWallet[];

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Add the ciphertext material with the first coprocessor transaction sender. This should
        // register the request under the first active context (ID 1)
        await ciphertextCommits
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

        // Add a new coprocessor context using a bigger set of coprocessors with different tx sender
        // and signer addresses
        const newCoprocessorContext = await addNewCoprocessorContext(10, coprocessorContexts, owner, true);
        timePeriods = newCoprocessorContext.timePeriods;
        newCoprocessorTxSenders = newCoprocessorContext.coprocessorTxSenders;
      });

      it("Should activate the new context and suspend the old one", async function () {
        // Increase the block timestamp to reach the end of the pre-activation period
        await time.increase(timePeriods.preActivationTimePeriod);

        // Add a new ciphertext material with the first new coprocessor transaction sender
        await ciphertextCommits
          .connect(newCoprocessorTxSenders[0])
          .addCiphertextMaterial(newCtHandle, keyId, ciphertextDigest, snsCiphertextDigest);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure the new context has been activated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Active);
      });

      it("Should deactivate the suspended context", async function () {
        // Increase the block timestamp to reach the end of the pre-activation period
        await time.increase(timePeriods.preActivationTimePeriod);

        // Add a new ciphertext material with the first new coprocessor transaction sender
        await ciphertextCommits
          .connect(newCoprocessorTxSenders[0])
          .addCiphertextMaterial(newCtHandle, keyId, ciphertextDigest, snsCiphertextDigest);

        // Increase the block timestamp to reach the end of the suspended period
        await time.increase(timePeriods.suspendedTimePeriod);

        // Add a new ciphertext material with the second new coprocessor transaction sender
        await ciphertextCommits
          .connect(newCoprocessorTxSenders[1])
          .addCiphertextMaterial(newCtHandle, keyId, ciphertextDigest, snsCiphertextDigest);

        // Make sure the old context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Deactivated);
      });

      it("Should add a ciphertext material with suspended context", async function () {
        // The second transaction should reach consensus and thus emit the expected event
        // This is because the consensus is reached amongst the suspended context (3 coprocessors)
        // and not the new one (10 coprocessors)
        const result = await ciphertextCommits
          .connect(coprocessorTxSenders[1])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

        await expect(result)
          .to.emit(ciphertextCommits, "AddCiphertextMaterial")
          .withArgs(ctHandle, ciphertextDigest, snsCiphertextDigest, contextId);
      });

      it("Should revert because the context is no longer valid", async function () {
        // Wait for the pre activation period to pass
        await refreshCoprocessorContextAfterTimePeriod(timePeriods.preActivationTimePeriod, coprocessorContexts);

        // Wait for the suspended period to pass
        await refreshCoprocessorContextAfterTimePeriod(timePeriods.suspendedTimePeriod, coprocessorContexts);

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
        await refreshCoprocessorContextAfterTimePeriod(timePeriods.preActivationTimePeriod, coprocessorContexts);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure that a new ciphertext material can't be added by a coprocessor from the suspended context
        await expect(
          ciphertextCommits
            .connect(coprocessorTxSenders[0])
            .addCiphertextMaterial(newCtHandle, keyId, ciphertextDigest, snsCiphertextDigest),
        )
          .revertedWithCustomError(ciphertextCommits, "NotCoprocessorTxSenderFromContext")
          .withArgs(newContextId, coprocessorTxSenders[0].address);
      });
    });
  });

  describe("Get ciphertext materials", async function () {
    let unusedCoprocessorTxSender: HardhatEthersSigner;
    let usedCoprocessorTxSender: HardhatEthersSigner[];

    beforeEach(async function () {
      const fixtureData = await loadFixture(prepareViewTestFixture);
      unusedCoprocessorTxSender = fixtureData.unusedCoprocessorTxSender;
      usedCoprocessorTxSender = fixtureData.usedCoprocessorTxSender;
    });

    it("Should get regular ciphertext materials", async function () {
      const result = await ciphertextCommits.getCiphertextMaterials([ctHandle]);

      expect(result).to.be.deep.eq([[ctHandle, keyId, ciphertextDigest]]);
    });

    it("Should get late transaction sender after consensus (regular)", async function () {
      const resultTx1 = await ciphertextCommits.getCiphertextMaterials([ctHandle]);

      // The consensus has been reached with only 2 coprocessors
      expect(resultTx1).to.be.deep.eq([[ctHandle, keyId, ciphertextDigest]]);

      // Trigger a "late" call with valid inputs, after the consensus has been reached
      await ciphertextCommits
        .connect(unusedCoprocessorTxSender)
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Fetch the material once again
      const resultTx2 = await ciphertextCommits.getCiphertextMaterials([ctHandle]);

      expect(resultTx2).to.be.deep.eq([[ctHandle, keyId, ciphertextDigest]]);
    });

    it("Should revert with CiphertextMaterialNotFound (regular)", async function () {
      await expect(ciphertextCommits.getCiphertextMaterials([newCtHandle]))
        .revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandle);
    });

    it("Should get SNS ciphertext materials", async function () {
      const result = await ciphertextCommits.getSnsCiphertextMaterials([ctHandle]);

      expect(result).to.be.deep.eq([[ctHandle, keyId, snsCiphertextDigest]]);
    });

    it("Should get late transaction sender after consensus (SNS) ", async function () {
      const result = await ciphertextCommits.getSnsCiphertextMaterials([ctHandle]);

      // The consensus has been reached with only 2 coprocessors
      expect(result).to.be.deep.eq([[ctHandle, keyId, snsCiphertextDigest]]);

      // Trigger a "late" call with valid inputs, after the consensus has been reached
      await ciphertextCommits
        .connect(unusedCoprocessorTxSender)
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Fetch the material once again
      const resultTx2 = await ciphertextCommits.getSnsCiphertextMaterials([ctHandle]);

      expect(resultTx2).to.be.deep.eq([[ctHandle, keyId, snsCiphertextDigest]]);
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

    it("Should be true as the ciphertext material have been added", async function () {
      expect(await ciphertextCommits.isCiphertextMaterialAdded(ctHandle)).to.be.true;
    });

    it("Should be false as the ciphertext material has not been added", async function () {
      expect(await ciphertextCommits.isCiphertextMaterialAdded(newCtHandle)).to.be.false;
    });
  });
});
