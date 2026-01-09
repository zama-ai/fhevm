import { expect } from "chai";
import { Wallet } from "ethers";
import { ethers, upgrades } from "hardhat";

import {
  EmptyUUPSProxyGatewayConfig__factory,
  EmptyUUPSProxy__factory,
  GatewayConfigV2Example__factory,
  GatewayConfig__factory,
  KMSGenerationV2Example__factory,
  KMSGeneration__factory,
  ProtocolPaymentV2Example__factory,
  ProtocolPayment__factory,
} from "../../typechain-types";
import { createAndFundRandomWallet } from "../utils";

describe("Upgrades", function () {
  let owner: Wallet;
  let regularEmptyUUPSFactory: EmptyUUPSProxy__factory;
  let gatewayConfigEmptyUUPSFactory: EmptyUUPSProxyGatewayConfig__factory;
  let gatewayConfigFactoryV1: GatewayConfig__factory;
  let gatewayConfigFactoryV2: GatewayConfigV2Example__factory;
  let kmsGenerationFactoryV1: KMSGeneration__factory;
  let kmsGenerationFactoryV2: KMSGenerationV2Example__factory;
  let protocolPaymentFactoryV1: ProtocolPayment__factory;
  let protocolPaymentFactoryV2: ProtocolPaymentV2Example__factory;

  before(async function () {
    owner = new Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    regularEmptyUUPSFactory = await ethers.getContractFactory("EmptyUUPSProxy", owner);
    gatewayConfigEmptyUUPSFactory = await ethers.getContractFactory("EmptyUUPSProxyGatewayConfig", owner);

    gatewayConfigFactoryV1 = await ethers.getContractFactory("GatewayConfig", owner);
    gatewayConfigFactoryV2 = await ethers.getContractFactory("GatewayConfigV2Example", owner);

    kmsGenerationFactoryV1 = await ethers.getContractFactory("KMSGeneration", owner);
    kmsGenerationFactoryV2 = await ethers.getContractFactory("KMSGenerationV2Example", owner);

    protocolPaymentFactoryV1 = await ethers.getContractFactory("ProtocolPayment", owner);
    protocolPaymentFactoryV2 = await ethers.getContractFactory("ProtocolPaymentV2Example", owner);
  });

  it("Should deploy upgradable GatewayConfig", async function () {
    const emptyUUPS = await upgrades.deployProxy(gatewayConfigEmptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const gatewayConfig = await upgrades.upgradeProxy(emptyUUPS, gatewayConfigFactoryV1);
    await gatewayConfig.waitForDeployment();
    const initialVersion = await gatewayConfig.getVersion();
    const gatewayConfigV2 = await upgrades.upgradeProxy(gatewayConfig, gatewayConfigFactoryV2);
    await gatewayConfigV2.waitForDeployment();
    const newVersion = await gatewayConfigV2.getVersion();
    expect(newVersion).to.not.be.equal(initialVersion);
    expect(newVersion).to.equal("GatewayConfig v1000.0.0");
  });

  it("Should deploy upgradable KMSGeneration", async function () {
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const kmsGeneration = await upgrades.upgradeProxy(emptyUUPS, kmsGenerationFactoryV1);
    await kmsGeneration.waitForDeployment();
    const initialVersion = await kmsGeneration.getVersion();
    const kmsGenerationV2 = await upgrades.upgradeProxy(kmsGeneration, kmsGenerationFactoryV2);
    await kmsGenerationV2.waitForDeployment();
    const newVersion = await kmsGenerationV2.getVersion();
    expect(newVersion).to.not.be.equal(initialVersion);
    expect(newVersion).to.equal("KMSGeneration v1000.0.0");
  });

  it("Should deploy upgradable ProtocolPayment", async function () {
    const emptyUUPS = await upgrades.deployProxy(regularEmptyUUPSFactory, [], {
      initializer: "initialize",
      kind: "uups",
    });
    const protocolPayment = await upgrades.upgradeProxy(emptyUUPS, protocolPaymentFactoryV1);
    await protocolPayment.waitForDeployment();
    const initialVersion = await protocolPayment.getVersion();
    const protocolPaymentV2 = await upgrades.upgradeProxy(protocolPayment, protocolPaymentFactoryV2);
    await protocolPaymentV2.waitForDeployment();
    const newVersion = await protocolPaymentV2.getVersion();
    expect(newVersion).to.not.be.equal(initialVersion);
    expect(newVersion).to.equal("ProtocolPayment v1000.0.0");
  });

  it("Should allow original owner to upgrade the GatewayConfig, transfer ownership and no longer upgrade the contract", async function () {
    // Create a new gateway contract in order to avoid upgrading the original one and thus break
    // some tests if it's not re-compiled in the mean time
    const emptyUUPS = await upgrades.deployProxy(gatewayConfigEmptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const gatewayConfig = await upgrades.upgradeProxy(emptyUUPS, gatewayConfigFactoryV1);
    await gatewayConfig.waitForDeployment();
    const initialVersion = await gatewayConfig.getVersion();

    const newSigner = await createAndFundRandomWallet();
    await gatewayConfig.transferOwnership(newSigner);
    await gatewayConfig.connect(newSigner).acceptOwnership();

    // Old owner should not be able to upgrade the contract
    const gatewayConfigV2ExampleFactoryOldOwner = await ethers.getContractFactory("GatewayConfigV2Example", owner);
    await expect(upgrades.upgradeProxy(gatewayConfig, gatewayConfigV2ExampleFactoryOldOwner)).to.be.reverted;

    // New owner should be able to upgrade the contract
    const gatewayConfigV2ExampleFactoryNewOwner = await ethers.getContractFactory("GatewayConfigV2Example", newSigner);
    const gatewayConfigV2 = await upgrades.upgradeProxy(gatewayConfig, gatewayConfigV2ExampleFactoryNewOwner);

    await gatewayConfigV2.waitForDeployment();
    const newVersion = await gatewayConfigV2.getVersion();
    expect(newVersion).to.not.be.equal(initialVersion);
    expect(newVersion).to.equal("GatewayConfig v1000.0.0");
  });
});
