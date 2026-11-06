import type { Interface, ParamType, Provider } from 'ethers';
import { task } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { KmsNodeParamsStruct, PcrValuesStruct } from '../types/contracts/ProtocolConfigReplica';
import { type KmsThresholds, assertCanonicalSnapshotBlockHash, readCanonicalSnapshot } from './protocolConfigMirror';
import {
  addMirrorTaskParams,
  broadcast,
  getCanonicalProvider,
  getProtocolConfig,
  getProtocolConfigReplica,
  getProtocolConfigReplicaInterface,
  requireProtocolConfigAddress,
} from './utils/protocolConfigTasks';

// This file defines tasks that mirror the canonical (Ethereum) ProtocolConfig's active KMS context
// and epoch onto a non-canonical replica (e.g. Polygon) after a rotation has gone active on
// canonical. It follows the same build-calldata (DAO path, never broadcasts) / broadcast (no-DAO
// path for devnet / test-suite) convention as kmsContext.ts, calling the replica's
// `mirrorKmsContextAndEpoch` (context switch) or `mirrorKmsEpoch` (same-set epoch rotation).

// -----------------------------------------------------------------------------------------
// mirrorKmsContextAndEpoch: mirrors a context switch (new signer set) plus its first epoch.
// -----------------------------------------------------------------------------------------

export interface MirrorContextArgs {
  contextId: bigint;
  epochId: bigint;
  kmsNodeParams: KmsNodeParamsStruct[];
  thresholds: KmsThresholds;
  softwareVersion: string;
  pcrValues: PcrValuesStruct[];
}

// Recomputes the on-chain `contextInfoHash` anchor from candidate event args. The encode types come
// straight off canonical's `NewKmsContext` event (minus the two indexed ids), so the tuple layout
// cannot drift from canonical's `abi.encode(...)`.
function computeContextInfoHash(
  hre: HardhatRuntimeEnvironment,
  encodeTypes: readonly ParamType[],
  args: Pick<MirrorContextArgs, 'kmsNodeParams' | 'thresholds' | 'softwareVersion' | 'pcrValues'>,
): string {
  const encoded = hre.ethers.AbiCoder.defaultAbiCoder().encode(encodeTypes, [
    args.kmsNodeParams,
    args.thresholds,
    args.softwareVersion,
    args.pcrValues,
  ]);
  return hre.ethers.keccak256(encoded);
}

// Reads canonical's active KMS context as `mirrorKmsContextAndEpoch` args. The node/software-version/
// PCR data only exists in the `NewKmsContext` event, so this reads the event at the block the context
// anchor records and verifies its hash against the anchor. Thresholds come from the same event: they
// are fixed at context creation, and the anchor hash check covers them too.
export async function readCanonicalContextSwitch(
  hre: HardhatRuntimeEnvironment,
  options: {
    canonicalProvider: Provider;
    canonicalProtocolConfigAddress: string;
    blockNumber?: number;
  },
): Promise<MirrorContextArgs> {
  const { canonicalProvider, canonicalProtocolConfigAddress, blockNumber } = options;
  const snapshot = await readCanonicalSnapshot(hre, { canonicalProvider, canonicalProtocolConfigAddress, blockNumber });

  const canonicalProtocolConfig = await getProtocolConfig(hre, canonicalProtocolConfigAddress, canonicalProvider);

  const [emissionBlockNumber, expectedHash] = await canonicalProtocolConfig.getKmsContextAnchor(
    snapshot.currentKmsContextId,
    { blockTag: snapshot.blockNumber },
  );
  if (emissionBlockNumber === 0n) {
    throw new Error(
      `${canonicalProtocolConfigAddress} has no context anchor recorded for context ${snapshot.currentKmsContextId}. `,
    );
  }

  const [event] = await canonicalProtocolConfig.queryFilter(
    canonicalProtocolConfig.filters.NewKmsContext(snapshot.currentKmsContextId),
    Number(emissionBlockNumber),
    Number(emissionBlockNumber),
  );
  if (!event) {
    throw new Error(
      `No NewKmsContext(${snapshot.currentKmsContextId}) event at block ${emissionBlockNumber} on ${canonicalProtocolConfigAddress}. `,
    );
  }

  const kmsNodeParams: KmsNodeParamsStruct[] = event.args.kmsNodeParams.map((node) => ({
    txSenderAddress: node.txSenderAddress,
    signerAddress: node.signerAddress,
    ipAddress: node.ipAddress,
    storageUrl: node.storageUrl,
    partyId: node.partyId,
    mpcIdentity: node.mpcIdentity,
    caCert: node.caCert,
    storagePrefix: node.storagePrefix,
  }));
  const eventThresholds: KmsThresholds = {
    publicDecryption: event.args.thresholds.publicDecryption,
    userDecryption: event.args.thresholds.userDecryption,
    kmsGen: event.args.thresholds.kmsGen,
    mpc: event.args.thresholds.mpc,
  };
  const pcrValues: PcrValuesStruct[] = event.args.pcrValues.map((pcr) => ({
    pcr0: pcr.pcr0,
    pcr1: pcr.pcr1,
    pcr2: pcr.pcr2,
  }));
  const softwareVersion = event.args.softwareVersion;

  const encodeTypes = canonicalProtocolConfig.interface.getEvent('NewKmsContext').inputs.slice(2);
  const hash = computeContextInfoHash(hre, encodeTypes, {
    kmsNodeParams,
    thresholds: eventThresholds,
    softwareVersion,
    pcrValues,
  });
  if (hash !== expectedHash) {
    throw new Error(
      `NewKmsContext(${snapshot.currentKmsContextId}) event at block ${emissionBlockNumber} on ${canonicalProtocolConfigAddress} does not match the on-chain contextInfoHash.`,
    );
  }

  await assertCanonicalSnapshotBlockHash(canonicalProvider, snapshot);

  return {
    contextId: snapshot.currentKmsContextId,
    epochId: snapshot.currentEpochId,
    kmsNodeParams,
    thresholds: eventThresholds,
    softwareVersion,
    pcrValues,
  };
}

export function encodeMirrorKmsContextAndEpoch(iface: Interface, args: MirrorContextArgs): string {
  return iface.encodeFunctionData('mirrorKmsContextAndEpoch', [
    args.contextId,
    args.epochId,
    args.kmsNodeParams,
    args.thresholds,
    args.softwareVersion,
    args.pcrValues,
  ]);
}

// Fails with an explicit mismatch message instead of the replica's NonIncreasingKmsContextId revert.
export async function assertReplicaNeedsKmsContextSwitch(
  hre: HardhatRuntimeEnvironment,
  replicaProtocolConfigAddress: string,
  canonicalKmsContextId: bigint,
): Promise<void> {
  const replica = await getProtocolConfigReplica(hre, replicaProtocolConfigAddress);
  const [replicaContextId] = await replica.getCurrentKmsContextAndEpoch();
  if (canonicalKmsContextId <= replicaContextId) {
    throw new Error(
      `Replica ${replicaProtocolConfigAddress} is already at context ${replicaContextId}, which is >= canonical's ${canonicalKmsContextId}. `,
    );
  }
}

addMirrorTaskParams(
  task(
    'task:buildMirrorKmsContextAndEpochCalldata',
    "Builds Aragon proposal calldata for ProtocolConfigReplica.mirrorKmsContextAndEpoch from canonical's active KMS context (DAO path, never broadcasts)",
  ),
  'the context switch',
).setAction(async function (
  { canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const args = await readCanonicalContextSwitch(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsKmsContextSwitch(hre, target, args.contextId);
  const calldata = encodeMirrorKmsContextAndEpoch(iface, args);

  console.log('ProtocolConfigReplica.mirrorKmsContextAndEpoch');
  console.log('  contextId:', args.contextId.toString());
  console.log('  epochId:', args.epochId.toString());
  console.log('  kmsNodes:', args.kmsNodeParams.length);
  console.log('  target:', target);
  console.log('  calldata:', calldata);
});

addMirrorTaskParams(
  task(
    'task:mirrorKmsContextAndEpoch',
    "Broadcasts ProtocolConfigReplica.mirrorKmsContextAndEpoch from canonical's active KMS context with the deployer key",
  ),
  'the context switch',
).setAction(async function (
  { canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const args = await readCanonicalContextSwitch(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsKmsContextSwitch(hre, target, args.contextId);
  const calldata = encodeMirrorKmsContextAndEpoch(iface, args);
  const hash = await broadcast(hre, target, calldata);
  console.log(
    `Broadcast mirrorKmsContextAndEpoch(context=${args.contextId}, epoch=${args.epochId}) on ${target} (tx: ${hash}).`,
  );
});

// -----------------------------------------------------------------------------------------
// mirrorKmsEpoch: mirrors a same-set epoch rotation (no signer-set change) under the context
// already active on the replica.
// -----------------------------------------------------------------------------------------

export function encodeMirrorKmsEpoch(iface: Interface, contextId: bigint, epochId: bigint): string {
  return iface.encodeFunctionData('mirrorKmsEpoch', [contextId, epochId]);
}

export async function assertReplicaNeedsKmsEpochMirror(
  hre: HardhatRuntimeEnvironment,
  replicaProtocolConfigAddress: string,
  canonicalKmsContextId: bigint,
  canonicalKmsEpochId: bigint,
): Promise<void> {
  const replica = await getProtocolConfigReplica(hre, replicaProtocolConfigAddress);
  const [replicaContextId, replicaEpochId] = await replica.getCurrentKmsContextAndEpoch();
  if (canonicalKmsContextId !== replicaContextId) {
    throw new Error(
      `Replica ${replicaProtocolConfigAddress} is at context ${replicaContextId}, but canonical's active context is ${canonicalKmsContextId}. ` +
        `Run task:mirrorKmsContextAndEpoch first to mirror the context switch.`,
    );
  }
  if (canonicalKmsEpochId <= replicaEpochId) {
    throw new Error(
      `Replica ${replicaProtocolConfigAddress} is already at epoch ${replicaEpochId}, which is >= canonical's ${canonicalKmsEpochId}. Nothing to mirror.`,
    );
  }
}

addMirrorTaskParams(
  task(
    'task:buildMirrorKmsEpochCalldata',
    "Builds Aragon proposal calldata for ProtocolConfigReplica.mirrorKmsEpoch from canonical's active KMS epoch (DAO path, never broadcasts)",
  ),
  'the active epoch',
).setAction(async function (
  { canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const snapshot = await readCanonicalSnapshot(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsKmsEpochMirror(hre, target, snapshot.currentKmsContextId, snapshot.currentEpochId);
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
).setAction(async function (
  { canonicalRpcUrl, canonicalProtocolConfigAddress, blockNumber, useInternalProxyAddress },
  hre,
): Promise<void> {
  const iface = await getProtocolConfigReplicaInterface(hre);
  const canonicalProvider = getCanonicalProvider(hre, canonicalRpcUrl);
  const snapshot = await readCanonicalSnapshot(hre, {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    blockNumber,
  });
  const target = requireProtocolConfigAddress(useInternalProxyAddress);
  await assertReplicaNeedsKmsEpochMirror(hre, target, snapshot.currentKmsContextId, snapshot.currentEpochId);
  const calldata = encodeMirrorKmsEpoch(iface, snapshot.currentKmsContextId, snapshot.currentEpochId);
  const hash = await broadcast(hre, target, calldata);
  console.log(
    `Broadcast mirrorKmsEpoch(context=${snapshot.currentKmsContextId}, epoch=${snapshot.currentEpochId}) on ${target} (tx: ${hash}).`,
  );
});
