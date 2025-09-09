import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { CiphertextCommits, CiphertextCommits__factory, GatewayConfig } from "../typechain-types";
import {
  createBytes32,
  createCtHandle,
  createCtHandles,
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

  // Define fake values
  const fakeHostChainId = 123;
  const ctHandleFakeChainId = createCtHandle(fakeHostChainId);
  const fakeTxSender = createRandomWallet();
  const fakeCiphertextDigest = createBytes32();

  let gatewayConfig: GatewayConfig;
  let ciphertextCommits: CiphertextCommits;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let owner: Wallet;
  let pauser: HardhatEthersSigner;

  async function prepareFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);

    return fixtureData;
  }

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
    const fixture = await loadFixture(prepareFixture);
    gatewayConfig = fixture.gatewayConfig;
    coprocessorTxSenders = fixture.coprocessorTxSenders;
    ciphertextCommits = fixture.ciphertextCommits;
    owner = fixture.owner;
    pauser = fixture.pauser;
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
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
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
        .withArgs(ctHandle, ciphertextDigest, snsCiphertextDigest, [
          coprocessorTxSenders[0].address,
          coprocessorTxSenders[1].address,
        ]);
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
        .withArgs(ctHandle, ciphertextDigest, snsCiphertextDigest, [
          coprocessorTxSenders[0].address,
          coprocessorTxSenders[2].address,
        ]);
    });

    it("Should get all valid coprocessor transaction senders from add ciphertext material consensus", async function () {
      // Trigger a valid add ciphertext material call using the first coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Check that the coprocessor transaction senders list is empty because consensus is not reached yet
      const addCiphertextMaterialConsensusTxSenders1 =
        await ciphertextCommits.getAddCiphertextMaterialConsensusTxSenders(ctHandle);
      expect(addCiphertextMaterialConsensusTxSenders1).to.deep.equal([]);

      // Trigger a valid add ciphertext material call using the second coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // 2 coprocessor transaction senders, at the moment the consensus is reached
      const addCiphertextMaterialConsensusTxSenders2 =
        await ciphertextCommits.getAddCiphertextMaterialConsensusTxSenders(ctHandle);
      expect(addCiphertextMaterialConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger a valid add ciphertext material call using the third coprocessor transaction sender
      await ciphertextCommits
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // 3 coprocessor transaction senders, after the consensus is reached
      const addCiphertextMaterialConsensusTxSenders3 =
        await ciphertextCommits.getAddCiphertextMaterialConsensusTxSenders(ctHandle);
      expect(addCiphertextMaterialConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(
        ciphertextCommits
          .connect(fakeTxSender)
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(gatewayConfig, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
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

    // TODO: Add test checking `checkCurrentKeyId` once keys are generated through the Gateway
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

      expect(result).to.be.deep.eq([
        [ctHandle, keyId, ciphertextDigest, usedCoprocessorTxSender.map((s) => s.address)],
      ]);
    });

    it("Should get late transaction sender after consensus (regular)", async function () {
      const resultTx1 = await ciphertextCommits.getCiphertextMaterials([ctHandle]);

      // The consensus has been reached with only 2 coprocessors
      expect(resultTx1).to.be.deep.eq([
        [ctHandle, keyId, ciphertextDigest, usedCoprocessorTxSender.map((s) => s.address)],
      ]);

      // Trigger a "late" call with valid inputs, after the consensus has been reached
      await ciphertextCommits
        .connect(unusedCoprocessorTxSender)
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Fetch the material once again
      const resultTx2 = await ciphertextCommits.getCiphertextMaterials([ctHandle]);

      // The list of coprocessor transaction sender addresses should now contain the late coprocessor,
      // at the end of the list
      const expectedTxSenderAddresses = [
        ...usedCoprocessorTxSender.map((s) => s.address),
        unusedCoprocessorTxSender.address,
      ];
      expect(resultTx2).to.be.deep.eq([[ctHandle, keyId, ciphertextDigest, expectedTxSenderAddresses]]);
    });

    it("Should revert with CiphertextMaterialNotFound (regular)", async function () {
      await expect(ciphertextCommits.getCiphertextMaterials([newCtHandle]))
        .revertedWithCustomError(ciphertextCommits, "CiphertextMaterialNotFound")
        .withArgs(newCtHandle);
    });

    it("Should get SNS ciphertext materials", async function () {
      const result = await ciphertextCommits.getSnsCiphertextMaterials([ctHandle]);

      expect(result).to.be.deep.eq([
        [ctHandle, keyId, snsCiphertextDigest, usedCoprocessorTxSender.map((s) => s.address)],
      ]);
    });

    it("Should get late transaction sender after consensus (SNS) ", async function () {
      const result = await ciphertextCommits.getSnsCiphertextMaterials([ctHandle]);

      // The consensus has been reached with only 2 coprocessors
      expect(result).to.be.deep.eq([
        [ctHandle, keyId, snsCiphertextDigest, usedCoprocessorTxSender.map((s) => s.address)],
      ]);

      // Trigger a "late" call with valid inputs, after the consensus has been reached
      await ciphertextCommits
        .connect(unusedCoprocessorTxSender)
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Fetch the material once again
      const resultTx2 = await ciphertextCommits.getSnsCiphertextMaterials([ctHandle]);

      // The list of coprocessor transaction sender addresses should now contain the late coprocessor,
      // at the end of the list
      const expectedTxSenderAddresses = [
        ...usedCoprocessorTxSender.map((s) => s.address),
        unusedCoprocessorTxSender.address,
      ];
      expect(resultTx2).to.be.deep.eq([[ctHandle, keyId, snsCiphertextDigest, expectedTxSenderAddresses]]);
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
    it("Should pause the contract with the pauser and unpause with the owner", async function () {
      // Check that the contract is not paused
      expect(await ciphertextCommits.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(ciphertextCommits.connect(pauser).pause()).to.emit(ciphertextCommits, "Paused").withArgs(pauser);
      expect(await ciphertextCommits.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(ciphertextCommits.connect(owner).unpause()).to.emit(ciphertextCommits, "Unpaused").withArgs(owner);
      expect(await ciphertextCommits.paused()).to.be.false;
    });

    it("Should revert on pause because sender is not the pauser", async function () {
      const fakePauser = createRandomWallet();

      await expect(ciphertextCommits.connect(fakePauser).pause())
        .to.be.revertedWithCustomError(ciphertextCommits, "NotPauserOrGatewayConfig")
        .withArgs(fakePauser.address);
    });

    it("Should revert on unpause because sender is not the owner", async function () {
      // Pause the contract with the pauser address
      await ciphertextCommits.connect(pauser).pause();

      const fakeOwner = createRandomWallet();

      await expect(ciphertextCommits.connect(fakeOwner).unpause())
        .to.be.revertedWithCustomError(ciphertextCommits, "NotOwnerOrGatewayConfig")
        .withArgs(fakeOwner.address);
    });
  });
});
