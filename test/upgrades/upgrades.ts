import { expect } from "chai";
import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import { ethers, upgrades } from "hardhat";

import { createAndFundRandomWallet } from "../utils";

describe("Upgrades", function () {
  before(async function () {
    this.owner = new Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    this.emptyUUPSFactory = await ethers.getContractFactory("EmptyUUPSProxy", this.owner);
    this.multichainAclFactory = await ethers.getContractFactory("MultichainAcl", this.owner);
    this.multichainAclFactoryUpgraded = await ethers.getContractFactory("MultichainAclUpgradedExample", this.owner);
    this.ciphertextCommitsFactory = await ethers.getContractFactory("CiphertextCommits", this.owner);
    this.ciphertextCommitsFactoryUpgraded = await ethers.getContractFactory(
      "CiphertextCommitsUpgradedExample",
      this.owner,
    );
    this.decryptionFactory = await ethers.getContractFactory("Decryption", this.owner);
    this.decryptionFactoryUpgraded = await ethers.getContractFactory("DecryptionUpgradedExample", this.owner);
    this.gatewayConfigFactory = await ethers.getContractFactory("GatewayConfig", this.owner);
    this.gatewayConfigFactoryUpgraded = await ethers.getContractFactory("GatewayConfigUpgradedExample", this.owner);
    this.kmsManagementFactory = await ethers.getContractFactory("KmsManagement", this.owner);
    this.kmsManagementFactoryUpgraded = await ethers.getContractFactory("KmsManagementUpgradedExample", this.owner);
    this.inputVerificationFactory = await ethers.getContractFactory("InputVerification", this.owner);
    this.inputVerificationFactoryUpgraded = await ethers.getContractFactory(
      "InputVerificationUpgradedExample",
      this.owner,
    );
  });

  it("deploy upgradable MultichainAcl", async function () {
    const nonceBef = await ethers.provider.getTransactionCount(this.owner);
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const multichainAcl = await upgrades.upgradeProxy(emptyUUPS, this.multichainAclFactory);
    await multichainAcl.waitForDeployment();
    const ownerBef = await multichainAcl.owner();
    expect(await multichainAcl.getVersion()).to.equal("MultichainAcl v0.1.0");
    const multichainAcl2 = await upgrades.upgradeProxy(multichainAcl, this.multichainAclFactoryUpgraded);
    await multichainAcl2.waitForDeployment();
    const ownerAft = await multichainAcl2.owner();
    expect(ownerBef).to.equal(ownerAft);
    expect(await multichainAcl2.getVersion()).to.equal("MultichainAcl v0.2.0");
    const multichainAclAddress = ethers.getCreateAddress({
      from: this.owner.address,
      nonce: nonceBef, // using nonce of nonceBef instead of nonceBef+1 here, since the original implementation has already been deployer during the setup phase, and hardhat-upgrades plugin is able to detect this and not redeploy twice same contract
    });
    expect(multichainAclAddress).to.equal(await multichainAcl2.getAddress());
  });

  it("deploy upgradable CiphertextCommits", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const ciphertextCommits = await upgrades.upgradeProxy(emptyUUPS, this.ciphertextCommitsFactory);
    await ciphertextCommits.waitForDeployment();
    expect(await ciphertextCommits.getVersion()).to.equal("CiphertextCommits v0.1.0");
    const ciphertextCommits2 = await upgrades.upgradeProxy(ciphertextCommits, this.ciphertextCommitsFactoryUpgraded);
    await ciphertextCommits2.waitForDeployment();
    expect(await ciphertextCommits2.getVersion()).to.equal("CiphertextCommits v0.2.0");
  });

  it("deploy upgradable Decryption", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const decryption = await upgrades.upgradeProxy(emptyUUPS, this.decryptionFactory);
    await decryption.waitForDeployment();
    expect(await decryption.getVersion()).to.equal("Decryption v0.1.0");
    const decryption2 = await upgrades.upgradeProxy(decryption, this.decryptionFactoryUpgraded);
    await decryption2.waitForDeployment();
    expect(await decryption2.getVersion()).to.equal("Decryption v0.2.0");
  });

  it("deploy upgradable GatewayConfig", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const gatewayConfig = await upgrades.upgradeProxy(emptyUUPS, this.gatewayConfigFactory);
    await gatewayConfig.waitForDeployment();
    expect(await gatewayConfig.getVersion()).to.equal("GatewayConfig v0.1.0");
    const gatewayConfig2 = await upgrades.upgradeProxy(gatewayConfig, this.gatewayConfigFactoryUpgraded);
    await gatewayConfig2.waitForDeployment();
    expect(await gatewayConfig2.getVersion()).to.equal("GatewayConfig v0.2.0");
  });

  it("deploy upgradable KmsManagement", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const kmsManagement = await upgrades.upgradeProxy(emptyUUPS, this.kmsManagementFactory);
    await kmsManagement.waitForDeployment();
    expect(await kmsManagement.getVersion()).to.equal("KmsManagement v0.1.0");
    const kmsManagement2 = await upgrades.upgradeProxy(kmsManagement, this.kmsManagementFactoryUpgraded);
    await kmsManagement2.waitForDeployment();
    expect(await kmsManagement2.getVersion()).to.equal("KmsManagement v0.2.0");
  });

  it("deploy upgradable InputVerification", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.owner.address], {
      initializer: "initialize",
      kind: "uups",
    });
    const inputVerification = await upgrades.upgradeProxy(emptyUUPS, this.inputVerificationFactory);
    await inputVerification.waitForDeployment();
    expect(await inputVerification.getVersion()).to.equal("InputVerification v0.1.0");
    const inputVerification2 = await upgrades.upgradeProxy(inputVerification, this.inputVerificationFactoryUpgraded);
    await inputVerification2.waitForDeployment();
    expect(await inputVerification2.getVersion()).to.equal("InputVerification v0.2.0");
  });

  it("original owner upgrades the original GatewayConfig and transfer ownership", async function () {
    const origGatewayConfigAdd = dotenv.parse(fs.readFileSync("addresses/.env.gateway_config")).GATEWAY_CONFIG_ADDRESS;
    const deployer = this.owner;
    const gatewayConfig = await this.gatewayConfigFactory.attach(origGatewayConfigAdd, deployer);
    expect(await gatewayConfig.getVersion()).to.equal("GatewayConfig v0.1.0");

    const newGatewayConfigFactoryUpgraded = await ethers.getContractFactory("GatewayConfigUpgradedExample", deployer);
    const gatewayConfig2 = await upgrades.upgradeProxy(gatewayConfig, newGatewayConfigFactoryUpgraded);
    await gatewayConfig2.waitForDeployment();
    expect(await gatewayConfig2.getVersion()).to.equal("GatewayConfig v0.2.0");
    expect(await gatewayConfig2.getAddress()).to.equal(origGatewayConfigAdd);

    const newSigner = await createAndFundRandomWallet();
    await gatewayConfig2.transferOwnership(newSigner);
    await gatewayConfig2.connect(newSigner).acceptOwnership();

    const newGatewayConfigFactoryUpgraded2 = await ethers.getContractFactory("GatewayConfigUpgradedExample2", deployer);
    await expect(upgrades.upgradeProxy(gatewayConfig2, newGatewayConfigFactoryUpgraded2)).to.be.reverted; // old owner can no longer upgrade ACL

    const newGatewayConfigFactoryUpgraded3 = await ethers.getContractFactory(
      "GatewayConfigUpgradedExample2",
      newSigner,
    );
    const gatewayConfig3 = await upgrades.upgradeProxy(gatewayConfig2, newGatewayConfigFactoryUpgraded3); // new owner can upgrade ACL

    await gatewayConfig3.waitForDeployment();
    expect(await gatewayConfig3.getVersion()).to.equal("GatewayConfig v0.3.0");
  });
});
