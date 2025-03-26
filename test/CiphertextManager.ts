import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { HDNodeWallet } from "ethers";
import hre from "hardhat";

import { CiphertextManager, HTTPZ } from "../typechain-types";
import { createAndFundRandomUser, loadTestVariablesFixture } from "./utils";

describe("CiphertextManager", function () {
  const ctHandle = 2025;
  const keyId = 0;
  const ciphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
  const snsCiphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));

  // Fake values
  const fakeCtHandle = 11111;
  const fakeHostChainId = 123;

  let httpz: HTTPZ;
  let ciphertextManager: CiphertextManager;
  let coprocessorTxSenders: HardhatEthersSigner[];
  let hostChainId: number;
  let fakeTxSender: HDNodeWallet;

  async function prepareCiphertextManagerFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { ciphertextManager, coprocessorTxSenders } = fixtureData;

    // Define the hostChainId
    hostChainId = fixtureData.chainIds[0];

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
    const fixture = await loadFixture(prepareCiphertextManagerFixture);
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
      // Given
      const newCtHandle = "0x0123";

      // When
      await ciphertextManager
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(newCtHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

      // This transaction should make the consensus to be reached and thus emit the expected event
      const result = ciphertextManager
        .connect(coprocessorTxSenders[1])
        .addCiphertextMaterial(newCtHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

      // Then
      await expect(result)
        .to.emit(ciphertextManager, "AddCiphertextMaterial")
        .withArgs(newCtHandle, ciphertextDigest, snsCiphertextDigest, [
          coprocessorTxSenders[0].address,
          coprocessorTxSenders[1].address,
        ]);

      // Then check that no other events get triggered
      await ciphertextManager
        .connect(coprocessorTxSenders[2])
        .addCiphertextMaterial(newCtHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);
      const events = await ciphertextManager.queryFilter(ciphertextManager.filters.AddCiphertextMaterial(newCtHandle));

      // It should emit only the event once consensus is reached which means only the second transaction emits the event
      expect(events.length).to.equal(1);
    });

    it("Should revert because the transaction sender is not a Coprocessor", async function () {
      const result = ciphertextManager
        .connect(fakeTxSender)
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

      // Then
      await expect(result)
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(fakeTxSender.address, httpz.COPROCESSOR_TX_SENDER_ROLE());
    });

    it("Should revert because the coprocessor transaction sender has already added the ciphertext handle", async function () {
      // When
      const result = ciphertextManager
        .connect(coprocessorTxSenders[0])
        .addCiphertextMaterial(ctHandle, keyId, hostChainId, ciphertextDigest, snsCiphertextDigest);

      // Then
      await expect(result).revertedWithCustomError(ciphertextManager, "CoprocessorTxSenderAlreadyAdded");
    });

    // TODO: Add test checking `isCurrentKeyId` once keys are generated through the Gateway
  });

  describe("Get regular ciphertext materials", async function () {
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
  });

  describe("Get SNS ciphertext materials", async function () {
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
    it("Should not revert", async function () {
      await expect(ciphertextManager.checkCiphertextMaterial(ctHandle)).not.to.be.reverted;
    });

    it("Should revert as the ciphertext material does not exist", async function () {
      // When
      const result = ciphertextManager.checkCiphertextMaterial(fakeCtHandle);

      // Then
      await expect(result)
        .to.be.revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotFound")
        .withArgs(fakeCtHandle);
    });
  });
});
