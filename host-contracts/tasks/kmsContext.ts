import { Interface } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig } from '../types';
import { buildProtocolConfigContextArgs } from './taskDeploy';
import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// This file defines tasks to drive a KMS context switch / epoch rotation on the canonical (Ethereum)
// ProtocolConfig: build the `defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext` /
// `destroyKmsContext` / `destroyKmsEpoch` governance proposal calldata (DAO path, never broadcasts)
// or broadcast it directly with the deployer key (devnet / test-suite), plus a read-only status task
// that tracks an in-flight switch from events. Context/epoch definition inputs come from the `KMS_*`
// env vars (reusing `buildProtocolConfigContextArgs`); the destroy tasks take the target id directly.

const PROTOCOL_CONFIG_ADDRESS_ENV_VAR = 'PROTOCOL_CONFIG_CONTRACT_ADDRESS';

// Builds a calldata-only ABI for ProtocolConfig without a deployer key, so the `build*` tasks never
// need a signer.
export async function getProtocolConfigInterface(hre: HardhatRuntimeEnvironment): Promise<Interface> {
  await hre.run('compile:specific', { contract: 'contracts' });
  const artifact = await hre.artifacts.readArtifact('ProtocolConfig');
  return new hre.ethers.Interface(artifact.abi);
}

// Resolves the ProtocolConfig proxy address from env (or the addresses directory when
// `useInternalProxyAddress` is set), returning undefined when none is configured.
function resolveProtocolConfigAddress(useInternalProxyAddress: boolean): string | undefined {
  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const address = process.env[PROTOCOL_CONFIG_ADDRESS_ENV_VAR];
  return address && address.trim() !== '' ? address : undefined;
}

function requireProtocolConfigAddress(useInternalProxyAddress: boolean): string {
  const address = resolveProtocolConfigAddress(useInternalProxyAddress);
  if (!address) {
    throw new Error(
      `No ProtocolConfig address configured. Set ${PROTOCOL_CONFIG_ADDRESS_ENV_VAR} or pass --use-internal-proxy-address.`,
    );
  }
  return address;
}

// Reads the canonical ProtocolConfig and returns the context id the next switch will create
// (allocation counter + 1). This is the value the operator sets as the Gateway proposal's
// KMS_CONTEXT_ID so the host and Gateway proposals stay aligned.
export async function predictNewKmsContextId(
  hre: HardhatRuntimeEnvironment,
  protocolConfigAddress: string,
): Promise<bigint> {
  const protocolConfig = (await hre.ethers.getContractAt(
    'ProtocolConfig',
    protocolConfigAddress,
  )) as unknown as ProtocolConfig;
  return (await protocolConfig.getCurrentKmsContextIdCounter()) + 1n;
}

interface EncodedCall {
  functionSignature: string;
  calldata: string;
  decodedArgs: unknown[];
}

export function encodeDefineNewKmsContextAndEpoch(iface: Interface): EncodedCall {
  const args = buildProtocolConfigContextArgs();
  const functionSignature = iface.getFunction('defineNewKmsContextAndEpoch')!.format('sighash');
  const calldata = iface.encodeFunctionData('defineNewKmsContextAndEpoch', args);
  const decodedArgs = iface.decodeFunctionData('defineNewKmsContextAndEpoch', calldata).toArray();
  return { functionSignature, calldata, decodedArgs };
}

export function encodeDefineNewEpochForCurrentKmsContext(iface: Interface): EncodedCall {
  const functionSignature = iface.getFunction('defineNewEpochForCurrentKmsContext')!.format('sighash');
  const calldata = iface.encodeFunctionData('defineNewEpochForCurrentKmsContext', []);
  return { functionSignature, calldata, decodedArgs: [] };
}

export function encodeDestroyKmsContext(iface: Interface, kmsContextId: bigint): EncodedCall {
  const functionSignature = iface.getFunction('destroyKmsContext')!.format('sighash');
  const calldata = iface.encodeFunctionData('destroyKmsContext', [kmsContextId]);
  return { functionSignature, calldata, decodedArgs: [kmsContextId] };
}

export function encodeDestroyKmsEpoch(iface: Interface, epochId: bigint): EncodedCall {
  const functionSignature = iface.getFunction('destroyKmsEpoch')!.format('sighash');
  const calldata = iface.encodeFunctionData('destroyKmsEpoch', [epochId]);
  return { functionSignature, calldata, decodedArgs: [epochId] };
}

// Broadcasts the byte-identical payload the DAO would sign, using the deployer key. On devnet / the
// test-suite the deployer is the ACL owner, so the call is authorized; this is the no-DAO path.
async function broadcast(hre: HardhatRuntimeEnvironment, target: string, calldata: string): Promise<string> {
  const deployer = new hre.ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(hre.ethers.provider);
  const tx = await deployer.sendTransaction({ to: target, data: calldata });
  await tx.wait();
  return tx.hash;
}

task(
  'task:buildDefineNewKmsContextAndEpochCalldata',
  'Builds Aragon proposal calldata for ProtocolConfig.defineNewKmsContextAndEpoch from KMS_* env vars (DAO path, never broadcasts)',
)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre): Promise<void> {
    const iface = await getProtocolConfigInterface(hre);
    const encoded = encodeDefineNewKmsContextAndEpoch(iface);
    const target = resolveProtocolConfigAddress(useInternalProxyAddress);

    // The host derives the new context id on-chain as the allocation counter + 1. When a ProtocolConfig address is
    // resolvable, surface that id so the operator can set it as the Gateway proposal's
    // KMS_CONTEXT_ID, keeping the two proposals aligned without a dedicated env var.
    const newContextId = target ? (await predictNewKmsContextId(hre, target)).toString() : undefined;

    console.log('ProtocolConfig.defineNewKmsContextAndEpoch');
    if (newContextId) {
      console.log("  newContextId (set as the Gateway proposal's KMS_CONTEXT_ID):", newContextId);
    }
    console.log(
      '  target:',
      target ?? `<unresolved — set ${PROTOCOL_CONFIG_ADDRESS_ENV_VAR} or pass --use-internal-proxy-address>`,
    );
    console.log('  calldata:', encoded.calldata);
  });

task(
  'task:defineNewKmsContextAndEpoch',
  'Broadcasts ProtocolConfig.defineNewKmsContextAndEpoch from KMS_* env vars with the deployer key (no-DAO path for devnet / test-suite)',
)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre): Promise<void> {
    const iface = await getProtocolConfigInterface(hre);
    const { calldata } = encodeDefineNewKmsContextAndEpoch(iface);
    const target = requireProtocolConfigAddress(useInternalProxyAddress);
    const hash = await broadcast(hre, target, calldata);
    console.log(`Broadcast defineNewKmsContextAndEpoch on ${target} (tx: ${hash}). Context + epoch are now PENDING.`);
  });

task(
  'task:buildDefineNewEpochForCurrentKmsContextCalldata',
  'Builds Aragon proposal calldata for ProtocolConfig.defineNewEpochForCurrentKmsContext (same-set epoch rotation, no-arg; DAO path)',
).setAction(async function (_, hre): Promise<void> {
  const iface = await getProtocolConfigInterface(hre);
  const encoded = encodeDefineNewEpochForCurrentKmsContext(iface);

  console.log('ProtocolConfig.defineNewEpochForCurrentKmsContext');
  console.log('  calldata:', encoded.calldata);
});

task(
  'task:defineNewEpochForCurrentKmsContext',
  'Broadcasts ProtocolConfig.defineNewEpochForCurrentKmsContext with the deployer key (no-DAO path for devnet / test-suite)',
)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre): Promise<void> {
    const iface = await getProtocolConfigInterface(hre);
    const { calldata } = encodeDefineNewEpochForCurrentKmsContext(iface);
    const target = requireProtocolConfigAddress(useInternalProxyAddress);
    const hash = await broadcast(hre, target, calldata);
    console.log(`Broadcast defineNewEpochForCurrentKmsContext on ${target} (tx: ${hash}). New epoch is now PENDING.`);
  });

task(
  'task:buildDestroyKmsContextCalldata',
  'Builds Aragon proposal calldata for ProtocolConfig.destroyKmsContext (retires a non-current context; DAO path, never broadcasts)',
)
  .addParam('contextId', 'The KMS context ID to destroy', undefined, types.string)
  .setAction(async function ({ contextId }, hre): Promise<void> {
    const iface = await getProtocolConfigInterface(hre);
    const encoded = encodeDestroyKmsContext(iface, BigInt(contextId));

    console.log('ProtocolConfig.destroyKmsContext');
    console.log('  contextId:', contextId);
    console.log('  calldata:', encoded.calldata);
  });

task(
  'task:destroyKmsContext',
  'Broadcasts ProtocolConfig.destroyKmsContext with the deployer key (no-DAO path for devnet / test-suite)',
)
  .addParam('contextId', 'The KMS context ID to destroy', undefined, types.string)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .setAction(async function ({ contextId, useInternalProxyAddress }, hre): Promise<void> {
    const iface = await getProtocolConfigInterface(hre);
    const { calldata } = encodeDestroyKmsContext(iface, BigInt(contextId));
    const target = requireProtocolConfigAddress(useInternalProxyAddress);
    const hash = await broadcast(hre, target, calldata);
    console.log(`Broadcast destroyKmsContext(${contextId}) on ${target} (tx: ${hash}). Context is now DESTROYED.`);
  });

task(
  'task:buildDestroyKmsEpochCalldata',
  'Builds Aragon proposal calldata for ProtocolConfig.destroyKmsEpoch (retires a superseded epoch; DAO path, never broadcasts)',
)
  .addParam('epochId', 'The KMS epoch ID to destroy', undefined, types.string)
  .setAction(async function ({ epochId }, hre): Promise<void> {
    const iface = await getProtocolConfigInterface(hre);
    const encoded = encodeDestroyKmsEpoch(iface, BigInt(epochId));

    console.log('ProtocolConfig.destroyKmsEpoch');
    console.log('  epochId:', epochId);
    console.log('  calldata:', encoded.calldata);
  });

task(
  'task:destroyKmsEpoch',
  'Broadcasts ProtocolConfig.destroyKmsEpoch with the deployer key (no-DAO path for devnet / test-suite)',
)
  .addParam('epochId', 'The KMS epoch ID to destroy', undefined, types.string)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .setAction(async function ({ epochId, useInternalProxyAddress }, hre): Promise<void> {
    const iface = await getProtocolConfigInterface(hre);
    const { calldata } = encodeDestroyKmsEpoch(iface, BigInt(epochId));
    const target = requireProtocolConfigAddress(useInternalProxyAddress);
    const hash = await broadcast(hre, target, calldata);
    console.log(`Broadcast destroyKmsEpoch(${epochId}) on ${target} (tx: ${hash}). Epoch is now DESTROYED.`);
  });

////////////////////////////////////////////////////////////////////////////////
// Status task (event-indexing monitor)
////////////////////////////////////////////////////////////////////////////////

type ContextState = 'PENDING' | 'CREATED' | 'ACTIVE';
type EpochState = 'PENDING' | 'ACTIVE';

export interface KmsContextSwitchStatus {
  protocolConfig: string;
  scannedFromBlock: number;
  scannedToBlock: number;
  activeContextId: bigint;
  activeEpochId: bigint;
  flow: 'idle' | 'context-switch' | 'same-set-rotation';
  aborted: boolean;
  abortReason: string | null;
  fullyLive: boolean;

  // Context-switch creation phase (undefined for same-set / idle).
  pendingContextId?: bigint;
  previousContextId?: bigint;
  contextState?: ContextState;
  newSigners?: string[];
  newTxSenders?: string[];
  newTxSendersConfirmed?: string[];
  newTxSendersOutstanding?: string[];
  previousTxSendersConfirmed?: string[];
  previousConfirmationCount?: number;
  previousTxSenderThreshold?: number; // the (n - t) old-side quorum target
  contextCreationQuorumReached?: boolean;
  stuckBelowPreviousThreshold?: boolean;

  // Epoch-activation phase (present once an epoch id is observable: same-set, or context CREATED+).
  pendingEpochId?: bigint;
  epochState?: EpochState;
  epochSigners?: string[];
  epochSignersConfirmed?: string[];
  epochSignersOutstanding?: string[];
  epochConfirmationsByDataHash?: Record<string, string[]>;
  epochConfirmationsDiverged?: boolean;
}

// Returns the elements of `expected` (checksummed addresses) that are not present in `confirmed`.
function outstanding(expected: string[], confirmed: Set<string>): string[] {
  return expected.filter((address) => !confirmed.has(address));
}

// Reconstructs the in-progress phase of a KMS context switch or epoch rotation. It detects an
// in-flight switch from authoritative state, where the allocation counter runs ahead of the active
// context pointer and the issued context is still live. This still reports a switch defined before
// the scanned window. It reads the old-side `(n - t)` target from the value the contract cached at
// define time.
export async function inspectKmsContextSwitch(
  hre: HardhatRuntimeEnvironment,
  protocolConfigAddress: string,
  fromBlock: number,
): Promise<KmsContextSwitchStatus> {
  const { ethers } = hre;
  const pc = (await ethers.getContractAt('ProtocolConfig', protocolConfigAddress)) as unknown as ProtocolConfig;
  const toBlock = await ethers.provider.getBlockNumber();

  const [activeContextId, activeEpochId] = await pc.getCurrentKmsContextAndEpoch();

  const newContextEvents = await pc.queryFilter(pc.filters.NewKmsContext(), fromBlock, toBlock);
  const newEpochEvents = await pc.queryFilter(pc.filters.NewKmsEpoch(), fromBlock, toBlock);

  const checksum = (address: string) => ethers.getAddress(address);

  const status: KmsContextSwitchStatus = {
    protocolConfig: protocolConfigAddress,
    scannedFromBlock: fromBlock,
    scannedToBlock: toBlock,
    activeContextId,
    activeEpochId,
    flow: 'idle',
    aborted: false,
    abortReason: null,
    fullyLive: false,
  };

  // The latest-issued context/epoch are the only candidates for an in-flight switch (the contract
  // forbids more than one non-active context or epoch at a time).
  const latestNewContext = newContextEvents.reduce<(typeof newContextEvents)[number] | undefined>(
    (max, event) => (max && max.args.contextId >= event.args.contextId ? max : event),
    undefined,
  );
  const latestNewEpoch = newEpochEvents.reduce<(typeof newEpochEvents)[number] | undefined>(
    (max, event) => (max && max.args.epochId >= event.args.epochId ? max : event),
    undefined,
  );

  // The allocation counter runs ahead of the active pointer once a context is issued. Destroying a
  // context does not rewind the counter, so the comparison alone cannot separate an in-flight switch
  // from an aborted one. The live state separates them. The task reads it from state, because a
  // destruction before `fromBlock` leaves no event in the scanned window.
  const latestKmsContextId = await pc.getCurrentKmsContextIdCounter();
  const contextSwitchIssued = latestKmsContextId > activeContextId;
  const pendingContextIsLive = contextSwitchIssued && (await pc.isLiveKmsContext(latestKmsContextId));

  // A same-set rotation is only observable from events: no view getter exposes the epoch counter. A
  // rotation opened before `fromBlock` is therefore not reported.
  const sameSetRotationPending =
    latestNewEpoch !== undefined &&
    latestNewEpoch.args.epochId > activeEpochId &&
    latestNewEpoch.args.kmsContextId === activeContextId;

  // The task still reports an aborted switch, because the abort needs attention. A rotation opened
  // after the abort takes precedence: that rotation is the work in flight.
  if (contextSwitchIssued && (pendingContextIsLive || !sameSetRotationPending)) {
    await fillContextSwitch(
      pc,
      status,
      latestKmsContextId,
      pendingContextIsLive,
      latestNewContext,
      newEpochEvents,
      fromBlock,
      toBlock,
      checksum,
    );
  } else if (sameSetRotationPending) {
    await fillSameSetRotation(pc, status, latestNewEpoch.args.epochId, fromBlock, toBlock, checksum);
  } else {
    // Nothing in flight: the latest-issued context and epoch are already the active ones.
    status.flow = 'idle';
    status.fullyLive =
      activeContextId > 0n && activeEpochId > 0n && (await pc.isValidEpochForContext(activeContextId, activeEpochId));
  }

  return status;
}

async function fillContextSwitch(
  pc: ProtocolConfig,
  status: KmsContextSwitchStatus,
  pendingContextId: bigint,
  pendingContextIsLive: boolean,
  newContextEvent:
    | {
        args: {
          contextId: bigint;
          previousContextId: bigint;
          kmsNodeParams: { txSenderAddress: string; signerAddress: string }[];
        };
      }
    | undefined,
  newEpochEvents: { args: { kmsContextId: bigint; epochId: bigint } }[],
  fromBlock: number,
  toBlock: number,
  checksum: (address: string) => string,
): Promise<void> {
  status.flow = 'context-switch';
  status.pendingContextId = pendingContextId;
  // The active context is demoted only on activation. During an in-flight switch it is still the
  // previous one, so it stands in for the previous context id when the defining event is out of range.
  status.previousContextId = newContextEvent?.args.previousContextId ?? status.activeContextId;

  // A destroyed pending context aborts the switch. To switch again, governance must define a new context.
  status.aborted = !pendingContextIsLive;
  if (status.aborted) {
    status.abortReason = 'context-destroyed';
  }

  // Old-side (n - t) target: read the value cached at define time. A recompute from the previous
  // context's live signer count and MPC threshold drifts if either is updated mid-switch.
  status.previousTxSenderThreshold = Number(await pc.getContextCreationPreviousTxSenderThreshold(pendingContextId));

  // New committee: read it from the event, because views cannot enumerate the pending context. When
  // the defining event is outside the scanned window, the committee, its confirmation count, and the
  // derived quorum flags are unknowable. They therefore stay undefined instead of reporting a value
  // off empty data.
  if (newContextEvent) {
    const newSigners = newContextEvent.args.kmsNodeParams.map((node) => checksum(node.signerAddress));
    status.newSigners = newSigners;
    const newTxSenders = newContextEvent.args.kmsNodeParams.map((node) => checksum(node.txSenderAddress));
    status.newTxSenders = newTxSenders;

    // Count creation confirmations from events.
    const creationConfirmations = await pc.queryFilter(
      pc.filters.KmsContextCreationConfirmation(pendingContextId),
      fromBlock,
      toBlock,
    );
    const newConfirmed = new Set<string>();
    const previousConfirmed = new Set<string>();
    for (const event of creationConfirmations) {
      const txSender = checksum(event.args.txSender);
      if (event.args.isNewTxSender) {
        newConfirmed.add(txSender);
      }
      if (event.args.isPreviousTxSender) {
        previousConfirmed.add(txSender);
      }
    }
    status.newTxSendersConfirmed = [...newConfirmed];
    status.newTxSendersOutstanding = outstanding(newTxSenders, newConfirmed);
    status.previousTxSendersConfirmed = [...previousConfirmed];
    status.previousConfirmationCount = previousConfirmed.size;

    status.contextCreationQuorumReached =
      status.newTxSendersOutstanding.length === 0 && previousConfirmed.size >= status.previousTxSenderThreshold;
    status.stuckBelowPreviousThreshold =
      !status.contextCreationQuorumReached && previousConfirmed.size < status.previousTxSenderThreshold;
  }

  // `Created` is signaled by the NewKmsEpoch emitted once the creation quorum is reached; it also
  // reveals the pending epoch id, which has no view getter.
  const pendingEpochEvent = newEpochEvents.find((event) => event.args.kmsContextId === pendingContextId);
  status.contextState = pendingEpochEvent ? 'CREATED' : 'PENDING';

  if (pendingEpochEvent && status.newSigners) {
    await fillEpochActivation(
      pc,
      status,
      pendingEpochEvent.args.epochId,
      status.newSigners,
      fromBlock,
      toBlock,
      checksum,
    );
  }
}

async function fillSameSetRotation(
  pc: ProtocolConfig,
  status: KmsContextSwitchStatus,
  pendingEpochId: bigint,
  fromBlock: number,
  toBlock: number,
  checksum: (address: string) => string,
): Promise<void> {
  status.flow = 'same-set-rotation';
  // The signer set is unchanged, so the active context's signers are the expected confirmers.
  const epochSigners: string[] = await pc.getKmsSignersForContext(status.activeContextId);
  await fillEpochActivation(pc, status, pendingEpochId, epochSigners.map(checksum), fromBlock, toBlock, checksum);
}

async function fillEpochActivation(
  pc: ProtocolConfig,
  status: KmsContextSwitchStatus,
  pendingEpochId: bigint,
  epochSigners: string[],
  fromBlock: number,
  toBlock: number,
  checksum: (address: string) => string,
): Promise<void> {
  status.pendingEpochId = pendingEpochId;
  status.epochSigners = epochSigners;

  const activationConfirmations = await pc.queryFilter(
    pc.filters.EpochActivationConfirmation(pendingEpochId),
    fromBlock,
    toBlock,
  );
  const confirmed = new Set<string>();
  const byDataHash: Record<string, string[]> = {};
  for (const event of activationConfirmations) {
    const signer = checksum(event.args.signer);
    const dataHash: string = event.args.dataHash;
    confirmed.add(signer);
    (byDataHash[dataHash] ??= []).push(signer);
  }
  status.epochSignersConfirmed = [...confirmed];
  status.epochSignersOutstanding = outstanding(epochSigners, confirmed);
  status.epochConfirmationsByDataHash = byDataHash;
  // The epoch activates only when all signers agree on one data hash; more than one hash means the
  // signers disagree on the reshared key/CRS material and the epoch cannot activate as-is.
  status.epochConfirmationsDiverged = Object.keys(byDataHash).length > 1;

  status.epochState = status.activeEpochId === pendingEpochId ? 'ACTIVE' : 'PENDING';
  if (status.epochState === 'ACTIVE') {
    status.contextState = 'ACTIVE';
    status.fullyLive = await pc.isValidEpochForContext(status.activeContextId, status.activeEpochId);
  }
}

function printStatus(status: KmsContextSwitchStatus): void {
  console.log('KMS context-switch status\n');
  console.log('protocolConfig:', status.protocolConfig);
  console.log('scanned blocks:', `${status.scannedFromBlock}..${status.scannedToBlock}`);
  console.log('active (contextId, epochId):', `(${status.activeContextId}, ${status.activeEpochId})`);
  console.log('flow:', status.flow);

  if (status.flow === 'idle') {
    console.log('fullyLive:', status.fullyLive);
    console.log('\nNo context switch or epoch rotation is in progress.');
    return;
  }

  if (status.aborted) {
    console.log('ABORTED:', status.abortReason);
  }

  if (status.flow === 'context-switch') {
    console.log('\n-- Context creation phase --');
    console.log('pendingContextId:', status.pendingContextId?.toString());
    console.log('previousContextId:', status.previousContextId?.toString());
    console.log('contextState:', status.contextState);
    // The new committee comes from the defining event. It is undefined when that event is out of the
    // scanned range, so print a note instead of undefined count and quorum values.
    if (status.newSigners) {
      console.log(
        'new tx senders confirmed:',
        `${status.newTxSendersConfirmed?.length}/${status.newTxSenders?.length}`,
      );
      if (status.newTxSendersOutstanding && status.newTxSendersOutstanding.length > 0) {
        console.log('  outstanding new tx senders:', status.newTxSendersOutstanding.join(', '));
      }
      console.log(
        'previous tx senders confirmed:',
        `${status.previousConfirmationCount} (need >= ${status.previousTxSenderThreshold} = n - t)`,
      );
      if (status.stuckBelowPreviousThreshold) {
        console.log('  ⚠ stuck below the (n - t) old-side confirmation target');
      }
      console.log('creation quorum reached:', status.contextCreationQuorumReached);
    } else {
      console.log('old-side confirmation target:', `need >= ${status.previousTxSenderThreshold} (n - t)`);
      console.log('new committee not in scanned range, confirmation count and quorum unknown');
    }
  }

  if (status.pendingEpochId !== undefined) {
    console.log('\n-- Epoch activation phase --');
    console.log('pendingEpochId:', status.pendingEpochId.toString());
    console.log('epochState:', status.epochState);
    console.log('epoch signers confirmed:', `${status.epochSignersConfirmed?.length}/${status.epochSigners?.length}`);
    if (status.epochSignersOutstanding && status.epochSignersOutstanding.length > 0) {
      console.log('  outstanding epoch signers:', status.epochSignersOutstanding.join(', '));
    }
    if (status.epochConfirmationsDiverged) {
      console.log('  ⚠ signers confirmed different data hashes; epoch cannot activate until they agree');
    }
  }

  console.log('\nfullyLive:', status.fullyLive);
}

task(
  'task:kmsContextSwitchStatus',
  'Reports the live progress of a KMS context switch / epoch rotation by indexing ProtocolConfig events (read-only)',
)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'fromBlock',
    'Block to start scanning confirmation events from (pass the deployment or a recent block on mainnet to bound the scan)',
    0,
    types.int,
  )
  .setAction(async function ({ useInternalProxyAddress, fromBlock }, hre): Promise<void> {
    const target = requireProtocolConfigAddress(useInternalProxyAddress);
    await hre.run('compile:specific', { contract: 'contracts' });
    const status = await inspectKmsContextSwitch(hre, target, fromBlock);
    printStatus(status);
  });
