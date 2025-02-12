import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { CiphertextStorage } from "../typechain-types";

describe("CiphertextStorage", function () {
  async function deployCiphertextStorageFixture() {
    const CiphertextStorageContract = await hre.ethers.getContractFactory("CiphertextStorage");
    // TODO: Implement the HTTPZ deployment and replace the address
    const ciphertextStorage = await CiphertextStorageContract.deploy("0xDA9FeD390f02F559E62240a112aBd2FAe06DCdB5");
    return { ciphertextStorage };
  }

  describe("Add ciphertext", async function () {
    let ciphertextStorageInstance: CiphertextStorage;
    beforeEach(async function () {
      const { ciphertextStorage } = await loadFixture(deployCiphertextStorageFixture);
      ciphertextStorageInstance = ciphertextStorage;
    });

    it("Should success", async function () {
      // Given
      const ctHandle = "0x01";
      const keyId = 0;
      const chainId = 1;
      const ciphertext64 = "0x02";
      const ciphertext128 = "0x03";
      const [sender1, sender2, sender3] = await hre.ethers.getSigners();

      // When
      await ciphertextStorageInstance
        .connect(sender1)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      await ciphertextStorageInstance
        .connect(sender2)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      const result = ciphertextStorageInstance
        .connect(sender3)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);

      // Then
      await expect(result).to.emit(ciphertextStorageInstance, "AddCiphertext").withArgs(ctHandle);
    });

    it("Should revert with CoprocessorHasAlreadyAdded", async function () {
      // Given
      const ctHandle = "0x01";
      const keyId = 0;
      const chainId = 1;
      const ciphertext64 = "0x02";
      const ciphertext128 = "0x03";

      // When
      await ciphertextStorageInstance.addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      const result = ciphertextStorageInstance.addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);

      // Then
      await expect(result).revertedWithCustomError(ciphertextStorageInstance, "CoprocessorHasAlreadyAdded");
    });
  });

  describe("Get ciphertexts", async function () {
    let ciphertextStorageInstance: CiphertextStorage;
    beforeEach(async function () {
      const { ciphertextStorage } = await loadFixture(deployCiphertextStorageFixture);
      ciphertextStorageInstance = ciphertextStorage;
    });

    it("Should success", async function () {
      // Given
      const ctHandle = "0x01";
      const keyId = 0;
      const chainId = 1;
      const ciphertext64 = "0x02";
      const ciphertext128 = "0x03";
      const [sender1, sender2, sender3] = await hre.ethers.getSigners();

      // When
      await ciphertextStorageInstance
        .connect(sender1)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      await ciphertextStorageInstance
        .connect(sender2)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      await ciphertextStorageInstance
        .connect(sender3)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      const result = await ciphertextStorageInstance.getCiphertexts([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([[ctHandle, ciphertext128]]);
    });
  });

  describe("Has ciphertext", async function () {
    let ciphertextStorageInstance: CiphertextStorage;
    beforeEach(async function () {
      const { ciphertextStorage } = await loadFixture(deployCiphertextStorageFixture);
      ciphertextStorageInstance = ciphertextStorage;
    });

    it("Should return true", async function () {
      // Given
      const ctHandle = "0x01";
      const keyId = 0;
      const chainId = 1;
      const ciphertext64 = "0x02";
      const ciphertext128 = "0x03";
      const [sender1, sender2, sender3] = await hre.ethers.getSigners();

      // When
      await ciphertextStorageInstance
        .connect(sender1)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      await ciphertextStorageInstance
        .connect(sender2)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      await ciphertextStorageInstance
        .connect(sender3)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      const result = await ciphertextStorageInstance.hasCiphertext(ctHandle);

      // Then
      expect(result).to.be.eq(true);
    });

    it("Should return false", async function () {
      // Given
      const ctHandle = "0x01";

      // When
      const result = await ciphertextStorageInstance.hasCiphertext(ctHandle);

      // Then
      expect(result).to.be.eq(false);
    });
  });

  describe("Is on network", async function () {
    let ciphertextStorageInstance: CiphertextStorage;
    beforeEach(async function () {
      const { ciphertextStorage } = await loadFixture(deployCiphertextStorageFixture);
      ciphertextStorageInstance = ciphertextStorage;
    });

    it("Should return true", async function () {
      // Given
      const ctHandle = "0x01";
      const keyId = 0;
      const chainId = 1;
      const ciphertext64 = "0x02";
      const ciphertext128 = "0x03";
      const [sender1, sender2, sender3] = await hre.ethers.getSigners();

      // When
      await ciphertextStorageInstance
        .connect(sender1)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      await ciphertextStorageInstance
        .connect(sender2)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      await ciphertextStorageInstance
        .connect(sender3)
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
      const result = await ciphertextStorageInstance.isOnNetwork(ctHandle, chainId);

      // Then
      expect(result).to.be.eq(true);
    });

    it("Should return false", async function () {
      // Given
      const ctHandle = "0x01";
      const chainId = 1;

      // When
      const result = await ciphertextStorageInstance.isOnNetwork(ctHandle, chainId);

      // Then
      expect(result).to.be.eq(false);
    });
  });
});
