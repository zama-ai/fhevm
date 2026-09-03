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
 * Under the revision the seed contains no chain coordinate beyond those
 * already committed by the output handle. Two consequences, and they are the
 * two cases below:
 *
 *   F1' A handle minted on both branches proves identical sourcing, so every
 *       operator must hold identical bytes for it -- ONE digest fleet-wide,
 *       first-write-wins, no divergence alarm. This is the case the retired
 *       suite got backwards.
 *
 *   F2' Content that genuinely differs across branches mints DIFFERENT
 *       handles, so there is nothing to collide. Rows produced under the
 *       orphaned branch are benign: they are keyed by a handle no canonical
 *       consumer resolves.
 *
 * Which case occurs is not left to chance. `FHEVMExecutor` binds
 * `blockhash(block.number - 1)` and `block.timestamp` into the handle
 * preimage, so competing blocks collide exactly when they share a parent and a
 * timestamp. Each test pins both explicitly; see `pinNextBlockTimestamp`.
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

import {
  type ForkConfig,
  defaultForkConfig,
  getCanonicalProvider,
  getForkProvider,
  getSignerForProvider,
  mineOneBlock,
  pinNextBlockTimestamp,
  branchesShareEvmParentHash,
  requireSharedParent,
  setIntervalMining,
  seedForkFromCanonical,
} from './forkHelper';
import { assertCanaryFiresWith } from './canary';
import { assertKeyMaterial, assertRunValidity, InvalidRunError, type KeyMaterialReport } from './validity';
import {
  getCoprocessorDbUrls,
  queryCanonicalOutputs,
  waitForConsensus,
  waitForDatabaseReadiness,
} from './helpers';

const ENABLE_FORK_CONSENSUS = process.env.RUN_FORK_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
/** Index of the operator routed to the fork by the scenario. */
const FORK_OPERATOR = Number.parseInt(process.env.FORK_OPERATOR_INDEX ?? '2', 10);
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const ALIAS_FIXTURE_GAS_LIMIT = 10_000_000;

let databaseUrls: string[] = [];
/** Set by F2', consumed by F3': a handle that existed only on the orphaned branch. */
let forkOnlyHandle = '';
let forkConfig: ForkConfig;

function required(value: string, name: string): string {
  if (!value) throw new Error(`${name} must be set for the fork consensus gate`);
  return value;
}

/** Operators that follow the canonical chain: everyone except the forked one. */
function canonicalOperators(): number[] {
  return Array.from({ length: COPROCESSOR_COUNT }, (_, index) => index).filter((index) => index !== FORK_OPERATOR);
}

/**
 * Waits until a handle is present and complete in one operator's database,
 * returning its canonical row.
 *
 * A plain "query once" would race ingestion and report an absent handle as a
 * consensus finding. Absence is only meaningful after the deadline.
 */
async function waitForHandle(
  databaseUrl: string,
  handle: string,
  timeoutMs = 5 * 60_000,
): Promise<ReturnType<typeof queryCanonicalOutputs> extends Promise<infer T> ? (T extends (infer R)[] ? R : never) : never> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  while (Date.now() < deadline) {
    try {
      const rows = await queryCanonicalOutputs(databaseUrl, [handle]);
      if (rows.length === 1) return rows[0];
      if (rows.length > 1) throw new Error(`${databaseUrl} holds ${rows.length} canonical rows for ${handle}`);
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 2_000));
  }
  throw new Error(`timed out waiting for ${handle} in ${databaseUrl}${lastError ? `: ${String(lastError)}` : ''}`);
}

/** Asserts a handle is absent from a database, after giving ingestion time to have produced it. */
async function assertHandleAbsent(databaseUrl: string, handle: string, settleMs = 30_000): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, settleMs));
  const rows = await queryCanonicalOutputs(databaseUrl, [handle]);
  expect(rows.length, `${databaseUrl} must not hold a row for the other branch's handle ${handle}`).to.eq(0);
}

/** Counts `allowed_handles` observations for a handle in one operator's database. */
async function countAllowObservations(databaseUrl: string, handle: string): Promise<number> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<{ count: string }>(
      'SELECT COUNT(*)::text AS count FROM allowed_handles WHERE handle = $1',
      [Buffer.from(handle.slice(2), 'hex')],
    );
    return Number.parseInt(result.rows[0].count, 10);
  } finally {
    await pool.end();
  }
}

/**
 * Chains that are stranded by the definition the repair path itself uses: the
 * gate is still closed, nobody owns the chain, and no unprocessed producer
 * chain still names it a dependent -- so nothing left in the system will ever
 * decrement it.
 *
 * Asserting the invariant rather than the mechanism keeps the test honest
 * about what matters. Whether the count is cleared by `acquire_stale_gated_lock`
 * or by the listener re-arming the producers, a chain left in this state is a
 * permanent stall.
 */
async function countStrandedChains(databaseUrl: string): Promise<number> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<{ count: string }>(
      `SELECT COUNT(*)::text AS count
         FROM dependence_chain AS child
        WHERE child.dependency_count > 0
          AND child.worker_id IS NULL
          AND NOT EXISTS (
                SELECT 1 FROM dependence_chain AS producer
                 WHERE producer.dependents @> ARRAY[child.dependence_chain_id]
                   AND producer.status <> 'processed')`,
    );
    return Number.parseInt(result.rows[0].count, 10);
  } finally {
    await pool.end();
  }
}

/**
 * Rewinds the poller's cursor so it re-scans blocks it has already ingested.
 * Returns false when there is no cursor yet, which would make the replay
 * assertion vacuous rather than passing.
 */
async function rewindPollerCursor(databaseUrl: string, blocks: number): Promise<boolean> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      `UPDATE host_listener_poller_state
          SET last_caught_up_block = GREATEST(last_caught_up_block - $1, 0)
        WHERE last_caught_up_block > $1`,
      [blocks],
    );
    return (result.rowCount ?? 0) > 0;
  } finally {
    await pool.end();
  }
}

/** Row counts that must not move when already-ingested events are replayed. */
async function ingestionCounts(databaseUrl: string): Promise<{
  computations: number;
  allows: number;
  chains: number;
  duplicatedDependents: number;
}> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<{
      computations: string;
      allows: string;
      chains: string;
      duplicated_dependents: string;
    }>(
      `SELECT (SELECT COUNT(*) FROM computations)::text     AS computations,
              (SELECT COUNT(*) FROM allowed_handles)::text  AS allows,
              (SELECT COUNT(*) FROM dependence_chain)::text AS chains,
              -- A dependent listed twice would be decremented twice, or never
              -- reach zero: array_length minus the distinct count catches an
              -- ingest that armed the same gate on a replayed event.
              (SELECT COALESCE(SUM(cardinality(dependents) - (
                        SELECT COUNT(DISTINCT element) FROM unnest(dependents) AS element)), 0)
                 FROM dependence_chain)::text AS duplicated_dependents`,
    );
    const row = result.rows[0];
    return {
      computations: Number.parseInt(row.computations, 10),
      allows: Number.parseInt(row.allows, 10),
      chains: Number.parseInt(row.chains, 10),
      duplicatedDependents: Number.parseInt(row.duplicated_dependents, 10),
    };
  } finally {
    await pool.end();
  }
}

/**
 * Waits for every operator to hold key material, mining one fork block between
 * attempts. Used only after the fork is seeded, where both chains are hand-mined
 * and a purely passive wait would never produce the blocks the fork operator's
 * listener needs to catch up.
 */
async function waitForKeyMaterialWhileMining(
  databaseUrls: readonly string[],
  fork: JsonRpcProvider,
  deadlineMs = 6 * 60_000,
): Promise<KeyMaterialReport[]> {
  const deadline = Date.now() + deadlineMs;
  let last: unknown;
  for (;;) {
    try {
      return await assertKeyMaterial(databaseUrls);
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
  this.timeout(30 * 60_000);

  let contractAddress: string;
  let fixtureAbi: ReadonlyArray<unknown>;

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

    // Before the suite stops either chain's miner: the liveness gate would
    // otherwise be measuring this suite's own deliberate stall.
    //
    // The fork operator is held out here and gated separately below, once the
    // fork carries the keygen history. `host-sc-trigger-keygen` runs on the
    // CANONICAL chain late in bring-up -- long after `fhevm-cli` seeded the
    // fork -- so the fork's chain holds no KMSGeneration events at this point
    // and the operator following it cannot yet have written a key row. Gating
    // it here reports a bring-up ordering fact as an invalid run.
    console.info(
      `[fork-consensus] validity gates: ${await assertRunValidity({
        databaseUrls: getCoprocessorDbUrls(COPROCESSOR_COUNT),
        rpcUrl: process.env.RPC_URL,
        operators: Array.from({ length: COPROCESSOR_COUNT }, (_, index) => index).filter(
          (index) => index !== FORK_OPERATOR,
        ),
      })}`,
    );

    forkConfig = defaultForkConfig();
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    const [{ getSigners, initSigners }, { deployAliasFixture }, { ethers: hardhatEthers }] = await Promise.all([
      import('../signers'),
      import('./aliasFixture'),
      import('hardhat'),
    ]);
    await initSigners(2);
    const signers = await getSigners();

    // Deploy and seed inputs on the CANONICAL chain only, while it is still
    // mining normally.
    const deployment = await deployAliasFixture(signers.alice);
    contractAddress = deployment.contractAddress;
    await (await deployment.contract.produceInputs({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT })).wait();

    fixtureAbi = (await hardhatEthers.getContractFactory('AliasFixture')).interface.fragments as ReadonlyArray<unknown>;

    // From here BOTH chains are mined by hand. The canonical chain's own
    // one-second interval would otherwise advance it past the tip the fork is
    // about to be seeded from -- leaving the branches without a shared parent
    // -- and could mine an empty block that consumes a pinned timestamp before
    // the transaction lands. Restored in `after`, so a stalled host chain is
    // not left behind for whatever runs next.
    await setIntervalMining(getCanonicalProvider(forkConfig), false);

    // Fork the canonical chain at its tip. Pre-fork contract code, stored
    // input handles and block hashes are all resolved from it lazily, and the
    // shared tip is what makes the next block a genuine fork rather than the
    // start of two unrelated chains.
    await seedForkFromCanonical(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl, false);

    // Now that the fork descends from a canonical tip that includes keygen, the
    // fork operator can ingest key material -- and must, before anything it
    // computes counts. Both chains are hand-mined from here, so the fork is
    // nudged forward between attempts: its listener needs new blocks to catch up
    // through the history the reset just handed it. Mining moves the fork ahead of
    // the canonical tip, so the fork is re-seeded afterwards: F1' needs the two
    // chains to share a height AND a tip hash, and requireSharedParent rejects any
    // drift. Key rows already ingested live in Postgres, not on chain, so the
    // second reset does not undo them.
    const keyReports = await waitForKeyMaterialWhileMining(
      getCoprocessorDbUrls(COPROCESSOR_COUNT),
      getForkProvider(forkConfig),
    );
    console.info(
      `[fork-consensus] key material on all ${keyReports.length} operator(s) after seeding, ` +
        `key ${keyReports[0].keyIdGw?.slice(0, 16) ?? 'none'}`,
    );

    // Restore the shared parent the key wait's mining consumed.
    await seedForkFromCanonical(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl, false);
  });

  after(async function () {
    if (!ENABLE_FORK_CONSENSUS) return;
    // Hand the host chain back in the state the rest of the stack expects.
    await setIntervalMining(getCanonicalProvider(forkConfig), true);
  });

  it("F1': a handle minted on both branches carries one set of bytes fleet-wide", async function () {
    const canonical = getCanonicalProvider(forkConfig);
    const fork = getForkProvider(forkConfig);

    // Same parent is a precondition, not an expectation: without it the two
    // branches could not mint a colliding handle whatever the RFC says, and a
    // failure here is a setup fault rather than a consensus finding.
    const { height } = await requireSharedParent(forkConfig);

    // Matching headers are not enough: the executor hashes
    // `blockhash(block.number - 1)` as the EVM sees it. A fork seeded by
    // `anvil_loadState` restored headers only, so the EVMs disagreed and this
    // case could never run; a *forked* Anvil serves the source chain's real
    // block hashes, so they agree and it can. The check stays as a
    // precondition -- if it ever fails again the collision case is
    // unconstructible and reporting that as a byte-consensus result would be a
    // lie in the more damaging direction -- but it should no longer fire.
    // Probe height-1, not height: `eth_call` executes IN the current block, and
    // BLOCKHASH(current) is 0 by EVM rule -- both chains would return zero and
    // trivially "agree", which is the same trap this check exists to catch.
    const evmParent = await branchesShareEvmParentHash(height - 1, forkConfig);
    if (!evmParent.agree) {
      console.warn(
        `[fork-consensus] F1' SKIPPED: the two chains' EVMs disagree on BLOCKHASH(${height - 1}) ` +
          `(canonical ${evmParent.canonical}, fork ${evmParent.fork}) even though their headers match. ` +
          'The fork is seeded by forking the canonical chain, which does serve its real block hashes, ' +
          'so this should not happen -- check that the fork actually came up in fork mode rather than ' +
          "as an independent chain. RFC 019's residual first-competing-block case is UNTESTED here, " +
          'not disproven.',
      );
      this.skip();
    }

    // One timestamp for both branches. With the shared parent above, this is
    // the whole of the residual collision case.
    const collidingTimestamp = (await canonical.getBlock(height))!.timestamp + 12;
    await pinNextBlockTimestamp(canonical, collidingTimestamp);
    await pinNextBlockTimestamp(fork, collidingTimestamp);
    // Record what the preimage inputs ACTUALLY were. A later test re-seeds the
    // fork from canonical, which overwrites the fork's history -- so comparing
    // the two chains after the run cannot show what they looked like here, and
    // a post-hoc diagnosis of "the parents matched" would be reading canonical
    // history off both sides. Capture it while it is still true.
    console.info(
      `[fork-consensus] F1' preimage inputs: parentHeight=${height} ` +
        `canonicalTip=${(await canonical.getBlock(height))!.hash} forkTip=${(await fork.getBlock(height))!.hash} ` +
        `pinnedTimestamp=${collidingTimestamp}`,
    );

    const canonicalContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(canonical, 0));
    const forkContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(fork, 0));

    // Send first (both now sit in their mempools), then mine, then wait for
    // the receipts. Without the receipts the test cannot know which block
    // actually included each transaction, and an empty block that consumed the
    // pinned timestamp would push the transaction into the next one -- with a
    // timestamp nobody chose, and no collision, reported as a consensus result.
    const [canonicalSent, forkSent] = await Promise.all([
      canonicalContract.combineFromStorage({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
      forkContract.combineFromStorage({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
    ]);
    await Promise.all([mineOneBlock(canonical), mineOneBlock(fork)]);
    const [canonicalReceipt, forkReceipt] = await Promise.all([canonicalSent.wait(), forkSent.wait()]);
    if (!canonicalReceipt || !forkReceipt) throw new Error('a branch never mined its F1 transaction');

    // Read the including blocks by number, bypassing any 'latest' caching.
    const [canonicalBlock, forkBlock] = await Promise.all([
      canonical.getBlock(canonicalReceipt.blockNumber),
      fork.getBlock(forkReceipt.blockNumber),
    ]);
    console.info(
      `[fork-consensus] F1' included in: canonical #${canonicalBlock!.number} ts=${canonicalBlock!.timestamp} ` +
        `parent=${canonicalBlock!.parentHash} | fork #${forkBlock!.number} ts=${forkBlock!.timestamp} ` +
        `parent=${forkBlock!.parentHash}`,
    );

    // These are the preimage inputs the handles are actually derived from, so
    // check them before blaming the result. A mismatch here is the harness
    // failing to build the collision case, not the protocol failing to hold.
    expect(forkBlock!.parentHash, 'the two including blocks must share a parent').to.eq(canonicalBlock!.parentHash);
    expect(forkBlock!.timestamp, 'the two including blocks must share a timestamp').to.eq(canonicalBlock!.timestamp);
    expect(canonicalBlock!.timestamp, 'the including block must carry the pinned timestamp').to.eq(collidingTimestamp);

    const [canonicalHandle, forkHandle] = await Promise.all([
      canonicalContract.combined() as Promise<string>,
      forkContract.combined() as Promise<string>,
    ]);

    // The heart of the revision. The retired suite asserted these differ.
    expect(forkHandle.toLowerCase(), 'a shared parent and timestamp must mint one handle on both branches').to.eq(
      canonicalHandle.toLowerCase(),
    );

    // Every operator computed it, including the one that saw it on the other
    // branch, and all of them agree byte for byte.
    // This suite's own comparison, named so the canary below can falsify the
    // very thing these cases rely on. It is deliberately narrower than the
    // shared comparator: an operator that followed the other branch attributes
    // the handle to a different transaction and block, both legitimately, so
    // only the bytes and the digest are comparable across branches.
    const compareBranchBytes = async (_phase?: 'clean' | 'poisoned' | 'restored') => {
      const current = await Promise.all(databaseUrls.map((url) => waitForHandle(url, canonicalHandle)));
      const first = current[0];
      for (let index = 1; index < current.length; index += 1) {
        expect(
          current[index].ciphertext.equals(first.ciphertext),
          `operator ${index} disagrees on the ciphertext bytes of the aliased handle`,
        ).to.eq(true);
        expect(
          current[index].ciphertextDigest?.equals(first.ciphertextDigest!),
          `operator ${index} disagrees on the digest of the aliased handle`,
        ).to.eq(true);
      }
    };
    await compareBranchBytes();

    // The canary this suite class owes, aimed at `compareBranchBytes` rather
    // than the shared comparator, because that is what the cases above call.
    await assertCanaryFiresWith(databaseUrls[databaseUrls.length - 1], canonicalHandle, "fork-consensus/F1'", compareBranchBytes);

    const rows = await Promise.all(databaseUrls.map((url) => waitForHandle(url, canonicalHandle)));
    const reference = rows[0];

    // First-write-wins is silent: a colliding handle is stored once per
    // operator, never as a duplicate-key error surfaced to the worker.
    for (const row of rows) {
      expect(row.transactionId, 'the aliased handle must carry a producing transaction').to.not.be.undefined;
    }

    // And the fleet reaches quorum on it rather than splitting.
    const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, canonicalHandle);
    expect(consensus, 'the aliased handle must reach on-chain quorum').to.not.be.null;
    const senders = consensus!.senders.map((sender) => sender.toLowerCase());
    expect(new Set(senders).size, 'quorum must come from distinct operators').to.eq(senders.length);
  });

  it("F2': divergent branch content mints distinct handles, and orphaned rows are inert", async function () {
    const canonical = getCanonicalProvider(forkConfig);
    const fork = getForkProvider(forkConfig);

    // Deliberately DIFFERENT timestamps: the branches now diverge in the
    // handle preimage, which is the ordinary case a reorg produces.
    const base = (await canonical.getBlock('latest'))!.timestamp;
    await pinNextBlockTimestamp(canonical, base + 12);
    await pinNextBlockTimestamp(fork, base + 13);

    const canonicalContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(canonical, 0));
    const forkContract = new Contract(contractAddress, fixtureAbi as InterfaceAbi, getSignerForProvider(fork, 0));

    await Promise.all([
      canonicalContract.combineFromStorageAgain({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
      forkContract.combineFromStorageAgain({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT }),
    ]);
    await Promise.all([mineOneBlock(canonical), mineOneBlock(fork)]);

    const [canonicalHandle, forkHandle] = await Promise.all([
      canonicalContract.combinedSecond() as Promise<string>,
      forkContract.combinedSecond() as Promise<string>,
    ]);

    expect(
      forkHandle.toLowerCase(),
      'branches differing in the handle preimage must mint distinct handles, leaving nothing to collide',
    ).to.not.eq(canonicalHandle.toLowerCase());

    // Each branch's handle exists only where that branch was observed. The
    // orphaned row is not corruption: it is keyed by a handle no canonical
    // consumer will ever resolve, which is why the scheme needs no
    // fork-aware storage.
    for (const index of canonicalOperators()) {
      const row = await waitForHandle(databaseUrls[index], canonicalHandle);
      expect(row.handle.length, `operator ${index} must hold the canonical handle`).to.be.greaterThan(0);
    }
    const forkRow = await waitForHandle(databaseUrls[FORK_OPERATOR], forkHandle);
    expect(forkRow.handle.length, 'the forked operator must hold its own branch handle').to.be.greaterThan(0);

    await assertHandleAbsent(databaseUrls[canonicalOperators()[0]], forkHandle);
    forkOnlyHandle = forkHandle;
  });

  it("F3': an allow seen only on the orphaned branch never makes its handle canonically resolvable", async function () {
    expect(forkOnlyHandle, "F2' must run first: F3' reasons about the handle it minted").to.not.eq('');

    // AliasFixture allows every value it mints (`FHE.allow(value, msg.sender)`
    // plus `makePubliclyDecryptable`), so the fork-only handle from F2' comes
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

    // The safety property, which must hold however the bookkeeping is
    // resolved: a handle that existed only on the orphaned branch never
    // reaches quorum, so nothing canonical resolves it and no decryption path
    // can be satisfied for it.
    // Returns null on timeout rather than throwing, which is exactly the
    // shape this assertion wants: absence proven by waiting, not by an error.
    const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, forkOnlyHandle, 60_000);
    expect(consensus, 'a handle only ever seen on an orphaned branch must not reach on-chain quorum').to.be.null;

    // NOT asserted, deliberately: that the orphaned `allowed_handles` rows are
    // deleted. They are not. The listener's reorg retraction covers bridge and
    // delegate observations; ACL allows are removed only by the operator-run
    // `revert_coprocessor_db_state.sql`. Whether that is a gap or is safe
    // because the canonical ACL is the authority is a question for the
    // register, not something this test should silently pin either way.
  });

  it("F4': a reorg that removes a producer must not strand its cross-block child forever", async function () {
    // By this point F3' has replaced the fork's history wholesale. Any chain
    // on the forked operator whose gate was armed by a producer in a removed
    // block now has a dependency count nothing will ever decrement -- unless
    // the stale-gate repair path notices.
    //
    // The invariant is asserted rather than the mechanism: a chain matching
    // the stranded predicate (gate closed, unowned, no unprocessed producer
    // naming it) is a permanent stall no matter how it got there, and no such
    // chain may survive.
    const deadline = Date.now() + 10 * 60_000;
    let stranded = await countStrandedChains(databaseUrls[FORK_OPERATOR]);
    while (Date.now() < deadline && stranded > 0) {
      await new Promise((resolve) => setTimeout(resolve, 5_000));
      stranded = await countStrandedChains(databaseUrls[FORK_OPERATOR]);
    }
    expect(
      stranded,
      'the forked operator has chains whose gate can never be decremented; the stale-gate repair did not run. ' +
        'Note --dcid-stale-gate-age-secs defaults to 300s, so this needs either patience or a scenario override',
    ).to.eq(0);

    // The canonical operators never reorged and must be clean throughout.
    for (const index of canonicalOperators()) {
      expect(await countStrandedChains(databaseUrls[index]), `operator ${index} stranded a chain without a reorg`).to.eq(
        0,
      );
    }
  });

  it("F5': replaying already-ingested events changes nothing", async function () {
    // Rewinding the poller's cursor makes it re-scan blocks it has already
    // ingested and re-deliver their events through the ordinary path. This
    // replaces restarting the listener container: the test container cannot
    // reach the Docker socket (permission denied), and rewinding is the more
    // targeted trigger anyway -- it names exactly which blocks get replayed
    // instead of depending on the catchup margin.
    const before = await ingestionCounts(databaseUrls[FORK_OPERATOR]);
    expect(before.duplicatedDependents, 'a dependents array already held a duplicate before any replay').to.eq(0);

    const rewound = await rewindPollerCursor(databaseUrls[FORK_OPERATOR], 10);
    expect(rewound, 'the poller must have a cursor to rewind, or nothing would be replayed').to.eq(true);
    // Long enough for the rewound range to be re-scanned and committed.
    await new Promise((resolve) => setTimeout(resolve, 90_000));

    const after = await ingestionCounts(databaseUrls[FORK_OPERATOR]);
    expect(after.computations, 'replayed events must not duplicate computation rows').to.eq(before.computations);
    expect(after.allows, 'replayed events must not duplicate allow observations').to.eq(before.allows);
    expect(after.chains, 'replayed events must not create duplicate dependence chains').to.eq(before.chains);
    // The one that would not show up as a row count: arming the same gate
    // twice leaves the dependent listed twice, and the count is then either
    // decremented twice or never reaches zero.
    expect(after.duplicatedDependents, 'a replayed event armed the same gate twice').to.eq(0);
  });
});
