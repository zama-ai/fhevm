import { Interface } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig } from '../types';
import { buildProtocolConfigContextArgs } from './taskDeploy';
import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// This file defines tasks to drive a KMS context switch / epoch rotation on the canonical (Ethereum)
// ProtocolConfig: build the `defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext`
// governance proposal calldata (DAO path, never broadcasts) or broadcast it directly with the
// deployer key (devnet / test-suite), plus a read-only status task that tracks an in-flight switch
// from events. Inputs come from the `KMS_*` env vars (reusing `buildProtocolConfigContextArgs`).

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
// (current + 1) — the value the operator sets as the Gateway proposal's KMS_CONTEXT_ID so the host
// and Gateway proposals stay aligned.
export async function predictNewKmsContextId(
  hre: HardhatRuntimeEnvironment,
  protocolConfigAddress: string,
): Promise<bigint> {
  const protocolConfig = (await hre.ethers.getContractAt(
    'ProtocolConfig',
    protocolConfigAddress,
  )) as unknown as ProtocolConfig;
  return (await protocolConfig.getCurrentKmsContextId()) + 1n;
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

    // The host derives the new context id on-chain as current + 1. When a ProtocolConfig address is
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

// Reconstructs the in-progress phase of a KMS context switch / epoch rotation purely from events
// plus the current active-context view. ProtocolConfig exposes no getter for the intermediate
// `Created` state or the live confirmation tally, and the new (pending) context's signer set is not
// readable via `getKmsSignersForContext` until it is `Active` — so the new set is taken from the
// `NewKmsContext` event, while the old-side `(n - t)` target is read from views on the still-
// active previous context.
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

  if (latestNewContext && latestNewContext.args.contextId > activeContextId) {
    await fillContextSwitch(pc, status, latestNewContext, newEpochEvents, fromBlock, toBlock, checksum);
  } else if (
    latestNewEpoch &&
    latestNewEpoch.args.epochId > activeEpochId &&
    latestNewEpoch.args.kmsContextId === activeContextId
  ) {
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
  newContextEvent: {
    args: {
      contextId: bigint;
      previousContextId: bigint;
      kmsNodeParams: { txSenderAddress: string; signerAddress: string }[];
    };
  },
  newEpochEvents: { args: { kmsContextId: bigint; epochId: bigint } }[],
  fromBlock: number,
  toBlock: number,
  checksum: (address: string) => string,
): Promise<void> {
  status.flow = 'context-switch';
  const pendingContextId = newContextEvent.args.contextId;
  const previousContextId = newContextEvent.args.previousContextId;
  status.pendingContextId = pendingContextId;
  status.previousContextId = previousContextId;

  // An abort/destroy of the pending context after it was issued means the switch is no longer in
  // flight and must be re-proposed.
  const contextAborted = await pc.queryFilter(pc.filters.PendingContextAborted(pendingContextId), fromBlock, toBlock);
  const contextDestroyed = await pc.queryFilter(pc.filters.KmsContextDestroyed(pendingContextId), fromBlock, toBlock);
  if (contextAborted.length > 0) {
    status.aborted = true;
    status.abortReason = 'context-aborted';
  } else if (contextDestroyed.length > 0) {
    status.aborted = true;
    status.abortReason = 'context-destroyed';
  }

  // New committee: read from the event (the pending context is not yet readable via views).
  // Signers drive the epoch phase; tx-senders drive the creation-confirmation tally.
  const newSigners = newContextEvent.args.kmsNodeParams.map((node) => checksum(node.signerAddress));
  status.newSigners = newSigners;
  const newTxSenders = newContextEvent.args.kmsNodeParams.map((node) => checksum(node.txSenderAddress));
  status.newTxSenders = newTxSenders;

  // Old-side (n - t) target (floored at 1): the previous context is still active, so its views are readable.
  const previousSigners: string[] = await pc.getKmsSignersForContext(previousContextId);
  const previousMpcThreshold: bigint = await pc.getMpcThresholdForContext(previousContextId);
  status.previousTxSenderThreshold = Math.max(previousSigners.length - Number(previousMpcThreshold), 1);

  // Tally creation confirmations from events.
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

  // `Created` is signaled by the NewKmsEpoch emitted once the creation quorum is reached; it also
  // reveals the pending epoch id, which has no view getter.
  const pendingEpochEvent = newEpochEvents.find((event) => event.args.kmsContextId === pendingContextId);
  status.contextState = pendingEpochEvent ? 'CREATED' : 'PENDING';

  if (pendingEpochEvent) {
    await fillEpochActivation(pc, status, pendingEpochEvent.args.epochId, newSigners, fromBlock, toBlock, checksum);
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

  const epochAborted = await pc.queryFilter(
    pc.filters.PendingEpochAborted(undefined, pendingEpochId),
    fromBlock,
    toBlock,
  );
  if (epochAborted.length > 0 && !status.aborted) {
    status.aborted = true;
    status.abortReason = 'epoch-aborted';
  }

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
    console.log('new tx senders confirmed:', `${status.newTxSendersConfirmed?.length}/${status.newTxSenders?.length}`);
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
