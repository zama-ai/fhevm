import { emitAssertions } from './assertionEvidence';
/**
 * The residual first-competing-block case, built as a replacement block on ONE
 * chain.
 *
 * This is the case the whole RFC 019 revision rests on: when the same handle is
 * minted twice, that proves identical sourcing, so every operator must hold
 * identical bytes for it and first-write-wins makes the second arrival
 * harmless. If handles could collide while bytes differed, fork-aware storage
 * would still be required.
 *
 * The first attempt at this used two Anvils and seeded the fork with
 * `anvil_loadState`. That cannot work, and the reason is worth keeping:
 * loadState restores block HEADERS but not the block-hash history the BLOCKHASH
 * opcode reads, so a seeded chain answers `eth_getBlockByNumber` with the
 * source chain's hashes while its EVM still returns its own originals.
 * `FHEVMExecutor` folds `blockhash(block.number - 1)` into the handle preimage,
 * so the two branches could never mint a colliding handle however carefully the
 * parent and timestamp were pinned.
 *
 * `evm_snapshot`/`evm_revert` has no such problem: it rolls back real history
 * rather than importing a foreign one. So the collision is built the way a real
 * one occurs — a block is replaced by a SIBLING with the same parent and the
 * same timestamp:
 *
 *   1. snapshot at parent P
 *   2. mine B1 = (P, timestamp T, transaction X) and let the fleet compute it
 *   3. revert to P — B1 never happened
 *   4. write an unrelated sentinel balance, then mine
 *      B2 = (P, timestamp T, transaction X)
 *
 * Two things this version fixes.
 *
 * The sentinel write is not decoration. Without a content difference B2 is not
 * a sibling of B1 at all: same parent, same timestamp, same single transaction
 * and the same resulting state produce the same header, so B1 and B2 are the
 * SAME BLOCK and the test compares pre-reorg state against itself. Measured
 * here on a real stack: with only the transaction re-sent, B1 and B2 came back
 * with byte-identical hashes.
 *
 * An extra transaction was the first attempt at the difference and it is the
 * wrong tool: whether it lands in the same block depends on the node's pool
 * ordering, and when it silently did not, the case was back to comparing one
 * block against itself. `anvil_setBalance` on an unrelated address changes the
 * state root the next block commits to, touches no FHEVM contract, and cannot
 * be reordered away. Both the state root and the block hash are then asserted
 * to differ, which is what makes this a replacement.
 *
 * And the fleet has to be shown to have OBSERVED the replacement before
 * anything is compared. Querying the handle after the revert finds the rows the
 * fleet wrote for B1, so "the bytes are unchanged" was true whether or not any
 * listener had seen B2. `host_chain_blocks_valid` records ingested block hashes
 * per operator, so this waits for B2's hash to appear there on every operator
 * first.
 *
 * Runs on any homogeneous topology; needs no fork Anvil.
 */
import { expect } from 'chai';

import { assertCanaryFires } from './canary';
import { rememberMiningState, restoreMiningState } from './abortRecovery';
import { queryStorageRowCount } from './comparator';
import { evmBlockhash } from './forkHelper';
import {
  requireQuorumConfiguration,
  getCoprocessorDbUrls,
  readGatewayMembership,
  waitForDatabaseReadiness,
} from './helpers';
import {
  type ProbeContract,
  assertOperatorsAgree,
  assertQuorumOutcome,
  deployProbe,
  operatorSet,
  waitForOperatorEvidence,
} from './probe';
import { assertRunValidity, withDeadline } from './validity';

const ENABLE_REORG_CONSENSUS = process.env.RUN_REORG_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const GATEWAY_CONFIG_ADDRESS = process.env.GATEWAY_CONFIG_ADDRESS ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const HOST_RPC_URL = process.env.RPC_URL ?? 'http://host-node:8545';
const PROBE_GAS_LIMIT = 10_000_000;
const MARKER = '[reorg-consensus] CASE COMPLETE';

/** Block hashes an operator has ingested for a height, with their status. */
async function ingestedBlocks(
  databaseUrl: string,
  blockNumber: number,
): Promise<{ hash: string; status: string }[]> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1, statement_timeout: 30_000 } as never);
  try {
    const result = await withDeadline(
      pool.query<{ hash: string; block_status: string }>(
        `SELECT '0x' || encode(block_hash, 'hex') AS hash, block_status
           FROM host_chain_blocks_valid
          WHERE block_number = $1
          ORDER BY block_status`,
        [blockNumber],
      ),
      45_000,
      'host_chain_blocks_valid read',
    );
    return result.rows.map((row) => ({ hash: row.hash.toLowerCase(), status: row.block_status }));
  } finally {
    await pool.end().catch(() => undefined);
  }
}

/**
 * Waits until every operator has ingested a specific block hash at a height.
 *
 * This is the acknowledgement the comparison needs. Rows for the handle already
 * exist from the original inclusion, so without this the case would compare
 * pre-reorg state and call it agreement across a reorg.
 */
async function waitForReplacementIngestion(
  databaseUrls: readonly string[],
  operators: readonly number[],
  blockNumber: number,
  expectedHash: string,
  timeoutMs = 8 * 60_000,
): Promise<Map<number, { hash: string; status: string }[]>> {
  const deadline = Date.now() + timeoutMs;
  const seen = new Map<number, { hash: string; status: string }[]>();
  for (;;) {
    let allSeen = true;
    for (const operator of operators) {
      const rows = await ingestedBlocks(databaseUrls[operator], blockNumber);
      seen.set(operator, rows);
      if (!rows.some((row) => row.hash === expectedHash.toLowerCase())) allSeen = false;
    }
    if (allSeen) return seen;
    if (Date.now() >= deadline) {
      throw new Error(
        `no operator was shown to ingest the replacement block ${expectedHash} at height ${blockNumber} within ` +
          `${Math.round(timeoutMs / 1000)}s. Observed: ` +
          `${[...seen]
            .map(([operator, rows]) => `${operator}:[${rows.map((row) => `${row.hash.slice(0, 12)}/${row.status}`).join(' ')}]`)
            .join(', ')}. Comparing the handle now would compare the pre-reorg rows against themselves`,
      );
    }
    await new Promise((resolve) => setTimeout(resolve, 5_000));
  }
}

describe('Replacement-block consensus', function () {
  this.timeout(40 * 60_000);

  let databaseUrls: string[] = [];
  let contract: ProbeContract;
  let provider: import('ethers').JsonRpcProvider;

  before(async function () {
    if (!ENABLE_REORG_CONSENSUS) this.skip();
    requireQuorumConfiguration('reorg-consensus', GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS, CIPHERTEXT_COMMITS_ADDRESS);
    const { JsonRpcProvider } = await import('ethers');
    provider = new JsonRpcProvider(HOST_RPC_URL);

    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // Gated here, before the suite takes deterministic control of block
    // production below: the liveness gate asks whether the chain advances, and
    // a few lines later this suite is the reason it does not.
    console.info(`[reorg-consensus] validity gates: ${await assertRunValidity({ databaseUrls, rpcUrl: HOST_RPC_URL })}`);

    const { getSigners, initSigners } = await import('../signers');
    await initSigners(2);
    const signers = await getSigners();
    contract = (await deployProbe(signers.alice)).contract;

    // Deterministic block production for the duration. With the chain's own
    // interval miner running, a block can be produced between pinning a
    // timestamp and sending the transaction, so the transaction lands under a
    // timestamp nobody chose and the two siblings stop being siblings.
    await rememberMiningState(provider, HOST_RPC_URL);
    await provider.send('evm_setIntervalMining', [0]);
    await provider.send('evm_setAutomine', [false]);
  });

  after(async function () {
    if (!ENABLE_REORG_CONSENSUS || !provider) return;
    // Give the chain back the way the rest of the stack expects it, and prove
    // it is producing blocks again rather than assuming the calls worked: a
    // suite that leaves a paused chain behind fails whatever runs next for its
    // own reason.
    await restoreMiningState(provider, HOST_RPC_URL);
    const before = await provider.getBlockNumber();
    await new Promise((resolve) => setTimeout(resolve, 6_000));
    const after = await provider.getBlockNumber();
    if (after <= before) {
      throw new Error(
        `the host chain is not advancing after this suite restored interval mining (${before} -> ${after}); ` +
          'leaving a stalled chain behind would fail the next suite for this suite\'s reason',
      );
    }
    console.info(`[reorg-consensus] host chain advancing again (${before} -> ${after})`);
    provider.destroy();
  });

  it('a replaced block re-mints the same handle, and the fleet holds one set of bytes for it', async function () {
    const operators = operatorSet(COPROCESSOR_COUNT);

    // Sanity-check the mechanism before relying on it. A revert that left the
    // EVM's BLOCKHASH disagreeing with the headers would silently make the
    // siblings non-siblings, which is precisely the trap the dual-Anvil attempt
    // fell into.
    const parent = await provider.getBlock('latest');
    if (!parent) throw new Error('cannot read the chain tip');
    // BLOCKHASH(current) is 0 by EVM rule, so probe the block below the tip.
    const probeHeight = parent.number - 1;
    const probeHeader = await provider.getBlock(probeHeight);
    const probeEvm = await evmBlockhash(provider, probeHeight);
    expect(
      probeEvm,
      'the chain must agree with its own headers before this test can build a sibling block; ' +
        'a chain seeded by anvil_loadState does not, and cannot host this case',
    ).to.eq(probeHeader!.hash);

    const snapshotId: string = await provider.send('evm_snapshot', []);
    const siblingTimestamp = parent.timestamp + 12;

    // --- first inclusion -------------------------------------------------
    await provider.send('evm_setNextBlockTimestamp', [siblingTimestamp]);
    const firstSent = await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT });
    await provider.send('evm_mine', []);
    // The block is identified by the RECEIPT of the transaction that defines
    // it, never by the `latest` tag. `latest` came back one block behind here
    // -- twice in a row, minutes apart -- so B1 and B2 were both read as the
    // shared parent and the suite compared one block against itself. A receipt
    // names the block its transaction landed in and cannot lag it.
    const firstReceipt = await (firstSent as unknown as {
      wait(): Promise<{ blockNumber: number } | null>;
    }).wait();
    if (!firstReceipt) throw new Error('the first inclusion produced no receipt, so B1 cannot be identified');
    const firstHandle = (await contract.combined()).toLowerCase();
    const firstBlock = await provider.getBlock(firstReceipt.blockNumber);
    expect(firstBlock!.number, 'B1 must be the block that directly follows the shared parent').to.eq(
      parent.number + 1,
    );
    console.info(
      `[reorg-consensus] B1 #${firstBlock!.number} hash=${firstBlock!.hash} ts=${firstBlock!.timestamp} ` +
        `parent=${firstBlock!.parentHash} handle=${firstHandle}`,
    );

    // Let the fleet actually compute it, so the revert lands on real state
    // rather than on an empty queue — the interesting case is a reorg that
    // removes work already done. And require the ORIGINAL block to have been
    // ingested, so "the replacement was ingested" later is a change of state
    // rather than a first sighting.
    const beforeEvidence = await Promise.all(
      operators.map((operator) => waitForOperatorEvidence(databaseUrls[operator], operator, firstHandle)),
    );
    const firstCiphertexts = beforeEvidence.map((evidence) => evidence.ciphertext.toString('hex'));
    await waitForReplacementIngestion(databaseUrls, operators, firstBlock!.number, firstBlock!.hash!);
    console.info('[reorg-consensus] every operator ingested B1 before the replacement');

    // --- the replacement -------------------------------------------------
    const reverted: boolean = await provider.send('evm_revert', [snapshotId]);
    expect(reverted, 'evm_revert must roll the chain back to the shared parent').to.eq(true);
    const afterRevert = Number.parseInt(await provider.send('eth_blockNumber', []), 16);
    expect(afterRevert, 'the chain must be back at the parent height').to.eq(parent.number);

    // The independently controlled content difference that makes B2 a distinct
    // sibling rather than a byte-identical re-mining of B1.
    //
    // It is a direct state write to an unrelated address rather than an extra
    // transaction, and that is deliberate: an extra transaction depends on the
    // node's pool ordering to land in the same block, which is one more thing
    // that has to go right for the sibling to differ. `anvil_setBalance`
    // changes the state root the next block commits to, touches no FHEVM
    // contract, and cannot be reordered away -- measured directly on this
    // Anvil build, where it changes both the state root and the block hash of
    // the next mined block.
    const sentinelAddress = '0x000000000000000000000000000000000000dEaD';
    const sentinelBalance = `0x${(BigInt(siblingTimestamp) * 1_000n).toString(16)}`;
    await provider.send('anvil_setBalance', [sentinelAddress, sentinelBalance]);

    await provider.send('evm_setNextBlockTimestamp', [siblingTimestamp]);
    const secondSent = await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT });
    await provider.send('evm_mine', []);
    const secondReceipt = await (secondSent as unknown as {
      wait(): Promise<{ blockNumber: number } | null>;
    }).wait();
    if (!secondReceipt) throw new Error('the replacement produced no receipt, so B2 cannot be identified');
    const secondHandle = (await contract.combined()).toLowerCase();
    const secondBlock = await provider.getBlock(secondReceipt.blockNumber);
    console.info(
      `[reorg-consensus] B2 #${secondBlock!.number} hash=${secondBlock!.hash} ts=${secondBlock!.timestamp} ` +
        `parent=${secondBlock!.parentHash} handle=${secondHandle} ` +
        `txs=${secondBlock!.transactions.length} (B1 had ${firstBlock!.transactions.length}) ` +
        `stateRoot=${secondBlock!.stateRoot ?? 'n/a'} (B1 ${firstBlock!.stateRoot ?? 'n/a'})`,
    );
    // The mechanism, checked before its consequence: if the sentinel write did
    // not change what B2 commits to, the two blocks are the same block and
    // every comparison below would be reading one of them twice.
    expect(
      secondBlock!.stateRoot,
      'the sentinel state write must change the state root B2 commits to; without a content difference ' +
        'the replacement is byte-identical to the block it replaces',
    ).to.not.eq(firstBlock!.stateRoot);

    // The moment the replacement landed, and what it re-minted. The runner
    // carries both into this case's record: a PASS that names neither cannot
    // be told apart from a run where no reorg happened at all.
    console.info(
      `[reorg-consensus] replacement observed: handle ${secondHandle} in block ${secondBlock!.hash} at ` +
        `${new Date().toISOString()}`,
    );

    // The siblings must really be siblings, and must really be two blocks.
    expect(secondBlock!.parentHash, 'B2 must share B1 parent').to.eq(firstBlock!.parentHash);
    expect(secondBlock!.timestamp, 'B2 must share B1 timestamp').to.eq(firstBlock!.timestamp);
    expect(secondBlock!.number, 'B2 must sit at B1 height').to.eq(firstBlock!.number);
    expect(
      secondBlock!.hash,
      'B2 must be a DIFFERENT block from B1. Identical parent, timestamp, transactions and resulting ' +
        'state produce an identical header, and this case would then be comparing one block against ' +
        'itself rather than measuring a replacement',
    ).to.not.eq(firstBlock!.hash);

    // --- the claim -------------------------------------------------------
    expect(
      secondHandle,
      'a replacement block sharing its predecessor parent and timestamp must re-mint the same handle; ' +
        'this is the residual first-competing-block case RFC 019 relies on',
    ).to.eq(firstHandle);

    // Every operator must be shown to have processed the replacement before any
    // byte comparison. Without this the comparison reads the rows written for
    // B1 and reports agreement about a reorg no listener had seen.
    const ingested = await waitForReplacementIngestion(
      databaseUrls,
      operators,
      secondBlock!.number,
      secondBlock!.hash!,
    );
    for (const [operator, rows] of ingested) {
      console.info(
        `[reorg-consensus] operator ${operator} height ${secondBlock!.number}: ` +
          rows.map((row) => `${row.hash.slice(0, 12)}/${row.status}`).join(' '),
      );
    }

    // Identical sourcing, therefore identical bytes — on every operator, and
    // unchanged from what they computed before the reorg.
    const report = await assertOperatorsAgree(databaseUrls, operators, firstHandle);
    for (const [position, operator] of operators.entries()) {
      const evidence = await waitForOperatorEvidence(databaseUrls[operator], operator, firstHandle);
      expect(
        evidence.ciphertext.toString('hex'),
        `operator ${operator} changed the bytes of ${firstHandle} across the reorg`,
      ).to.eq(firstCiphertexts[position]);
      expect(
        await queryStorageRowCount(databaseUrls[operator], firstHandle),
        `operator ${operator} must hold exactly one storage row for the re-minted handle`,
      ).to.eq(1);
    }

    // The documented contract, asserted rather than assumed: first-write-wins
    // keeps the ORIGINAL attribution. The replacement carries the same
    // transaction (same nonce, same calldata, same signature, therefore the
    // same hash) at the same height, so the provenance is expected to be
    // unchanged, and every operator must agree on it.
    expect(
      report.provenance,
      'the re-minted handle must keep one agreed provenance across the replacement',
    ).to.deep.eq(beforeEvidence[0].provenance);
    console.info(
      `[reorg-consensus] fleet agrees after the replacement; provenance ${report.provenance.join(', ')}; ` +
        `compared ${report.compared.join(', ')}`,
    );

    // The canary this suite class owes. It runs on the handle this case just
    // produced rather than minting another: the chain is under deterministic
    // control here, so an extra mint would need its own mining choreography
    // for no extra falsification.
    const canary = await assertCanaryFires(databaseUrls, operators, firstHandle, 'reorg-consensus');
    expect(canary.kind, 'the canary must fire as a compute-digest mismatch').to.eq('compute-digest');

    const membership = await readGatewayMembership(GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS);
    const outcome = await assertQuorumOutcome({
      mode: 'required',
      gatewayRpcUrl: GATEWAY_RPC_URL,
      ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS,
      handle: firstHandle,
      authorizedSenders: membership.txSenders,
      threshold: membership.threshold,
      label: 'reorg-consensus',
    });
    console.info(`[reorg-consensus] ${outcome.detail}`);

    emitAssertions('REORG-01-REPLACEMENT-BLOCK', ['precondition', 'liveness', 'bytes', 'safety', 'provenance', 'quorum'], 'Distinct sibling blocks satisfied the EVM-visible preimage; all operators ingested the replacement and retained original bytes, storage uniqueness and first-write-wins attribution.');
    console.info(MARKER);
  });
});
