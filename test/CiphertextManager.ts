import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { HDNodeWallet } from "ethers";
import hre from "hardhat";

import { CiphertextManager, HTTPZ } from "../typechain-types";
import { createAndFundRandomUser, createBytes32, createCtHandle, loadTestVariablesFixture } from "./utils";

describe("CiphertextManager", function () {
  const ctHandle = createCtHandle();
  const keyId = 0;
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Fake values
  const fakeCtHandle = createCtHandle();
  const fakeHostChainId = 123;

  let httpz: HTTPZ;
  let ciphertextManager: CiphertextManager;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let hostChainId: number;
  let fakeTxSender: HDNodeWallet;

  async function prepareFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);

    // Define the hostChainId
    hostChainId = fixtureData.chainIds[0];

    return fixtureData;
  }

  async function prepareViewTestFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextManager, coprocessorTxSenders } = fixtureData;

    // Setup the CiphertextManager contract state with a ciphertext used during tests
    for (let txSender of coprocessorTxSenders) {
      await ciphertextManager
        .connect(txSender)
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);
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
          .addCiphertextMaterial(ctHandle, keyId, fakeHostChainId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(httpz, "NetworkNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should add a ciphertext material", async function () {
      // When
      await ciphertextManager
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

      // This transaction should make the consensus to be reached and thus emit the expected event
      const result = ciphertextManager
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

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
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

      const events = await ciphertextManager.queryFilter(ciphertextManager.filters.AddCiphertextMaterial(ctHandle));

      // It should emit only the event once consensus is reached which means only the second transaction emits the event
      expect(events.length).to.equal(1);
    });

    it("Should revert because the transaction sender is not a Coprocessor", async function () {
      await expect(
        ciphertextManager
          .connect(fakeTxSender)
          .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest),
      )
        .revertedWithCustomError(httpz, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should revert because the coprocessor transaction sender has already added the ciphertext handle", async function () {
      // Add the ciphertext with the first coprocessor transaction sender
      ciphertextManager
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

      // Check that trying to add the same ciphertext with the same coprocessor transaction sender reverts
      await expect(
        ciphertextManager
          .connect(coprocessorTxSenders[0])
          .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest),
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
      await expect(ciphertextManager.getCiphertextMaterials([fakeCtHandle]))
        .revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(fakeCtHandle);
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
      await expect(ciphertextManager.getSnsCiphertextMaterials([fakeCtHandle]))
        .revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(fakeCtHandle);
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
      await expect(ciphertextManager.checkCiphertextMaterial(fakeCtHandle))
        .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(fakeCtHandle);
    });
  });
});
