import {
  time,
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
import hre from "hardhat";

describe("DecryptionOracle", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployDecryptionOracleFixture() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await hre.ethers.getSigners();

    const DecryptionOracle = await hre.ethers.getContractFactory(
      "DecryptionOracle"
    );
    const decryptionOracle = await DecryptionOracle.deploy();

    return { owner, otherAccount, decryptionOracle };
  }

  describe("Deployment", function () {
    it("Should deploy DecryptionOracle", async function () {
      const { decryptionOracle } = await loadFixture(
        deployDecryptionOracleFixture
      );

      expect(await decryptionOracle.getVersion()).to.equal(
        "DecryptionOracle v0.1.0"
      );
    });
  });
});
