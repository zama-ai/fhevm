import { expect } from 'chai';
import { ethers, upgrades } from 'hardhat';

import { ACL, ACLUpgradedExample } from '../../types';
import { getSigners, initSigners } from '../signers';
import { buildProtocolConfigNodes, buildProtocolConfigThresholds, readHostAddress } from '../tasks/taskHelpers';
import { deployEmptyProxy } from '../utils/deploymentHelpers';

const KEY_COUNTER_BASE = BigInt(4) << BigInt(248);
const CRS_COUNTER_BASE = BigInt(5) << BigInt(248);

describe('Upgrades', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.emptyUUPSFactoryACL = await ethers.getContractFactory('EmptyUUPSProxyACL');
    this.emptyUUPSFactory = await ethers.getContractFactory('EmptyUUPSProxy');
    this.aclFactory = await ethers.getContractFactory('ACL');
    this.aclFactoryUpgraded = await ethers.getContractFactory('ACLUpgradedExample');
  });

  it('deploy upgradeable ACL', async function () {
    const nonceBef = await ethers.provider.getTransactionCount(this.signers.alice);
    const emptyUUPSACL = await deployEmptyProxy(this.emptyUUPSFactoryACL, [this.signers.alice.address]);
    const acl = await upgrades.upgradeProxy(emptyUUPSACL, this.aclFactory, {
      call: { fn: 'initializeFromEmptyProxy' },
    });
    await acl.waitForDeployment();
    const ownerBef = await acl.owner();
    expect(await acl.getVersion()).to.equal('ACL v0.4.0');
    const acl2 = await upgrades.upgradeProxy(acl, this.aclFactoryUpgraded);
    await acl2.waitForDeployment();
    const ownerAft = await acl2.owner();
    expect(ownerBef).to.equal(ownerAft);
    expect(await acl2.getVersion()).to.equal('ACL v0.5.0');
    const aclAddress = ethers.getCreateAddress({
      from: this.signers.alice.address,
      nonce: nonceBef + 1,
    });
    expect(aclAddress).to.equal(await acl2.getAddress());
  });

  it('deploy upgradeable ProtocolConfig', async function () {
    const factory = await ethers.getContractFactory('ProtocolConfig', this.signers.fred);
    const factoryUpgraded = await ethers.getContractFactory('ProtocolConfigUpgradedExample', this.signers.fred);
    const emptyUUPS = await deployEmptyProxy(this.emptyUUPSFactory);
    const pc = await upgrades.upgradeProxy(emptyUUPS, factory, {
      call: { fn: 'initializeFromEmptyProxy', args: [buildProtocolConfigNodes(), buildProtocolConfigThresholds()] },
    });
    await pc.waitForDeployment();
    expect(await pc.getVersion()).to.equal('ProtocolConfig v0.1.0');
    const expectThresholds = async (c: any) => {
      expect(await c.getPublicDecryptionThreshold()).to.equal(1n);
      expect(await c.getUserDecryptionThreshold()).to.equal(2n);
      expect(await c.getKmsGenThreshold()).to.equal(3n);
      expect(await c.getMpcThreshold()).to.equal(4n);
    };
    await expectThresholds(pc);
    const pc2 = await upgrades.upgradeProxy(pc, factoryUpgraded);
    await pc2.waitForDeployment();
    expect(await pc2.getVersion()).to.equal('ProtocolConfig v0.2.0');
    await expectThresholds(pc2);
  });

  it('deploy upgradeable KMSGeneration', async function () {
    const factory = await ethers.getContractFactory('KMSGeneration', this.signers.fred);
    const factoryUpgraded = await ethers.getContractFactory('KMSGenerationUpgradedExample', this.signers.fred);
    const emptyUUPS = await deployEmptyProxy(this.emptyUUPSFactory);
    const kg = await upgrades.upgradeProxy(emptyUUPS, factory, {
      call: { fn: 'initializeFromEmptyProxy' },
    });
    await kg.waitForDeployment();
    expect(await kg.getVersion()).to.equal('KMSGeneration v0.1.0');
    const expectInitialState = async (c: any) => {
      expect(await c.getActiveKeyId()).to.equal(0n);
      expect(await c.getActiveCrsId()).to.equal(0n);
      expect(await c.getKeyCounter()).to.equal(KEY_COUNTER_BASE);
      expect(await c.getCrsCounter()).to.equal(CRS_COUNTER_BASE);
    };
    await expectInitialState(kg);
    const kg2 = await upgrades.upgradeProxy(kg, factoryUpgraded);
    await kg2.waitForDeployment();
    expect(await kg2.getVersion()).to.equal('KMSGeneration v0.2.0');
    await expectInitialState(kg2);
  });

  it('deploy upgradeable KMSVerifier', async function () {
    const kmsFactory = await ethers.getContractFactory('contracts/KMSVerifier.sol:KMSVerifier', this.signers.fred);
    const kmsFactoryUpgraded = await ethers.getContractFactory('KMSVerifierUpgradedExample', this.signers.fred); // because account[5] is set in `.env to be owner of ACL/Host
    const emptyUUPS = await deployEmptyProxy(this.emptyUUPSFactory);
    const verifyingContractSource = process.env.DECRYPTION_ADDRESS!;
    const chainIDSource = +process.env.CHAIN_ID_GATEWAY!;
    const kms = await upgrades.upgradeProxy(emptyUUPS, kmsFactory, {
      call: {
        fn: 'initializeFromEmptyProxy',
        args: [verifyingContractSource, chainIDSource],
      },
      unsafeAllow: ['missing-initializer'],
    });
    await kms.waitForDeployment();
    expect(await kms.getVersion()).to.equal('KMSVerifier v0.3.0');

    const domain = Array.from(await kms.eip712Domain());
    expect(domain.slice(1, 5)).to.deep.equal(['Decryption', '1', BigInt(chainIDSource), verifyingContractSource]);

    const kmsAddress = await kms.getAddress();
    const kms2 = await upgrades.upgradeProxy(kms, kmsFactoryUpgraded);
    await kms2.waitForDeployment();
    expect(await kms2.getAddress()).to.equal(kmsAddress);
    expect(await kms2.getVersion()).to.equal('KMSVerifier v0.4.0');
    expect(Array.from(await kms2.eip712Domain())).to.deep.equal(domain);
  });

  it('deploy upgradeable FHEVMExecutor', async function () {
    const executorFactory = await ethers.getContractFactory(
      'contracts/FHEVMExecutor.sol:FHEVMExecutor',
      this.signers.fred,
    );
    const executorFactoryUpgraded = await ethers.getContractFactory('FHEVMExecutorUpgradedExample', this.signers.fred); // because account[5] is set in `.env to be owner of ACL/Host
    const emptyUUPS = await deployEmptyProxy(this.emptyUUPSFactory);
    const executor = await upgrades.upgradeProxy(emptyUUPS, executorFactory, {
      call: { fn: 'initializeFromEmptyProxy' },
    });
    await executor.waitForDeployment();
    expect(await executor.getVersion()).to.equal('FHEVMExecutor v0.4.0');
    const executor2 = await upgrades.upgradeProxy(executor, executorFactoryUpgraded);
    await executor2.waitForDeployment();
    expect(await executor2.getVersion()).to.equal('FHEVMExecutor v0.5.0');
  });

  it('deploy upgradeable HCULimit', async function () {
    const paymentFactory = await ethers.getContractFactory('HCULimit', this.signers.fred); // because account[5] is set in `.env to be owner of ACL/Host
    const paymentFactoryUpgraded = await ethers.getContractFactory('HCULimitUpgradedExample', this.signers.fred);
    const emptyUUPS = await deployEmptyProxy(this.emptyUUPSFactory);
    const payment = await upgrades.upgradeProxy(emptyUUPS, paymentFactory, {
      call: {
        fn: 'initializeFromEmptyProxy',
        args: [BigInt('281474976710655'), BigInt('5000000'), BigInt('20000000')],
      },
    });
    await payment.waitForDeployment();
    expect(await payment.getVersion()).to.equal('HCULimit v0.3.0');
    const payment2 = await upgrades.upgradeProxy(payment, paymentFactoryUpgraded);
    await payment2.waitForDeployment();
    expect(await payment2.getVersion()).to.equal('HCULimit v0.4.0');
  });

  it('original owner upgrades the original ACL and transfer ownership', async function () {
    const origACLAdd = readHostAddress('ACL_CONTRACT_ADDRESS');
    const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    const acl = (await this.aclFactory.attach(origACLAdd, deployer)) as ACL;
    expect(await acl.getVersion()).to.equal('ACL v0.4.0');
    const newaclFactoryUpgraded = await ethers.getContractFactory('ACLUpgradedExample', deployer);
    const acl2 = (await upgrades.upgradeProxy(acl, newaclFactoryUpgraded)) as unknown as ACLUpgradedExample;
    await acl2.waitForDeployment();
    expect(await acl2.getVersion()).to.equal('ACL v0.5.0');
    expect(await acl2.getAddress()).to.equal(origACLAdd);
    const newSigner = (await ethers.getSigners())[1];
    await acl2.transferOwnership(newSigner);
    await acl2.connect(newSigner).acceptOwnership();
    const newaclFactoryUpgraded2 = await ethers.getContractFactory('ACLUpgradedExample2', deployer);
    await expect(upgrades.upgradeProxy(acl2, newaclFactoryUpgraded2)).to.be.reverted; // old owner can no longer upgrade ACL
    const newaclFactoryUpgraded3 = await ethers.getContractFactory('ACLUpgradedExample2', newSigner);
    const acl3 = await upgrades.upgradeProxy(acl2, newaclFactoryUpgraded3); // new owner can upgrade ACL
    await acl3.waitForDeployment();
    expect(await acl3.getVersion()).to.equal('ACL v0.5.0');
  });
});
