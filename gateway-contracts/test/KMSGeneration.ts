import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { KMSGeneration, KMSGeneration__factory } from "../typechain-types";
import { loadTestVariablesFixture } from "./utils";

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

    // Arbitrary non-zero IDs for testing reverts on a fresh (empty) deployment
    const fakeKeyId = 1n;
    const fakeCrsId = 2n;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsGeneration = fixtureData.kmsGeneration;
    });

    it("Should return version", async function () {
      expect(await kmsGeneration.getVersion()).to.equal("KMSGeneration v0.5.0");
    });

    it("Should return empty array for consensus tx senders when no request exists", async function () {
      expect(await kmsGeneration.getConsensusTxSenders(fakeKeyId)).to.deep.equal([]);
    });

    it("Should revert on getKeyParamsType for non-existent key", async function () {
      await expect(kmsGeneration.getKeyParamsType(fakeKeyId))
        .to.be.revertedWithCustomError(kmsGeneration, "KeyNotGenerated")
        .withArgs(fakeKeyId);
    });

    it("Should revert on getCrsParamsType for non-existent CRS", async function () {
      await expect(kmsGeneration.getCrsParamsType(fakeCrsId))
        .to.be.revertedWithCustomError(kmsGeneration, "CrsNotGenerated")
        .withArgs(fakeCrsId);
    });

    it("Should revert on getKeyMaterials for non-existent key", async function () {
      await expect(kmsGeneration.getKeyMaterials(fakeKeyId))
        .to.be.revertedWithCustomError(kmsGeneration, "KeyNotGenerated")
        .withArgs(fakeKeyId);
    });

    it("Should revert on getCrsMaterials for non-existent CRS", async function () {
      await expect(kmsGeneration.getCrsMaterials(fakeCrsId))
        .to.be.revertedWithCustomError(kmsGeneration, "CrsNotGenerated")
        .withArgs(fakeCrsId);
    });
  });
});
