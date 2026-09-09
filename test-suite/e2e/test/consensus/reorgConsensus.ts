/**
 * F1' — the residual first-competing-block case, built as a replacement block
 * on ONE chain.
 *
 * This is the case the whole RFC 019 revision rests on: when the same handle
 * is minted twice, that proves identical sourcing, so every operator must hold
 * identical bytes for it and first-write-wins makes the second arrival
 * harmless. If handles could collide while bytes differed, fork-aware storage
 * would still be required.
 *
 * The first attempt at this used two Anvils and seeded the fork with
 * `anvil_loadState`. That cannot work, and the reason is worth keeping:
 * loadState restores block HEADERS but not the block-hash history the
 * BLOCKHASH opcode reads, so a seeded chain answers `eth_getBlockByNumber`
 * with the source chain's hashes while its EVM still returns its own
 * originals. Measured on a seeded fork, its EVM disagreed with its OWN headers
 * at tip-1, tip-5 and tip-50, where a naturally mined chain agreed everywhere.
 * `FHEVMExecutor` folds `blockhash(block.number - 1)` into the handle
 * preimage, so the two branches could never mint a colliding handle however
 * carefully the parent and timestamp were pinned.
 *
 * `evm_snapshot`/`evm_revert` has no such problem: it rolls back real history
 * rather than importing a foreign one, and BLOCKHASH stays consistent with the
 * headers afterwards (verified before this test was written). So the collision
 * is built the way a real one occurs — a block is replaced by a sibling with
 * the same parent and the same timestamp:
 *
 *   1. snapshot at parent P
 *   2. mine B1 = (P, timestamp T, transaction X) and let the fleet compute it
 *   3. revert to P — B1 never happened
 *   4. mine B2 = (P, timestamp T, transaction X) again
 *
 * B1 and B2 share a parent and a timestamp, so they mint the same handle, and
 * that is the collision RFC 019 calls residual. What the fleet does across it
 * is the actual subject.
 *
 * Runs on any homogeneous topology; needs no fork Anvil.
 */
import { expect } from 'chai';

import { getCoprocessorDbUrls, queryCanonicalOutputs, waitForConsensus, waitForDatabaseReadiness } from './helpers';
import { type ProbeContract, assertOperatorsAgree, deployProbe, operatorSet, waitForOperatorRow } from './probe';
import { assertCanaryFires } from './canary';
import { assertRunValidity } from './validity';

const ENABLE_REORG_CONSENSUS = process.env.RUN_REORG_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const HOST_RPC_URL = process.env.RPC_URL ?? 'http://host-node:8545';
const PROBE_GAS_LIMIT = 10_000_000;

describe('Replacement-block consensus (F1)', function () {
  this.timeout(30 * 60_000);

  let databaseUrls: string[] = [];
  let contract: ProbeContract;
  let provider: import('ethers').JsonRpcProvider;

  before(async function () {
    if (!ENABLE_REORG_CONSENSUS) this.skip();
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
    await provider.send('evm_setIntervalMining', [0]);
    await provider.send('evm_setAutomine', [false]);
  });

  after(async function () {
    if (!ENABLE_REORG_CONSENSUS) return;
    // Give the chain back the way the rest of the stack expects it.
    await provider.send('evm_setAutomine', [false]);
    await provider.send('evm_setIntervalMining', [1]);
  });

  it("F1': a replaced block re-mints the same handle, and the fleet holds one set of bytes for it", async function () {
    const operators = operatorSet(COPROCESSOR_COUNT);

    // Sanity-check the mechanism before relying on it. A revert that left the
    // EVM's BLOCKHASH disagreeing with the headers would silently make the
    // siblings non-siblings, which is precisely the trap the dual-Anvil
    // attempt fell into.
    const parent = await provider.getBlock('latest');
    if (!parent) throw new Error('cannot read the chain tip');
    const evmParentHash = await provider.call({
      data: `0x61${parent.number.toString(16).padStart(4, '0')}4060005260206000f3`,
    });
    // BLOCKHASH(current) is 0 by EVM rule, so probe the block below the tip.
    const probeHeight = parent.number - 1;
    const probeHeader = await provider.getBlock(probeHeight);
    const probeEvm = await provider.call({
      data: `0x61${probeHeight.toString(16).padStart(4, '0')}4060005260206000f3`,
    });
    expect(
      probeEvm,
      'the chain must agree with its own headers before this test can build a sibling block; ' +
        'a chain seeded by anvil_loadState does not, and cannot host this case',
    ).to.eq(probeHeader!.hash);
    void evmParentHash;

    const snapshotId: string = await provider.send('evm_snapshot', []);
    const siblingTimestamp = parent.timestamp + 12;

    // --- first inclusion -------------------------------------------------
    await provider.send('evm_setNextBlockTimestamp', [siblingTimestamp]);
    const firstSent = await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT });
    await provider.send('evm_mine', []);
    await (firstSent as unknown as { wait(): Promise<unknown> }).wait();
    const firstHandle = (await contract.combined()).toLowerCase();
    const firstBlock = await provider.getBlock('latest');
    console.info(
      `[reorg-consensus] B1 #${firstBlock!.number} ts=${firstBlock!.timestamp} parent=${firstBlock!.parentHash} ` +
        `handle=${firstHandle}`,
    );

    // Let the fleet actually compute it, so the revert lands on real state
    // rather than on an empty queue — the interesting case is a reorg that
    // removes work already done.
    const beforeRows = await Promise.all(
      operators.map((index) => waitForOperatorRow(databaseUrls[index], firstHandle)),
    );
    const firstCiphertexts = beforeRows.map((row) => row.ciphertext.toString('hex'));

    // --- the replacement -------------------------------------------------
    const reverted: boolean = await provider.send('evm_revert', [snapshotId]);
    expect(reverted, 'evm_revert must roll the chain back to the shared parent').to.eq(true);
    const afterRevert = await provider.getBlock('latest');
    expect(afterRevert!.number, 'the chain must be back at the parent height').to.eq(parent.number);

    await provider.send('evm_setNextBlockTimestamp', [siblingTimestamp]);
    const secondSent = await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT });
    await provider.send('evm_mine', []);
    await (secondSent as unknown as { wait(): Promise<unknown> }).wait();
    const secondHandle = (await contract.combined()).toLowerCase();
    const secondBlock = await provider.getBlock('latest');
    console.info(
      `[reorg-consensus] B2 #${secondBlock!.number} ts=${secondBlock!.timestamp} parent=${secondBlock!.parentHash} ` +
        `handle=${secondHandle}`,
    );

    // The siblings must really be siblings, or the rest proves nothing.
    expect(secondBlock!.parentHash, 'B2 must share B1 parent').to.eq(firstBlock!.parentHash);
    expect(secondBlock!.timestamp, 'B2 must share B1 timestamp').to.eq(firstBlock!.timestamp);
    expect(secondBlock!.number, 'B2 must sit at B1 height').to.eq(firstBlock!.number);

    // --- the claim -------------------------------------------------------
    expect(
      secondHandle,
      'a replacement block sharing its predecessor parent and timestamp must re-mint the same handle; ' +
        'this is the residual first-competing-block case RFC 019 relies on',
    ).to.eq(firstHandle);

    // Identical sourcing, therefore identical bytes — on every operator, and
    // unchanged from what they computed before the reorg.
    const report = await assertOperatorsAgree(databaseUrls, operators, firstHandle);
    const afterRows = await Promise.all(
      operators.map((index) => queryCanonicalOutputs(databaseUrls[index], [firstHandle])),
    );
    afterRows.forEach((rows, position) => {
      expect(rows.length, `operator ${operators[position]} must hold exactly one row for the re-minted handle`).to.eq(1);
      expect(
        rows[0].ciphertext.toString('hex'),
        `operator ${operators[position]} changed the bytes of ${firstHandle} across the reorg`,
      ).to.eq(firstCiphertexts[position]);
    });
    console.info(
      `[reorg-consensus] fleet agrees after replacement; ` +
        `sns=${report.snsDigestsChecked ? 'agreed' : 'not comparable'}`,
    );

    // The canary this suite class owes. It runs on the handle this case just
    // produced rather than minting another: the chain is under deterministic
    // control here, so an extra mint would need its own mining choreography
    // for no extra falsification.
    await assertCanaryFires(databaseUrls, operators, firstHandle, 'reorg-consensus');

    // Quorum is checked only when asked for. Defect B-1 -- one operator
    // emitting a different SNS digest for computed handles -- makes a
    // unanimous quorum unreachable regardless of anything this test does, and
    // asserting it here would attribute that to the reorg.
    if (process.env.REORG_EXPECT_QUORUM === '1') {
      const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, firstHandle, 5 * 60_000);
      expect(consensus, 'the re-minted handle must reach on-chain quorum').to.not.be.null;
    }
  });
});
