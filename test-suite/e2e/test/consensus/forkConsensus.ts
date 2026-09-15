import { emitAssertions } from './assertionEvidence';
import { successfulForkReceipt } from './forkRecovery';
import { isTerminalForkChild } from './forkHelper';
import { INSTALL_FORK_GATE_CONTROL, DROP_FORK_GATE_CONTROL, assertRepairDisabledControl, assertReplayHasEffects, replayOutputFingerprint, INSTALL_REPLAY_INSERT_AUDIT, DROP_REPLAY_INSERT_AUDIT, assertReplayAttempts } from './forkRecovery';
/**
 * Fork byte-consensus gate: what a reorg may and may not do to ciphertext
 * bytes, under the revised RFC 019.
 *
 * This suite asserts the OPPOSITE of the fork suite it replaces. The retired
 * scheme seeded re-randomization from the including block hash, so the same
 * handle legitimately denoted different ciphertexts on competing branches and
 * the old test asserted `digests.size >= 2` -- divergence was the expected
 * result, and fork-aware branch storage existed to keep the two apart.
 *
 * Under the revision the seed contains no chain coordinate beyond those already
 * committed by the output handle. Two consequences:
 *
 *   F1  A handle minted on both branches proves identical sourcing, so every
 *       operator must hold identical bytes for it -- ONE digest fleet-wide,
 *       first-write-wins, no divergence alarm.
 *
 *   F2  Content that genuinely differs across branches mints DIFFERENT handles,
 *       so there is nothing to collide. Rows produced under the orphaned branch
 *       are benign: they are keyed by a handle no canonical consumer resolves.
 *
 * Which case occurs is not left to chance. `FHEVMExecutor` binds
 * `blockhash(block.number - 1)` and `block.timestamp` into the handle preimage,
 * so competing blocks collide exactly when they share a parent and a timestamp.
 * Each case pins both explicitly.
 *
 * Then three cases about what a reorg must NOT leave behind:
 *
 *   F3  An ACL allow observed only on the orphaned branch must not authorize a
 *       canonical decryption. Asserted through the real decryption interface,
 *       with a canonical positive control -- absence of quorum on its own says
 *       nothing about authorization, and describing it as an authorization test
 *       was the weakest claim in the previous version.
 *
 *   F4  A reorg that removes a producer must not strand its cross-block child.
 *       The child is CONSTRUCTED and observed gated before the reorg, because a
 *       database-wide "zero stranded chains" count passes on a stack where no
 *       gated child was ever created, and passed with the repair path disabled.
 *
 *   F5  Replaying an identified, already-ingested block range must change
 *       nothing. The poller is stopped, its cursor rewound and the poller
 *       restarted, because the poller reads its cursor at startup and then
 *       keeps it in memory: rewinding the row under a running poller replays
 *       nothing at all, and the unchanged row counts that follow are unchanged
 *       because nothing happened.
 *
 * Phases (FORK_PHASE), sequenced by `run-fork-consensus.sh`, which owns the
 * service control this container does not have:
 *
 *   main       F1, F2, F3
 *   f4-arm     construct and observe the gated child, then replace the branch
 *   f4-verify  require the identified child to reach a terminal state
 *   f5-arm     record the poller watermark and rewind its cursor
 *   f5-verify  require the rewound range to have been re-scanned
 *
 * Deliberately absent: any read or write of `*_branch` or
 * `coprocessor_settlement`. That schema is deprecated in v0.15 and dropped in
 * v0.16, and -- worse for a test -- it is still trigger-populated in the
 * meantime, so a query against it would return rows and pass while asserting
 * nothing about the tables the binaries actually use.
 *
 * Requires the `three-of-three-fork` topology: operators 0 and 1 follow the
 * canonical Anvil, operator 2 follows the fork.
 */
import { expect } from 'chai';
import { Contract, type InterfaceAbi, type JsonRpcProvider } from 'ethers';

import { assertCanaryFiresWith } from './canary';
import { BRANCH_COMPARISON, type OperatorEvidence } from './comparator';
import {
  type ForkConfig,
  branchesShareEvmParentHash,
  defaultForkConfig,
  getCanonicalProvider,
  getForkProvider,
  getSignerForProvider,
  mineOneBlock,
  pinNextBlockTimestamp,
  requireSharedParent,
  seedForkFromCanonical,
  setIntervalMining,
} from './forkHelper';
import { publishHandshake, readHandshake } from './handshake';
import { restoreMiningState } from './abortRecovery';
import {
  assertCanonicalOutputDigestBindings,
  getCoprocessorDbUrls,
  queryCanonicalOutputs,
  waitForConsensus,
  waitForDatabaseReadiness,
} from './helpers';
import { assertOperatorsAgree, operatorSet, waitForOperatorEvidence } from './probe';
import {
  type KeyMaterialReport,
  InvalidRunError,
  assertKeyMaterial,
  assertRunValidity,
  withDeadline,
} from './validity';

const ENABLE_FORK_CONSENSUS = process.env.RUN_FORK_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
/** Index of the operator routed to the fork by the scenario. */
const FORK_OPERATOR = Number.parseInt(process.env.FORK_OPERATOR_INDEX ?? '2', 10);
const PHASE = process.env.FORK_PHASE ?? 'main';
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const ALIAS_FIXTURE_GAS_LIMIT = 10_000_000;
const MARKER = (phase: string) => `[fork-consensus/${phase}] CASE COMPLETE`;

let databaseUrls: string[] = [];
let forkConfig: ForkConfig;

function required(value: string, name: string): string {
  if (!value) throw new Error(`${name} must be set for the fork consensus gate`);
  return value;
}

/** Operators that follow the canonical chain: everyone except the forked one. */
function canonicalOperators(): number[] {
  return Array.from({ length: COPROCESSOR_COUNT }, (_, index) => index).filter((index) => index !== FORK_OPERATOR);
}

async function withPool<T>(databaseUrl: string, fn: (pool: import('pg').Pool) => Promise<T>): Promise<T> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1, statement_timeout: 30_000 } as never);
  try {
    return await withDeadline(fn(pool), 60_000, 'fork-suite query');
  } finally {
    await pool.end().catch(() => undefined);
  }
}

/** Counts `allowed_handles` observations for a handle in one operator's database. */
async function countAllowObservations(databaseUrl: string, handle: string): Promise<number> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ count: string }>(
      'SELECT COUNT(*)::text AS count FROM allowed_handles WHERE handle = $1',
      [Buffer.from(handle.replace(/^0x/, ''), 'hex')],
    );
    return Number.parseInt(result.rows[0].count, 10);
  });
}

interface ChainState {
  chain: string;
  status: string;
  dependencyCount: number;
  owned: boolean;
  /** Producers that still name this chain a dependent and are unprocessed. */
  unprocessedProducers: number;
}

/** The dependence chain that owns a handle's computation, with its gate state. */
async function chainForHandle(databaseUrl: string, handle: string): Promise<ChainState | null> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{
      chain: string;
      status: string;
      dependency_count: number;
      owned: boolean;
      unprocessed_producers: string;
    }>(
      `SELECT encode(dc.dependence_chain_id, 'hex') AS chain,
              dc.status,
              dc.dependency_count,
              dc.worker_id IS NOT NULL AS owned,
              (SELECT COUNT(*)::text FROM dependence_chain p
                WHERE p.dependents @> ARRAY[dc.dependence_chain_id]
                  AND p.status <> 'processed') AS unprocessed_producers
         FROM computations c
         JOIN dependence_chain dc ON dc.dependence_chain_id = c.dependence_chain_id
        WHERE c.output_handle = $1
        LIMIT 1`,
      [Buffer.from(handle.replace(/^0x/, ''), 'hex')],
    );
    const row = result.rows[0];
    if (!row) return null;
    return {
      chain: row.chain,
      status: row.status,
      dependencyCount: row.dependency_count,
      owned: row.owned,
      unprocessedProducers: Number.parseInt(row.unprocessed_producers, 10),
    };
  });
}

/** One named chain's current state, by id. */
async function chainById(databaseUrl: string, chain: string): Promise<ChainState | null> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{
      status: string;
      dependency_count: number;
      owned: boolean;
      unprocessed_producers: string;
    }>(
      `SELECT dc.status,
              dc.dependency_count,
              dc.worker_id IS NOT NULL AS owned,
              (SELECT COUNT(*)::text FROM dependence_chain p
                WHERE p.dependents @> ARRAY[dc.dependence_chain_id]
                  AND p.status <> 'processed') AS unprocessed_producers
         FROM dependence_chain dc
        WHERE dc.dependence_chain_id = decode($1, 'hex')`,
      [chain],
    );
    const row = result.rows[0];
    if (!row) return null;
    return {
      chain,
      status: row.status,
      dependencyCount: row.dependency_count,
      owned: row.owned,
      unprocessedProducers: Number.parseInt(row.unprocessed_producers, 10),
    };
  });
}

/**
 * Chains stranded by the definition the repair path itself uses: the gate is
 * closed, nobody owns the chain, and no unprocessed producer still names it a
 * dependent -- so nothing left in the system will ever decrement it.
 */
async function strandedChains(databaseUrl: string): Promise<string[]> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ chain: string }>(
      `SELECT encode(child.dependence_chain_id, 'hex') AS chain
         FROM dependence_chain AS child
        WHERE child.dependency_count > 0
          AND child.worker_id IS NULL
          AND NOT EXISTS (
                SELECT 1 FROM dependence_chain AS producer
                 WHERE producer.dependents @> ARRAY[child.dependence_chain_id]
                   AND producer.status <> 'processed')`,
    );
    return result.rows.map((row) => row.chain);
  });
}

/** The poller's persisted cursor. */
async function pollerCursor(databaseUrl: string, chainId: string): Promise<number | null> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ last_caught_up_block: string | null }>(
      'SELECT last_caught_up_block::text FROM host_listener_poller_state WHERE chain_id = $1',
      [chainId],
    );
    const value = result.rows[0]?.last_caught_up_block;
    return value === undefined || value === null ? null : Number.parseInt(value, 10);
  });
}

/** Rewinds the poller's cursor. Returns the new value, or null if there was none. */
async function rewindPollerCursor(databaseUrl: string, chainId: string, blocks: number): Promise<number | null> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ last_caught_up_block: string }>(
      `UPDATE host_listener_poller_state
          SET last_caught_up_block = GREATEST(last_caught_up_block - $1, 0)
        WHERE last_caught_up_block > $1 AND chain_id = $2
      RETURNING last_caught_up_block::text`,
      [blocks, chainId],
    );
    const value = result.rows[0]?.last_caught_up_block;
    return value === undefined ? null : Number.parseInt(value, 10);
  });
}

/** Row counts that must not move when already-ingested events are replayed. */
async function ingestionCounts(databaseUrl: string, transactions: string[], handles: string[]): Promise<{
  computations: number;
  allows: number;
  chains: number;
  duplicatedDependents: number;
}> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{
      computations: string;
      allows: string;
      chains: string;
      duplicated_dependents: string;
    }>(
      `WITH selected_chains AS (SELECT DISTINCT dependence_chain_id FROM computations WHERE transaction_id = ANY($1::bytea[]))
       SELECT (SELECT COUNT(*) FROM computations WHERE transaction_id = ANY($1::bytea[]))::text AS computations,
              (SELECT COUNT(*) FROM allowed_handles WHERE handle = ANY($2::bytea[]))::text AS allows,
              (SELECT COUNT(*) FROM dependence_chain WHERE dependence_chain_id IN (SELECT dependence_chain_id FROM selected_chains))::text AS chains,
              -- A dependent listed twice would be decremented twice, or never
              -- reach zero: array_length minus the distinct count catches an
              -- ingest that armed the same gate on a replayed event.
              (SELECT COALESCE(SUM(cardinality(dependents) - (
                        SELECT COUNT(DISTINCT element) FROM unnest(dependents) AS element)), 0)
                 FROM dependence_chain WHERE dependence_chain_id IN (SELECT dependence_chain_id FROM selected_chains))::text AS duplicated_dependents`,
      [transactions.map(value => Buffer.from(value.replace(/^0x/, ''), 'hex')), handles.map(value => Buffer.from(value.replace(/^0x/, ''), 'hex'))],
    );
    const row = result.rows[0];
    return {
      computations: Number.parseInt(row.computations, 10),
      allows: Number.parseInt(row.allows, 10),
      chains: Number.parseInt(row.chains, 10),
      duplicatedDependents: Number.parseInt(row.duplicated_dependents, 10),
    };
  });
}

/**
 * Waits for every operator to hold key material, mining one fork block between
 * attempts. Used only after the fork is seeded, where both chains are hand-mined
 * and a purely passive wait would never produce the blocks the fork operator's
 * listener needs to catch up.
 */
async function waitForKeyMaterialWhileMining(
  urls: readonly string[],
  fork: JsonRpcProvider,
  deadlineMs = 6 * 60_000,
): Promise<KeyMaterialReport[]> {
  const deadline = Date.now() + deadlineMs;
  let last: unknown;
  for (;;) {
    try {
      return await assertKeyMaterial(urls);
    } catch (error) {
      last = error;
      if (Date.now() >= deadline) {
        throw new InvalidRunError(
          `key material never landed on all operators within ${Math.round(deadlineMs / 1000)}s ` +
            `after the fork was seeded: ${last instanceof Error ? last.message : String(last)}`,
        );
      }
      await mineOneBlock(fork);
      await new Promise((resolve) => setTimeout(resolve, 5_000));
    }
  }
}

describe('Fork byte consensus', function () {
  // Fork orchestration mines, waits for ingestion across three operators, and
  // then waits for a Gateway quorum.
  this.timeout(40 * 60_000);

  let contractAddress: string;
  let fixtureAbi: ReadonlyArray<unknown>;
  let forkOnlyHandle = '';
  let chainId: string;

  before(async function () {
    if (!ENABLE_FORK_CONSENSUS) {
      this.skip();
    }
    required(GATEWAY_RPC_URL, 'GATEWAY_RPC_URL');
    required(CIPHERTEXT_COMMITS_ADDRESS, 'CIPHERTEXT_COMMITS_ADDRESS');
    if (COPROCESSOR_COUNT < 3) {
      throw new Error('the fork gate needs three operators: two on the canonical chain and one on the fork');
    }
    if (FORK_OPERATOR < 0 || FORK_OPERATOR >= COPROCESSOR_COUNT) {
      throw new Error(`FORK_OPERATOR_INDEX ${FORK_OPERATOR} is outside the topology`);
    }

    forkConfig = defaultForkConfig();
    chainId = (await getCanonicalProvider(forkConfig).getNetwork()).chainId.toString();
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    const [{ getSigners, initSigners }, { ethers: hardhatEthers }] = await Promise.all([
      import('../signers'),
      import('hardhat'),
    ]);
    await initSigners(2);
    const signers = await getSigners();
    fixtureAbi = (await hardhatEthers.getContractFactory('AliasFixture')).interface.fragments as ReadonlyArray<unknown>;

    // Later phases reuse the fixture the first phase deployed, so the reorg
    // they reason about is the reorg that removed THAT work.
    if (PHASE !== 'main') {
      contractAddress = readHandshake<{ contractAddress: string }>('fork-fixture').payload.contractAddress;
      console.info(`[fork-consensus/${PHASE}] reusing fixture ${contractAddress}`);
      // A phase that only reads databases must not take the chains over.
      if (PHASE === 'f4-arm') {
        await setIntervalMining(getCanonicalProvider(forkConfig), false);
        await setIntervalMining(getForkProvider(forkConfig), false);
      }
      return;
    }

    // Before the suite stops either chain's miner: the liveness gate would
    // otherwise be measuring this suite's own deliberate stall.
    //
    // The fork operator is held out here and gated separately below, once the
    // fork carries the keygen history. `host-sc-trigger-keygen` runs on the
    // CANONICAL chain late in bring-up -- long after `fhevm-cli` seeded the
    // fork -- so the fork's chain holds no KMSGeneration events at this point
    // and the operator following it cannot yet have written a key row.
    console.info(
      `[fork-consensus] validity gates: ${await assertRunValidity({
        databaseUrls,
        rpcUrl: process.env.RPC_URL,
        operators: canonicalOperators(),
      })}`,
    );

    const { deployAliasFixture } = await import('./aliasFixture');
    const deployment = await deployAliasFixture(signers.alice);
    contractAddress = deployment.contractAddress;
    await (await deployment.contract.produceInputs({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT })).wait();
    publishHandshake('fork-fixture', { contractAddress });

    // From here BOTH chains are mined by hand. The canonical chain's own
    // one-second interval would otherwise advance it past the tip the fork is
    // about to be seeded from -- leaving the branches without a shared parent
    // -- and could mine an empty block that consumes a pinned timestamp before
    // the transaction lands. Restored in `after`.
    await setIntervalMining(getCanonicalProvider(forkConfig), false);

    // Fork the canonical chain at its tip. Pre-fork contract code, stored input
    // handles and block hashes are all resolved from it lazily, and the shared
    // tip is what makes the next block a genuine fork rather than the start of
    // two unrelated chains.
    await seedForkFromCanonical(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl, false);

    const keyReports = await waitForKeyMaterialWhileMining(
      getCoprocessorDbUrls(COPROCESSOR_COUNT),
      getForkProvider(forkConfig),
    );
    console.info(
      `[fork-consensus] key material on all ${keyReports.length} operator(s) after seeding, ` +
        `active key ${keyReports[0].activeKeyIdGw?.slice(0, 16) ?? 'none'}`,
    );

    // Restore the shared parent the key wait's mining consumed.
    await seedForkFromCanonical(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl, false);
  });

  after(async function () {
    if (!ENABLE_FORK_CONSENSUS) return;
    if (PHASE !== 'main' && PHASE !== 'f4-arm') return;
    // Hand the host chain back in the state the rest of the stack expects, and
    // prove it: a suite that leaves a paused chain behind fails whatever runs
    // next for its own reason.
    const canonical = getCanonicalProvider(forkConfig);
    await restoreMiningState(canonical, forkConfig.canonicalRpcUrl);
    await restoreMiningState(getForkProvider(forkConfig), forkConfig.forkRpcUrl);
    const before = await canonical.getBlockNumber();
    await new Promise((resolve) => setTimeout(resolve, 6_000));
    const after = await canonical.getBlockNumber();
    if (after <= before) {
      throw new Error(
        `the canonical chain is not advancing after this suite restored interval mining (${before} -> ${after})`,
      );
    }
    console.info(`[fork-consensus] canonical chain advancing again (${before} -> ${after})`);
  });

  // ------------------------------------------------------------------- F1
  it('F1: a handle minted on both branches carries one set of bytes fleet-wide', async function () {
    if (PHASE !== 'main') this.skip();
    const canonical = getCanonicalProvider(forkConfig);
    const fork = getForkProvider(forkConfig);
    const operators = operatorSet(COPROCESSOR_COUNT);

    // Same parent is a precondition, not an expectation: without it the two
    // branches could not mint a colliding handle whatever the RFC says, and a
    // failure here is a setup fault rather than a consensus finding.
    const { height } = await requireSharedParent(forkConfig);

    // Matching headers are not enough: the executor hashes
    // `blockhash(block.number - 1)` as the EVM sees it. Probe height-1, not
    // height: `eth_call` executes IN the current block, and BLOCKHASH(current)
    // is 0 by EVM rule -- both chains would return zero and trivially "agree",
    // which is the same trap this check exists to catch.
    const evmParent = await branchesShareEvmParentHash(height - 1, forkConfig);
    if (!evmParent.agree) {
      // INVALID, not skipped. `this.skip()` here produced a green job for a
      // required case that had not run.
      throw new InvalidRunError(
        `the two chains' EVMs disagree on BLOCKHASH(${height - 1}) (canonical ${evmParent.canonical}, ` +
          `fork ${evmParent.fork}) even though their headers match, so a colliding handle cannot be ` +
          'constructed. The fork is seeded by forking the canonical chain, which does serve its real block ' +
          'hashes, so check that fork-anvil came up in fork mode rather than as an independent chain',
      );
    }

    const collidingTimestamp = (await canonical.getBlock(height))!.timestamp + 12;
    await pinNextBlockTimestamp(canonical, collidingTimestamp);
    await pinNextBlockTimestamp(fork, collidingTimestamp);
    console.info(
      `[fork-consensus] F1 preimage inputs: parentHeight=${height} ` +
        `canonicalTip=${(await canonical.getBlock(height))!.hash} forkTip=${(await fork.getBlock(height))!.hash} ` +
        `pinnedTimestamp=${collidingTimestamp}`,
    );

    const canonicalContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(canonical, 0));
    const forkContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(fork, 0));

    // Send first (both now sit in their mempools), then mine, then wait for the
    // receipts. Without the receipts the test cannot know which block actually
    // included each transaction, and an empty block that consumed the pinned
    // timestamp would push the transaction into the next one -- with a
    // timestamp nobody chose, and no collision, reported as a consensus result.
    const [canonicalSent, forkSent] = await Promise.all([
      canonicalContract.combineFromStorage({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
      forkContract.combineFromStorage({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
    ]);
    await Promise.all([mineOneBlock(canonical), mineOneBlock(fork)]);
    const [canonicalReceipt, forkReceipt] = await Promise.all([canonicalSent.wait(), forkSent.wait()]);
    if (!canonicalReceipt || !forkReceipt) throw new Error('a branch never mined its F1 transaction');

    const [canonicalBlock, forkBlock] = await Promise.all([
      canonical.getBlock(canonicalReceipt.blockNumber),
      fork.getBlock(forkReceipt.blockNumber),
    ]);
    console.info(
      `[fork-consensus] F1 included in: canonical #${canonicalBlock!.number} ts=${canonicalBlock!.timestamp} ` +
        `parent=${canonicalBlock!.parentHash} hash=${canonicalBlock!.hash} | fork #${forkBlock!.number} ` +
        `ts=${forkBlock!.timestamp} parent=${forkBlock!.parentHash} hash=${forkBlock!.hash}`,
    );

    // These are the preimage inputs the handles are actually derived from, so
    // check them before blaming the result.
    expect(forkBlock!.parentHash, 'the two including blocks must share a parent').to.eq(canonicalBlock!.parentHash);
    expect(forkBlock!.timestamp, 'the two including blocks must share a timestamp').to.eq(canonicalBlock!.timestamp);
    expect(canonicalBlock!.timestamp, 'the including block must carry the pinned timestamp').to.eq(collidingTimestamp);
    // And they must be two DIFFERENT blocks on two branches: identical headers
    // would mean one chain, not a fork.
    expect(
      forkBlock!.hash,
      'the two branches must include the transaction in DIFFERENT blocks; identical hashes mean the ' +
        'branches are the same chain and there is no competition to measure',
    ).to.not.eq(canonicalBlock!.hash);

    const [canonicalHandle, forkHandle] = await Promise.all([
      canonicalContract.combined() as Promise<string>,
      forkContract.combined() as Promise<string>,
    ]);

    // The heart of the revision. The retired suite asserted these differ.
    expect(forkHandle.toLowerCase(), 'a shared parent and timestamp must mint one handle on both branches').to.eq(
      canonicalHandle.toLowerCase(),
    );

    // The fault this family injects, in a form the runner records: two branches
    // that really are different, and the handle minted on both. Without it the
    // fork cases report PASS with nothing to distinguish them from a run on a
    // single chain, which is what the aggregate refused.
    console.info(
      `[fork-consensus] branches diverged: handle ${canonicalHandle.toLowerCase()} canonical ` +
        `${canonicalBlock!.hash} fork ${forkBlock!.hash} at ${new Date().toISOString()}`,
    );

    // Every operator computed it, including the one that saw it on the other
    // branch, and all of them agree byte for byte. Deliberately narrower than
    // the full comparison: an operator that followed the other branch
    // legitimately attributes the handle to a different block, so provenance is
    // not comparable across branches. Everything else is.
    const compareBranchBytes = async () => {
      await assertOperatorsAgree(databaseUrls, operators, canonicalHandle.toLowerCase(), {
        fields: BRANCH_COMPARISON,
        timeoutMs: 6 * 60_000,
      });
    };
    await compareBranchBytes();

    // The canary this suite class owes, aimed at the comparison the cases above
    // actually call.
    const canary = await assertCanaryFiresWith(
      databaseUrls[databaseUrls.length - 1],
      canonicalHandle.toLowerCase(),
      'fork-consensus/F1',
      compareBranchBytes,
      ['compute-digest'],
    );
    expect(canary.kind, 'the fork canary must fire as a compute-digest mismatch').to.eq('compute-digest');

    // First-write-wins is silent: a colliding handle is stored once per
    // operator, never as a duplicate-key error surfaced to the worker.
    const { queryStorageRowCount } = await import('./comparator');
    for (const operator of operators) {
      expect(
        await queryStorageRowCount(databaseUrls[operator], canonicalHandle.toLowerCase()),
        `operator ${operator} must hold exactly one storage row for the colliding handle`,
      ).to.eq(1);
    }

    // And the fleet reaches quorum on it rather than splitting.
    const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, canonicalHandle);
    expect(consensus, 'the aliased handle must reach on-chain quorum').to.not.be.null;
    const senders = consensus!.senders.map((sender) => sender.toLowerCase());
    expect(new Set(senders).size, 'quorum must come from distinct operators').to.eq(senders.length);
    emitAssertions('FORK-01-COLLIDING-HANDLE', ['precondition', 'bytes', 'quorum'], 'Branches shared the EVM-visible preimage, minted one handle, agreed on ciphertext/digest and reached distinct-member quorum.');
    console.info(MARKER('F1'));
  });

  // ------------------------------------------------------------------- F2
  it('F2: divergent branch content mints distinct handles, and orphaned rows are inert', async function () {
    if (PHASE !== 'main') this.skip();
    const canonical = getCanonicalProvider(forkConfig);
    const fork = getForkProvider(forkConfig);

    // Deliberately DIFFERENT timestamps: the branches now diverge in the handle
    // preimage, which is the ordinary case a reorg produces.
    const base = (await canonical.getBlock('latest'))!.timestamp;
    await pinNextBlockTimestamp(canonical, base + 12);
    await pinNextBlockTimestamp(fork, base + 13);

    const canonicalContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(canonical, 0));
    const forkContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(fork, 0));

    const [canonicalSent, forkSent] = await Promise.all([
      canonicalContract.combineFromStorageAgain({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
      forkContract.combineFromStorageAgain({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
    ]);
    await Promise.all([mineOneBlock(canonical), mineOneBlock(fork)]);
    const [canonicalReceipt, forkReceipt] = await Promise.all([
      successfulForkReceipt(canonicalSent, 'F2 canonical transaction'),
      successfulForkReceipt(forkSent, 'F2 fork transaction'),
    ]);
    const [canonicalBlock, forkBlock] = await Promise.all([
      canonical.getBlock(canonicalReceipt.blockNumber), fork.getBlock(forkReceipt.blockNumber),
    ]);
    expect(canonicalBlock?.timestamp, 'F2 canonical inclusion must use the pinned timestamp').to.eq(base + 12);
    expect(forkBlock?.timestamp, 'F2 fork inclusion must use the pinned timestamp').to.eq(base + 13);

    const [canonicalHandle, forkHandle] = await Promise.all([
      canonicalContract.combinedSecond() as Promise<string>,
      forkContract.combinedSecond() as Promise<string>,
    ]);

    expect(
      forkHandle.toLowerCase(),
      'branches differing in the handle preimage must mint distinct handles, leaving nothing to collide',
    ).to.not.eq(canonicalHandle.toLowerCase());

    // Each branch's handle exists where that branch was observed.
    for (const index of canonicalOperators()) {
      await waitForOperatorEvidence(databaseUrls[index], index, canonicalHandle.toLowerCase(), {
        requireSnsDigest: false,
      });
    }
    await waitForOperatorEvidence(databaseUrls[FORK_OPERATOR], FORK_OPERATOR, forkHandle.toLowerCase(), {
      requireSnsDigest: false,
    });

    // Absence, established against an ingestion WATERMARK rather than against a
    // sleep. A canonical operator that has ingested work created AFTER the
    // fork's divergent block, and still holds no row for the fork's handle,
    // is not merely lagging.
    await pinNextBlockTimestamp(canonical, base + 24);
    const sentinelSent = await canonicalContract.combineLocal({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT });
    await mineOneBlock(canonical);
    const sentinelReceipt = await successfulForkReceipt(sentinelSent, 'F2 canonical sentinel');
    expect(sentinelReceipt.blockNumber, 'sentinel must follow the divergent canonical transaction').to.be.greaterThan(canonicalReceipt.blockNumber);
    const sentinel = (await canonicalContract.combinedLocal() as string).toLowerCase();
    for (const index of canonicalOperators()) {
      await waitForOperatorEvidence(databaseUrls[index], index, sentinel, { requireSnsDigest: false });
      const rows = await queryCanonicalOutputs(databaseUrls[index], [forkHandle.toLowerCase()]);
      expect(
        rows.length,
        `operator ${index} has ingested a canonical sentinel minted after the fork's divergent block, so its ` +
          `holding no row for ${forkHandle} is a real absence -- but it holds ${rows.length}`,
      ).to.eq(0);
    }
    console.info(`[fork-consensus] F2 absence confirmed past the canonical sentinel ${sentinel}`);

    forkOnlyHandle = forkHandle.toLowerCase();
    publishHandshake('fork-orphan', { forkOnlyHandle, canonicalHandle: canonicalHandle.toLowerCase() });
    emitAssertions('FORK-02-DISTINCT-HANDLES', ['safety'], 'Exact successful divergent receipts minted different handles; canonical operators past the later sentinel held no fork-only row.');
    console.info(MARKER('F2'));
  });

  // ------------------------------------------------------------------- F3
  it('F3: an allow seen only on the orphaned branch never authorizes a canonical decryption', async function () {
    if (PHASE !== 'main') this.skip();
    expect(forkOnlyHandle, 'F2 must run first: F3 reasons about the handle it minted').to.not.eq('');
    const { canonicalHandle } = readHandshake<{ canonicalHandle: string }>('fork-orphan').payload;

    // AliasFixture allows every value it mints, so the fork-only handle comes
    // with ACL observations that exist ONLY on the orphaned branch.
    const forkAllows = await countAllowObservations(databaseUrls[FORK_OPERATOR], forkOnlyHandle);
    expect(forkAllows, 'the forked operator must have observed the allow on its branch').to.be.greaterThan(0);
    for (const index of canonicalOperators()) {
      expect(
        await countAllowObservations(databaseUrls[index], forkOnlyHandle),
        `operator ${index} never saw the orphaned branch and must hold no allow for its handle`,
      ).to.eq(0);
    }

    // Replace the fork's history with the canonical chain: from the forked
    // operator's point of view its branch is orphaned and replaced, which is
    // what a reorg is.
    await seedForkFromCanonical(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl, false);

    // The safety property. Two halves, and the previous version had only the
    // first: absence of quorum says nothing about authorization.
    const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, forkOnlyHandle, 60_000);
    expect(consensus, 'a handle only ever seen on an orphaned branch must not reach on-chain quorum').to.be.null;

    // The real authorization/decryption interface, with a positive control on
    // the same interface so a rejection cannot be an outage.
    const [{ createInstances }, { getSigners }] = await Promise.all([import('../instance'), import('../signers')]);
    const signers = await getSigners();
    const instances = await createInstances(signers);
    const instance = instances.alice;

    let canonicalPlaintext: unknown;
    try {
      canonicalPlaintext = await withDeadline(
        instance.userDecryptSingleHandle({
          handle: canonicalHandle,
          contractAddress,
          signer: signers.alice,
        }) as Promise<unknown>,
        6 * 60_000,
        'canonical control decryption',
      );
    } catch (error) {
      throw new Error(
        `the canonical positive control could not be decrypted (${
          error instanceof Error ? error.message : String(error)
        }); without it, the orphan rejection below could be an outage rather than an authorization decision`,
      );
    }
    console.info(`[fork-consensus] F3 canonical control decrypted to ${String(canonicalPlaintext)}`);

    expect(String(canonicalPlaintext), 'canonical control plaintext').to.eq('12');
    const [{ requestUnifiedUserDecrypt, expectRelayerAclRejection, directHandle, backdatedStartTimestamp }, config] = await Promise.all([
      import('../sdk/unified/unifiedUserDecrypt'), import('../instance'),
    ]);
    const keypair = await instance.generateKeypair();
    const outcome = await requestUnifiedUserDecrypt({
      relayerUrl: config.relayerUrl,
      decryptionContractAddress: config.verifyingContractAddressDecryption,
      apiKey: config.relayerApiKey,
    }, {
      handles: [directHandle(forkOnlyHandle, contractAddress, signers.alice.address)],
      userAddress: signers.alice.address, allowedContracts: [contractAddress],
      publicKey: keypair.publicKey, startTimestamp: backdatedStartTimestamp(), durationSeconds: 3600,
    }, { kind: 'eoa', signer: signers.alice }, { waitForTerminal: true, timeoutMs: 4 * 60_000 });
    expect(outcome.post.httpStatus, 'the valid signed request must reach the ACL check').to.eq(202);
    expectRelayerAclRejection(outcome.poll);
    console.info('[fork-consensus] F3 orphan decryption refused by the canonical ACL');

    // NOT asserted, deliberately: that the orphaned `allowed_handles` rows are
    // deleted. They are not. The listener's reorg retraction covers bridge and
    // delegate observations; ACL allows are removed only by the operator-run
    // `revert_coprocessor_db_state.sql`. The property that matters is the one
    // above -- that those rows authorize nothing canonically -- and demanding a
    // deletion the implementation deliberately does not perform would pin the
    // wrong contract.
    emitAssertions('FORK-03-ORPHAN-ALLOW-INERT', ['safety', 'liveness'], 'Orphan handle lacked quorum and its real authorization request was rejected while canonical control decryption succeeded.');
    console.info(MARKER('F3'));
  });

  // ------------------------------------------------------------------- F4
  it('F4 (arm): construct a cross-block child, observe it gated, then remove its producer', async function () {
    if (PHASE !== 'f4-arm') this.skip();
    const fork = getForkProvider(forkConfig);
    const forkSigner = getSignerForProvider(fork, 0);
    const forkContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, forkSigner);
    // Reseeding the fork from canonical rewinds account nonces, and a signer
    // that has already sent transactions on the pre-reseed chain then collides
    // with itself ("nonce has already been used"). Every send below takes the
    // nonce this chain reports at the moment it is sent.
    const forkNonce = async () => ({
      gasLimit: ALIAS_FIXTURE_GAS_LIMIT,
      // Raw, because the cached answer was wrong: the rejected transaction
      // decoded to nonce 5 against a chain that had already used it. Same
      // staleness that made `getBlock('latest')` return the parent twice in the
      // reorg suite, and the same remedy -- ask the node, not the provider.
      nonce: Number.parseInt(
        await fork.send('eth_getTransactionCount', [await forkSigner.getAddress(), 'pending']),
        16,
      ),
    });

    // The runner has stalled the fork operator's tfhe-worker, so the producer's
    // chain is ingested but never retired -- which is the only state in which a
    // gated child can be observed rather than raced.
    const base = (await fork.getBlock('latest'))!.timestamp;
    await pinNextBlockTimestamp(fork, base + 12);
    await forkContract.combineFromStorage(await forkNonce());
    await mineOneBlock(fork);
    const producerHandle = ((await forkContract.combined()) as string).toLowerCase();

    // The child, in a LATER block, consuming the producer's output from
    // storage: a genuine cross-block dependency.
    await pinNextBlockTimestamp(fork, base + 24);
    await forkContract.consumeCombined(await forkNonce());
    await mineOneBlock(fork);
    const childHandle = ((await forkContract.consumed()) as string).toLowerCase();
    console.info(`[fork-consensus] F4 producer ${producerHandle} -> child ${childHandle} on the fork branch`);

    // Observe the child GATED, by chain id, before the disruptive event. This
    // is the precondition the previous version never established: it asserted a
    // database-wide stranded count of zero, which passes on a stack where no
    // gated child was ever created and passed with the repair path disabled.
    const deadline = Date.now() + 5 * 60_000;
    let child: ChainState | null = null;
    for (;;) {
      child = await chainForHandle(databaseUrls[FORK_OPERATOR], childHandle);
      if (child && child.dependencyCount > 0 && !child.owned) break;
      if (Date.now() >= deadline) {
        throw new Error(
          `the child ${childHandle} was never observed gated on the fork operator (last seen ` +
            `${child ? `chain ${child.chain} status ${child.status} count ${child.dependencyCount} owned ${child.owned}` : 'absent'}). ` +
            'Without a gated child this case cannot distinguish a working repair path from a stack that ' +
            'never had a stranded chain. The runner must stall the fork operator\'s tfhe-worker first',
        );
      }
      await mineOneBlock(fork);
      await new Promise((resolve) => setTimeout(resolve, 3_000));
    }
    console.info(
      `[fork-consensus] F4 child chain ${child.chain} is gated (dependency_count ${child.dependencyCount}, ` +
        `unowned, ${child.unprocessedProducers} unprocessed producer(s))`,
    );

    // Reorg retains compute work. Inject the missing release decrement on this
    // one real child; the producer must still finish and supply its ciphertext.
    // A second control disables only repair acquisition, proving the terminal
    // assertion cannot be satisfied by normal parent completion.
    await withPool(databaseUrls[FORK_OPERATOR], async pool => {
      await pool.query(INSTALL_FORK_GATE_CONTROL);
      await pool.query("INSERT INTO public.consensus_test_fork_gate(child) VALUES (decode($1, 'hex'))", [child!.chain]);
      await pool.query("UPDATE dependence_chain SET last_updated_at = NOW() - INTERVAL '10 minutes' WHERE dependence_chain_id = decode($1, 'hex')", [child!.chain]);
    });
    await seedForkFromCanonical(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl, false);

    publishHandshake('fork-child', {
      childChain: child.chain,
      childHandle,
      producerHandle,
      dependencyCountWhenGated: child.dependencyCount,
    });
    emitAssertions('FORK-04-STRANDED-CHILD', ['precondition'], 'The selected child chain was observed gated before releasing the producer.');
    console.info(MARKER('F4-arm'));
  });

  it('F4 (verify): the identified child reaches a terminal state rather than stalling', async function () {
    if (PHASE !== 'f4-verify') this.skip();
    const record = readHandshake<{
      childChain: string;
      childHandle: string;
      dependencyCountWhenGated: number;
    }>('fork-child').payload;
    expect(
      record.dependencyCountWhenGated,
      'the arming phase must have observed the child gated, or a zero stranded count proves nothing',
    ).to.be.greaterThan(0);

    try {
      const control = () => withPool(databaseUrls[FORK_OPERATOR], async pool => {
        const result = await pool.query<{ repair_attempts: number; repair_claims: number; lost_decrements: number }>(
          "SELECT repair_attempts, repair_claims, lost_decrements FROM public.consensus_test_fork_gate WHERE child = decode($1, 'hex')", [record.childChain]);
        if (result.rows.length !== 1) throw new Error('missing per-child repair mutation control');
        return result.rows[0];
      });
      const negativeDeadline = Date.now() + 8 * 60_000;
      for (;;) {
        const state = await chainById(databaseUrls[FORK_OPERATOR], record.childChain);
        const observed = await control();
        if (state === null || isTerminalForkChild(state)) throw new Error('child completed with repair disabled: the negative control did not expose the fault');
        if (observed.repair_attempts > 0) {
          assertRepairDisabledControl(state, observed.repair_attempts);
          expect(observed.repair_claims).to.eq(0);
          expect(observed.lost_decrements, 'exactly one producer decrement was lost').to.eq(1);
          break;
        }
        if (Date.now() >= negativeDeadline) throw new Error('repair-disabled control never reached the stale-gate acquisition path');
        await new Promise(resolve => setTimeout(resolve, 1_000));
      }
      console.info('[fork-consensus] F4 negative control: attempted repair was disabled and the identified child remained stranded');
      await withPool(databaseUrls[FORK_OPERATOR], pool => pool.query(
        "UPDATE public.consensus_test_fork_gate SET disable_repair = false WHERE child = decode($1, 'hex')", [record.childChain]));
      const deadline = Date.now() + 8 * 60_000;
      for (;;) {
        const state = await chainById(databaseUrls[FORK_OPERATOR], record.childChain);
        if (isTerminalForkChild(state)) break;
        if (Date.now() >= deadline) throw new Error('the identified child did not complete after enabling stale-gate repair');
        await new Promise(resolve => setTimeout(resolve, 1_000));
      }
      expect((await control()).repair_claims, 'a committed repair acquisition must explain child recovery').to.be.greaterThan(0);
      const completion = await withPool(databaseUrls[FORK_OPERATOR], async pool => {
        const result = await pool.query<{ is_completed: boolean; is_error: boolean }>(
          "SELECT is_completed, is_error FROM computations WHERE output_handle = decode($1, 'hex')", [record.childHandle.replace(/^0x/, '')]);
        return result.rows;
      });
      expect(completion).to.have.length(1);
      expect(completion[0].is_completed).to.eq(true);
      expect(completion[0].is_error).to.eq(false);
      await waitForOperatorEvidence(databaseUrls[FORK_OPERATOR], FORK_OPERATOR, record.childHandle);
      const outputs = await queryCanonicalOutputs(databaseUrls[FORK_OPERATOR], [record.childHandle]);
      expect(outputs, 'repair must materialize the identified orphan child').to.have.length(1);
      assertCanonicalOutputDigestBindings([outputs]);
      console.info(`[fork-consensus] F4 child ${record.childChain} completed through the enabled repair path`);
    } finally {
      await withPool(databaseUrls[FORK_OPERATOR], pool => pool.query(DROP_FORK_GATE_CONTROL));
    }

    // And nothing else on the fork operator is stranded either.
    const stranded = await strandedChains(databaseUrls[FORK_OPERATOR]);
    expect(
      stranded,
      `the forked operator has chain(s) whose gate can never be decremented: ${stranded.join(', ')}`,
    ).to.have.length(0);

    // The canonical operators never reorged and must be clean throughout.
    for (const index of canonicalOperators()) {
      expect(
        await strandedChains(databaseUrls[index]),
        `operator ${index} stranded a chain without a reorg`,
      ).to.have.length(0);
    }
    emitAssertions('FORK-04-STRANDED-CHILD', ['liveness', 'bytes', 'safety', 'sensitivity'], 'Repair-disabled acquisition failed as required; enabled repair committed a claim and completed the exact child with bound material and no stranded chains.');
    console.info(MARKER('F4-verify'));
  });

  // ------------------------------------------------------------------- F5
  // F5 runs against a CANONICAL operator, not the fork follower.
  //
  // The property is the poller's idempotence when an already-ingested range is
  // re-scanned; nothing about it is fork-specific. The fork follower cannot
  // host it at all: every phase that reseeds the fork from canonical rewinds
  // that chain, leaving it permanently shorter than what its poller has already
  // ingested -- head 614 against a cursor of 623 -- so a replay would be asked
  // for blocks the chain no longer has, in any case order.
  const REPLAY_OPERATOR = 0;

  it('F5 (prepare): mint and identify the graph that will actually be replayed', async function () {
    if (PHASE !== 'f5-prepare') this.skip();
    const canonical = getCanonicalProvider(forkConfig);
    const fixture = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(canonical, 0));
    const producerTx = await fixture.combineFromStorage({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT });
    const producerReceipt = await producerTx.wait();
    const producer = ((await fixture.combined()) as string).toLowerCase();
    const childTx = await fixture.consumeCombined({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT });
    const childReceipt = await childTx.wait();
    const child = ((await fixture.consumed()) as string).toLowerCase();
    expect(childReceipt.blockNumber, 'replay graph must cross an actual block boundary').to.be.greaterThan(producerReceipt.blockNumber);
    const handles = [producer, child];
    const transactions = [producerReceipt.hash.toLowerCase(), childReceipt.hash.toLowerCase()];
    const outputs: string[] = [];
    for (const handle of handles) outputs.push(replayOutputFingerprint(await waitForOperatorEvidence(databaseUrls[REPLAY_OPERATOR], REPLAY_OPERATOR, handle)));
    const deadline = Date.now() + 5 * 60_000;
    while ((await pollerCursor(databaseUrls[REPLAY_OPERATOR], chainId) ?? -1) < childReceipt.blockNumber) {
      if (Date.now() >= deadline) throw new Error('poller did not ingest the selected replay graph before stopping');
      await new Promise(resolve => setTimeout(resolve, 1_000));
    }
    const counts = await ingestionCounts(databaseUrls[REPLAY_OPERATOR], transactions, handles);
    assertReplayHasEffects(counts);
    publishHandshake('fork-replay-target', {
      handles, transactions, outputs, firstBlock: producerReceipt.blockNumber, lastBlock: childReceipt.blockNumber,
    });
    console.info(MARKER('F5-prepare'));
  });

  it('F5 (arm): rewind the poller cursor over an identified range', async function () {
    if (PHASE !== 'f5-arm') this.skip();
    // The runner has STOPPED the poller. That is the whole point: the poller
    // reads `host_listener_poller_state` at startup and then keeps its position
    // in memory (`poller/mod.rs`), so rewinding the row under a running poller
    // replays nothing, and the unchanged row counts that used to follow were
    // unchanged because nothing had happened.
    const target = readHandshake<{ handles: string[]; transactions: string[]; outputs: string[]; firstBlock: number; lastBlock: number }>('fork-replay-target').payload;
    const before = await ingestionCounts(databaseUrls[REPLAY_OPERATOR], target.transactions, target.handles);
    assertReplayHasEffects(before);
    expect(before.duplicatedDependents, 'a dependents array already held a duplicate before any replay').to.eq(0);

    const watermark = await pollerCursor(databaseUrls[REPLAY_OPERATOR], chainId);
    expect(watermark, 'the poller must have a cursor to rewind, or nothing would be replayed').to.not.be.null;

    // The chain must still HOLD the range this case asks to be re-scanned.
    // F4 reseeds the fork from canonical, which rewinds the chain, and a cursor
    // rewound to 603 then waits forever to climb back to a height 612 that no
    // longer exists -- reported as "the range was not re-scanned" when the
    // range was gone.
    const canonicalHead = await getCanonicalProvider(forkConfig).getBlockNumber();
    expect(
      canonicalHead,
      `the canonical chain's head is ${canonicalHead} but the poller's cursor is ${watermark}; a replay ` +
        'cannot be asked for blocks the chain no longer has',
    ).to.be.greaterThanOrEqual(watermark!);

    expect(watermark!, 'selected graph must already be below the poller cursor').to.be.at.least(target.lastBlock);
    const poller = readHandshake<{ clientAddress: string; processBefore: string }>('fork-replay-poller').payload;
    const { isIP } = await import('node:net');
    expect(isIP(poller.clientAddress), 'host must identify the poller database-client IP').not.to.eq(0);
    expect(poller.processBefore).to.be.a('string').and.not.empty;
    await withPool(databaseUrls[REPLAY_OPERATOR], async pool => {
      await pool.query(INSTALL_REPLAY_INSERT_AUDIT);
      for (const [index, handle] of target.handles.entries()) {
        await pool.query(
          "INSERT INTO public.consensus_test_replay_inserts(output_handle,transaction_id,client_address) VALUES (decode($1,'hex'),decode($2,'hex'),$3::inet)",
          [handle.replace(/^0x/, ''), target.transactions[index].replace(/^0x/, ''), poller.clientAddress]);
      }
    });
    const rewound = await rewindPollerCursor(databaseUrls[REPLAY_OPERATOR], chainId, watermark! - target.firstBlock + 1);
    expect(rewound, 'the cursor must actually move back').to.not.be.null;
    expect(rewound!, 'the cursor must move BACKWARDS').to.be.lessThan(watermark!);
    console.info(
      `[fork-consensus] F5 cursor rewound ${watermark} -> ${rewound}; the range ${rewound! + 1}..${watermark} ` +
        'must be re-scanned when the poller restarts',
    );

    publishHandshake('fork-replay', {
      ...target,
      watermarkBefore: watermark,
      rewoundTo: rewound,
      counts: before,
    });
    emitAssertions('FORK-05-REPLAY', ['precondition'], 'The rewind range contains the identified producer/child graph and its attributed insert audit was armed.');
    console.info(MARKER('F5-arm'));
  });

  it('F5 (verify): the rewound range was re-scanned and committed nothing new', async function () {
    if (PHASE !== 'f5-verify') this.skip();
    const record = readHandshake<{
      watermarkBefore: number;
      rewoundTo: number;
      handles: string[];
      transactions: string[];
      outputs: string[];
      counts: { computations: number; allows: number; chains: number; duplicatedDependents: number };
    }>('fork-replay').payload;

    try {
      // Evidence that the range was replayed: the restarted poller's cursor has
      // to climb back through it. A successful UPDATE of the row is not evidence,
      // and neither is a sleep.
      const deadline = Date.now() + 8 * 60_000;
      let cursor = await pollerCursor(databaseUrls[REPLAY_OPERATOR], chainId);
      for (;;) {
        cursor = await pollerCursor(databaseUrls[REPLAY_OPERATOR], chainId);
        if (cursor !== null && cursor >= record.watermarkBefore) break;
        if (Date.now() >= deadline) {
          throw new Error(
            `the restarted poller's cursor is ${cursor} and has not returned to ${record.watermarkBefore}, so the ` +
              `range ${record.rewoundTo + 1}..${record.watermarkBefore} was not re-scanned. Unchanged row counts ` +
              'would be unchanged because nothing happened',
          );
        }
        await new Promise((resolve) => setTimeout(resolve, 5_000));
      }
      console.info(
        `[fork-consensus] F5 poller re-scanned ${record.rewoundTo + 1}..${record.watermarkBefore} ` +
          `(cursor now ${cursor})`,
      );

      const attempts = await withPool(databaseUrls[REPLAY_OPERATOR], async pool => {
        const result = await pool.query<{ attempts: number }>('SELECT attempts FROM public.consensus_test_replay_inserts');
        return result.rows.map(row => row.attempts);
      });
      assertReplayAttempts(attempts, record.handles.length);
      console.info('[fork-consensus] F5 committed insert attempts observed for every named event from the restarted poller');
      const after = await ingestionCounts(databaseUrls[REPLAY_OPERATOR], record.transactions, record.handles);
      assertReplayHasEffects(after);
      for (const [index, handle] of record.handles.entries()) {
        const current = replayOutputFingerprint(await waitForOperatorEvidence(databaseUrls[REPLAY_OPERATOR], REPLAY_OPERATOR, handle));
        expect(current, `replay must preserve every durable output field of ${handle}`).to.eq(record.outputs[index]);
      }
      expect(after.computations, 'replayed events must not duplicate computation rows').to.eq(
        record.counts.computations,
      );
      expect(after.allows, 'replayed events must not duplicate allow observations').to.eq(record.counts.allows);
      expect(after.chains, 'replayed events must not create duplicate dependence chains').to.eq(record.counts.chains);
      // The one that would not show up as a row count: arming the same gate twice
      // leaves the dependent listed twice, and the count is then either
      // decremented twice or never reaches zero.
      expect(after.duplicatedDependents, 'a replayed event armed the same gate twice').to.eq(0);
      emitAssertions('FORK-05-REPLAY', ['liveness', 'safety'], 'Every selected event had a committed replay attempt from the replacement poller; canonical row counts, output fingerprints and dependency uniqueness remained unchanged.');
      console.info(MARKER('F5-verify'));
    } finally {
      await withPool(databaseUrls[REPLAY_OPERATOR], pool => pool.query(DROP_REPLAY_INSERT_AUDIT));
    }
  });
});
