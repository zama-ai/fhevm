import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { CiphertextStorage, HTTPZ } from "../typechain-types";
import { deployKeyManagerFixture } from "./utils";

describe("CiphertextStorage", function () {
  const ctHandle = 2025;
  const keyId = 0;
  const chainId = 1;
  const ciphertext = "0x02";
  const snsCiphertext = "0x03";

  // Fake values
  const fakeCtHandle = 11111;
  const fakeChainId = 123;

  let httpz: HTTPZ;
  let ciphertextStorage: CiphertextStorage;
  let coprocessorSigners: HardhatEthersSigner[];
  let user: HardhatEthersSigner;
  async function deployCiphertextStorageFixture() {
    const { httpz, keyManager, coprocessorSigners, signers, user } = await loadFixture(deployKeyManagerFixture);
    const CiphertextStorageContract = await hre.ethers.getContractFactory("CiphertextStorage");
    const ciphertextStorage = await CiphertextStorageContract.deploy(httpz, keyManager);

    // Setup the ciphertext storage state with a ciphertext used during tests
    for (let signer of coprocessorSigners) {
      await ciphertextStorage.connect(signer).addCiphertext(ctHandle, keyId, chainId, ciphertext, snsCiphertext);
    }
    return { httpz, ciphertextStorage, coprocessorSigners, signers, user };
  }

  beforeEach(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(deployCiphertextStorageFixture);
    httpz = fixture.httpz;
    coprocessorSigners = fixture.coprocessorSigners;
    ciphertextStorage = fixture.ciphertextStorage;
    user = fixture.user;
  });

  describe("Add ciphertext", async function () {
    it("Should add a ciphertext", async function () {
      // Given
      const newCtHandle = "0x0123";

      // When
      await ciphertextStorage
        .connect(coprocessorSigners[0])
        .addCiphertext(newCtHandle, keyId, chainId, ciphertext, snsCiphertext);

      // This transaction should make the consensus to be reached and thus emit the expected event
      const result = ciphertextStorage
        .connect(coprocessorSigners[1])
        .addCiphertext(newCtHandle, keyId, chainId, ciphertext, snsCiphertext);

      // Then
      await expect(result).to.emit(ciphertextStorage, "AddCiphertext").withArgs(newCtHandle);

      // Then check that no other events get triggered
      await ciphertextStorage
        .connect(coprocessorSigners[2])
        .addCiphertext(newCtHandle, keyId, chainId, ciphertext, snsCiphertext);
      const events = await ciphertextStorage.queryFilter(ciphertextStorage.filters.AddCiphertext(newCtHandle));

      // It should emit only the event once consensus is reached which means only the second transaction emits the event
      expect(events.length).to.equal(1);
    });

    it("Should revert because the signer is not a Coprocessor", async function () {
      // Use a signer that is not a Coprocessor
      const result = ciphertextStorage.connect(user).addCiphertext(ctHandle, keyId, chainId, ciphertext, snsCiphertext);

      // Then
      await expect(result)
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.COPROCESSOR_ROLE());
    });

    it("Should revert with CoprocessorHasAlreadyAdded", async function () {
      // When
      const result = ciphertextStorage
        .connect(coprocessorSigners[0])
        .addCiphertext(ctHandle, keyId, chainId, ciphertext, snsCiphertext);

      // Then
      await expect(result).revertedWithCustomError(ciphertextStorage, "CoprocessorHasAlreadyAdded");
    });

    // TODO: Add test checking `isCurrentKeyId` once keys are generated through the Gateway
  });

  describe("Get regular ciphertext materials", async function () {
    it("Should get regular ciphertext materials", async function () {
      // When
      const result = await ciphertextStorage.getCiphertextMaterials([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([[ctHandle, keyId, ciphertext]]);
    });

    it("Should revert with CiphertextNotFound (regular)", async function () {
      await expect(ciphertextStorage.getCiphertextMaterials([fakeCtHandle]))
        .revertedWithCustomError(ciphertextStorage, "CiphertextNotFound")
        .withArgs(fakeCtHandle);
    });
  });

  describe("Get SNS ciphertext materials", async function () {
    it("Should get SNS ciphertext materials", async function () {
      // When
      const result = await ciphertextStorage.getSnsCiphertextMaterials([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([[ctHandle, keyId, snsCiphertext]]);
    });

    it("Should revert with CiphertextNotFound (SNS)", async function () {
      await expect(ciphertextStorage.getSnsCiphertextMaterials([fakeCtHandle]))
        .revertedWithCustomError(ciphertextStorage, "CiphertextNotFound")
        .withArgs(fakeCtHandle);
    });
  });

  describe("Has ciphertext", async function () {
    it("Should return true", async function () {
      // When
      const result = await ciphertextStorage.hasCiphertext(ctHandle);

      // Then
      expect(result).to.be.eq(true);
    });

    it("Should return false", async function () {
      // When
      const result = await ciphertextStorage.hasCiphertext(fakeCtHandle);

      // Then
      expect(result).to.be.eq(false);
    });
  });

  describe("Is on network", async function () {
    it("Should not revert", async function () {
      // When
      await expect(ciphertextStorage.checkIsOnNetwork(ctHandle, chainId)).not.to.be.reverted;
    });

    it("Should revert because the ciphertext is not on the network", async function () {
      // When
      await expect(ciphertextStorage.checkIsOnNetwork(ctHandle, fakeChainId))
        .revertedWithCustomError(ciphertextStorage, "CiphertextNotOnNetwork")
        .withArgs(ctHandle, fakeChainId);
    });
  });
});
