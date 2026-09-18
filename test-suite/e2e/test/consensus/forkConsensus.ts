import { emitAssertions } from './assertionEvidence';
import { successfulForkReceipt } from './forkRecovery';
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
 * The authorization case also checks what a reorg must not authorize:
 *
 *   F3  An ACL allow observed only on the orphaned branch must not authorize a
 *       canonical decryption. Asserted through the real decryption interface,
 *       with a canonical positive control -- absence of quorum on its own says
 *       nothing about authorization, and describing it as an authorization test
 *       was the weakest claim in the previous version.
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
    if (PHASE !== 'main') throw new Error('Only the main fork phase is available in the consensus layer');
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

});
