import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { GatewayConfig } from "../../typechain-types";
import { loadTestVariablesFixture } from "../utils";

describe("Ownership tasks", function () {
  // Define the private key of the new owner (Account 2)
  const newOwnerPrivateKey = "0x7ae52cf0d3011ef7fecbe22d9537aeda1a9e42a0596e8def5d49970eb59e7a40";
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

    await hre.run("task:transferGatewayOwnership", {
      currentOwnerPrivateKey: owner.privateKey,
      newOwnerAddress: newOwner.address,
    });

    // Check that the ownership has not been transferred as the new owner has not accepted it yet
    expect(await gatewayConfig.owner()).to.eq(owner.address);
  });

  it("Should accept ownership of the GatewayConfig contract", async function () {
    await hre.run("task:transferGatewayOwnership", {
      currentOwnerPrivateKey: owner.privateKey,
      newOwnerAddress: newOwner.address,
    });

    await hre.run("task:acceptGatewayOwnership", { newOwnerPrivateKey: newOwner.privateKey });
    expect(await gatewayConfig.owner()).to.eq(newOwner.address);
  });
});
