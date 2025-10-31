import { expect } from "chai";
import hre from "hardhat";

import { getRequiredEnvVar } from "../../tasks/utils/loadVariables";

describe("Deploy Tasks", function () {
  it("Should make sure the SafeL2 contract is properly deployed", async function () {
    // Get the deployed contract (deployed once at the beginning of the test suite):
    // - SafeL2Proxy is the name of the proxy contract
    // - SafeL2 is the name of the implementation contract
    const safeL2Deployment = await hre.deployments.get("SafeL2Proxy");
    const safeL2 = await hre.ethers.getContractAt(
      "SafeL2",
      safeL2Deployment.address,
    );

    // Initially:
    // - owners: the deployer only
    // - threshold: 1
    const deployer = (await hre.getNamedAccounts()).deployer;
    const expectedOwners = [deployer];
    const expectedThreshold = 1;

    // Check that the owners and threshold are correct
    const owners = await safeL2.getOwners();
    const threshold = await safeL2.getThreshold();
    expect(owners).to.deep.equal(expectedOwners);
    expect(threshold).to.equal(expectedThreshold);
  });

  it("Should make sure the AdminModule contract is properly deployed", async function () {
    // Get the deployed contract (deployed once at the beginning of the test suite):
    const adminModuleDeployment = await hre.deployments.get("AdminModule");
    const adminModule = await hre.ethers.getContractAt(
      "AdminModule",
      adminModuleDeployment.address,
    );

    // Get the expected addresses
    const expectedAdminAddress = getRequiredEnvVar("ADMIN_ADDRESS");
    const expectedSafeProxyAddress = getRequiredEnvVar("SAFE_ADDRESS");

    // Check that the admin address and safe proxy address are correct
    const adminAddress = await adminModule.ADMIN_ACCOUNT();
    const safeProxyAddress = await adminModule.SAFE_PROXY();
    expect(adminAddress).to.equal(expectedAdminAddress);
    expect(safeProxyAddress).to.equal(expectedSafeProxyAddress);
  });
});
