import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { KMSGeneration, KMSGeneration__factory } from "../typechain-types";
import { getKeyId, getCrsId, loadTestVariablesFixture } from "./utils";

describe("KMSGeneration", function () {
  describe("Deployment", function () {
    let kmsGeneration: KMSGeneration;
    let owner: Wallet;
    let kmsGenerationFactory: KMSGeneration__factory;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsGeneration = fixtureData.kmsGeneration;
      owner = fixtureData.owner;

      // Get the KMSGeneration contract factory
      kmsGenerationFactory = await hre.ethers.getContractFactory("KMSGeneration", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(kmsGeneration, kmsGenerationFactory, {
          call: { fn: "initializeFromEmptyProxy", args: [] },
        }),
      ).to.be.revertedWithCustomError(kmsGeneration, "NotInitializingFromEmptyProxy");
    });
  });

  describe("View functions on fresh deployment (no historical data)", function () {
    let kmsGeneration: KMSGeneration;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsGeneration = fixtureData.kmsGeneration;
    });

    it("Should return version", async function () {
      expect(await kmsGeneration.getVersion()).to.equal("KMSGeneration v0.5.0");
    });

    it("Should return zero for active key ID when no key has been generated", async function () {
      expect(await kmsGeneration.getActiveKeyId()).to.equal(0n);
    });

    it("Should return zero for active CRS ID when no CRS has been generated", async function () {
      expect(await kmsGeneration.getActiveCrsId()).to.equal(0n);
    });

    it("Should return empty array for consensus tx senders when no request exists", async function () {
      const fakeRequestId = getKeyId(1);
      expect(await kmsGeneration.getConsensusTxSenders(fakeRequestId)).to.deep.equal([]);
    });

    it("Should revert on getKeyParamsType for non-existent key", async function () {
      const fakeKeyId = getKeyId(1);
      await expect(kmsGeneration.getKeyParamsType(fakeKeyId))
        .to.be.revertedWithCustomError(kmsGeneration, "KeyNotGenerated")
        .withArgs(fakeKeyId);
    });

    it("Should revert on getCrsParamsType for non-existent CRS", async function () {
      const fakeCrsId = getCrsId(1);
      await expect(kmsGeneration.getCrsParamsType(fakeCrsId))
        .to.be.revertedWithCustomError(kmsGeneration, "CrsNotGenerated")
        .withArgs(fakeCrsId);
    });

    it("Should revert on getKeyMaterials for non-existent key", async function () {
      const fakeKeyId = getKeyId(1);
      await expect(kmsGeneration.getKeyMaterials(fakeKeyId))
        .to.be.revertedWithCustomError(kmsGeneration, "KeyNotGenerated")
        .withArgs(fakeKeyId);
    });

    it("Should revert on getCrsMaterials for non-existent CRS", async function () {
      const fakeCrsId = getCrsId(1);
      await expect(kmsGeneration.getCrsMaterials(fakeCrsId))
        .to.be.revertedWithCustomError(kmsGeneration, "CrsNotGenerated")
        .withArgs(fakeCrsId);
    });
  });
});
