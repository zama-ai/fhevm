import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { getRequiredEnvVar } from "../../tasks/utils/loadVariables";
import { GatewayConfig } from "../../typechain-types";
import { loadTestVariablesFixture } from "../utils";

describe("Ownership tasks", function () {
  // Get the private key of the new owner
  const newOwnerPrivateKey = getRequiredEnvVar("NEW_OWNER_PRIVATE_KEY");
  const newOwner = new Wallet(newOwnerPrivateKey).connect(hre.ethers.provider);

  let gatewayConfig: GatewayConfig;
  let owner: Wallet;

  before(async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixtureData.gatewayConfig;
    owner = fixtureData.owner;
  });

  it("Should ask transfer ownership of the GatewayConfig contract", async function () {
    expect(await gatewayConfig.owner()).to.eq(owner.address);

    await hre.run("task:transferGatewayOwnership", { newOwnerAddress: newOwner.address });

    // Check that the ownership has not been transferred as the transfer is only pending since the
    // new owner has not accepted it yet.
    expect(await gatewayConfig.owner()).to.eq(owner.address);

    // Check that the pending owner is the new owner.
    expect(await gatewayConfig.pendingOwner()).to.eq(newOwner.address);
  });

  it("Should accept ownership of the GatewayConfig contract", async function () {
    await hre.run("task:acceptGatewayOwnership");

    // Check that the ownership has been transferred to the new owner.
    expect(await gatewayConfig.owner()).to.eq(newOwner.address);
  });
});
