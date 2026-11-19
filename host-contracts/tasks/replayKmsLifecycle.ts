import type { Provider } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig, ProtocolConfigReplica } from '../types';
import { faultTolerantQuorum, getProtocolConfigReplicaInterface, requireProtocolConfigAddress } from './kmsContext';
import { addCanonicalSourceParams, canonicalProvider } from './mirrorKmsContext';
import type { KmsNode, KmsThresholds } from './protocolConfigMirror';
import { toJsonString } from './utils/upgradeProposal';

// Use the replay task when a replica is behind the canonical ProtocolConfig. It reads the lifecycle
// events of the canonical (Ethereum) ProtocolConfig. It builds the `submit*` calldata that brings a
// ProtocolConfigReplica up to date. The replica verifies every signature, so anyone can send the
// calldata. The task never broadcasts.

type LogPosition = { readonly blockNumber: number; readonly logIndex: number };

type DestructionConfirmation = {
  readonly destroyedId: bigint;
  readonly signer: string;
  readonly signature: string;
  readonly extraData: string;
};

type DestructionQuorum = LogPosition & { readonly destroyedId: bigint; readonly extraData: string };

/** The decoded lifecycle events of the canonical ProtocolConfig in the scanned block range. */
export type CanonicalLifecycleEvents = {
  /** `NewKmsContext`, with the nodes projected onto the stored KmsNode shape. */
  readonly newContexts: readonly {
    readonly contextId: bigint;
    readonly previousContextId: bigint;
    readonly nodes: readonly KmsNode[];
    readonly thresholds: KmsThresholds;
  }[];
  /** `NewKmsEpoch`. */
  readonly newEpochs: readonly { readonly epochId: bigint; readonly previousEpochId: bigint }[];
  /** `ActivateEpoch`. */
  readonly epochActivations: readonly (LogPosition & { readonly contextId: bigint; readonly epochId: bigint })[];
  /** `KmsContextCreationConfirmation`. */
  readonly contextCreationConfirmations: readonly {
    readonly contextId: bigint;
    readonly signer: string;
    readonly signature: string;
    readonly extraData: string;
  }[];
  /** `EpochActivationConfirmation`. */
  readonly epochActivationConfirmations: readonly {
    readonly epochId: bigint;
    readonly epochMaterialHash: string;
    readonly signature: string;
    readonly extraData: string;
  }[];
  /** `KmsContextDestructionConfirmation`. */
  readonly contextDestructionConfirmations: readonly DestructionConfirmation[];
  /** `KmsContextDestructionQuorumReached`. */
  readonly contextDestructionQuorums: readonly DestructionQuorum[];
  /** `KmsEpochDestructionConfirmation`. */
  readonly epochDestructionConfirmations: readonly DestructionConfirmation[];
  /** `KmsEpochDestructionQuorumReached`. */
  readonly epochDestructionQuorums: readonly DestructionQuorum[];
};

/** The replica state that the replay starts from. */
type ReplicaStartState = {
  /** The active context ID of the replica. */
  readonly contextId: bigint;
  /** The active epoch ID of the replica. */
  readonly epochId: bigint;
  /** The signers of the active committee, as the replica stores them. */
  readonly signers: readonly string[];
  /** The MPC threshold of the active committee. */
  readonly mpcThreshold: bigint;
  /** The destroyed context IDs of the scan that are live on the replica. */
  readonly liveContextIds: ReadonlySet<bigint>;
  /**
   * The destroyed epoch IDs of the scan that the replica reports as valid. The replica has no getter
   * for the state of one epoch, so an Active epoch can be absent from this set.
   */
  readonly activeEpochIds: ReadonlySet<bigint>;
};

type KmsEpochSubmission = {
  readonly epochId: bigint;
  readonly epochMaterialHash: string;
  readonly signatures: readonly string[];
  readonly extraData: string;
};

type KmsContextSubmission = {
  readonly contextId: bigint;
  readonly nodes: readonly KmsNode[];
  readonly thresholds: KmsThresholds;
  readonly signatures: readonly string[];
  readonly extraData: string;
};

/** One call to a `submit*` function of ProtocolConfigReplica, with its arguments in ABI order. */
type ReplayCall =
  | {
      readonly functionName: 'submitKmsContextAndEpoch';
      readonly args: readonly [KmsContextSubmission, KmsEpochSubmission];
    }
  | { readonly functionName: 'submitKmsEpoch'; readonly args: readonly [KmsEpochSubmission] }
  | {
      readonly functionName: 'submitKmsContextDestruction' | 'submitKmsEpochDestruction';
      readonly args: readonly [destroyedId: bigint, signatures: readonly string[], extraData: string];
    };

/** One canonical destruction that the plan does not replay. */
type SkippedDestruction = { readonly target: 'context' | 'epoch'; readonly destroyedId: bigint } & (
  | {
      /** `notLive`: the context is not live on the replica. `unknownState`: the task cannot read the state of the epoch. */
      readonly reason: 'notLive' | 'unknownState';
    }
  | {
      /** The active committee gave fewer signatures than its quorum. */
      readonly reason: 'belowQuorum';
      readonly signatureCount: number;
      readonly quorum: number;
    }
);

/** The replay of the canonical lineage onto one replica. */
type ReplicaReplayPlan = {
  /** The calls to send, in lineage order. */
  readonly calls: readonly ReplayCall[];
  /** The canonical destructions that the plan does not replay. */
  readonly skipped: readonly SkippedDestruction[];
  /** The active context ID of the replica after the calls. */
  readonly contextId: bigint;
  /** The active epoch ID of the replica after the calls. */
  readonly epochId: bigint;
};

/** A canonical activation that the scanned events do not chain to the state the replay reached. */
type LineageGap = {
  readonly _tag: 'lineageGap';
  readonly target: 'context' | 'epoch';
  /** The context ID or epoch ID of the activation. */
  readonly id: bigint;
  /** The context ID or epoch ID that the replay reached. */
  readonly reachedId: bigint;
};

type Committee = { readonly signers: readonly string[]; readonly mpcThreshold: bigint };

/**
 * Plans the `submit*` calls that bring a replica up to date with the canonical lineage.
 *
 * The plan follows the canonical activations and destruction quorums in log order. The replica
 * verifies context creation signatures and destruction signatures against its active committee. The
 * plan keeps only the signatures of that committee. The replica verifies epoch activation signatures
 * against the committee of the epoch. The plan sends all of them. A context switch or resharing that
 * canonical aborted has no activation, so the plan ignores its confirmations.
 *
 * @param events - The canonical lifecycle events of the scanned block range.
 * @param replica - The replica state to start from.
 * @returns The plan, or the lineage gap that stops it.
 */
export function planReplicaReplay(
  events: CanonicalLifecycleEvents,
  replica: ReplicaStartState,
): ({ readonly _tag: 'ok' } & ReplicaReplayPlan) | LineageGap {
  let contextId = replica.contextId;
  let epochId = replica.epochId;
  let committee: Committee = { signers: replica.signers, mpcThreshold: replica.mpcThreshold };
  const liveContextIds = new Set([...replica.liveContextIds, replica.contextId]);
  const activeEpochIds = new Set([...replica.activeEpochIds, replica.epochId]);
  const calls: ReplayCall[] = [];
  const skipped: SkippedDestruction[] = [];

  const steps = [
    ...events.epochActivations
      // The replica rejects an epoch ID that is not above its active epoch ID.
      .filter((activation) => activation.epochId > replica.epochId)
      .map((activation) => ({ kind: 'activation' as const, ...activation })),
    ...events.contextDestructionQuorums.map((quorum) => ({ kind: 'contextDestruction' as const, ...quorum })),
    ...events.epochDestructionQuorums.map((quorum) => ({ kind: 'epochDestruction' as const, ...quorum })),
  ].sort((a, b) => a.blockNumber - b.blockNumber || a.logIndex - b.logIndex);

  for (const step of steps) {
    if (step.kind !== 'activation') {
      const isContext = step.kind === 'contextDestruction';
      const skip = { target: isContext ? ('context' as const) : ('epoch' as const), destroyedId: step.destroyedId };
      const liveIds = isContext ? liveContextIds : activeEpochIds;
      const confirmations = isContext ? events.contextDestructionConfirmations : events.epochDestructionConfirmations;
      const signatures = confirmations
        .filter(
          (confirmation) =>
            confirmation.destroyedId === step.destroyedId &&
            confirmation.extraData === step.extraData &&
            committee.signers.includes(confirmation.signer),
        )
        .map((confirmation) => confirmation.signature);
      const quorum = faultTolerantQuorum(committee.signers.length, committee.mpcThreshold);
      if (!liveIds.has(step.destroyedId)) {
        skipped.push({ ...skip, reason: isContext ? 'notLive' : 'unknownState' });
      } else if (signatures.length < quorum) {
        skipped.push({ ...skip, reason: 'belowQuorum', signatureCount: signatures.length, quorum });
      } else {
        liveIds.delete(step.destroyedId);
        calls.push({
          functionName: isContext ? 'submitKmsContextDestruction' : 'submitKmsEpochDestruction',
          args: [step.destroyedId, signatures, step.extraData],
        });
      }
      continue;
    }

    // The activation signatures commit to the previous epoch ID, so the pending epoch must follow the
    // epoch that the replay reached.
    const newEpoch = events.newEpochs.find((event) => event.epochId === step.epochId);
    if (newEpoch?.previousEpochId !== epochId) {
      return { _tag: 'lineageGap', target: 'epoch', id: step.epochId, reachedId: epochId };
    }
    // Canonical activates an epoch only when every signer of its committee confirmed one digest, and
    // each signer confirms once. All the confirmations of an activated epoch therefore agree.
    const epochConfirmations = events.epochActivationConfirmations.filter(
      (confirmation) => confirmation.epochId === step.epochId,
    );
    const epoch: KmsEpochSubmission = {
      epochId: step.epochId,
      epochMaterialHash: epochConfirmations[0].epochMaterialHash,
      signatures: epochConfirmations.map((confirmation) => confirmation.signature),
      extraData: epochConfirmations[0].extraData,
    };

    if (step.contextId === contextId) {
      calls.push({ functionName: 'submitKmsEpoch', args: [epoch] });
    } else {
      const newContext = events.newContexts.find((event) => event.contextId === step.contextId);
      if (newContext?.previousContextId !== contextId) {
        return { _tag: 'lineageGap', target: 'context', id: step.contextId, reachedId: contextId };
      }
      const newSigners = newContext.nodes.map((node) => node.signerAddress);
      const creationConfirmations = events.contextCreationConfirmations.filter(
        (confirmation) => confirmation.contextId === step.contextId,
      );
      // Canonical counts confirmations per digest and needs every new signer on the quorum digest. Each
      // signer confirms once, so the extraData of any new signer is the quorum extraData. The status
      // task (fillContextSwitch in kmsContext.ts) finds the same group from the quorum rule.
      const [{ extraData }] = creationConfirmations.filter((confirmation) => newSigners.includes(confirmation.signer));
      calls.push({
        functionName: 'submitKmsContextAndEpoch',
        args: [
          {
            contextId: step.contextId,
            nodes: newContext.nodes,
            thresholds: newContext.thresholds,
            // The replica reverts on a signer outside the previous committee.
            signatures: creationConfirmations
              .filter(
                (confirmation) =>
                  confirmation.extraData === extraData && committee.signers.includes(confirmation.signer),
              )
              .map((confirmation) => confirmation.signature),
            extraData,
          },
          epoch,
        ],
      });
      contextId = step.contextId;
      committee = { signers: newSigners, mpcThreshold: newContext.thresholds.mpc };
      liveContextIds.add(contextId);
    }
    epochId = step.epochId;
    activeEpochIds.add(epochId);
  }

  return { _tag: 'ok', calls, skipped, contextId, epochId };
}

/** A replay with the calldata of each call and the end of the scanned canonical block range. */
type ReplicaReplay = {
  /** The calls to send, in lineage order. */
  readonly calls: readonly (ReplayCall & { readonly calldata: string })[];
  /** The canonical destructions that the replay does not contain. */
  readonly skipped: readonly SkippedDestruction[];
  /** The last canonical block of the scan, which is the latest finalized block. */
  readonly toBlock: number;
};

const LINEAGE_GAP_ADVICE =
  'Pass a lower --from-block. If the replica is not on the canonical lineage, use the owner mirror tasks.';

/**
 * Reads the canonical lifecycle events and the replica state, then builds the replay calldata.
 *
 * The scan ends at the latest finalized canonical block. The submit path cannot undo a call, and a
 * reorg can remove an event that is not finalized.
 *
 * @param hre - The Hardhat runtime environment, whose network holds the replica.
 * @param options - The canonical provider and address, the replica address and the first block to scan.
 * @returns The calls with their calldata, the skipped destructions and the last scanned block.
 * @throws If the canonical RPC has no finalized block, or the replay does not reach the canonical state.
 */
export async function buildReplicaReplay(
  hre: HardhatRuntimeEnvironment,
  {
    canonicalProvider,
    canonicalProtocolConfigAddress,
    replicaAddress,
    fromBlock,
  }: {
    readonly canonicalProvider: Provider;
    readonly canonicalProtocolConfigAddress: string;
    readonly replicaAddress: string;
    readonly fromBlock: number;
  },
): Promise<ReplicaReplay> {
  const iface = await getProtocolConfigReplicaInterface(hre);

  const finalizedBlock = await canonicalProvider.getBlock('finalized');
  if (finalizedBlock === null) {
    throw new Error('Canonical RPC returned no finalized block.');
  }
  const toBlock = finalizedBlock.number;

  const canonical = (await hre.ethers.getContractAt('ProtocolConfig', canonicalProtocolConfigAddress)).connect(
    canonicalProvider,
  ) as ProtocolConfig;
  const position = (log: { blockNumber: number; index: number }): LogPosition => ({
    blockNumber: log.blockNumber,
    logIndex: log.index,
  });
  const [
    newContexts,
    newEpochs,
    epochActivations,
    contextCreationConfirmations,
    epochActivationConfirmations,
    contextDestructionConfirmations,
    contextDestructionQuorums,
    epochDestructionConfirmations,
    epochDestructionQuorums,
  ] = await Promise.all([
    canonical.queryFilter(canonical.filters.NewKmsContext(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.NewKmsEpoch(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.ActivateEpoch(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.KmsContextCreationConfirmation(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.EpochActivationConfirmation(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.KmsContextDestructionConfirmation(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.KmsContextDestructionQuorumReached(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.KmsEpochDestructionConfirmation(), fromBlock, toBlock),
    canonical.queryFilter(canonical.filters.KmsEpochDestructionQuorumReached(), fromBlock, toBlock),
  ]);
  const events: CanonicalLifecycleEvents = {
    newContexts: newContexts.map(({ args }) => ({
      contextId: args.contextId,
      previousContextId: args.previousContextId,
      // The same projection as ProtocolConfig._toKmsNodes, in the same order. The context creation
      // signatures commit to the hash of these nodes and thresholds.
      nodes: args.kmsNodeParams.map((node) => ({
        txSenderAddress: node.txSenderAddress,
        signerAddress: node.signerAddress,
        ipAddress: node.ipAddress,
        storageUrl: node.storageUrl,
      })),
      thresholds: {
        publicDecryption: args.thresholds.publicDecryption,
        userDecryption: args.thresholds.userDecryption,
        kmsGen: args.thresholds.kmsGen,
        mpc: args.thresholds.mpc,
      },
    })),
    newEpochs: newEpochs.map(({ args }) => ({ epochId: args.epochId, previousEpochId: args.previousEpochId })),
    epochActivations: epochActivations.map((log) => ({
      ...position(log),
      contextId: log.args.kmsContextId,
      epochId: log.args.epochId,
    })),
    contextCreationConfirmations: contextCreationConfirmations.map(({ args }) => ({
      contextId: args.kmsContextId,
      signer: args.signer,
      signature: args.signature,
      extraData: args.extraData,
    })),
    epochActivationConfirmations: epochActivationConfirmations.map(({ args }) => ({
      epochId: args.epochId,
      epochMaterialHash: args.epochMaterialHash,
      signature: args.signature,
      extraData: args.extraData,
    })),
    contextDestructionConfirmations: contextDestructionConfirmations.map(({ args }) => ({
      destroyedId: args.destroyedContextId,
      signer: args.signer,
      signature: args.signature,
      extraData: args.extraData,
    })),
    contextDestructionQuorums: contextDestructionQuorums.map((log) => ({
      ...position(log),
      destroyedId: log.args.destroyedContextId,
      extraData: log.args.extraData,
    })),
    epochDestructionConfirmations: epochDestructionConfirmations.map(({ args }) => ({
      destroyedId: args.destroyedEpochId,
      signer: args.signer,
      signature: args.signature,
      extraData: args.extraData,
    })),
    epochDestructionQuorums: epochDestructionQuorums.map((log) => ({
      ...position(log),
      destroyedId: log.args.destroyedEpochId,
      extraData: log.args.extraData,
    })),
  };

  const replica = (await hre.ethers.getContractAt(
    'ProtocolConfigReplica',
    replicaAddress,
  )) as unknown as ProtocolConfigReplica;
  const [contextId, epochId] = await replica.getCurrentKmsContextAndEpoch();
  const [signers, mpcThreshold] = await Promise.all([
    replica.getKmsSignersForContext(contextId),
    replica.getMpcThresholdForContext(contextId),
  ]);
  // The replica has no list of its live IDs, so the task asks the replica about each destroyed ID.
  const liveContextIds = new Set<bigint>();
  for (const { destroyedId } of events.contextDestructionQuorums) {
    if (await replica.isLiveKmsContext(destroyedId)) {
      liveContextIds.add(destroyedId);
    }
  }
  // The replica has no getter for the state of one epoch. isValidEpochForContext needs the context
  // of the epoch, which only an activation in the scanned range gives.
  const activeEpochIds = new Set<bigint>();
  for (const { destroyedId } of events.epochDestructionQuorums) {
    const activation = events.epochActivations.find((event) => event.epochId === destroyedId);
    if (activation !== undefined && (await replica.isValidEpochForContext(activation.contextId, destroyedId))) {
      activeEpochIds.add(destroyedId);
    }
  }

  const plan = planReplicaReplay(events, {
    contextId,
    epochId,
    signers,
    mpcThreshold,
    liveContextIds,
    activeEpochIds,
  });
  if (plan._tag === 'lineageGap') {
    throw new Error(
      `Cannot replay ${plan.target} ${plan.id}: the scanned canonical events do not chain it to ${plan.target} ${plan.reachedId} of the replica. ${LINEAGE_GAP_ADVICE}`,
    );
  }
  // A scan that starts after an activation that the replica does not have gives a plan that stops
  // before the canonical state.
  const [canonicalContextId, canonicalEpochId] = await canonical.getCurrentKmsContextAndEpoch({ blockTag: toBlock });
  if (plan.contextId !== canonicalContextId || plan.epochId !== canonicalEpochId) {
    throw new Error(
      `The replay ends at context ${plan.contextId} and epoch ${plan.epochId}, but canonical is at context ${canonicalContextId} and epoch ${canonicalEpochId} at block ${toBlock}. ${LINEAGE_GAP_ADVICE} If the replica is ahead of this finalized block, wait for finality.`,
    );
  }
  return {
    calls: plan.calls.map((call) => ({ ...call, calldata: iface.encodeFunctionData(call.functionName, call.args) })),
    skipped: plan.skipped,
    toBlock,
  };
}

function describeSkippedDestruction(skip: SkippedDestruction): string {
  const name = `${skip.target} ${skip.destroyedId}`;
  switch (skip.reason) {
    case 'notLive':
      return `Destruction of ${name}: the ${skip.target} is not live on the replica.`;
    case 'unknownState':
      return `Destruction of ${name}: the task cannot read the state of this epoch on the replica. The submit path can still accept the call.`;
    case 'belowQuorum':
      return `Destruction of ${name}: the active committee gave ${skip.signatureCount} signatures, below its quorum of ${skip.quorum}.`;
  }
}

addCanonicalSourceParams(
  task(
    'task:buildReplicaReplayCalldata',
    'Builds the ProtocolConfigReplica submit* calldata that replays the canonical KMS lifecycle. Reads the canonical lifecycle events. Never broadcasts.',
  ),
)
  .addOptionalParam(
    'fromBlock',
    'Canonical block to start the event scan from. It must be at or before the block that defined the first context or epoch that the replica does not have.',
    0,
    types.int,
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfigReplica address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .setAction(async function (
    {
      canonicalRpcUrl,
      canonicalProtocolConfigAddress,
      fromBlock,
      useInternalProxyAddress,
    }: {
      canonicalRpcUrl: string;
      canonicalProtocolConfigAddress: string;
      fromBlock: number;
      useInternalProxyAddress: boolean;
    },
    hre,
  ): Promise<void> {
    const target = requireProtocolConfigAddress(useInternalProxyAddress);
    const replay = await buildReplicaReplay(hre, {
      canonicalProvider: canonicalProvider(hre, canonicalRpcUrl),
      canonicalProtocolConfigAddress,
      replicaAddress: target,
      fromBlock,
    });

    console.log('ProtocolConfigReplica replay');
    console.log('  target:', target);
    console.log('  scanned canonical blocks:', `${fromBlock}..${replay.toBlock}`);
    replay.calls.forEach(({ functionName, args, calldata }, i) => {
      console.log(`\n${i + 1}. ${functionName}`);
      console.log('  decodedArgs:', toJsonString(args));
      console.log('  calldata:', calldata);
    });
    if (replay.calls.length === 0) {
      console.log('\nNothing to replay.');
    }
    if (replay.skipped.length > 0) {
      console.log(
        '\nSkipped destructions. The owner can apply one with task:mirrorKmsContextDestruction or task:mirrorKmsEpochDestruction:',
      );
      replay.skipped.forEach((skip) => console.log(`  - ${describeSkippedDestruction(skip)}`));
    }
  });
