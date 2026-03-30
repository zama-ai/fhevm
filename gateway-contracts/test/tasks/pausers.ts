import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { PauserSet } from "../../typechain-types";
import { createRandomAddress, loadTestVariablesFixture } from "../utils";

describe("Pauser tasks", function () {
  let pauserSet: PauserSet;

  const managedEnvVars = [
    "NUM_PAUSERS",
    "PAUSER_ADDRESS_0",
    "OLD_PAUSER_ADDRESS_0",
    "NEW_PAUSER_ADDRESS_0",
  ] as const;

  const originalEnv = new Map<string, string | undefined>();

  before(async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    pauserSet = fixtureData.pauserSet;
  });

  // These tasks read pauser inputs from process.env at runtime, so restore overrides per test.
  beforeEach(function () {
    for (const envVar of managedEnvVars) {
      originalEnv.set(envVar, process.env[envVar]);
    }
  });

  afterEach(function () {
    for (const envVar of managedEnvVars) {
      const originalValue = originalEnv.get(envVar);
      if (originalValue === undefined) {
        delete process.env[envVar];
      } else {
        process.env[envVar] = originalValue;
      }
    }
    originalEnv.clear();
  });

  it("Should add pausers through the task", async function () {
    const newPauser = createRandomAddress();

    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    process.env.NUM_PAUSERS = "1";
    process.env.PAUSER_ADDRESS_0 = newPauser;

    await hre.run("task:addGatewayPausers", { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });

  it("Should remove pausers through the task", async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const owner = fixtureData.owner;
    const pauserToRemove = createRandomAddress();
    await pauserSet.connect(owner).addPauser(pauserToRemove);

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(true);

    process.env.NUM_PAUSERS = "1";
    process.env.PAUSER_ADDRESS_0 = pauserToRemove;

    await hre.run("task:removeGatewayPausers", { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(false);
  });

  it("Should swap pausers through the task", async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const owner = fixtureData.owner;
    const oldPauser = createRandomAddress();
    const newPauser = createRandomAddress();
    await pauserSet.connect(owner).addPauser(oldPauser);

    expect(await pauserSet.isPauser(oldPauser)).to.eq(true);
    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    process.env.NUM_PAUSERS = "1";
    process.env.OLD_PAUSER_ADDRESS_0 = oldPauser;
    process.env.NEW_PAUSER_ADDRESS_0 = newPauser;

    await hre.run("task:swapGatewayPausers", { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(oldPauser)).to.eq(false);
    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });
});
