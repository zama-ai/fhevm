import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { GatewayConfig } from "../../typechain-types";
import { loadTestVariablesFixture } from "../utils";

// Asserts that the given promise rejects with an error message containing `substring`.
// Used for the tasks' pre-flight checks, which throw plain JS errors (not contract reverts).
async function expectTaskToThrow(promise: Promise<unknown>, substring: string) {
  let threw = false;
  try {
    await promise;
  } catch (error) {
    threw = true;
    expect((error as Error).message).to.include(substring);
  }
  expect(threw, `Expected task to throw containing "${substring}"`).to.eq(true);
}

describe("Host chain management tasks", function () {
  let gatewayConfig: GatewayConfig;
  let registeredChainId: number;
  // A chain ID that is guaranteed not to be registered by the test setup.
  const unregisteredChainId = 999999;

  before(async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    gatewayConfig = fixtureData.gatewayConfig;
    // The test setup registers the host chains via `task:addHostChainsToGatewayConfig`.
    registeredChainId = fixtureData.chainIds[0];
  });

  it("Should start with a registered, enabled host chain", async function () {
    expect(await gatewayConfig.isHostChainRegistered(registeredChainId)).to.eq(true);
    expect(await gatewayConfig.isHostChainDisabled(registeredChainId)).to.eq(false);
  });

  it("Should disable a host chain", async function () {
    await hre.run("task:disableHostChainOnGatewayConfig", {
      chainId: registeredChainId.toString(),
      useInternalProxyAddress: true,
    });
    expect(await gatewayConfig.isHostChainDisabled(registeredChainId)).to.eq(true);
    expect(await gatewayConfig.isHostChainRegistered(registeredChainId)).to.eq(true);
  });

  it("Should reject disabling an already-disabled host chain", async function () {
    await expectTaskToThrow(
      hre.run("task:disableHostChainOnGatewayConfig", {
        chainId: registeredChainId.toString(),
        useInternalProxyAddress: true,
      }),
      "already disabled",
    );
  });

  it("Should reject removing an unregistered host chain", async function () {
    await expectTaskToThrow(
      hre.run("task:removeHostChainOnGatewayConfig", {
        chainId: unregisteredChainId.toString(),
        useInternalProxyAddress: true,
      }),
      "not registered",
    );
  });

  it("Should enable a disabled host chain", async function () {
    await hre.run("task:enableHostChainOnGatewayConfig", {
      chainId: registeredChainId.toString(),
      useInternalProxyAddress: true,
    });
    expect(await gatewayConfig.isHostChainDisabled(registeredChainId)).to.eq(false);
  });

  it("Should reject enabling an already-enabled host chain", async function () {
    await expectTaskToThrow(
      hre.run("task:enableHostChainOnGatewayConfig", {
        chainId: registeredChainId.toString(),
        useInternalProxyAddress: true,
      }),
      "already enabled",
    );
  });

  it("Should reject removing an enabled (not-disabled) host chain", async function () {
    await expectTaskToThrow(
      hre.run("task:removeHostChainOnGatewayConfig", {
        chainId: registeredChainId.toString(),
        useInternalProxyAddress: true,
      }),
      "must be disabled before removal",
    );
  });

  it("Should remove a host chain after it has been disabled", async function () {
    await hre.run("task:disableHostChainOnGatewayConfig", {
      chainId: registeredChainId.toString(),
      useInternalProxyAddress: true,
    });
    await hre.run("task:removeHostChainOnGatewayConfig", {
      chainId: registeredChainId.toString(),
      useInternalProxyAddress: true,
    });
    expect(await gatewayConfig.isHostChainRegistered(registeredChainId)).to.eq(false);
  });
});
