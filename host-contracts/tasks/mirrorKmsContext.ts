import type { Interface } from 'ethers';
import { task, types } from 'hardhat/config';
import type { ConfigurableTaskDefinition, HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfigReplica } from '../types';
import { broadcast, getProtocolConfigReplicaInterface, requireProtocolConfigAddress } from './kmsContext';
import { type CanonicalSnapshot, readCanonicalSnapshot } from './protocolConfigMirror';

// This file defines tasks that mirror the canonical (Ethereum) ProtocolConfig's active KMS context
// and epoch onto a non-canonical replica (e.g. Polygon) after a rotation has gone active on
// canonical. It follows the same build-calldata (DAO path, never broadcasts) / broadcast (no-DAO
// path for devnet / test-suite) convention as kmsContext.ts, calling the replica's
// `mirrorKmsContextAndEpoch` (context switch) or `mirrorKmsEpoch` (same-set epoch rotation).

// -----------------------------------------------------------------------------------------
// mirrorKmsContextAndEpoch: mirrors a context switch (new signer set) plus its first epoch.
// -----------------------------------------------------------------------------------------

export function encodeMirrorKmsContextAndEpoch(iface: Interface, snapshot: CanonicalSnapshot): string {
  return iface.encodeFunctionData('mirrorKmsContextAndEpoch', [
    snapshot.currentKmsContextId,
    snapshot.currentEpochId,
    snapshot.kmsNodes,
    snapshot.thresholds,
  ]);
}

// Rejects a stale context switch. The mirror methods have no ordering check, so stale calldata moves
// the replica back. A DAO proposal can execute later, so check the replica again before it executes.
export async function assertReplicaNeedsContextSwitch(
  hre: HardhatRuntimeEnvironment,
  replicaProtocolConfigAddress: string,
  canonicalContextId: bigint,
): Promise<void> {
  const replica = (await hre.ethers.getContractAt(
    'ProtocolConfigReplica',
    replicaProtocolConfigAddress,
  )) as unknown as ProtocolConfigReplica;
  const [replicaContextId] = await replica.getCurrentKmsContextAndEpoch();
  if (canonicalContextId <= replicaContextId) {
    throw new Error(
      `Replica ${replicaProtocolConfigAddress} is already at context ${replicaContextId}, which is >= canonical's ${canonicalContextId}. `,
    );
  }
}

// All four mirror tasks share the same CLI surface.
function addMirrorTaskParams(definition: ConfigurableTaskDefinition, readTarget: string): ConfigurableTaskDefinition {
  return definition
    .addParam(
      'canonicalRpcUrl',
      'RPC URL of the canonical host chain (Ethereum) to read ProtocolConfig from.',
      undefined,
      types.string,
    )
    .addParam(
      'canonicalProtocolConfigAddress',
      'Address of the ProtocolConfig contract on the canonical host chain.',
      undefined,
      types.string,
    )
    .addOptionalParam(
      'blockNumber',
      `Canonical block height to read ${readTarget} from; must already be finalized. Defaults to the latest finalized block.`,
      undefined,
      types.int,
    )
    .addOptionalParam(
      'useInternalProxyAddress',
      'Resolve the ProtocolConfigReplica address from the /addresses directory instead of the environment',
      false,
      types.boolean,
    );
}

type MirrorTaskArgs = {
  canonicalRpcUrl: string;
  canonicalProtocolConfigAddress: string;
  blockNumber?: number;
  useInternalProxyAddress: boolean;
};

// Reads canonical's active state and resolves the replica target, shared by all four mirror tasks.
async function loadMirrorInputs(
  hre: HardhatRuntimeEnvironment,
  { canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress }: MirrorTaskArgs,
): Promise<{ iface: Interface; snapshot: CanonicalSnapshot; target: string }> {
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = new hre.ethers.JsonRpcProvider(canonicalRpcUrl, undefined, { cacheTimeout: -1 });
  const snapshot = await readCanonicalSnapshot(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  return { iface, snapshot, target };
}

addMirrorTaskParams(
  task(
    'task:buildMirrorKmsContextAndEpochCalldata',
    "Builds Aragon proposal calldata for ProtocolConfigReplica.mirrorKmsContextAndEpoch from canonical's active KMS context (DAO path, never broadcasts)",
  ),
  'the context switch',
).setAction(async function (args: MirrorTaskArgs, hre): Promise<void> {
  const { iface, snapshot, target } = await loadMirrorInputs(hre, args);
  await assertReplicaNeedsContextSwitch(hre, target, snapshot.currentKmsContextId);
  const calldata = encodeMirrorKmsContextAndEpoch(iface, snapshot);

  console.log('ProtocolConfigReplica.mirrorKmsContextAndEpoch');
  console.log('  contextId:', snapshot.currentKmsContextId.toString());
  console.log('  epochId:', snapshot.currentEpochId.toString());
  console.log('  kmsNodes:', snapshot.kmsNodes.length);
  console.log('  target:', target);
  console.log('  calldata:', calldata);
});

addMirrorTaskParams(
  task(
    'task:mirrorKmsContextAndEpoch',
    "Broadcasts ProtocolConfigReplica.mirrorKmsContextAndEpoch from canonical's active KMS context with the deployer key",
  ),
  'the context switch',
).setAction(async function (args: MirrorTaskArgs, hre): Promise<void> {
  const { iface, snapshot, target } = await loadMirrorInputs(hre, args);
  await assertReplicaNeedsContextSwitch(hre, target, snapshot.currentKmsContextId);
  const calldata = encodeMirrorKmsContextAndEpoch(iface, snapshot);
  const hash = await broadcast(hre, target, calldata);
  console.log(
    `Broadcast mirrorKmsContextAndEpoch(context=${snapshot.currentKmsContextId}, epoch=${snapshot.currentEpochId}) on ${target} (tx: ${hash}).`,
  );
});

// -----------------------------------------------------------------------------------------
// mirrorKmsEpoch: mirrors a same-set epoch rotation (no signer-set change) under the context
// already active on the replica.
// -----------------------------------------------------------------------------------------

export function encodeMirrorKmsEpoch(iface: Interface, contextId: bigint, epochId: bigint): string {
  return iface.encodeFunctionData('mirrorKmsEpoch', [contextId, epochId]);
}

export async function assertReplicaNeedsEpochMirror(
  hre: HardhatRuntimeEnvironment,
  replicaProtocolConfigAddress: string,
  canonicalContextId: bigint,
  canonicalEpochId: bigint,
): Promise<void> {
  const replica = (await hre.ethers.getContractAt(
    'ProtocolConfigReplica',
    replicaProtocolConfigAddress,
  )) as unknown as ProtocolConfigReplica;
  const [replicaContextId, replicaEpochId] = await replica.getCurrentKmsContextAndEpoch();
  if (canonicalContextId !== replicaContextId) {
    throw new Error(
      `Replica ${replicaProtocolConfigAddress} is at context ${replicaContextId}, but canonical's active context is ${canonicalContextId}. ` +
        `Run task:mirrorKmsContextAndEpoch first to mirror the context switch.`,
    );
  }
  if (canonicalEpochId <= replicaEpochId) {
    throw new Error(
      `Replica ${replicaProtocolConfigAddress} is already at epoch ${replicaEpochId}, which is >= canonical's ${canonicalEpochId}. Nothing to mirror.`,
    );
  }
}

addMirrorTaskParams(
  task(
    'task:buildMirrorKmsEpochCalldata',
    "Builds Aragon proposal calldata for ProtocolConfigReplica.mirrorKmsEpoch from canonical's active KMS epoch (DAO path, never broadcasts)",
  ),
  'the active epoch',
).setAction(async function (args: MirrorTaskArgs, hre): Promise<void> {
  const { iface, snapshot, target } = await loadMirrorInputs(hre, args);
  await assertReplicaNeedsEpochMirror(hre, target, snapshot.currentKmsContextId, snapshot.currentEpochId);
  const calldata = encodeMirrorKmsEpoch(iface, snapshot.currentKmsContextId, snapshot.currentEpochId);

  console.log('ProtocolConfigReplica.mirrorKmsEpoch');
  console.log('  contextId:', snapshot.currentKmsContextId.toString());
  console.log('  epochId:', snapshot.currentEpochId.toString());
  console.log('  target:', target);
  console.log('  calldata:', calldata);
});

addMirrorTaskParams(
  task(
    'task:mirrorKmsEpoch',
    "Broadcasts ProtocolConfigReplica.mirrorKmsEpoch from canonical's active KMS epoch with the deployer key",
  ),
  'the active epoch',
).setAction(async function (args: MirrorTaskArgs, hre): Promise<void> {
  const { iface, snapshot, target } = await loadMirrorInputs(hre, args);
  await assertReplicaNeedsEpochMirror(hre, target, snapshot.currentKmsContextId, snapshot.currentEpochId);
  const calldata = encodeMirrorKmsEpoch(iface, snapshot.currentKmsContextId, snapshot.currentEpochId);
  const hash = await broadcast(hre, target, calldata);
  console.log(
    `Broadcast mirrorKmsEpoch(context=${snapshot.currentKmsContextId}, epoch=${snapshot.currentEpochId}) on ${target} (tx: ${hash}).`,
  );
});
