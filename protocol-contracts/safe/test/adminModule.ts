import { expect } from "chai";
import { Signer, ZeroAddress } from "ethers";
import { ethers } from "hardhat";

import { AdminModule, GatewayConfigMock, SafeL2 } from "../typechain-types";
import { execTransaction } from "./utils/utils";

describe("AdminModule Tests", function () {
  // Define variables
  let deployer: Signer;
  let alice: Signer;
  let bob: Signer;
  let charlie: Signer;
  let masterCopy: any;
  let safe: SafeL2;
  let safeAddress: string;
  let gatewayConfigMock: GatewayConfigMock;

  before(async () => {
    [deployer, alice, bob, charlie] = await ethers.getSigners();

    const safeFactory = await ethers.getContractFactory("SafeL2", deployer); // L2 version for easier debugging and because gas is cheap on gateway
    masterCopy = await safeFactory.deploy(); // deploys the singleton Safe implementation

    const proxyFactory = await (await ethers.getContractFactory("SafeProxyFactory", deployer)).deploy();

    // Setup the Safe, Step 1, generate transaction data, with one owner, alice, and threshold of 1
    const safeData = masterCopy.interface.encodeFunctionData("setup", [
      [await alice.getAddress()],
      1,
      ZeroAddress,
      "0x",
      ZeroAddress,
      ZeroAddress,
      0,
      ZeroAddress,
    ]);

    // this statiCall allows to predict the address of the upcoming Safe proxy not deployed yet
    safeAddress = await proxyFactory.createProxyWithNonce.staticCall(await masterCopy.getAddress(), safeData, 0n);

    if (safeAddress === ZeroAddress) {
      throw new Error("Safe address not found");
    }

    await proxyFactory.createProxyWithNonce(await masterCopy.getAddress(), safeData, 0n);

    safe = await ethers.getContractAt("SafeL2", safeAddress);

    gatewayConfigMock = await (await ethers.getContractFactory("GatewayConfigMock", deployer)).deploy(safeAddress);
  });

  // A Safe Module is a smart contract that is allowed to execute transactions on behalf of a Safe Smart Account.
  // This function deploys the AdminModule contract and enables it in the Safe.
  const enableModule = async (): Promise<{
    adminModule: AdminModule;
  }> => {
    const adminModule = await (
      await ethers.getContractFactory("AdminModule", deployer)
    ).deploy(await charlie.getAddress(), safeAddress); // charlie is set to be the owner of AdminModule

    // Enable the module in the safe, Step 1, generate transaction data
    const enableModuleData = masterCopy.interface.encodeFunctionData("enableModule", [adminModule.target]);

    // Enable the module in the safe, Step 2, execute the transaction
    await execTransaction([alice], safe, safe.target, 0, enableModuleData, 0);

    // Verify that the module is enabled
    expect(await safe.isModuleEnabled(adminModule.target)).to.be.true;

    return { adminModule };
  };

  it("Should successfully propagate tx from the admin account", async function () {
    // Enable the module in the Safe
    const { adminModule } = await enableModule();
    const gatewayConfigMockAddress = await gatewayConfigMock.getAddress();
    const data = gatewayConfigMock.interface.encodeFunctionData("setByOwner", [42n]);
    await adminModule.connect(charlie).execTransactionFromModuleReturnData(gatewayConfigMockAddress, 0n, data, 0n);
    expect(await gatewayConfigMock.value()).to.equal(42n);
  });
});
