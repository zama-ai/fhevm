import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { CiphertextStorage } from "../typechain-types";
import { deployKeyManagerFixture } from "./utils";

describe("CiphertextStorage", function () {
  const ctHandle = 2025;
  const keyId = 0;
  const chainId = 1;
  const ciphertext64 = "0x02";
  const ciphertext128 = "0x03";

  // Fake values
  const fakeCtHandle = 11111;
  const fakeChainId = 123;

  let ciphertextStorage: CiphertextStorage;
  let coprocessorSigners: HardhatEthersSigner[];

  async function deployCiphertextStorageFixture() {
    const { httpz, keyManager, coprocessorSigners, signers } = await loadFixture(deployKeyManagerFixture);
    const CiphertextStorageContract = await hre.ethers.getContractFactory("CiphertextStorage");
    const ciphertextStorage = await CiphertextStorageContract.deploy(httpz, keyManager);

    // Setup the ciphertext storage state with a ciphertext used during tests
    for (let signer of coprocessorSigners) {
      await ciphertextStorage.connect(signer).addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);
    }
    return { ciphertextStorage, coprocessorSigners, signers };
  }

  beforeEach(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(deployCiphertextStorageFixture);
    ciphertextStorage = fixture.ciphertextStorage;
    coprocessorSigners = fixture.coprocessorSigners;
  });

  describe("Add ciphertext", async function () {
    it("Should success", async function () {
      // Given
      const newCtHandle = "0x0123";

      // When
      await ciphertextStorage
        .connect(coprocessorSigners[0])
        .addCiphertext(newCtHandle, keyId, chainId, ciphertext64, ciphertext128);

      // This transaction should make the consensus to be reached and thus emit the expected event
      const result = ciphertextStorage
        .connect(coprocessorSigners[1])
        .addCiphertext(newCtHandle, keyId, chainId, ciphertext64, ciphertext128);

      // Then
      await expect(result).to.emit(ciphertextStorage, "AddCiphertext").withArgs(newCtHandle);

      // Then check that no other events get triggered
      await ciphertextStorage
        .connect(coprocessorSigners[2])
        .addCiphertext(newCtHandle, keyId, chainId, ciphertext64, ciphertext128);
      const events = await ciphertextStorage.queryFilter(ciphertextStorage.filters.AddCiphertext(newCtHandle));

      // It should emit only the event once consensus is reached which means only the second transaction emits the event
      expect(events.length).to.equal(1);
    });

    it("Should revert with CoprocessorHasAlreadyAdded", async function () {
      // When
      const result = ciphertextStorage
        .connect(coprocessorSigners[0])
        .addCiphertext(ctHandle, keyId, chainId, ciphertext64, ciphertext128);

      // Then
      await expect(result).revertedWithCustomError(ciphertextStorage, "CoprocessorHasAlreadyAdded");
    });
  });

  describe("Get ciphertexts", async function () {
    it("Should success", async function () {
      // When
      const result = await ciphertextStorage.getCiphertexts([ctHandle]);

      // Then
      expect(result).to.be.deep.eq([[ctHandle, keyId, ciphertext128]]);
    });

    it("Should revert with CiphertextNotFound", async function () {
      await expect(ciphertextStorage.getCiphertexts([fakeCtHandle]))
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
    it("Should return true", async function () {
      // When
      const result = await ciphertextStorage.isOnNetwork(ctHandle, chainId);

      // Then
      expect(result).to.be.eq(true);
    });

    it("Should return false", async function () {
      // When
      const txResponse1 = await ciphertextStorage.isOnNetwork(ctHandle, fakeChainId);
      const txResponse2 = await ciphertextStorage.isOnNetwork(fakeCtHandle, chainId);

      // Then
      expect(txResponse1).to.be.eq(false);
      expect(txResponse2).to.be.eq(false);
    });
  });
});
