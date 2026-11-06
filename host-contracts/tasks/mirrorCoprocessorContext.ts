import type { Interface, Provider } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { type Coprocessor, assertCanonicalSnapshotBlockHash, readCanonicalSnapshot } from './protocolConfigMirror';
import { COPROCESSOR_CONTEXT_COUNTER_BASE } from './utils/kmsGenerationConstants';
import {
  addMirrorTaskParams,
  broadcast,
  getCanonicalProvider,
  getProtocolConfig,
  getProtocolConfigReplica,
  getProtocolConfigReplicaInterface,
  requireProtocolConfigAddress,
} from './utils/protocolConfigTasks';

// This file defines tasks that mirror the canonical (Ethereum) ProtocolConfig's coprocessor contexts
// onto a non-canonical replica (e.g. Polygon). It follows the same build-calldata (DAO path, never
// broadcasts) / broadcast (no-DAO path for devnet / test-suite) convention as mirrorKmsContext.ts,
// calling the replica's `mirrorCoprocessorContext` or `mirrorCoprocessorContextDestruction`.

// -----------------------------------------------------------------------------------------
// mirrorCoprocessorContext: mirrors canonical's current coprocessor context. Only the current one
// is mirrored: the replica accepts gaps, so contexts defined and superseded on canonical between
// two mirrors are skipped.
// -----------------------------------------------------------------------------------------

export interface MirrorCoprocessorContextArgs {
  contextId: bigint;
  coprocessors: Coprocessor[];
  majorityThreshold: bigint;
}

export async function readCanonicalCoprocessorContext(
  hre: HardhatRuntimeEnvironment,
  options: { canonicalProvider: Provider; canonicalProtocolConfigAddress: string; blockNumber?: number },
): Promise<MirrorCoprocessorContextArgs> {
  const snapshot = await readCanonicalSnapshot(hre, options);
  return {
    contextId: snapshot.currentCoprocessorContextId,
    coprocessors: snapshot.coprocessors,
    majorityThreshold: snapshot.coprocessorMajorityThreshold,
  };
}

export function encodeMirrorCoprocessorContext(iface: Interface, args: MirrorCoprocessorContextArgs): string {
  return iface.encodeFunctionData('mirrorCoprocessorContext', [
    args.contextId,
    args.coprocessors,
    args.majorityThreshold,
  ]);
}

// Fails with an explicit message instead of the replica's NonIncreasingCoprocessorContextId revert.
export async function assertReplicaNeedsCoprocessorContextMirror(
  hre: HardhatRuntimeEnvironment,
  replicaProtocolConfigAddress: string,
  canonicalCoprocessorContextId: bigint,
): Promise<void> {
  const replica = await getProtocolConfigReplica(hre, replicaProtocolConfigAddress);
  const replicaContextId = await replica.getCurrentCoprocessorContextId();
  if (canonicalCoprocessorContextId <= replicaContextId) {
    throw new Error(
      `Replica ${replicaProtocolConfigAddress} is already at coprocessor context ${replicaContextId}, which is >= canonical's ${canonicalCoprocessorContextId}. Nothing to mirror.`,
    );
  }
}

addMirrorTaskParams(
  task(
    'task:buildMirrorCoprocessorContextCalldata',
    "Builds Aragon proposal calldata for ProtocolConfigReplica.mirrorCoprocessorContext from canonical's current coprocessor context (DAO path, never broadcasts)",
  ),
  'the coprocessor context',
).setAction(async function (
  { canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const args = await readCanonicalCoprocessorContext(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsCoprocessorContextMirror(hre, target, args.contextId);
  const calldata = encodeMirrorCoprocessorContext(iface, args);

  console.log('ProtocolConfigReplica.mirrorCoprocessorContext');
  console.log('  contextId:', args.contextId.toString());
  console.log('  coprocessors:', args.coprocessors.length);
  console.log('  majorityThreshold:', args.majorityThreshold.toString());
  console.log('  target:', target);
  console.log('  calldata:', calldata);
});

addMirrorTaskParams(
  task(
    'task:mirrorCoprocessorContext',
    "Broadcasts ProtocolConfigReplica.mirrorCoprocessorContext from canonical's current coprocessor context with the deployer key",
  ),
  'the coprocessor context',
).setAction(async function (
  { canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const args = await readCanonicalCoprocessorContext(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsCoprocessorContextMirror(hre, target, args.contextId);
  const calldata = encodeMirrorCoprocessorContext(iface, args);
  const hash = await broadcast(hre, target, calldata);
  console.log(`Broadcast mirrorCoprocessorContext(context=${args.contextId}) on ${target} (tx: ${hash}).`);
});

// -----------------------------------------------------------------------------------------
// mirrorCoprocessorContextDestruction: mirrors the destruction of a past coprocessor context.
// -----------------------------------------------------------------------------------------

// Checks at a finalized canonical block that the context exists and is destroyed. Canonical IDs are
// contiguous, so any ID between the first and the current one exists.
export async function assertCanonicalCoprocessorContextDestroyed(
  hre: HardhatRuntimeEnvironment,
  options: {
    canonicalProvider: Provider;
    canonicalProtocolConfigAddress: string;
    blockNumber?: number;
    coprocessorContextId: bigint;
  },
): Promise<void> {
  const { canonicalProvider, canonicalProtocolConfigAddress, coprocessorContextId } = options;
  const snapshot = await readCanonicalSnapshot(hre, options);
  if (
    coprocessorContextId <= COPROCESSOR_CONTEXT_COUNTER_BASE ||
    coprocessorContextId >= snapshot.currentCoprocessorContextId
  ) {
    throw new Error(
      `Coprocessor context ${coprocessorContextId} is not a past context on canonical (current is ${snapshot.currentCoprocessorContextId}).`,
    );
  }

  const canonicalProtocolConfig = await getProtocolConfig(hre, canonicalProtocolConfigAddress, canonicalProvider);
  const isValid = await canonicalProtocolConfig.isValidCoprocessorContext(coprocessorContextId, {
    blockTag: snapshot.blockNumber,
  });
  if (isValid) {
    throw new Error(`Coprocessor context ${coprocessorContextId} is not destroyed on canonical. Nothing to mirror.`);
  }

  await assertCanonicalSnapshotBlockHash(canonicalProvider, snapshot);
}

export function encodeMirrorCoprocessorContextDestruction(iface: Interface, coprocessorContextId: bigint): string {
  return iface.encodeFunctionData('mirrorCoprocessorContextDestruction', [coprocessorContextId]);
}

// Fails with an explicit message instead of the replica's CurrentCoprocessorContextCannotBeDestroyed
// or InvalidCoprocessorContext revert.
export async function assertReplicaNeedsCoprocessorContextDestructionMirror(
  hre: HardhatRuntimeEnvironment,
  replicaProtocolConfigAddress: string,
  coprocessorContextId: bigint,
): Promise<void> {
  const replica = await getProtocolConfigReplica(hre, replicaProtocolConfigAddress);
  if (coprocessorContextId === (await replica.getCurrentCoprocessorContextId())) {
    throw new Error(
      `Coprocessor context ${coprocessorContextId} is the current one on replica ${replicaProtocolConfigAddress}. ` +
        `Run task:mirrorCoprocessorContext first to mirror canonical's current context.`,
    );
  }
  if (!(await replica.isValidCoprocessorContext(coprocessorContextId))) {
    throw new Error(
      `Coprocessor context ${coprocessorContextId} is unknown or already destroyed on replica ${replicaProtocolConfigAddress}. Nothing to mirror.`,
    );
  }
}

addMirrorTaskParams(
  task(
    'task:buildMirrorCoprocessorContextDestructionCalldata',
    'Builds Aragon proposal calldata for ProtocolConfigReplica.mirrorCoprocessorContextDestruction once canonical has destroyed the context (DAO path, never broadcasts)',
  ).addParam('coprocessorContextId', 'The destroyed coprocessor context ID to mirror', undefined, types.string),
  'the destroyed coprocessor context',
).setAction(async function (
  { coprocessorContextId, canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const contextId = BigInt(coprocessorContextId);
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  await assertCanonicalCoprocessorContextDestroyed(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
    coprocessorContextId: contextId,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsCoprocessorContextDestructionMirror(hre, target, contextId);
  const calldata = encodeMirrorCoprocessorContextDestruction(iface, contextId);

  console.log('ProtocolConfigReplica.mirrorCoprocessorContextDestruction');
  console.log('  coprocessorContextId:', contextId.toString());
  console.log('  target:', target);
  console.log('  calldata:', calldata);
});

addMirrorTaskParams(
  task(
    'task:mirrorCoprocessorContextDestruction',
    'Broadcasts ProtocolConfigReplica.mirrorCoprocessorContextDestruction with the deployer key once canonical has destroyed the context',
  ).addParam('coprocessorContextId', 'The destroyed coprocessor context ID to mirror', undefined, types.string),
  'the destroyed coprocessor context',
).setAction(async function (
  { coprocessorContextId, canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const contextId = BigInt(coprocessorContextId);
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  await assertCanonicalCoprocessorContextDestroyed(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
    coprocessorContextId: contextId,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsCoprocessorContextDestructionMirror(hre, target, contextId);
  const calldata = encodeMirrorCoprocessorContextDestruction(iface, contextId);
  const hash = await broadcast(hre, target, calldata);
  console.log(`Broadcast mirrorCoprocessorContextDestruction(${contextId}) on ${target} (tx: ${hash}).`);
});
