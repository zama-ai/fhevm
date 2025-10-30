import Safe from "@safe-global/protocol-kit";
import MultiSendJson from "@safe-global/safe-contracts/build/artifacts/contracts/libraries/MultiSend.sol/MultiSend.json";
import { expect } from "chai";
import { ContractFactory, Signer, ZeroAddress } from "ethers";
import { ethers, network } from "hardhat";

import { AdminModule, GatewayConfigMock, SafeL2 } from "../typechain-types";
import { execTransaction } from "./utils/utils";

describe("AdminModule Tests", function () {
  // Define variables
  let deployer: Signer;
  let alice: Signer;
  let bob: Signer;
  let charlie: Signer;
  let masterCopy: any;
  let proxyFactory: any;
  let safe: SafeL2;
  let safeAddress: string;
  let gatewayConfigMock: GatewayConfigMock;
  let multiSendAddress: string;

  before(async () => {
    [deployer, alice, bob, charlie] = await ethers.getSigners();

    const safeFactory = await ethers.getContractFactory("SafeL2", deployer); // L2 version for easier debugging and because gas is cheap on gateway
    masterCopy = await safeFactory.deploy(); // deploys the singleton Safe implementation

    proxyFactory = await (
      await ethers.getContractFactory("SafeProxyFactory", deployer)
    ).deploy();

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
    safeAddress = await proxyFactory.createProxyWithNonce.staticCall(
      await masterCopy.getAddress(),
      safeData,
      0n,
    );

    if (safeAddress === ZeroAddress) {
      throw new Error("Safe address not found");
    }

    await proxyFactory.createProxyWithNonce(
      await masterCopy.getAddress(),
      safeData,
      0n,
    );

    safe = await ethers.getContractAt("SafeL2", safeAddress);

    gatewayConfigMock = await (
      await ethers.getContractFactory("GatewayConfigMock", deployer)
    ).deploy(safeAddress);

    const multiSend = await new ContractFactory(
      MultiSendJson.abi,
      MultiSendJson.bytecode,
      deployer,
    ).deploy();
    multiSendAddress = await multiSend.getAddress();
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
    const enableModuleData = masterCopy.interface.encodeFunctionData(
      "enableModule",
      [adminModule.target],
    );

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
    const data = gatewayConfigMock.interface.encodeFunctionData("setByOwner", [
      42n,
    ]);
    await adminModule
      .connect(charlie)
      .executeSafeTransactions([gatewayConfigMockAddress], [0n], [data], [0n]);
    expect(await gatewayConfigMock.value()).to.equal(42n);
  });

  it("Transfer ownership in a single step", async function () {
    const aliceAddress = await alice.getAddress();
    let owners = await safe.getOwners();
    let threshold = await safe.getThreshold();
    expect(new Set(owners)).to.deep.equal(new Set([aliceAddress]));
    expect(threshold).to.equal(1);

    const chain = await ethers.provider.getNetwork();
    const chainIdKey = chain.chainId.toString();

    const contractNetworks = {
      [chainIdKey]: {
        multiSendAddress,
        multiSendCallOnlyAddress: multiSendAddress,
      },
    };

    const safeKit = await Safe.init({
      provider: network.provider,
      signer: await alice.getAddress(),
      safeAddress,
      contractNetworks,
    });

    const newOwners = Array.from(
      { length: 9 },
      (_, i) => "0x" + String(i + 1).repeat(40),
    );

    const txs = [];

    for (const addr of newOwners) {
      txs.push(await safeKit.createAddOwnerTx({ ownerAddress: addr }));
    }

    const partials = txs.map((t) => t.data);
    const batch = await safeKit.createTransaction({ transactions: partials });
    await safeKit.signTransaction(batch);
    await safeKit.executeTransaction(batch);

    owners = await safe.getOwners();
    expect(new Set(owners)).to.deep.equal(
      new Set([...newOwners, aliceAddress]),
    );
    threshold = await safe.getThreshold();
    expect(threshold).to.equal(1);

    const txs2 = [];
    txs2.push(
      await safeKit.createRemoveOwnerTx({
        ownerAddress: aliceAddress,
        threshold: 6,
      }),
    );
    const partials2 = txs2.map((t) => t.data);
    const batch2 = await safeKit.createTransaction({ transactions: partials2 });
    await safeKit.signTransaction(batch2);
    await safeKit.executeTransaction(batch2);

    owners = await safe.getOwners();
    expect(new Set(owners)).to.deep.equal(new Set(newOwners));
    threshold = await safe.getThreshold();
    expect(threshold).to.equal(6);
  });
});
