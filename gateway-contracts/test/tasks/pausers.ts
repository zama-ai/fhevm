import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { PauserSet } from "../../typechain-types";
import { createRandomAddress, loadTestVariablesFixture } from "../utils";

describe("Pauser tasks", function () {
  let pauserSet: PauserSet;

  before(async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    pauserSet = fixtureData.pauserSet;
  });

  it("Should add pausers through the task", async function () {
    const newPauser = createRandomAddress();

    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    process.env.NUM_PAUSERS = "1";
    process.env.PAUSER_ADDRESS_0 = newPauser;

    await hre.run("task:addGatewayPausers", { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });

  it("Should remove a pauser through the task", async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const owner = fixtureData.owner;
    const pauserToRemove = createRandomAddress();
    await pauserSet.connect(owner).addPauser(pauserToRemove);

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(true);

    await hre.run("task:removeGatewayPauser", {
      useInternalProxyAddress: true,
      pauserAddress: pauserToRemove,
    });

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(false);
  });

  it("Should swap a pauser through the task", async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const owner = fixtureData.owner;
    const oldPauser = createRandomAddress();
    const newPauser = createRandomAddress();
    await pauserSet.connect(owner).addPauser(oldPauser);

    expect(await pauserSet.isPauser(oldPauser)).to.eq(true);
    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    await hre.run("task:swapGatewayPauser", {
      useInternalProxyAddress: true,
      oldPauserAddress: oldPauser,
      newPauserAddress: newPauser,
    });

    expect(await pauserSet.isPauser(oldPauser)).to.eq(false);
    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });
});
