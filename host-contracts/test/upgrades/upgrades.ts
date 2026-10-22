import { expect } from 'chai';
import { ethers, upgrades } from 'hardhat';

import { CRS_COUNTER_BASE, KEY_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { findEventArg } from '../../tasks/utils/protocolConfigEip712';
import { ACL, ACLUpgradedExample, ProtocolConfig } from '../../types';
import type { KmsThresholdsStruct } from '../../types/contracts/ProtocolConfig';
import { getSigners, initSigners } from '../signers';
import {
  buildConnectorUrls,
  buildControllableKmsCommittee,
  buildProtocolConfigNodes,
  buildProtocolConfigThresholds,
  confirmEpochActivationBySigners,
  readHostAddress,
} from '../tasks/taskHelpers';
import { deployEmptyProxy } from '../utils/deploymentHelpers';

describe('Upgrades', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.emptyUUPSFactoryACL = await ethers.getContractFactory('EmptyUUPSProxyACL');
    this.emptyUUPSFactory = await ethers.getContractFactory('EmptyUUPSProxy');
    this.aclFactory = await ethers.getContractFactory('ACL');
    this.aclFactoryUpgraded = await ethers.getContractFactory('ACLUpgradedExample');
  });

  const expectThresholds = async (c: any) => {
    expect(await c.getPublicDecryptionThreshold()).to.equal(1n);
    expect(await c.getUserDecryptionThreshold()).to.equal(2n);
    expect(await c.getKmsGenThreshold()).to.equal(3n);
    expect(await c.getMpcThreshold()).to.equal(4n);
  };

  it('deploy upgradeable ACL', async function () {
    const nonceBef = await ethers.provider.getTransactionCount(this.signers.alice);
    const emptyUUPSACL = await deployEmptyProxy(this.emptyUUPSFactoryACL, [this.signers.alice.address]);
    const acl = await upgrades.upgradeProxy(emptyUUPSACL, this.aclFactory, {
      call: { fn: 'initializeFromEmptyProxy' },
    });
    await acl.waitForDeployment();
    const ownerBef = await acl.owner();
    expect(await acl.getVersion()).to.equal('ACL v0.6.0');
    const acl2 = await upgrades.upgradeProxy(acl, this.aclFactoryUpgraded);
    await acl2.waitForDeployment();
    const ownerAft = await acl2.owner();
    expect(ownerBef).to.equal(ownerAft);
    expect(await acl2.getVersion()).to.equal('ACL v0.7.0');
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
      call: {
        fn: 'initializeFromEmptyProxy',
        args: [
          buildProtocolConfigNodes(),
          buildConnectorUrls(buildProtocolConfigNodes().length),
          buildProtocolConfigThresholds(),
          '',
          [],
        ],
      },
    });
    await pc.waitForDeployment();
    expect(await pc.getVersion()).to.equal('ProtocolConfig v0.4.0');
    await expectThresholds(pc);
    const pc2 = await upgrades.upgradeProxy(pc, factoryUpgraded);
    await pc2.waitForDeployment();
    expect(await pc2.getVersion()).to.equal('ProtocolConfig v0.5.0');
    await expectThresholds(pc2);
  });

  // Deploys a proxy with the v0.15 ProtocolConfig storage layout, at initialized version 4.
  async function deployV015ProtocolConfig(
    fred: any,
    emptyUUPSFactory: any,
    nodes = buildProtocolConfigNodes(),
    thresholds: KmsThresholdsStruct = buildProtocolConfigThresholds(),
  ) {
    const factory = await ethers.getContractFactory('ProtocolConfigV015LayoutExample', fred);
    const pc = await upgrades.upgradeProxy(await deployEmptyProxy(emptyUUPSFactory), factory, {
      call: { fn: 'initializeFromEmptyProxy', args: [nodes, thresholds] },
    });
    await pc.waitForDeployment();
    return pc;
  }

  const v015ContextId = KMS_CONTEXT_COUNTER_BASE + 1n;

  const toKmsNode = (n: ReturnType<typeof buildProtocolConfigNodes>[number], connectorUrl: string) => ({
    txSenderAddress: n.txSenderAddress,
    signerAddress: n.signerAddress,
    ipAddress: n.ipAddress,
    storageUrl: n.storageUrl,
    connectorUrl,
  });

  const emptyKmsNode = {
    txSenderAddress: ethers.ZeroAddress,
    signerAddress: ethers.ZeroAddress,
    ipAddress: '',
    storageUrl: '',
    connectorUrl: '',
  };

  const expectKmsNodes = async (c: any, contextId: bigint, expected: ReturnType<typeof toKmsNode>[]) => {
    const nodes = await c.getKmsNodesForContext(contextId);
    expect(nodes.map((n: any) => n.toObject())).to.deep.equal(expected);
    for (const node of expected) {
      expect((await c.getKmsNodeForContext(contextId, node.txSenderAddress)).toObject()).to.deep.equal(node);
    }
  };

  const expectV015State = async (c: any) => {
    const nodes = buildProtocolConfigNodes();
    await expectKmsNodes(
      c,
      v015ContextId,
      nodes.map((n) => toKmsNode(n, '')),
    );
    expect(await c.getKmsSignersForContext(v015ContextId)).to.deep.equal(nodes.map((n) => n.signerAddress));
    expect(await c.getKmsSigners()).to.deep.equal(nodes.map((n) => n.signerAddress));
    await expectThresholds(c);
  };

  it('upgrades a v0.15 ProtocolConfig proxy in place to ProtocolConfigReplica', async function () {
    const pc = await deployV015ProtocolConfig(this.signers.fred, this.emptyUUPSFactory);
    const replicaFactory = await ethers.getContractFactory('ProtocolConfigReplica', this.signers.fred);
    const { chainId } = await ethers.provider.getNetwork();
    const canonicalProtocolConfig = '0x' + '11'.repeat(20);
    const replica = await upgrades.upgradeProxy(pc, replicaFactory, {
      call: { fn: 'reinitializeV4', args: [chainId, canonicalProtocolConfig] },
    });
    await replica.waitForDeployment();
    expect(await replica.getVersion()).to.equal('ProtocolConfigReplica v0.4.0');
    const domain = Array.from(await replica.eip712Domain());
    expect(domain.slice(1, 5)).to.deep.equal(['ProtocolConfig', '1', chainId, canonicalProtocolConfig]);
    const [contextId, epochId] = await replica.getCurrentKmsContextAndEpoch();
    expect(contextId).to.equal(v015ContextId);
    expect(await replica.isValidEpochForContext(contextId, epochId)).to.be.true;
    await expectV015State(replica);

    await (await replica.mirrorKmsEpoch(contextId, epochId + 1n)).wait();
    expect(await replica.getCurrentKmsContextAndEpoch()).to.deep.equal([contextId, epochId + 1n]);

    const oldNodes = buildProtocolConfigNodes();
    const newNodes = oldNodes.map((n, i) =>
      toKmsNode(
        {
          ...n,
          txSenderAddress: ethers.getAddress(ethers.zeroPadValue(ethers.toBeHex(0xa000 + i), 20)),
          signerAddress: ethers.getAddress(ethers.zeroPadValue(ethers.toBeHex(0xb000 + i), 20)),
        },
        buildConnectorUrls(oldNodes.length)[i],
      ),
    );
    await (
      await replica.recoverKmsState(contextId, epochId + 2n, newNodes, buildProtocolConfigThresholds(), [], [])
    ).wait();
    await expectKmsNodes(replica, contextId, newNodes);
    expect(await replica.getKmsSigners()).to.deep.equal(newNodes.map((n) => n.signerAddress));
    for (const n of oldNodes) {
      expect((await replica.getKmsNodeForContext(contextId, n.txSenderAddress)).toObject()).to.deep.equal(emptyKmsNode);
      expect(await replica.isKmsTxSenderForContext(contextId, n.txSenderAddress)).to.be.false;
      expect(await replica.isKmsSignerForContext(contextId, n.signerAddress)).to.be.false;
    }
  });

  it('upgrades a v0.15 ProtocolConfig proxy in place to ProtocolConfig', async function () {
    const committee = await buildControllableKmsCommittee();
    const pc = await deployV015ProtocolConfig(
      this.signers.fred,
      this.emptyUUPSFactory,
      committee.nodes,
      committee.thresholds,
    );
    const factory = await ethers.getContractFactory('ProtocolConfig', this.signers.fred);
    const canonical = (await upgrades.upgradeProxy(pc, factory, {
      call: { fn: 'reinitializeV4' },
    })) as unknown as ProtocolConfig;
    await canonical.waitForDeployment();
    expect(await canonical.getVersion()).to.equal('ProtocolConfig v0.4.0');
    const [contextId, epochId] = await canonical.getCurrentKmsContextAndEpoch();
    expect(await canonical.isValidEpochForContext(contextId, epochId)).to.be.true;
    const v015Nodes = committee.nodes.map((n) => toKmsNode(n, ''));
    await expectKmsNodes(canonical, contextId, v015Nodes);
    expect(await canonical.getKmsSigners()).to.deep.equal(committee.nodes.map((n) => n.signerAddress));
    expect(await canonical.getPublicDecryptionThreshold()).to.equal(BigInt(committee.thresholds.publicDecryption));
    expect(await canonical.getUserDecryptionThreshold()).to.equal(BigInt(committee.thresholds.userDecryption));
    expect(await canonical.getKmsGenThreshold()).to.equal(BigInt(committee.thresholds.kmsGen));
    expect(await canonical.getMpcThreshold()).to.equal(BigInt(committee.thresholds.mpc));

    // The new confirm paths read the committee that v0.15 stored, so a rotation completes after the upgrade.
    const receipt = await (await canonical.defineNewEpochForCurrentKmsContext()).wait();
    const newEpochId = findEventArg(canonical, receipt!.logs, 'NewKmsEpoch', 'epochId')!;
    const canonicalAddress = await canonical.getAddress();
    await confirmEpochActivationBySigners(canonicalAddress, committee.signerSigners, contextId, newEpochId);
    expect(await canonical.getCurrentKmsContextAndEpoch()).to.deep.equal([contextId, newEpochId]);
    expect(await canonical.isValidEpochForContext(contextId, newEpochId)).to.be.true;

    // A context defined after the upgrade stores its connector URLs. The v0.15 context keeps empty ones.
    const newNodeParams = buildProtocolConfigNodes();
    const connectorUrls = buildConnectorUrls(newNodeParams.length);
    await (
      await canonical.defineNewKmsContextAndEpoch(newNodeParams, connectorUrls, buildProtocolConfigThresholds(), '', [])
    ).wait();
    for (const [i, n] of newNodeParams.entries()) {
      expect((await canonical.getKmsNodeForContext(contextId + 1n, n.txSenderAddress)).toObject()).to.deep.equal(
        toKmsNode(n, connectorUrls[i]),
      );
    }
    await expectKmsNodes(canonical, contextId, v015Nodes);
  });

  it('deploy upgradeable KMSGeneration', async function () {
    const factory = await ethers.getContractFactory('KMSGeneration', this.signers.fred);
    const factoryUpgraded = await ethers.getContractFactory('KMSGenerationUpgradedExample', this.signers.fred);
    const emptyUUPS = await deployEmptyProxy(this.emptyUUPSFactory);
    const kg = await upgrades.upgradeProxy(emptyUUPS, factory, {
      call: { fn: 'initializeFromEmptyProxy' },
    });
    await kg.waitForDeployment();
    expect(await kg.getVersion()).to.equal('KMSGeneration v0.3.0');
    const expectInitialState = async (c: any) => {
      expect(await c.getActiveKeyId()).to.equal(0n);
      expect(await c.getActiveCrsId()).to.equal(0n);
      expect(await c.getKeyCounter()).to.equal(KEY_COUNTER_BASE);
      expect(await c.getCrsCounter()).to.equal(CRS_COUNTER_BASE);
    };
    await expectInitialState(kg);
    const kg2 = await upgrades.upgradeProxy(kg, factoryUpgraded);
    await kg2.waitForDeployment();
    expect(await kg2.getVersion()).to.equal('KMSGeneration v0.4.0');
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
    expect(await kms.getVersion()).to.equal('KMSVerifier v0.5.0');

    const domain = Array.from(await kms.eip712Domain());
    expect(domain.slice(1, 5)).to.deep.equal(['Decryption', '1', BigInt(chainIDSource), verifyingContractSource]);

    const kmsAddress = await kms.getAddress();
    const kms2 = await upgrades.upgradeProxy(kms, kmsFactoryUpgraded);
    await kms2.waitForDeployment();
    expect(await kms2.getAddress()).to.equal(kmsAddress);
    expect(await kms2.getVersion()).to.equal('KMSVerifier v0.5.0');
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
    expect(await executor.getVersion()).to.equal('FHEVMExecutor v0.7.0');
    const executor2 = await upgrades.upgradeProxy(executor, executorFactoryUpgraded);
    await executor2.waitForDeployment();
    expect(await executor2.getVersion()).to.equal('FHEVMExecutor v0.8.0');
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
    expect(await payment.getVersion()).to.equal('HCULimit v0.4.0');
    const payment2 = await upgrades.upgradeProxy(payment, paymentFactoryUpgraded);
    await payment2.waitForDeployment();
    expect(await payment2.getVersion()).to.equal('HCULimit v0.5.0');
  });

  it('original owner upgrades the original ACL and transfer ownership', async function () {
    const origACLAdd = readHostAddress('ACL_CONTRACT_ADDRESS');
    const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    const acl = (await this.aclFactory.attach(origACLAdd, deployer)) as ACL;
    expect(await acl.getVersion()).to.equal('ACL v0.6.0');
    const newaclFactoryUpgraded = await ethers.getContractFactory('ACLUpgradedExample', deployer);
    const acl2 = (await upgrades.upgradeProxy(acl, newaclFactoryUpgraded)) as unknown as ACLUpgradedExample;
    await acl2.waitForDeployment();
    expect(await acl2.getVersion()).to.equal('ACL v0.7.0');
    expect(await acl2.getAddress()).to.equal(origACLAdd);
    const newSigner = (await ethers.getSigners())[1];
    await acl2.transferOwnership(newSigner);
    await acl2.connect(newSigner).acceptOwnership();
    const newaclFactoryUpgraded2 = await ethers.getContractFactory('ACLUpgradedExample2', deployer);
    await expect(upgrades.upgradeProxy(acl2, newaclFactoryUpgraded2)).to.be.reverted; // old owner can no longer upgrade ACL
    const newaclFactoryUpgraded3 = await ethers.getContractFactory('ACLUpgradedExample2', newSigner);
    const acl3 = await upgrades.upgradeProxy(acl2, newaclFactoryUpgraded3); // new owner can upgrade ACL
    await acl3.waitForDeployment();
    expect(await acl3.getVersion()).to.equal('ACL v0.8.0');
  });
});
