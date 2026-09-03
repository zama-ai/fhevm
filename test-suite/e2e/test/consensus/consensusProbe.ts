/**
 * A single consensus probe, runnable on demand.
 *
 * The failure-matrix orchestrator injects a fault host-side, heals it, then
 * invokes this to answer the only question that matters: after all that, do
 * the operators still agree on the bytes? Keeping the assertion in one place
 * means every matrix cell ends the same way, and a cell cannot accidentally
 * settle for "the container came back".
 *
 * Environment contract:
 *   RUN_CONSENSUS_PROBE=1        opt in (this file is inert otherwise)
 *   COPROCESSOR_COUNT            fleet size, default 3
 *   PROBE_EXCLUDE_OPERATORS      comma-separated indexes to leave out of the
 *                                comparison, for cells that deliberately hold
 *                                an operator down
 *   PROBE_EXPECT_QUORUM=1        additionally require on-chain quorum
 *   PROBE_CONTRACT_ADDRESS       reuse an already-deployed fixture instead of
 *                                deploying (much faster across many cells)
 *   PROBE_LABEL                  free text, echoed into the output so a cell's
 *                                result is attributable in a long run
 */
import { expect } from 'chai';

import { getCoprocessorDbUrls, waitForConsensus, waitForDatabaseReadiness } from './helpers';
import { assertCanaryFires } from './canary';
import { assertRunValidity } from './validity';
import {
  type ProbeContract,
  type ProbeShape,
  assertOperatorsAgree,
  deployProbe,
  mintProbeHandle,
  operatorSet,
} from './probe';

const ENABLE_PROBE = process.env.RUN_CONSENSUS_PROBE === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const EXPECT_QUORUM = process.env.PROBE_EXPECT_QUORUM === '1';
const LABEL = process.env.PROBE_LABEL ?? 'probe';
// `boundary` (default) adds two trivially encrypted values persisted in an
// earlier transaction; `local` recomputes them inside the consuming
// transaction. Same values and opcode, different operand provenance.
const SHAPE: ProbeShape = process.env.PROBE_SHAPE === 'local' ? 'local' : 'boundary';
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';

function excludedOperators(): number[] {
  const raw = process.env.PROBE_EXCLUDE_OPERATORS ?? '';
  return raw
    .split(',')
    .map((part) => part.trim())
    .filter((part) => part.length > 0)
    .map((part) => {
      const index = Number.parseInt(part, 10);
      if (!Number.isInteger(index) || index < 0 || index >= COPROCESSOR_COUNT) {
        throw new Error(`PROBE_EXCLUDE_OPERATORS names operator ${part}, which is outside the topology`);
      }
      return index;
    });
}

describe('Consensus probe', function () {
  this.timeout(15 * 60_000);

  let databaseUrls: string[] = [];
  let contract: ProbeContract;

  before(async function () {
    if (!ENABLE_PROBE) this.skip();
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // Gate before measuring. A matrix cell heals its fault and then asks this
    // probe whether the operators agree; if the stack is wedged or unprovisioned
    // the honest answer is "this run cannot tell you", not a green cell. The
    // held-out operators are excluded, because a cell that keeps one down on
    // purpose must not have its own fault reported back as an invalid run.
    const gated = await assertRunValidity({
      databaseUrls,
      operators: operatorSet(COPROCESSOR_COUNT, excludedOperators()),
      rpcUrl: process.env.RPC_URL,
    });
    console.info(`[${LABEL}] validity gates: ${gated}`);

    const [{ getSigners, initSigners }] = await Promise.all([import('../signers')]);
    await initSigners(2);
    const signers = await getSigners();

    const reuse = process.env.PROBE_CONTRACT_ADDRESS;
    if (reuse) {
      const { ethers } = await import('hardhat');
      contract = (await ethers.getContractAt('AliasFixture', reuse, signers.alice)) as unknown as ProbeContract;
    } else {
      contract = (await deployProbe(signers.alice)).contract;
      console.info(`[${LABEL}] deployed probe fixture at ${await contract.getAddress()}`);
    }
  });

  it('operators agree on the bytes of a freshly computed handle', async function () {
    const excluded = excludedOperators();
    const operators = operatorSet(COPROCESSOR_COUNT, excluded);
    if (operators.length < 2) {
      throw new Error('a consensus probe needs at least two operators left in the comparison');
    }

    const handle = await mintProbeHandle(contract, SHAPE);
    console.info(`[${LABEL}] shape=${SHAPE} minted ${handle}; comparing operators ${operators.join(',')}` +
      (excluded.length ? ` (holding ${excluded.join(',')} out)` : ''));

    // Agreement on the SNS digest is asserted inside `assertOperatorsAgree`,
    // so reaching this line means the operators agreed on everything they still
    // held. It reports whether the digest was comparable at all: the
    // transaction sender clears `ciphertext128` once its AddCiphertextMaterial
    // transaction lands, so a cell that ran slowly can legitimately have too
    // few digests left to compare.
    const report = await assertOperatorsAgree(databaseUrls, operators, handle);
    console.info(
      `[${LABEL}] AGREE ct=${report.ciphertextDigest.slice(0, 16)} ` +
        `sns=${report.snsDigestsChecked ? 'agreed' : 'not comparable (already submitted)'}`,
    );

    // The canary, opt-in via PROBE_CANARY=1. This probe is invoked once per
    // failure-matrix cell, and poisoning a digest in every cell would triple the
    // cost of a matrix run for no extra information -- one falsification per
    // run is what the rule asks for, so the orchestrator arms it on its
    // baseline invocation and leaves it off for the cells.
    if (process.env.PROBE_CANARY === '1') {
      await assertCanaryFires(databaseUrls, operators, await mintProbeHandle(contract, SHAPE), LABEL);
    }

    if (EXPECT_QUORUM) {
      const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, handle, 5 * 60_000);
      expect(consensus, `${LABEL}: ${handle} must reach on-chain quorum after recovery`).to.not.be.null;
      const senders = consensus!.senders.map((sender) => sender.toLowerCase());
      expect(new Set(senders).size, `${LABEL}: quorum must come from distinct operators`).to.eq(senders.length);
      console.info(`[${LABEL}] QUORUM ${senders.length} distinct submitter(s)`);
    }
  });
});
