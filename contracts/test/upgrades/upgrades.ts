import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers, upgrades } from 'hardhat';

import { ACL, ACLUpgradedExample } from '../../types';
import { getSigners, initSigners } from '../signers';

describe('Upgrades', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.emptyUUPSFactory = await ethers.getContractFactory('EmptyUUPSProxy');
    this.aclFactory = await ethers.getContractFactory('ACL');
    this.aclFactoryUpgraded = await ethers.getContractFactory('ACLUpgradedExample');
    this.kmsFactory = await ethers.getContractFactory('KMSVerifier');
    this.kmsFactoryUpgraded = await ethers.getContractFactory('KMSVerifierUpgradedExample');
    this.executorFactory = await ethers.getContractFactory('contracts/HTTPZExecutor.sol:HTTPZExecutor');
    this.executorFactoryUpgraded = await ethers.getContractFactory('HTTPZExecutorUpgradedExample');
    this.paymentFactory = await ethers.getContractFactory('FHEGasLimit');
    this.paymentFactoryUpgraded = await ethers.getContractFactory('FHEGasLimitUpgradedExample');
    this.decryptionOracleFactory = await ethers.getContractFactory(
      'decryptionOracle/DecryptionOracle.sol:DecryptionOracle',
    );
    this.decryptionOracleFactoryUpgraded = await ethers.getContractFactory('DecryptionOracleUpgradedExample');
  });

  it('deploy upgradable ACL', async function () {
    const nonceBef = await ethers.provider.getTransactionCount(this.signers.alice);
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers.alice.address], {
      initializer: 'initialize',
      kind: 'uups',
    });
    const acl = await upgrades.upgradeProxy(emptyUUPS, this.aclFactory);
    await acl.waitForDeployment();
    const ownerBef = await acl.owner();
    expect(await acl.getVersion()).to.equal('ACL v0.1.0');
    const acl2 = await upgrades.upgradeProxy(acl, this.aclFactoryUpgraded);
    await acl2.waitForDeployment();
    const ownerAft = await acl2.owner();
    expect(ownerBef).to.equal(ownerAft);
    expect(await acl2.getVersion()).to.equal('ACL v0.2.0');
    const aclAddress = ethers.getCreateAddress({
      from: this.signers.alice.address,
      nonce: nonceBef, // using nonce of nonceBef instead of nonceBef+1 here, since the original implementation has already been deployer during the setup phase, and hardhat-upgrades plugin is able to detect this and not redeploy twice same contract
    });
    expect(aclAddress).to.equal(await acl2.getAddress());
  });

  it('deploy upgradable KMSVerifier', async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers.alice.address], {
      initializer: 'initialize',
      kind: 'uups',
    });
    const kms = await upgrades.upgradeProxy(emptyUUPS, this.kmsFactory);
    await kms.waitForDeployment();
    expect(await kms.getVersion()).to.equal('KMSVerifier v0.1.0');
    const kms2 = await upgrades.upgradeProxy(kms, this.kmsFactoryUpgraded);
    await kms2.waitForDeployment();
    expect(await kms2.getVersion()).to.equal('KMSVerifier v0.2.0');
  });

  it('deploy upgradable HTTPZExecutor', async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers.alice.address], {
      initializer: 'initialize',
      kind: 'uups',
    });
    const executor = await upgrades.upgradeProxy(emptyUUPS, this.executorFactory);
    await executor.waitForDeployment();
    expect(await executor.getVersion()).to.equal('HTTPZExecutor v0.1.0');
    const executor2 = await upgrades.upgradeProxy(executor, this.executorFactoryUpgraded);
    await executor2.waitForDeployment();
    expect(await executor2.getVersion()).to.equal('HTTPZExecutor v0.2.0');
  });

  it('deploy upgradable FHEGasLimit', async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers.alice.address], {
      initializer: 'initialize',
      kind: 'uups',
    });
    const payment = await upgrades.upgradeProxy(emptyUUPS, this.paymentFactory);
    await payment.waitForDeployment();
    expect(await payment.getVersion()).to.equal('FHEGasLimit v0.1.0');
    const payment2 = await upgrades.upgradeProxy(payment, this.paymentFactoryUpgraded);
    await payment2.waitForDeployment();
    expect(await payment2.getVersion()).to.equal('FHEGasLimit v0.2.0');
  });

  it('deploy upgradable DecryptionOracle', async function () {
    const emptyUUPS = await upgrades.deployProxy(this.emptyUUPSFactory, [this.signers.alice.address], {
      initializer: 'initialize',
      kind: 'uups',
    });
    const decryptionOracle = await upgrades.upgradeProxy(emptyUUPS, this.decryptionOracleFactory);
    await decryptionOracle.waitForDeployment();
    expect(await decryptionOracle.getVersion()).to.equal('DecryptionOracle v0.1.0');
    const decryptionOracle2 = await upgrades.upgradeProxy(decryptionOracle, this.decryptionOracleFactoryUpgraded);
    await decryptionOracle2.waitForDeployment();
    expect(await decryptionOracle2.getVersion()).to.equal('DecryptionOracle v0.2.0');
  });

  it('original owner upgrades the original ACL and transfer ownership', async function () {
    const origACLAdd = dotenv.parse(fs.readFileSync('addresses/.env.acl')).ACL_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    const acl = (await this.aclFactory.attach(origACLAdd, deployer)) as ACL;
    expect(await acl.getVersion()).to.equal('ACL v0.1.0');
    const newaclFactoryUpgraded = await ethers.getContractFactory('ACLUpgradedExample', deployer);
    const acl2 = (await upgrades.upgradeProxy(acl, newaclFactoryUpgraded)) as unknown as ACLUpgradedExample;
    await acl2.waitForDeployment();
    expect(await acl2.getVersion()).to.equal('ACL v0.2.0');
    expect(await acl2.getAddress()).to.equal(origACLAdd);
    const newSigner = (await ethers.getSigners())[1];
    await acl2.transferOwnership(newSigner);
    await acl2.connect(newSigner).acceptOwnership();
    const newaclFactoryUpgraded2 = await ethers.getContractFactory('ACLUpgradedExample2', deployer);
    await expect(upgrades.upgradeProxy(acl2, newaclFactoryUpgraded2)).to.be.reverted; // old owner can no longer upgrade ACL
    const newaclFactoryUpgraded3 = await ethers.getContractFactory('ACLUpgradedExample2', newSigner);
    const acl3 = await upgrades.upgradeProxy(acl2, newaclFactoryUpgraded3); // new owner can upgrade ACL
    await acl3.waitForDeployment();
    expect(await acl3.getVersion()).to.equal('ACL v0.3.0');
  });
});
