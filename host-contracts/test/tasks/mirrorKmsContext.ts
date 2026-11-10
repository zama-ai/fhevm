import { expect } from 'chai';
import hre, { ethers } from 'hardhat';

import { getProtocolConfigReplicaInterface } from '../../tasks/kmsContext';
import {
  assertReplicaNeedsContextSwitch,
  assertReplicaNeedsEpochMirror,
  encodeMirrorKmsContextAndEpoch,
  encodeMirrorKmsEpoch,
} from '../../tasks/mirrorKmsContext';
import { readCanonicalSnapshot } from '../../tasks/protocolConfigMirror';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { ProtocolConfig, ProtocolConfigReplica } from '../../types';
import { confirmEpochActivation } from '../utils/protocolConfigEip712';
import {
  buildConnectorUrls,
  buildControllableKmsCommittee,
  deployFreshProtocolConfigProxy,
  deployFreshProtocolConfigReplicaProxy,
  rotateToNewKmsContext,
} from './taskHelpers';

// These tests drive the mirroring helpers directly: `ethers.provider` stands in for the canonical
// RPC provider. The CLI layer itself (`--canonical-rpc-url` constructing a real JsonRpcProvider) is
// untested.
describe('KMS mirror tasks', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);

  describe('encodeMirrorKmsContextAndEpoch', function () {
    it('builds calldata that decodes back to the canonical snapshot', async function () {
      const committee = await buildControllableKmsCommittee();
      const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
      await rotateToNewKmsContext(canonicalAddress, deployer, committee);

      const iface = await getProtocolConfigReplicaInterface(hre);
      const snapshot = await readCanonicalSnapshot(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: canonicalAddress,
      });
      const calldata = encodeMirrorKmsContextAndEpoch(iface, snapshot);
      const decoded = iface.decodeFunctionData('mirrorKmsContextAndEpoch', calldata);

      expect(decoded[0]).to.equal(snapshot.currentKmsContextId);
      expect(decoded[1]).to.equal(snapshot.currentEpochId);
      expect(decoded[2].length).to.equal(snapshot.kmsNodes.length);
      decoded[2].forEach((node: unknown[], i: number) => {
        expect(node).to.deep.equal([
          snapshot.kmsNodes[i].txSenderAddress,
          snapshot.kmsNodes[i].signerAddress,
          snapshot.kmsNodes[i].ipAddress,
          snapshot.kmsNodes[i].storageUrl,
          snapshot.kmsNodes[i].connectorUrl,
        ]);
      });
      expect(decoded[3].toObject()).to.deep.equal(snapshot.thresholds);
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

      const iface = await getProtocolConfigReplicaInterface(hre);
      const snapshot = await readCanonicalSnapshot(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: canonicalAddress,
      });
      await assertReplicaNeedsContextSwitch(hre, replicaAddress, snapshot.currentKmsContextId);
      const contextCalldata = encodeMirrorKmsContextAndEpoch(iface, snapshot);

      await (await deployer.sendTransaction({ to: replicaAddress, data: contextCalldata })).wait();

      const replica = (await ethers.getContractAt(
        'ProtocolConfigReplica',
        replicaAddress,
      )) as unknown as ProtocolConfigReplica;
      const [mirroredContextId, mirroredEpochId] = await replica.getCurrentKmsContextAndEpoch();
      expect(mirroredContextId).to.equal(snapshot.currentKmsContextId);
      expect(mirroredEpochId).to.equal(snapshot.currentEpochId);
      const mirroredNodes = await replica.getKmsNodesForContext(snapshot.currentKmsContextId);
      expect(mirroredNodes.map((n) => n.connectorUrl)).to.deep.equal(buildConnectorUrls(committee.nodes.length));

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
      await confirmEpochActivation(asCanonicalOwner, committee.signerSigners, snapshot.currentKmsContextId, newEpochId);

      const [, canonicalEpochIdAfterRotation] = await asCanonicalOwner.getCurrentKmsContextAndEpoch();
      expect(canonicalEpochIdAfterRotation).to.equal(newEpochId);

      await assertReplicaNeedsEpochMirror(hre, replicaAddress, snapshot.currentKmsContextId, newEpochId);
      const epochCalldata = encodeMirrorKmsEpoch(iface, snapshot.currentKmsContextId, newEpochId);
      await (await deployer.sendTransaction({ to: replicaAddress, data: epochCalldata })).wait();

      const [finalContextId, finalEpochId] = await replica.getCurrentKmsContextAndEpoch();
      expect(finalContextId).to.equal(snapshot.currentKmsContextId);
      expect(finalEpochId).to.equal(newEpochId);
    });
  });
});
