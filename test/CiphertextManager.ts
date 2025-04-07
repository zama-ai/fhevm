import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { HDNodeWallet } from "ethers";

import { CiphertextManager, HTTPZ } from "../typechain-types";
import {
  createAndFundRandomUser,
  createBytes32,
  createCtHandleWithChainId,
  loadChainIds,
  loadTestVariablesFixture,
} from "./utils";

describe("CiphertextManager", function () {
  // Define the host chainId(s)
  const hostChainIds = loadChainIds();
  const hostChainId = hostChainIds[0];

  // Create a ctHandle with the host chain ID
  const ctHandle = createCtHandleWithChainId(hostChainId);

  // Define input values
  const keyId = 0;
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Define fake values
  const fakeHostChainId = 123;
  const ctHandleFakeChainId = createCtHandleWithChainId(fakeHostChainId);
  const notAddedCtHandle = createCtHandleWithChainId(hostChainId);

  let httpz: HTTPZ;
  let ciphertextManager: CiphertextManager;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let fakeTxSender: HDNodeWallet;

  async function prepareFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);

    return fixtureData;
  }

  async function prepareViewTestFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextManager, coprocessorTxSenders } = fixtureData;

    // Setup the CiphertextManager contract state with a ciphertext used during tests
    for (let txSender of coprocessorTxSenders) {
      await ciphertextManager
        .connect(txSender)
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);
    }
    return fixtureData;
  }

  beforeEach(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(prepareFixture);
    httpz = fixture.httpz;
    coprocessorTxSenders = fixture.coprocessorTxSenders;
    ciphertextManager = fixture.ciphertextManager;

    fakeTxSender = await createAndFundRandomUser();
  });

  describe("Add ciphertext material", async function () {
    it("Should revert because the hostChainId is not registered in the HTTPZ contract", async function () {
      // Check that adding a ciphertext material on a fake chain ID reverts
      await expect(
        ciphertextManager
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandleFakeChainId, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(httpz, "NetworkNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should add a ciphertext material", async function () {
      // When
      await ciphertextManager
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // This transaction should make the consensus to be reached and thus emit the expected event
      const result = ciphertextManager
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Then
      await expect(result)
        .to.emit(ciphertextManager, "AddCiphertextMaterial")
        .withArgs(ctHandle, ciphertextDigest, snsCiphertextDigest, [
          coprocessorTxSenders[0].address,
          coprocessorTxSenders[1].address,
        ]);

      // Then check that no other events get triggered
      await ciphertextManager
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      const events = await ciphertextManager.queryFilter(ciphertextManager.filters.AddCiphertextMaterial(ctHandle));

      // It should emit only the event once consensus is reached which means only the second transaction emits the event
      expect(events.length).to.equal(1);
    });

    it("Should revert because the transaction sender is not a Coprocessor", async function () {
      await expect(
        ciphertextManager
          .connect(fakeTxSender)
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(httpz, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because the coprocessor transaction sender has already added the ciphertext handle", async function () {
      // Add the ciphertext with the first coprocessor transaction sender
      ciphertextManager
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest);

      // Check that trying to add the same ciphertext with the same coprocessor transaction sender reverts
      await expect(
        ciphertextManager
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest),
      ).revertedWithCustomError(ciphertextManager, "CoprocessorTxSenderAlreadyAdded");
    });

    // TODO: Add test checking `isCurrentKeyId` once keys are generated through the Gateway
  });

  describe("Get ciphertext materials", async function () {
    beforeEach(async function () {
      await loadFixture(prepareViewTestFixture);
    });

    it("Should get regular ciphertext materials", async function () {
      // When
      const result = await ciphertextManager.getCiphertextMaterials([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([[ctHandle, keyId, ciphertextDigest, coprocessorTxSenders.map((s) => s.address)]]);
    });

    it("Should revert with CiphertextMaterialNotFound (regular)", async function () {
      await expect(ciphertextManager.getCiphertextMaterials([notAddedCtHandle]))
        .revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(notAddedCtHandle);
    });

    it("Should get SNS ciphertext materials", async function () {
      // When
      const result = await ciphertextManager.getSnsCiphertextMaterials([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([
        [ctHandle, keyId, snsCiphertextDigest, coprocessorTxSenders.map((s) => s.address)],
      ]);
    });

    it("Should revert with CiphertextMaterialNotFound (SNS)", async function () {
      await expect(ciphertextManager.getSnsCiphertextMaterials([notAddedCtHandle]))
        .revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(notAddedCtHandle);
    });
  });

  describe("Check ciphertext material", async function () {
    beforeEach(async function () {
      await loadFixture(prepareViewTestFixture);
    });

    it("Should not revert", async function () {
      await expect(ciphertextManager.checkCiphertextMaterial(ctHandle)).not.to.be.reverted;
    });

    it("Should revert as the ciphertext material does not exist", async function () {
      await expect(ciphertextManager.checkCiphertextMaterial(notAddedCtHandle))
        .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(notAddedCtHandle);
    });
  });
});
