import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { loadFixture, time } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

describe("PauserSetWrapper", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployPauserSetMockAndWrapper() {
    // Contracts are deployed using the first signer/account by default
    const [owner, alice, bob] = await hre.ethers.getSigners();

    const PauserSetMock = await hre.ethers.getContractFactory("PauserSetMock");
    const pauserSetMock = await PauserSetMock.deploy();

    const PauserSetWrapper = await hre.ethers.getContractFactory("PauserSetWrapper");
    const pauserSetWrapper = await PauserSetWrapper.deploy(await pauserSetMock.getAddress());

    return { pauserSetMock, pauserSetWrapper, owner, alice, bob };
  }

  describe("Deployment", function () {
    it("Should set the right PAUSER_SET address", async function () {
      const { pauserSetMock, pauserSetWrapper } = await loadFixture(deployPauserSetMockAndWrapper);

      expect(await pauserSetWrapper.PAUSER_SET()).to.equal(await pauserSetMock.getAddress());
    });
  });

  describe("Execution", function () {
    it("Only pauser from PauserSet could execute", async function () {
      const { pauserSetMock, pauserSetWrapper, alice, bob } = await loadFixture(deployPauserSetMockAndWrapper);

      await pauserSetMock.addPauser(alice.address); // owner adds alice as a pauser
      expect(await pauserSetMock.isPauser(alice.address)).to.be.true;

      // empty tx to bob made by the owner via the wrapper
      // expected to revert since owner is not a pauser
      await expect(pauserSetWrapper.execute(bob, "0x")).to.be.revertedWithCustomError(
        pauserSetWrapper,
        "SenderNotPauser",
      );

      // empty tx to bob made by alice via the wrapper
      // expected to succeed since alice is indeed a pauser
      await expect(pauserSetWrapper.connect(alice).execute(bob, "0x")).not.be.reverted;
    });
  });
});
