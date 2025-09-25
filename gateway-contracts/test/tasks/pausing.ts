import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { Decryption, InputVerification } from "../../typechain-types";
import { loadTestVariablesFixture } from "../utils";

describe("Pausing and Unpausing Tasks", function () {
  let decryption: Decryption;
  let inputVerification: InputVerification;
  describe("Hardhat pausing/unpausing tasks", function () {
    before(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      decryption = fixtureData.decryption;
      inputVerification = fixtureData.inputVerification;
    });

    it("Should pause all contracts", async function () {
      expect(await inputVerification.paused()).to.eq(false);
      expect(await decryption.paused()).to.eq(false);
      await hre.run("task:pauseAllGatewayContracts", { useInternalGatewayConfigAddress: true });
      expect(await inputVerification.paused()).to.eq(true);
      expect(await decryption.paused()).to.eq(true);
    });

    it("Should unpause all contracts", async function () {
      await hre.run("task:unpauseAllGatewayContracts", { useInternalGatewayConfigAddress: true });
      expect(await inputVerification.paused()).to.eq(false);
      expect(await decryption.paused()).to.eq(false);
    });

    it("Should pause inputVerification", async function () {
      await hre.run("task:pauseInputVerification", { useInternalGatewayConfigAddress: true });
      expect(await inputVerification.paused()).to.eq(true);
      expect(await decryption.paused()).to.eq(false);
    });

    it("Should pause decryption", async function () {
      await hre.run("task:pauseDecryption", { useInternalGatewayConfigAddress: true });
      expect(await inputVerification.paused()).to.eq(true);
      expect(await decryption.paused()).to.eq(true);
    });

    it("Should unpause inputVerification", async function () {
      await hre.run("task:unpauseInputVerification", { useInternalGatewayConfigAddress: true });
      expect(await inputVerification.paused()).to.eq(false);
      expect(await decryption.paused()).to.eq(true);
    });

    it("Should unpause decryption", async function () {
      await hre.run("task:unpauseDecryption", { useInternalGatewayConfigAddress: true });
      expect(await inputVerification.paused()).to.eq(false);
      expect(await decryption.paused()).to.eq(false);
    });
  });
});
