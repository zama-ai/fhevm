import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { KMSGeneration } from "../typechain-types";
import { loadTestVariablesFixture } from "./utils";

describe("KMSGeneration", function () {
  describe("View functions on fresh proxy (no historical data)", function () {
    let kmsGeneration: KMSGeneration;

    // Arbitrary non-zero IDs for testing reverts on a fresh (empty) proxy
    const fakeKeyId = 1n;
    const fakeCrsId = 2n;

    beforeEach(async function () {
      const { owner } = await loadFixture(loadTestVariablesFixture);
      const emptyUUPSFactory = await hre.ethers.getContractFactory("EmptyUUPSProxy", owner);
      const emptyUUPS = await hre.upgrades.deployProxy(emptyUUPSFactory, [], {
        initializer: "initialize",
        kind: "uups",
      });
      const kmsGenerationFactory = await hre.ethers.getContractFactory("KMSGeneration", owner);
      const upgraded = await hre.upgrades.upgradeProxy(emptyUUPS, kmsGenerationFactory);
      kmsGeneration = await hre.ethers.getContractAt("KMSGeneration", await upgraded.getAddress());
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
