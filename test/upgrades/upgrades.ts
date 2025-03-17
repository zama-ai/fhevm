import { expect } from "chai";
import dotenv from "dotenv";
import fs from "fs";
import { ethers, upgrades } from "hardhat";

describe("Upgrades", function () {
  before(async function () {
    this.signers = await ethers.getSigners();
    this.emptyUUPSFactory = await ethers.getContractFactory("EmptyUUPSProxy");
    this.aclManagerFactory = await ethers.getContractFactory("ACLManager");
    this.aclManagerFactoryUpgraded = await ethers.getContractFactory("ACLManagerUpgradedExample");
    this.ciphertextManagerFactory = await ethers.getContractFactory("CiphertextManager");
    this.ciphertextManagerFactoryUpgraded = await ethers.getContractFactory("CiphertextManagerUpgradedExample");
    this.decryptionManagerFactory = await ethers.getContractFactory("DecryptionManager");
    this.decryptionManagerFactoryUpgraded = await ethers.getContractFactory("DecryptionManagerUpgradedExample");
    this.httpzFactory = await ethers.getContractFactory("HTTPZ");
    this.httpzFactoryUpgraded = await ethers.getContractFactory("HTTPZUpgradedExample");
    this.keyManagerFactory = await ethers.getContractFactory("KeyManager");
    this.keyManagerFactoryUpgraded = await ethers.getContractFactory("KeyManagerUpgradedExample");
    this.zkpokManagerFactory = await ethers.getContractFactory("ZKPoKManager");
    this.zkpokManagerFactoryUpgraded = await ethers.getContractFactory("ZKPoKManagerUpgradedExample");
  });

  it("deploy upgradable ACLManager", async function () {
    const nonceBef = await ethers.provider.getTransactionCount(this.signers[0]);
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers[0].address], {
      initializer: "initialize",
      kind: "uups",
    });
    const aclManager = await upgrades.upgradeProxy(emptyUUPS, this.aclManagerFactory);
    await aclManager.waitForDeployment();
    const ownerBef = await aclManager.owner();
    expect(await aclManager.getVersion()).to.equal("ACLManager v0.1.0");
    const aclManager2 = await upgrades.upgradeProxy(aclManager, this.aclManagerFactoryUpgraded);
    await aclManager2.waitForDeployment();
    const ownerAft = await aclManager2.owner();
    expect(ownerBef).to.equal(ownerAft);
    expect(await aclManager2.getVersion()).to.equal("ACLManager v0.2.0");
    const aclManagerAddress = ethers.getCreateAddress({
      from: this.signers[0].address,
      nonce: nonceBef, // using nonce of nonceBef instead of nonceBef+1 here, since the original implementation has already been deployer during the setup phase, and hardhat-upgrades plugin is able to detect this and not redeploy twice same contract
    });
    expect(aclManagerAddress).to.equal(await aclManager2.getAddress());
  });

  it("deploy upgradable CiphertextManager", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers[0].address], {
      initializer: "initialize",
      kind: "uups",
    });
    const ciphertextManager = await upgrades.upgradeProxy(emptyUUPS, this.ciphertextManagerFactory);
    await ciphertextManager.waitForDeployment();
    expect(await ciphertextManager.getVersion()).to.equal("CiphertextManager v0.1.0");
    const ciphertextManager2 = await upgrades.upgradeProxy(ciphertextManager, this.ciphertextManagerFactoryUpgraded);
    await ciphertextManager2.waitForDeployment();
    expect(await ciphertextManager2.getVersion()).to.equal("CiphertextManager v0.2.0");
  });

  it("deploy upgradable DecryptionManager", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers[0].address], {
      initializer: "initialize",
      kind: "uups",
    });
    const decryptionManager = await upgrades.upgradeProxy(emptyUUPS, this.decryptionManagerFactory);
    await decryptionManager.waitForDeployment();
    expect(await decryptionManager.getVersion()).to.equal("DecryptionManager v0.1.0");
    const decryptionManager2 = await upgrades.upgradeProxy(decryptionManager, this.decryptionManagerFactoryUpgraded);
    await decryptionManager2.waitForDeployment();
    expect(await decryptionManager2.getVersion()).to.equal("DecryptionManager v0.2.0");
  });

  it("deploy upgradable HTTPZ", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers[0].address], {
      initializer: "initialize",
      kind: "uups",
    });
    const httpz = await upgrades.upgradeProxy(emptyUUPS, this.httpzFactory);
    await httpz.waitForDeployment();
    expect(await httpz.getVersion()).to.equal("HTTPZ v0.1.0");
    const httpz2 = await upgrades.upgradeProxy(httpz, this.httpzFactoryUpgraded);
    await httpz2.waitForDeployment();
    expect(await httpz2.getVersion()).to.equal("HTTPZ v0.2.0");
  });

  it("deploy upgradable KeyManager", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers[0].address], {
      initializer: "initialize",
      kind: "uups",
    });
    const keyManager = await upgrades.upgradeProxy(emptyUUPS, this.keyManagerFactory);
    await keyManager.waitForDeployment();
    expect(await keyManager.getVersion()).to.equal("KeyManager v0.1.0");
    const keyManager2 = await upgrades.upgradeProxy(keyManager, this.keyManagerFactoryUpgraded);
    await keyManager2.waitForDeployment();
    expect(await keyManager2.getVersion()).to.equal("KeyManager v0.2.0");
  });

  it("deploy upgradable ZKPoKManager", async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers[0].address], {
      initializer: "initialize",
      kind: "uups",
    });
    const zkpokManager = await upgrades.upgradeProxy(emptyUUPS, this.zkpokManagerFactory);
    await zkpokManager.waitForDeployment();
    expect(await zkpokManager.getVersion()).to.equal("ZKPoKManager v0.1.0");
    const zkpokManager2 = await upgrades.upgradeProxy(zkpokManager, this.zkpokManagerFactoryUpgraded);
    await zkpokManager2.waitForDeployment();
    expect(await zkpokManager2.getVersion()).to.equal("ZKPoKManager v0.2.0");
  });

  it("original owner upgrades the original HTTPZ and transfer ownership", async function () {
    const origHTTPZAdd = dotenv.parse(fs.readFileSync("addresses/.env.httpz")).HTTPZ_ADDRESS;
    const deployer = this.signers[0];
    const httpz = await this.httpzFactory.attach(origHTTPZAdd, deployer);
    expect(await httpz.getVersion()).to.equal("HTTPZ v0.1.0");
    const newHttpzFactoryUpgraded = await ethers.getContractFactory("HTTPZUpgradedExample", deployer);
    const httpz2 = await upgrades.upgradeProxy(httpz, newHttpzFactoryUpgraded);
    await httpz2.waitForDeployment();
    expect(await httpz2.getVersion()).to.equal("HTTPZ v0.2.0");
    expect(await httpz2.getAddress()).to.equal(origHTTPZAdd);
    const newSigner = this.signers[1];
    await httpz2.transferOwnership(newSigner);
    await httpz2.connect(newSigner).acceptOwnership();
    const newHttpzFactoryUpgraded2 = await ethers.getContractFactory("HTTPZUpgradedExample2", deployer);
    await expect(upgrades.upgradeProxy(httpz2, newHttpzFactoryUpgraded2)).to.be.reverted; // old owner can no longer upgrade ACL
    const newHttpzFactoryUpgraded3 = await ethers.getContractFactory("HTTPZUpgradedExample2", newSigner);
    const httpz3 = await upgrades.upgradeProxy(httpz2, newHttpzFactoryUpgraded3); // new owner can upgrade ACL
    await httpz3.waitForDeployment();
    expect(await httpz3.getVersion()).to.equal("HTTPZ v0.3.0");
  });
});
