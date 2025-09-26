import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { GatewayConfig, PauserSet } from "../typechain-types";
import { createRandomWallet, loadTestVariablesFixture } from "./utils";

describe("PauserSet", function () {
  // Define fake values
  const fakeOwner = createRandomWallet();

  let gatewayConfig: GatewayConfig;
  let pauserSet: PauserSet;
  let owner: Wallet;
  let pauser: Wallet;
  let newPauser: string;

  before(async function () {
    // Initialize globally used variables before each test
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixtureData.gatewayConfig;
    pauserSet = fixtureData.pauserSet;
    owner = fixtureData.owner;
    pauser = fixtureData.pauser;
  });

  it("Should return true for the initial pauser address", async function () {
    expect(await gatewayConfig.isPauser(pauser.address)).to.equal(true);
  });

  it("Should revert because the sender is not the owner", async function () {
    await expect(pauserSet.connect(fakeOwner).addPauser(fakeOwner.address))
      .to.be.revertedWithCustomError(pauserSet, "NotGatewayOwner")
      .withArgs(fakeOwner.address);
    await expect(pauserSet.connect(fakeOwner).removePauser(pauser.address))
      .to.be.revertedWithCustomError(pauserSet, "NotGatewayOwner")
      .withArgs(fakeOwner.address);
    const newPauser = createRandomWallet();
    await expect(pauserSet.connect(fakeOwner).swapPauser(pauser.address, newPauser))
      .to.be.revertedWithCustomError(pauserSet, "NotGatewayOwner")
      .withArgs(fakeOwner.address);
  });

  it("Should add the pauser", async function () {
    const newPauser = createRandomWallet();

    const tx = await pauserSet.connect(owner).addPauser(newPauser.address);

    await expect(tx).to.emit(pauserSet, "AddPauser").withArgs(newPauser.address);
  });

  it("Should revert when adding an already added pauser", async function () {
    await expect(pauserSet.connect(owner).addPauser(pauser.address))
      .to.be.revertedWithCustomError(pauserSet, "AccountAlreadyPauser")
      .withArgs(pauser.address);
  });

  it("Should revert when removing a non-pauser", async function () {
    const newPauser = createRandomWallet();
    await expect(pauserSet.connect(owner).removePauser(newPauser.address))
      .to.be.revertedWithCustomError(pauserSet, "AccountNotPauser")
      .withArgs(newPauser.address);
  });

  it("Should remove when removing a pauser", async function () {
    const newPauser = createRandomWallet();
    await pauserSet.connect(owner).addPauser(newPauser.address);
    const tx = await pauserSet.connect(owner).removePauser(newPauser.address);

    await expect(tx).to.emit(pauserSet, "RemovePauser").withArgs(newPauser.address);
  });

  it("Should revert because the pauser is the null address", async function () {
    const nullPauser = hre.ethers.ZeroAddress;

    await expect(pauserSet.connect(owner).addPauser(nullPauser)).to.be.revertedWithCustomError(
      pauserSet,
      "InvalidNullPauser",
    );
  });

  it("Should swap the pauser", async function () {
    const oldPauser = createRandomWallet();
    await pauserSet.connect(owner).addPauser(oldPauser.address);
    const newPauser = createRandomWallet();
    const tx = await pauserSet.connect(owner).swapPauser(oldPauser.address, newPauser.address);
    await expect(tx).to.emit(pauserSet, "SwapPauser").withArgs(oldPauser.address, newPauser.address);
    expect(await pauserSet.isPauser(oldPauser)).to.be.false;
    expect(await pauserSet.isPauser(newPauser)).to.be.true;
  });

  it("Should revert swappig the pauser", async function () {
    const newPauser = createRandomWallet();
    await expect(pauserSet.connect(owner).swapPauser(newPauser.address, newPauser.address))
      .to.be.revertedWithCustomError(pauserSet, "AccountNotPauser")
      .withArgs(newPauser.address);
    await expect(pauserSet.connect(owner).swapPauser(pauser.address, pauser.address))
      .to.be.revertedWithCustomError(pauserSet, "AccountAlreadyPauser")
      .withArgs(pauser.address);
    const nullPauser = hre.ethers.ZeroAddress;
    await expect(pauserSet.connect(owner).swapPauser(nullPauser, newPauser.address)).to.be.revertedWithCustomError(
      pauserSet,
      "InvalidNullPauser",
    );
    await expect(pauserSet.connect(owner).swapPauser(pauser.address, nullPauser)).to.be.revertedWithCustomError(
      pauserSet,
      "InvalidNullPauser",
    );
  });
});
