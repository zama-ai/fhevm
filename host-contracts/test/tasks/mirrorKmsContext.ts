import { expect } from 'chai';
import hre, { ethers } from 'hardhat';

import { getProtocolConfigInterface } from '../../tasks/kmsContext';
import {
  assertReplicaNeedsContextSwitch,
  assertReplicaNeedsEpochMirror,
  encodeMirrorKmsContextAndEpoch,
  encodeMirrorKmsEpoch,
  readCanonicalContextSwitch,
} from '../../tasks/mirrorKmsContext';
import { EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { ProtocolConfig, ProtocolConfigReplica } from '../../types';
import {
  buildControllableKmsCommittee,
  buildProtocolConfigNodes,
  buildProtocolConfigThresholds,
  buildSingleKeyAndCrsActivationPayload,
  deployFreshProtocolConfigProxy,
  deployFreshProtocolConfigReplicaProxy,
  rotateToNewKmsContext,
} from './taskHelpers';

// These tests drive the mirroring helpers directly: `ethers.provider` stands in for the canonical
// RPC provider. The CLI layer itself (`--canonical-rpc-url` constructing a real JsonRpcProvider) is
// untested.
describe('KMS mirror tasks', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);

  describe('readCanonicalContextSwitch', function () {
    it('recovers the rotated context from the NewKmsContext event and matches live ids', async function () {
      const committee = await buildControllableKmsCommittee();
      const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
      const contextId = await rotateToNewKmsContext(canonicalAddress, deployer, committee);

      const canonical = (await ethers.getContractAt('ProtocolConfig', canonicalAddress)) as unknown as ProtocolConfig;
      const [liveContextId, liveEpochId] = await canonical.getCurrentKmsContextAndEpoch();
      expect(liveContextId).to.equal(contextId);

      const args = await readCanonicalContextSwitch(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: canonicalAddress,
      });

      expect(args.contextId).to.equal(liveContextId);
      expect(args.epochId).to.equal(liveEpochId);
      expect(args.kmsNodeParams.length).to.equal(committee.nodes.length);
      committee.nodes.forEach((node, i) => {
        expect(args.kmsNodeParams[i].txSenderAddress).to.equal(node.txSenderAddress);
        expect(args.kmsNodeParams[i].signerAddress).to.equal(node.signerAddress);
        expect(args.kmsNodeParams[i].mpcIdentity).to.equal(node.mpcIdentity);
      });
      expect(args.thresholds.publicDecryption).to.equal(BigInt(committee.thresholds.publicDecryption));
      expect(args.thresholds.userDecryption).to.equal(BigInt(committee.thresholds.userDecryption));
      expect(args.thresholds.kmsGen).to.equal(BigInt(committee.thresholds.kmsGen));
      expect(args.thresholds.mpc).to.equal(BigInt(committee.thresholds.mpc));
    });

    it('rejects a replica, then the same proxy upgraded back to ProtocolConfig with no context anchor', async function () {
      const replicaAddress = await deployFreshProtocolConfigReplicaProxy(
        deployer,
        KMS_CONTEXT_COUNTER_BASE + 1n,
        EPOCH_COUNTER_BASE + 1n,
        buildProtocolConfigNodes(),
        buildProtocolConfigThresholds(),
      );
      const read = () =>
        readCanonicalContextSwitch(hre, {
          canonicalProvider: ethers.provider,
          canonicalProtocolConfigAddress: replicaAddress,
        });

      // The version identity check rejects a replica before any event scan.
      await expect(read()).to.be.rejectedWith(/reports version "ProtocolConfigReplica v/);

      // Upgraded back to ProtocolConfig, the proxy passes the identity check but holds no anchor.
      const implementation = await (await ethers.getContractFactory('ProtocolConfig', deployer)).deploy();
      const replica = await ethers.getContractAt('ProtocolConfigReplica', replicaAddress, deployer);
      await (await replica.upgradeToAndCall(await implementation.getAddress(), '0x')).wait();
      await expect(read()).to.be.rejectedWith(/has no context anchor recorded/);
    });
  });

  describe('encodeMirrorKmsContextAndEpoch', function () {
    it('builds calldata that decodes back to the recovered args', async function () {
      const committee = await buildControllableKmsCommittee();
      const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
      await rotateToNewKmsContext(canonicalAddress, deployer, committee);

      const iface = await getProtocolConfigInterface(hre, 'ProtocolConfigReplica');
      const args = await readCanonicalContextSwitch(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: canonicalAddress,
      });
      const calldata = encodeMirrorKmsContextAndEpoch(iface, args);
      const decoded = iface.decodeFunctionData('mirrorKmsContextAndEpoch', calldata);

      expect(decoded[0]).to.equal(args.contextId);
      expect(decoded[1]).to.equal(args.epochId);
      expect(decoded[2].length).to.equal(args.kmsNodeParams.length);
    });
  });

  describe('replica readiness guards', function () {
    it('assertReplicaNeedsContextSwitch throws once the replica is already at the target context', async function () {
      const nodes = (await buildControllableKmsCommittee()).nodes;
      const thresholds = { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 };
      const replicaAddress = await deployFreshProtocolConfigProxy(deployer, nodes, thresholds);
      const replica = (await ethers.getContractAt('ProtocolConfig', replicaAddress)) as unknown as ProtocolConfig;
      const [replicaContextId] = await replica.getCurrentKmsContextAndEpoch();

      await expect(assertReplicaNeedsContextSwitch(hre, replicaAddress, replicaContextId)).to.be.rejectedWith(
        /is already at context/,
      );
      await expect(assertReplicaNeedsContextSwitch(hre, replicaAddress, replicaContextId + 1n)).to.not.be.rejected;
    });

    it('assertReplicaNeedsEpochMirror throws on a context mismatch and on a non-increasing epoch', async function () {
      const nodes = (await buildControllableKmsCommittee()).nodes;
      const thresholds = { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 };
      const replicaAddress = await deployFreshProtocolConfigProxy(deployer, nodes, thresholds);
      const replica = (await ethers.getContractAt('ProtocolConfig', replicaAddress)) as unknown as ProtocolConfig;
      const [replicaContextId, replicaEpochId] = await replica.getCurrentKmsContextAndEpoch();

      await expect(
        assertReplicaNeedsEpochMirror(hre, replicaAddress, replicaContextId + 1n, replicaEpochId + 1n),
      ).to.be.rejectedWith(/but canonical's active context is/);
      await expect(
        assertReplicaNeedsEpochMirror(hre, replicaAddress, replicaContextId, replicaEpochId),
      ).to.be.rejectedWith(/Nothing to mirror/);
      await expect(assertReplicaNeedsEpochMirror(hre, replicaAddress, replicaContextId, replicaEpochId + 1n)).to.not.be
        .rejected;
    });
  });

  describe('end-to-end mirror onto a replica', function () {
    it('mirrors a context switch, then a same-set epoch rotation, onto an independent replica', async function () {
      const committee = await buildControllableKmsCommittee();
      const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
      const canonical = (await ethers.getContractAt('ProtocolConfig', canonicalAddress)) as unknown as ProtocolConfig;
      const [genesisContextId, genesisEpochId] = await canonical.getCurrentKmsContextAndEpoch();
      await rotateToNewKmsContext(canonicalAddress, deployer, committee);

      // The replica is a wholly separate deployment, starting at canonical's genesis context (id 1), so
      // canonical's rotated context (id 2) is strictly ahead of it. This is the context-switch case.
      const replicaAddress = await deployFreshProtocolConfigReplicaProxy(
        deployer,
        genesisContextId,
        genesisEpochId,
        committee.nodes,
        committee.thresholds,
      );

      const iface = await getProtocolConfigInterface(hre, 'ProtocolConfigReplica');
      const switchArgs = await readCanonicalContextSwitch(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: canonicalAddress,
      });
      await assertReplicaNeedsContextSwitch(hre, replicaAddress, switchArgs.contextId);
      const contextCalldata = encodeMirrorKmsContextAndEpoch(iface, switchArgs);

      await (await deployer.sendTransaction({ to: replicaAddress, data: contextCalldata })).wait();

      const replica = (await ethers.getContractAt(
        'ProtocolConfigReplica',
        replicaAddress,
      )) as unknown as ProtocolConfigReplica;
      const [mirroredContextId, mirroredEpochId] = await replica.getCurrentKmsContextAndEpoch();
      expect(mirroredContextId).to.equal(switchArgs.contextId);
      expect(mirroredEpochId).to.equal(switchArgs.epochId);

      // Now drive a same-set rotation on canonical and mirror just the epoch.
      const asCanonicalOwner = (await ethers.getContractAt(
        'ProtocolConfig',
        canonicalAddress,
        deployer,
      )) as unknown as ProtocolConfig;
      const rotateReceipt = await (await asCanonicalOwner.defineNewEpochForCurrentKmsContext()).wait();
      const [newEpochEvent] = await asCanonicalOwner.queryFilter(
        asCanonicalOwner.filters.NewKmsEpoch(),
        rotateReceipt!.blockNumber,
        rotateReceipt!.blockNumber,
      );
      const newEpochId = newEpochEvent.args.epochId;
      for (let i = 0; i < committee.txSenderSigners.length; i++) {
        const asTxSender = (await ethers.getContractAt(
          'ProtocolConfig',
          canonicalAddress,
          committee.txSenderSigners[i],
        )) as unknown as ProtocolConfig;
        const { keys, crsList } = await buildSingleKeyAndCrsActivationPayload(
          committee.signerSigners[i],
          canonicalAddress,
          switchArgs.contextId,
          newEpochId,
        );
        await (await asTxSender.confirmEpochActivation(newEpochId, keys, crsList)).wait();
      }

      const [, canonicalEpochIdAfterRotation] = await asCanonicalOwner.getCurrentKmsContextAndEpoch();
      expect(canonicalEpochIdAfterRotation).to.equal(newEpochId);

      await assertReplicaNeedsEpochMirror(hre, replicaAddress, switchArgs.contextId, newEpochId);
      const epochCalldata = encodeMirrorKmsEpoch(iface, switchArgs.contextId, newEpochId);
      await (await deployer.sendTransaction({ to: replicaAddress, data: epochCalldata })).wait();

      const [finalContextId, finalEpochId] = await replica.getCurrentKmsContextAndEpoch();
      expect(finalContextId).to.equal(switchArgs.contextId);
      expect(finalEpochId).to.equal(newEpochId);
    });
  });
});
