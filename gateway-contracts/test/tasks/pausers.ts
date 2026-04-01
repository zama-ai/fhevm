import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { getPauserSetContract } from "../../tasks/utils/loadVariables";

describe("Pauser tasks", function () {
  let pauserSet: Awaited<ReturnType<typeof getPauserSetContract>>;

  before(async function () {
    pauserSet = await getPauserSetContract(true, hre);
  });

  it("Should add pausers through the task", async function () {
    const newPauser = Wallet.createRandom().address;

    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    process.env.NUM_PAUSERS = "1";
    process.env.PAUSER_ADDRESS_0 = newPauser;

    await hre.run("task:addGatewayPausers", { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });

  it("Should remove a pauser through the task", async function () {
    const pauserToRemove = Wallet.createRandom().address;
    await pauserSet.addPauser(pauserToRemove);

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(true);

    await hre.run("task:removeGatewayPauser", {
      useInternalProxyAddress: true,
      pauserAddress: pauserToRemove,
    });

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(false);
  });

  it("Should swap a pauser through the task", async function () {
    const oldPauser = Wallet.createRandom().address;
    const newPauser = Wallet.createRandom().address;
    await pauserSet.addPauser(oldPauser);

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
