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
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { ProtocolConfig } from '../../types';
import {
  buildControllableKmsCommittee,
  buildProtocolConfigNodes,
  buildProtocolConfigThresholds,
  buildSingleKeyAndCrsActivationPayload,
  deployFreshProtocolConfigProxy,
  deployFreshUninitializedProtocolConfigProxy,
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

      const iface = await getProtocolConfigInterface(hre);
      const args = await readCanonicalContextSwitch(
        hre,
        { canonicalProvider: ethers.provider, canonicalProtocolConfigAddress: canonicalAddress },
        iface,
      );

      expect(args.contextId).to.equal(liveContextId);
      expect(args.epochId).to.equal(liveEpochId);
      expect(args.kmsNodeParams.length).to.equal(committee.nodes.length);
      committee.nodes.forEach((node, i) => {
        expect(args.kmsNodeParams[i].txSenderAddress).to.equal(node.txSenderAddress);
        expect(args.kmsNodeParams[i].signerAddress).to.equal(node.signerAddress);
        expect(args.kmsNodeParams[i].mpcIdentity).to.equal(node.mpcIdentity);
      });
      expect(args.thresholds.mpc).to.equal(BigInt(committee.thresholds.mpc));
    });

    it('mirrors live thresholds rather than the thresholds in the NewKmsContext event', async function () {
      const committee = await buildControllableKmsCommittee();
      const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
      const contextId = await rotateToNewKmsContext(canonicalAddress, deployer, committee);

      // updateMpcThresholdForContext mutates live state without touching the NewKmsContext anchor, so
      // a naive event-only read would mirror the stale value defined at context-creation time.
      const asOwner = (await ethers.getContractAt(
        'ProtocolConfig',
        canonicalAddress,
        deployer,
      )) as unknown as ProtocolConfig;
      await (await asOwner.updateMpcThresholdForContext(contextId, 2)).wait();

      const iface = await getProtocolConfigInterface(hre);
      const args = await readCanonicalContextSwitch(
        hre,
        { canonicalProvider: ethers.provider, canonicalProtocolConfigAddress: canonicalAddress },
        iface,
      );

      expect(args.thresholds.mpc).to.equal(2n);
      expect(args.thresholds.mpc).to.not.equal(BigInt(committee.thresholds.mpc));
    });

    it('rejects an address with no context anchor as non-canonical', async function () {
      // `initializeFromCanonical` (the replica bootstrap path) never writes a context anchor, so a
      // replica passed as canonical must be rejected before any event scan. It requires ids matching
      // a canonical genesis, so read them from a real canonical.
      const canonicalAddress = await deployFreshProtocolConfigProxy(
        deployer,
        buildProtocolConfigNodes(),
        buildProtocolConfigThresholds(),
      );
      const canonical = (await ethers.getContractAt('ProtocolConfig', canonicalAddress)) as unknown as ProtocolConfig;
      const [canonicalContextId, canonicalEpochId] = await canonical.getCurrentKmsContextAndEpoch();

      const replicaAddress = await deployFreshUninitializedProtocolConfigProxy(deployer);
      const replica = (await ethers.getContractAt(
        'ProtocolConfig',
        replicaAddress,
        deployer,
      )) as unknown as ProtocolConfig;
      await (
        await replica.initializeFromCanonical(
          canonicalContextId,
          canonicalEpochId,
          buildProtocolConfigNodes(),
          buildProtocolConfigThresholds(),
        )
      ).wait();

      const iface = await getProtocolConfigInterface(hre);
      await expect(
        readCanonicalContextSwitch(
          hre,
          { canonicalProvider: ethers.provider, canonicalProtocolConfigAddress: replicaAddress },
          iface,
        ),
      ).to.be.rejectedWith(/has no context anchor recorded/);
    });
  });

  describe('encodeMirrorKmsContextAndEpoch', function () {
    it('builds calldata that decodes back to the recovered args', async function () {
      const committee = await buildControllableKmsCommittee();
      const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
      await rotateToNewKmsContext(canonicalAddress, deployer, committee);

      const iface = await getProtocolConfigInterface(hre);
      const args = await readCanonicalContextSwitch(
        hre,
        { canonicalProvider: ethers.provider, canonicalProtocolConfigAddress: canonicalAddress },
        iface,
      );
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
      await rotateToNewKmsContext(canonicalAddress, deployer, committee);

      // The replica is a wholly separate deployment, starting at its own genesis context (id 1), so
      // canonical's rotated context (id 2) is strictly ahead of it. This is the context-switch case.
      const replicaAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);

      const iface = await getProtocolConfigInterface(hre);
      const switchArgs = await readCanonicalContextSwitch(
        hre,
        { canonicalProvider: ethers.provider, canonicalProtocolConfigAddress: canonicalAddress },
        iface,
      );
      await assertReplicaNeedsContextSwitch(hre, replicaAddress, switchArgs.contextId);
      const contextCalldata = encodeMirrorKmsContextAndEpoch(iface, switchArgs);

      await (await deployer.sendTransaction({ to: replicaAddress, data: contextCalldata })).wait();

      const replica = (await ethers.getContractAt('ProtocolConfig', replicaAddress)) as unknown as ProtocolConfig;
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
