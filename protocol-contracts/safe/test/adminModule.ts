import { expect } from "chai";
import { Signer } from "ethers";
import { ethers } from "hardhat";
import hre from "hardhat";

import { getRequiredEnvVar } from "../tasks/utils/loadVariables";
import { AdminModule, GatewayConfigMock } from "../typechain-types";
import { execTransaction } from "../tasks/utils/execTransaction";

describe("AdminModule Tests", function () {
  let deployer: Signer;
  let safeSingleton: any;
  let safeProxy: any;
  let safeProxyAddress: string;
  let gatewayConfigMock: GatewayConfigMock;

  before(async () => {
    // Get the deployer
    const namedAccounts = await hre.getNamedAccounts();
    deployer = await hre.ethers.getSigner(namedAccounts.deployer);

    // Get the deployed contract:
    // - SafeL2Proxy is the name of the proxy contract
    // - SafeL2 is the name of the implementation contract
    const safeProxyDeployment = await hre.deployments.get("SafeL2Proxy");
    safeProxyAddress = safeProxyDeployment.address;
    safeProxy = await hre.ethers.getContractAt("SafeL2", safeProxyAddress);
    safeSingleton = await hre.ethers.getContractAt("SafeL2", safeProxyAddress);

    // Deploy the GatewayConfigMock contract
    gatewayConfigMock = await (
      await ethers.getContractFactory("GatewayConfigMock", deployer)
    ).deploy(safeProxyAddress);
  });

  // A Safe Module is a smart contract that is allowed to execute transactions on behalf of a Safe
  // Smart Account, without having to consider the threshold and gather signatures.
  // This function deploys the AdminModule contract and enables it in the Safe.
  const enableModule = async (): Promise<{
    adminModule: AdminModule;
  }> => {
    // Get the deployed contract
    const adminModuleDeployment = await hre.deployments.get("AdminModule");
    const adminModule = await hre.ethers.getContractAt(
      "AdminModule",
      adminModuleDeployment.address,
    );

    // Step 1, generate transaction data
    const enableModuleData = safeSingleton.interface.encodeFunctionData(
      "enableModule",
      [adminModule.target],
    );

    // Step 2, execute the transaction using the deployer account (the Safe owner)
    await execTransaction(
      [deployer],
      safeProxy,
      safeProxy.target,
      0,
      enableModuleData,
      0,
    );

    // Verify that the module is enabled
    expect(await safeProxy.isModuleEnabled(adminModule.target)).to.be.true;

    return { adminModule };
  };

  it("Should successfully propagate tx from the admin account", async function () {
    // Enable the module in the Safe
    const { adminModule } = await enableModule();

    const gatewayConfigMockAddress = await gatewayConfigMock.getAddress();

    // Step 1, encode the transaction data
    const data = gatewayConfigMock.interface.encodeFunctionData("setByOwner", [
      42n,
    ]);

    // Get the admin account
    const adminAddress = getRequiredEnvVar("ADMIN_ADDRESS");
    const admin = await hre.ethers.getSigner(adminAddress);

    // Step 2, execute the transaction using the admin account
    await adminModule
      .connect(admin)
      .executeSafeTransactions([gatewayConfigMockAddress], [0n], [data], [0n]);

    // Check that the transaction was successful
    expect(await gatewayConfigMock.value()).to.equal(42n);
  });
});
