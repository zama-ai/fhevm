import type { Interface, Provider } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { type Coprocessor, getCoprocessorConfig, readCanonicalCoprocessorSnapshot } from './coprocessorConfigMirror';
import { broadcast } from './kmsContext';
import { assertCanonicalSnapshotBlockHash } from './protocolConfigMirror';
import {
  addMirrorTaskParams,
  getCanonicalProvider,
  getCoprocessorConfigReplica,
  getCoprocessorConfigReplicaInterface,
  requireCoprocessorConfigAddress,
} from './utils/coprocessorConfigTasks';
import { COPROCESSOR_CONTEXT_COUNTER_BASE } from './utils/kmsGenerationConstants';

// This file defines tasks that mirror the canonical (Ethereum) CoprocessorConfig's coprocessor contexts
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
  options: { canonicalProvider: Provider; canonicalCoprocessorConfigAddress: string; blockNumber?: number },
): Promise<MirrorCoprocessorContextArgs> {
  const snapshot = await readCanonicalCoprocessorSnapshot(hre, options);
  return {
    contextId: snapshot.currentCoprocessorContextId,
    coprocessors: snapshot.coprocessors,
    majorityThreshold: snapshot.majorityThreshold,
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
  replicaCoprocessorConfigAddress: string,
  canonicalCoprocessorContextId: bigint,
): Promise<void> {
  const replica = await getCoprocessorConfigReplica(hre, replicaCoprocessorConfigAddress);
  const replicaContextId = await replica.getCurrentCoprocessorContextId();
  if (canonicalCoprocessorContextId <= replicaContextId) {
    throw new Error(
      `Replica ${replicaCoprocessorConfigAddress} is already at coprocessor context ${replicaContextId}, which is >= canonical's ${canonicalCoprocessorContextId}. Nothing to mirror.`,
    );
  }
}

addMirrorTaskParams(
  task(
    'task:buildMirrorCoprocessorContextCalldata',
    "Builds Aragon proposal calldata for CoprocessorConfigReplica.mirrorCoprocessorContext from canonical's current coprocessor context (DAO path, never broadcasts)",
  ),
  'the coprocessor context',
).setAction(async function (
  { canonicalRpcUrl, canonicalCoprocessorConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getCoprocessorConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const args = await readCanonicalCoprocessorContext(hre, {
    canonicalProvider,
    canonicalCoprocessorConfigAddress,
    blockNumber,
  });
  const target = requireCoprocessorConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsCoprocessorContextMirror(hre, target, args.contextId);
  const calldata = encodeMirrorCoprocessorContext(iface, args);

  console.log('CoprocessorConfigReplica.mirrorCoprocessorContext');
  console.log('  contextId:', args.contextId.toString());
  console.log('  coprocessors:', args.coprocessors.length);
  console.log('  majorityThreshold:', args.majorityThreshold.toString());
  console.log('  target:', target);
  console.log('  calldata:', calldata);
});

addMirrorTaskParams(
  task(
    'task:mirrorCoprocessorContext',
    "Broadcasts CoprocessorConfigReplica.mirrorCoprocessorContext from canonical's current coprocessor context with the deployer key",
  ),
  'the coprocessor context',
).setAction(async function (
  { canonicalRpcUrl, canonicalCoprocessorConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getCoprocessorConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const args = await readCanonicalCoprocessorContext(hre, {
    canonicalProvider,
    canonicalCoprocessorConfigAddress,
    blockNumber,
  });
  const target = requireCoprocessorConfigAddress(useInternalProxyAddress);
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
    canonicalCoprocessorConfigAddress: string;
    blockNumber?: number;
    coprocessorContextId: bigint;
  },
): Promise<void> {
  const { canonicalProvider, canonicalCoprocessorConfigAddress, coprocessorContextId } = options;
  const snapshot = await readCanonicalCoprocessorSnapshot(hre, options);
  if (
    coprocessorContextId <= COPROCESSOR_CONTEXT_COUNTER_BASE ||
    coprocessorContextId >= snapshot.currentCoprocessorContextId
  ) {
    throw new Error(
      `Coprocessor context ${coprocessorContextId} is not a past context on canonical (current is ${snapshot.currentCoprocessorContextId}).`,
    );
  }

  const canonicalCoprocessorConfig = await getCoprocessorConfig(
    hre,
    canonicalCoprocessorConfigAddress,
    canonicalProvider,
  );
  const isValid = await canonicalCoprocessorConfig.isValidCoprocessorContext(coprocessorContextId, {
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
  replicaCoprocessorConfigAddress: string,
  coprocessorContextId: bigint,
): Promise<void> {
  const replica = await getCoprocessorConfigReplica(hre, replicaCoprocessorConfigAddress);
  if (coprocessorContextId === (await replica.getCurrentCoprocessorContextId())) {
    throw new Error(
      `Coprocessor context ${coprocessorContextId} is the current one on replica ${replicaCoprocessorConfigAddress}. ` +
        `Run task:mirrorCoprocessorContext first to mirror canonical's current context.`,
    );
  }
  if (!(await replica.isValidCoprocessorContext(coprocessorContextId))) {
    throw new Error(
      `Coprocessor context ${coprocessorContextId} is unknown or already destroyed on replica ${replicaCoprocessorConfigAddress}. Nothing to mirror.`,
    );
  }
}

addMirrorTaskParams(
  task(
    'task:buildMirrorCoprocessorContextDestructionCalldata',
    'Builds Aragon proposal calldata for CoprocessorConfigReplica.mirrorCoprocessorContextDestruction once canonical has destroyed the context (DAO path, never broadcasts)',
  ).addParam('coprocessorContextId', 'The destroyed coprocessor context ID to mirror', undefined, types.string),
  'the destroyed coprocessor context',
).setAction(async function (
  { coprocessorContextId, canonicalRpcUrl, canonicalCoprocessorConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const contextId = BigInt(coprocessorContextId);
  const iface = await getCoprocessorConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  await assertCanonicalCoprocessorContextDestroyed(hre, {
    canonicalProvider,
    canonicalCoprocessorConfigAddress,
    blockNumber,
    coprocessorContextId: contextId,
  });
  const target = requireCoprocessorConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsCoprocessorContextDestructionMirror(hre, target, contextId);
  const calldata = encodeMirrorCoprocessorContextDestruction(iface, contextId);

  console.log('CoprocessorConfigReplica.mirrorCoprocessorContextDestruction');
  console.log('  coprocessorContextId:', contextId.toString());
  console.log('  target:', target);
  console.log('  calldata:', calldata);
});

addMirrorTaskParams(
  task(
    'task:mirrorCoprocessorContextDestruction',
    'Broadcasts CoprocessorConfigReplica.mirrorCoprocessorContextDestruction with the deployer key once canonical has destroyed the context',
  ).addParam('coprocessorContextId', 'The destroyed coprocessor context ID to mirror', undefined, types.string),
  'the destroyed coprocessor context',
).setAction(async function (
  { coprocessorContextId, canonicalRpcUrl, canonicalCoprocessorConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const contextId = BigInt(coprocessorContextId);
  const iface = await getCoprocessorConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  await assertCanonicalCoprocessorContextDestroyed(hre, {
    canonicalProvider,
    canonicalCoprocessorConfigAddress,
    blockNumber,
    coprocessorContextId: contextId,
  });
  const target = requireCoprocessorConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsCoprocessorContextDestructionMirror(hre, target, contextId);
  const calldata = encodeMirrorCoprocessorContextDestruction(iface, contextId);
  const hash = await broadcast(hre, target, calldata);
  console.log(`Broadcast mirrorCoprocessorContextDestruction(${contextId}) on ${target} (tx: ${hash}).`);
});
