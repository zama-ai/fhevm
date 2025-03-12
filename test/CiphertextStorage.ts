import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { CiphertextManager, HTTPZ } from "../typechain-types";
import { deployKeyManagerFixture } from "./utils";

describe("CiphertextManager", function () {
  const ctHandle = 2025;
  const keyId = 0;
  const chainId = 1;
  const ciphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));
  const snsCiphertextDigest = hre.ethers.hexlify(hre.ethers.randomBytes(32));

  // Fake values
  const fakeCtHandle = 11111;
  const fakeChainId = 123;

  let httpz: HTTPZ;
  let ciphertextManager: CiphertextManager;
  let coprocessorSigners: HardhatEthersSigner[];
  let user: HardhatEthersSigner;
  async function deployCiphertextManagerFixture() {
    const { httpz, keyManager, coprocessorSigners, signers, user } = await loadFixture(deployKeyManagerFixture);
    const CiphertextManagerContract = await hre.ethers.getContractFactory("CiphertextManager");
    const ciphertextManager = await CiphertextManagerContract.deploy(httpz, keyManager);

    // Setup the CiphertextManager contract state with a ciphertext used during tests
    for (let signer of coprocessorSigners) {
      await ciphertextManager
        .connect(signer)
        .addCiphertextMaterial(ctHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest);
    }
    return { httpz, ciphertextManager, coprocessorSigners, signers, user };
  }

  beforeEach(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(deployCiphertextManagerFixture);
    httpz = fixture.httpz;
    coprocessorSigners = fixture.coprocessorSigners;
    ciphertextManager = fixture.ciphertextManager;
    user = fixture.user;
  });

  describe("Add ciphertext material", async function () {
    it("Should add a ciphertext material", async function () {
      // Given
      const newCtHandle = "0x0123";

      // When
      await ciphertextManager
        .connect(coprocessorSigners[0])
        .addCiphertextMaterial(newCtHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest);

      // This transaction should make the consensus to be reached and thus emit the expected event
      const result = ciphertextManager
        .connect(coprocessorSigners[1])
        .addCiphertextMaterial(newCtHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest);

      // Then
      await expect(result)
        .to.emit(ciphertextManager, "AddCiphertextMaterial")
        .withArgs(newCtHandle, ciphertextDigest, snsCiphertextDigest, [
          coprocessorSigners[0].address,
          coprocessorSigners[1].address,
        ]);

      // Then check that no other events get triggered
      await ciphertextManager
        .connect(coprocessorSigners[2])
        .addCiphertextMaterial(newCtHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest);
      const events = await ciphertextManager.queryFilter(ciphertextManager.filters.AddCiphertextMaterial(newCtHandle));

      // It should emit only the event once consensus is reached which means only the second transaction emits the event
      expect(events.length).to.equal(1);
    });

    it("Should revert because the signer is not a Coprocessor", async function () {
      // Use a signer that is not a Coprocessor
      const result = ciphertextManager
        .connect(user)
        .addCiphertextMaterial(ctHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest);

      // Then
      await expect(result)
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(user.address, httpz.COPROCESSOR_ROLE());
    });

    it("Should revert with CoprocessorHasAlreadyAdded", async function () {
      // When
      const result = ciphertextManager
        .connect(coprocessorSigners[0])
        .addCiphertextMaterial(ctHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest);

      // Then
      await expect(result).revertedWithCustomError(ciphertextManager, "CoprocessorHasAlreadyAdded");
    });

    // TODO: Add test checking `isCurrentKeyId` once keys are generated through the Gateway
  });

  describe("Get regular ciphertext materials", async function () {
    it("Should get regular ciphertext materials", async function () {
      // When
      const result = await ciphertextManager.getCiphertextMaterials([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([[ctHandle, keyId, ciphertextDigest, coprocessorSigners.map((s) => s.address)]]);
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
      expect(result).to.be.deep.eq([[ctHandle, keyId, snsCiphertextDigest, coprocessorSigners.map((s) => s.address)]]);
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

  describe("Check is on network", async function () {
    it("Should not revert", async function () {
      await expect(ciphertextManager.checkIsOnNetwork(ctHandle, chainId)).not.to.be.reverted;
    });

    it("Should revert because the ciphertext is not on the network", async function () {
      await expect(ciphertextManager.checkIsOnNetwork(ctHandle, fakeChainId))
        .revertedWithCustomError(ciphertextManager, "CiphertextMaterialNotOnNetwork")
        .withArgs(ctHandle, fakeChainId);
    });
  });
});
