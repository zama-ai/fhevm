import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import { ethers, upgrades } from "hardhat";

import {
  CiphertextCommitsV2Example__factory,
  CiphertextCommits__factory,
  DecryptionV2Example__factory,
  Decryption__factory,
  EmptyUUPSProxy__factory,
  GatewayConfigV2Example__factory,
  GatewayConfigV3Example__factory,
  GatewayConfig__factory,
  InputVerificationV2Example__factory,
  InputVerification__factory,
  KmsManagementV2Example__factory,
  KmsManagement__factory,
  MultichainAclV2Example__factory,
  MultichainAcl__factory,
} from "../../typechain-types";
import { createAndFundRandomWallet, loadTestVariablesFixture, toValues } from "../utils";

describe("Upgrades", function () {
  let owner: Wallet;
  let emptyUUPSFactory: EmptyUUPSProxy__factory;
  let ciphertextCommitsFactoryV1: CiphertextCommits__factory;
  let ciphertextCommitsFactoryV2: CiphertextCommitsV2Example__factory;
  let decryptionFactoryV1: Decryption__factory;
  let decryptionFactoryV2: DecryptionV2Example__factory;
  let gatewayConfigFactoryV1: GatewayConfig__factory;
  let gatewayConfigFactoryV2: GatewayConfigV2Example__factory;
  let gatewayConfigFactoryV3: GatewayConfigV3Example__factory;
  let inputVerificationFactoryV1: InputVerification__factory;
  let inputVerificationFactoryV2: InputVerificationV2Example__factory;
  let kmsManagementFactoryV1: KmsManagement__factory;
  let kmsManagementFactoryV2: KmsManagementV2Example__factory;
  let multichainAclFactoryV1: MultichainAcl__factory;
  let multichainAclFactoryV2: MultichainAclV2Example__factory;

  before(async function () {
    owner = new Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    emptyUUPSFactory = await ethers.getContractFactory("EmptyUUPSProxy", owner);

    ciphertextCommitsFactoryV1 = await ethers.getContractFactory("CiphertextCommits", owner);
    ciphertextCommitsFactoryV2 = await ethers.getContractFactory("CiphertextCommitsV2Example", owner);

    decryptionFactoryV1 = await ethers.getContractFactory("Decryption", owner);
    decryptionFactoryV2 = await ethers.getContractFactory("DecryptionV2Example", owner);

    gatewayConfigFactoryV1 = await ethers.getContractFactory("GatewayConfig", owner);
    gatewayConfigFactoryV2 = await ethers.getContractFactory("GatewayConfigV2Example", owner);
    gatewayConfigFactoryV3 = await ethers.getContractFactory("GatewayConfigV3Example", owner);

    inputVerificationFactoryV1 = await ethers.getContractFactory("InputVerification", owner);
    inputVerificationFactoryV2 = await ethers.getContractFactory("InputVerificationV2Example", owner);

    kmsManagementFactoryV1 = await ethers.getContractFactory("KmsManagement", owner);
    kmsManagementFactoryV2 = await ethers.getContractFactory("KmsManagementV2Example", owner);

    multichainAclFactoryV1 = await ethers.getContractFactory("MultichainAcl", owner);
    multichainAclFactoryV2 = await ethers.getContractFactory("MultichainAclV2Example", owner);
  });

  it("Should deploy upgradable MultichainAcl", async function () {
    const nonceBef = await ethers.provider.getTransactionCount(owner);
    const emptyUUPS = await upgrades.deployProxy(emptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const multichainAcl = await upgrades.upgradeProxy(emptyUUPS, multichainAclFactoryV1);
    await multichainAcl.waitForDeployment();
    const ownerBef = await multichainAcl.owner();
    expect(await multichainAcl.getVersion()).to.equal("MultichainAcl v0.1.0");
    const multichainAcl2 = await upgrades.upgradeProxy(multichainAcl, multichainAclFactoryV2);
    await multichainAcl2.waitForDeployment();
    const ownerAft = await multichainAcl2.owner();
    expect(ownerBef).to.equal(ownerAft);
    expect(await multichainAcl2.getVersion()).to.equal("MultichainAcl v0.2.0");
    const multichainAclAddress = ethers.getCreateAddress({
      from: owner.address,
      nonce: nonceBef, // using nonce of nonceBef instead of nonceBef+1 here, since the original implementation has already been deployer during the setup phase, and hardhat-upgrades plugin is able to detect this and not redeploy twice same contract
    });
    expect(multichainAclAddress).to.equal(await multichainAcl2.getAddress());
  });

  it("Should deploy upgradable CiphertextCommits", async function () {
    const emptyUUPS = await upgrades.deployProxy(emptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const ciphertextCommits = await upgrades.upgradeProxy(emptyUUPS, ciphertextCommitsFactoryV1);
    await ciphertextCommits.waitForDeployment();
    expect(await ciphertextCommits.getVersion()).to.equal("CiphertextCommits v0.1.0");
    const ciphertextCommits2 = await upgrades.upgradeProxy(ciphertextCommits, ciphertextCommitsFactoryV2);
    await ciphertextCommits2.waitForDeployment();
    expect(await ciphertextCommits2.getVersion()).to.equal("CiphertextCommits v0.2.0");
  });

  it("Should deploy upgradable Decryption", async function () {
    const emptyUUPS = await upgrades.deployProxy(emptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const decryption = await upgrades.upgradeProxy(emptyUUPS, decryptionFactoryV1);
    await decryption.waitForDeployment();
    expect(await decryption.getVersion()).to.equal("Decryption v0.1.0");
    const decryption2 = await upgrades.upgradeProxy(decryption, decryptionFactoryV2);
    await decryption2.waitForDeployment();
    expect(await decryption2.getVersion()).to.equal("Decryption v0.2.0");
  });

  it("Should deploy upgradable GatewayConfig", async function () {
    const emptyUUPS = await upgrades.deployProxy(emptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const gatewayConfig = await upgrades.upgradeProxy(emptyUUPS, gatewayConfigFactoryV1);
    await gatewayConfig.waitForDeployment();
    expect(await gatewayConfig.getVersion()).to.equal("GatewayConfig v0.1.0");
    const gatewayConfig2 = await upgrades.upgradeProxy(gatewayConfig, gatewayConfigFactoryV2);
    await gatewayConfig2.waitForDeployment();
    expect(await gatewayConfig2.getVersion()).to.equal("GatewayConfig v0.2.0");
  });

  it("Should deploy upgradable KmsManagement", async function () {
    const emptyUUPS = await upgrades.deployProxy(emptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const kmsManagement = await upgrades.upgradeProxy(emptyUUPS, kmsManagementFactoryV1);
    await kmsManagement.waitForDeployment();
    expect(await kmsManagement.getVersion()).to.equal("KmsManagement v0.1.0");
    const kmsManagement2 = await upgrades.upgradeProxy(kmsManagement, kmsManagementFactoryV2);
    await kmsManagement2.waitForDeployment();
    expect(await kmsManagement2.getVersion()).to.equal("KmsManagement v0.2.0");
  });

  it("Should deploy upgradable InputVerification", async function () {
    const emptyUUPS = await upgrades.deployProxy(emptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const inputVerification = await upgrades.upgradeProxy(emptyUUPS, inputVerificationFactoryV1);
    await inputVerification.waitForDeployment();
    expect(await inputVerification.getVersion()).to.equal("InputVerification v0.1.0");
    const inputVerification2 = await upgrades.upgradeProxy(inputVerification, inputVerificationFactoryV2);
    await inputVerification2.waitForDeployment();
    expect(await inputVerification2.getVersion()).to.equal("InputVerification v0.2.0");
  });

  it("Should allow original owner to upgrade the original GatewayConfig and transfer ownership", async function () {
    // Create a new gateway contract in order to avoid upgrading the original one and thus break
    // some tests if it's not re-compiled in the mean time
    const emptyUUPS = await upgrades.deployProxy(emptyUUPSFactory, [owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const originalGatewayConfig = await upgrades.upgradeProxy(emptyUUPS, gatewayConfigFactoryV1);
    await originalGatewayConfig.waitForDeployment();
    expect(await originalGatewayConfig.getVersion()).to.equal("GatewayConfig v0.1.0");

    const originalGatewayConfigAddress = await originalGatewayConfig.getAddress();
    const deployer = owner;

    const newGatewayConfigFactoryUpgraded = await ethers.getContractFactory("GatewayConfigV2Example", deployer);
    const gatewayConfig2 = await upgrades.upgradeProxy(originalGatewayConfig, newGatewayConfigFactoryUpgraded);
    await gatewayConfig2.waitForDeployment();
    expect(await gatewayConfig2.getVersion()).to.equal("GatewayConfig v0.2.0");
    expect(await gatewayConfig2.getAddress()).to.equal(originalGatewayConfigAddress);

    const newSigner = await createAndFundRandomWallet();
    await gatewayConfig2.transferOwnership(newSigner);
    await gatewayConfig2.connect(newSigner).acceptOwnership();

    const newGatewayConfigFactoryUpgraded2 = await ethers.getContractFactory("GatewayConfigV3Example", deployer);
    await expect(upgrades.upgradeProxy(gatewayConfig2, newGatewayConfigFactoryUpgraded2)).to.be.reverted; // old owner can no longer upgrade ACL

    const newGatewayConfigFactoryUpgraded3 = await ethers.getContractFactory("GatewayConfigV3Example", newSigner);
    const gatewayConfig3 = await upgrades.upgradeProxy(gatewayConfig2, newGatewayConfigFactoryUpgraded3); // new owner can upgrade ACL

    await gatewayConfig3.waitForDeployment();
    expect(await gatewayConfig3.getVersion()).to.equal("GatewayConfig v0.3.0");
  });

  it("Should maintain state consistency after upgrade", async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { gatewayConfig } = fixtureData;

    // Check that GatewayConfig is at version 0.1.0
    expect(await gatewayConfig.getVersion()).to.equal("GatewayConfig v0.1.0");

    // Check that the protocol metadata is correct
    const metadata = await gatewayConfig.getProtocolMetadata();
    const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
    expect(metadata).to.deep.equal(toValues(protocolMetadata));

    // Upgrade the GatewayConfig contract to version 0.3.0
    const protocolNewField = "Protocol new field";
    const gatewayConfigV2 = await upgrades.upgradeProxy(gatewayConfig, gatewayConfigFactoryV3, {
      call: { fn: "initialize", args: [protocolNewField] },
    });
    await gatewayConfigV2.waitForDeployment();
    expect(await gatewayConfigV2.getVersion()).to.equal("GatewayConfig v0.3.0");
    expect(await gatewayConfig.getAddress()).to.equal(await gatewayConfigV2.getAddress());

    // Check that the protocol metadata is still correct after the upgrade
    const protocolMetadataV2 = { name: "Protocol", website: "https://protocol.com", newField: protocolNewField };
    const metadataAfterUpgrade = await gatewayConfigV2.getProtocolMetadata();
    expect(metadataAfterUpgrade).to.deep.equal(toValues(protocolMetadataV2));
  });
});
