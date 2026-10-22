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

// Builds a calldata-only ABI for `contractName` without a deployer key, so the `build*` tasks never
// need a signer.
async function getContractInterface(hre: HardhatRuntimeEnvironment, contractName: string): Promise<Interface> {
  await hre.run('compile:specific', { contract: 'contracts' });
  const artifact = await hre.artifacts.readArtifact(contractName);
  return new hre.ethers.Interface(artifact.abi);
}

export async function getProtocolConfigInterface(hre: HardhatRuntimeEnvironment): Promise<Interface> {
  return getContractInterface(hre, 'ProtocolConfig');
}

export async function getProtocolConfigReplicaInterface(hre: HardhatRuntimeEnvironment): Promise<Interface> {
  return getContractInterface(hre, 'ProtocolConfigReplica');
}

// Resolves the ProtocolConfig proxy address from env (or the addresses directory when
// `useInternalProxyAddress` is set), returning undefined when none is configured.
export function resolveProtocolConfigAddress(useInternalProxyAddress: boolean): string | undefined {
  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const address = process.env[PROTOCOL_CONFIG_ADDRESS_ENV_VAR];
  return address && address.trim() !== '' ? address : undefined;
}

export function requireProtocolConfigAddress(useInternalProxyAddress: boolean): string {
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
export async function broadcast(hre: HardhatRuntimeEnvironment, target: string, calldata: string): Promise<string> {
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
  // The confirmed and outstanding lists and the count below span every digest.
  newSignersConfirmed?: string[];
  newSignersOutstanding?: string[];
  previousSignersConfirmed?: string[];
  previousConfirmationCount?: number;
  previousSignerThreshold?: number; // the (n - t) old-side quorum target
  contextCreationQuorumReached?: boolean;
  stuckBelowPreviousThreshold?: boolean;
  // The contract counts confirmations per signed digest, which varies only with extraData here.
  contextCreationConfirmationsDiverged?: boolean;

  // Epoch-activation phase (present once an epoch id is observable: same-set, or context CREATED+).
  pendingEpochId?: bigint;
  epochState?: EpochState;
  epochSigners?: string[];
  // Confirmed and outstanding span every digest.
  epochSignersConfirmed?: string[];
  epochSignersOutstanding?: string[];
  // Keyed by `${epochMaterialHash}:${extraData}`, the inputs of the signed digest that vary per signer.
  epochConfirmationsByDigest?: Record<string, string[]>;
  epochConfirmationsDiverged?: boolean;

  // Destruction confirmations of the active committee. Contexts: every KmsContextDestroyed in the scanned
  // window, plus the pending context of an aborted switch. Epochs: every KmsEpochDestroyed in the scanned window.
  contextDestructions?: DestructionConfirmationStatus[];
  epochDestructions?: DestructionConfirmationStatus[];
}

export interface DestructionConfirmationStatus {
  destroyedId: bigint;
  // Both lists cover only the active committee.
  signersConfirmed: string[];
  signersOutstanding: string[];
  // True when any destruction quorum event exists for `destroyedId`.
  quorumReached: boolean;
}

// The active committee, the signer set the contract accepts for destruction confirmations.
interface ActiveCommittee {
  signers: string[]; // checksummed
  threshold: number; // n - t, floored at 1
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

  // A proxy that is not initialized has context 0 and no destruction to report.
  const committee: ActiveCommittee =
    activeContextId > 0n ? await fetchActiveCommittee(pc, activeContextId, checksum) : { signers: [], threshold: 0 };

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
      committee,
    );
  } else if (sameSetRotationPending) {
    await fillSameSetRotation(pc, status, latestNewEpoch.args.epochId, committee.signers, fromBlock, toBlock, checksum);
  } else {
    // Nothing in flight: the latest-issued context and epoch are already the active ones.
    status.flow = 'idle';
    status.fullyLive =
      activeContextId > 0n && activeEpochId > 0n && (await pc.isValidEpochForContext(activeContextId, activeEpochId));
  }

  const [
    destroyedContextEvents,
    contextConfirmations,
    contextQuorums,
    destroyedEpochEvents,
    epochConfirmations,
    epochQuorums,
  ] = await Promise.all([
    pc.queryFilter(pc.filters.KmsContextDestroyed(), fromBlock, toBlock),
    pc.queryFilter(pc.filters.KmsContextDestructionConfirmation(), fromBlock, toBlock),
    pc.queryFilter(pc.filters.KmsContextDestructionQuorumReached(), fromBlock, toBlock),
    pc.queryFilter(pc.filters.KmsEpochDestroyed(), fromBlock, toBlock),
    pc.queryFilter(pc.filters.KmsEpochDestructionConfirmation(), fromBlock, toBlock),
    pc.queryFilter(pc.filters.KmsEpochDestructionQuorumReached(), fromBlock, toBlock),
  ]);
  const destroyedContextIds = new Set(destroyedContextEvents.map((event) => event.args.kmsContextId));
  // The task reads an aborted switch from state, so its context is reported even when the destruction
  // happened before `fromBlock`.
  if (status.aborted) {
    destroyedContextIds.add(latestKmsContextId);
  }
  status.contextDestructions = [...destroyedContextIds].map((contextId) =>
    destructionConfirmationStatus(
      contextId,
      contextConfirmations.filter((confirmation) => confirmation.args.destroyedContextId === contextId),
      contextQuorums.some((quorum) => quorum.args.destroyedContextId === contextId),
      committee.signers,
      checksum,
    ),
  );
  status.epochDestructions = destroyedEpochEvents.map((event) =>
    destructionConfirmationStatus(
      event.args.epochId,
      epochConfirmations.filter((confirmation) => confirmation.args.destroyedEpochId === event.args.epochId),
      epochQuorums.some((quorum) => quorum.args.destroyedEpochId === event.args.epochId),
      committee.signers,
      checksum,
    ),
  );

  return status;
}

async function fetchActiveCommittee(
  pc: ProtocolConfig,
  activeContextId: bigint,
  checksum: (address: string) => string,
): Promise<ActiveCommittee> {
  const signers = (await pc.getKmsSignersForContext(activeContextId)).map(checksum);
  // Floored at 1, like the creation quorum in ProtocolConfig, so `t = n` does not print a target of 0.
  const threshold = Math.max(1, signers.length - Number(await pc.getMpcThresholdForContext(activeContextId)));
  return { signers, threshold };
}

// Groups the checksummed signers of `confirmations` by `digestKey`. `digestKey` returns the signed fields
// that vary between confirmations, so equal keys mean equal digests. Each key leaves out the signed fields
// that are fixed for one operation:
// - ContextCreationConfirmation: previousContextId, newContextId and nodeConfigHash. The in-flight guard
//   keeps the active context fixed, and the pending context and its node set never change.
// - EpochActivationConfirmation: contextId, previousEpochId and epochId. They are fixed for one pending epoch.
// A new signed field that can differ between signers of one operation must also go into its key here.
function groupSignersByDigest<E extends { args: { signer: string } }>(
  confirmations: E[],
  digestKey: (confirmation: E) => string,
  checksum: (address: string) => string,
): Record<string, string[]> {
  const byDigest: Record<string, string[]> = {};
  for (const confirmation of confirmations) {
    (byDigest[digestKey(confirmation)] ??= []).push(checksum(confirmation.args.signer));
  }
  return byDigest;
}

// Reports the active committee's destruction confirmations of `destroyedId`.
function destructionConfirmationStatus(
  destroyedId: bigint,
  confirmations: { args: { signer: string } }[],
  quorumReached: boolean,
  activeSigners: string[],
  checksum: (address: string) => string,
): DestructionConfirmationStatus {
  const active = new Set(activeSigners);
  const signersConfirmed = confirmations
    .map((confirmation) => checksum(confirmation.args.signer))
    .filter((signer) => active.has(signer));
  return {
    destroyedId,
    signersConfirmed,
    signersOutstanding: outstanding(activeSigners, new Set(signersConfirmed)),
    quorumReached,
  };
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
  committee: ActiveCommittee,
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

  // The old-side target is n - t of the previous committee, which is the active one during an in-flight switch.
  status.previousSignerThreshold = committee.threshold;

  // New committee: read it from the event, because views cannot enumerate the pending context. When
  // the defining event is outside the scanned window, the committee, its confirmation count, and the
  // derived quorum flags are unknowable. They therefore stay undefined instead of reporting a value
  // off empty data.
  if (newContextEvent) {
    const newSigners = newContextEvent.args.kmsNodeParams.map((node) => checksum(node.signerAddress));
    status.newSigners = newSigners;
    // The previous committee is the active context during an in-flight switch. A signer in both
    // committees counts toward both sides, as in the contract.
    const previousSigners = new Set(committee.signers);

    // Count creation confirmations from events.
    const creationConfirmations = await pc.queryFilter(
      pc.filters.KmsContextCreationConfirmation(pendingContextId),
      fromBlock,
      toBlock,
    );
    // The contract counts per digest, so the quorum is checked per extraData group.
    const groups = Object.values(
      groupSignersByDigest(creationConfirmations, (event) => event.args.extraData, checksum),
    );
    const confirmed = new Set(groups.flat());
    status.newSignersConfirmed = newSigners.filter((signer) => confirmed.has(signer));
    status.newSignersOutstanding = outstanding(newSigners, confirmed);
    status.previousSignersConfirmed = committee.signers.filter((signer) => confirmed.has(signer));
    status.previousConfirmationCount = status.previousSignersConfirmed.length;
    status.contextCreationConfirmationsDiverged = groups.length > 1;

    const previousSignerThreshold = status.previousSignerThreshold;
    status.contextCreationQuorumReached = groups.some(
      (group) =>
        newSigners.every((signer) => group.includes(signer)) &&
        group.filter((signer) => previousSigners.has(signer)).length >= previousSignerThreshold,
    );
    status.stuckBelowPreviousThreshold = status.previousConfirmationCount < previousSignerThreshold;
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
  activeSigners: string[],
  fromBlock: number,
  toBlock: number,
  checksum: (address: string) => string,
): Promise<void> {
  status.flow = 'same-set-rotation';
  // The signer set is unchanged, so the active context's signers are the expected confirmers.
  await fillEpochActivation(pc, status, pendingEpochId, activeSigners, fromBlock, toBlock, checksum);
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
  const byDigest = groupSignersByDigest(
    activationConfirmations,
    (event) => `${event.args.epochMaterialHash}:${event.args.extraData}`,
    checksum,
  );
  const confirmed = new Set(Object.values(byDigest).flat());
  status.epochSignersConfirmed = [...confirmed];
  status.epochSignersOutstanding = outstanding(epochSigners, confirmed);
  status.epochConfirmationsByDigest = byDigest;
  // The epoch activates only when all signers sign one digest. More than one key means the signers
  // disagree on the reshared key/CRS material or on extraData, and the epoch cannot activate as-is.
  status.epochConfirmationsDiverged = Object.keys(byDigest).length > 1;

  status.epochState = status.activeEpochId === pendingEpochId ? 'ACTIVE' : 'PENDING';
  if (status.epochState === 'ACTIVE') {
    status.contextState = 'ACTIVE';
    status.fullyLive = await pc.isValidEpochForContext(status.activeContextId, status.activeEpochId);
  }
}

function printDestruction(destruction: DestructionConfirmationStatus): void {
  console.log(
    `destroyed id ${destruction.destroyedId}:`,
    `${destruction.signersConfirmed.length} active signers confirmed`,
  );
  console.log('  quorum reached:', destruction.quorumReached);
  if (destruction.signersOutstanding.length > 0) {
    console.log('  outstanding active signers:', destruction.signersOutstanding.join(', '));
  }
}

function printDestructions(status: KmsContextSwitchStatus): void {
  if (status.contextDestructions && status.contextDestructions.length > 0) {
    console.log('\n-- Context destruction confirmations --');
    status.contextDestructions.forEach(printDestruction);
  }
  if (status.epochDestructions && status.epochDestructions.length > 0) {
    console.log('\n-- Epoch destruction confirmations --');
    status.epochDestructions.forEach(printDestruction);
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
    printDestructions(status);
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
      console.log('new signers confirmed:', `${status.newSignersConfirmed?.length}/${status.newSigners.length}`);
      if (status.newSignersOutstanding && status.newSignersOutstanding.length > 0) {
        console.log('  outstanding new signers:', status.newSignersOutstanding.join(', '));
      }
      console.log(
        'previous signers confirmed:',
        `${status.previousConfirmationCount} (need >= ${status.previousSignerThreshold} = n - t)`,
      );
      if (status.stuckBelowPreviousThreshold) {
        console.log('  ⚠ stuck below the (n - t) old-side confirmation target');
      }
      if (status.contextCreationConfirmationsDiverged) {
        console.log('  ⚠ signers confirmed different extraData, which splits the per-digest count');
      }
      console.log('creation quorum reached:', status.contextCreationQuorumReached);
    } else {
      console.log('old-side confirmation target:', `need >= ${status.previousSignerThreshold} (n - t)`);
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
      console.log(
        '  ⚠ signers confirmed different epochMaterialHash or extraData. The epoch cannot activate until they agree',
      );
    }
  }

  printDestructions(status);

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
