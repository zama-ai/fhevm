/**
 * A single consensus probe, runnable on demand.
 *
 * The cheap end of the oracle: mint one handle, hand it to the shared
 * comparator, and optionally require the quorum outcome the caller names. It
 * exists so a caller that needs "do the operators still agree?" between two
 * steps does not restate the assertion, and so no caller can accidentally
 * settle for "the container came back".
 *
 * It is deliberately NOT the failure matrix's contract any more. Every matrix
 * cell used to end here, which made the matrix a set of variations on "the
 * service restarted and fresh work still agrees" -- true, useful, and not what
 * the cells claimed. The cells now arm a workload on the service they are about
 * to fault and require THAT work to recover (`failureCase.ts`); this probe
 * remains for baselines and for callers who want the cheap check by name.
 *
 * Environment contract:
 *   RUN_CONSENSUS_PROBE=1        opt in (this file is inert otherwise)
 *   COPROCESSOR_COUNT            fleet size, default 3
 *   PROBE_EXCLUDE_OPERATORS      comma-separated indexes to leave out of the
 *                                comparison, for callers that deliberately hold
 *                                an operator down
 *   PROBE_QUORUM                 required | forbidden | not_checked
 *   PROBE_CONTRACT_ADDRESS       reuse an already-deployed fixture instead of
 *                                deploying (much faster across many calls)
 *   PROBE_CANARY=1               additionally falsify the comparator
 *   PROBE_LABEL                  free text, echoed into the output so a
 *                                caller's result is attributable in a long run
 */
import { expect } from 'chai';

import { assertCanaryFires } from './canary';
import { getCoprocessorDbUrls, readGatewayMembership, waitForDatabaseReadiness } from './helpers';
import {
  type ProbeContract,
  type ProbeShape,
  type QuorumMode,
  assertOperatorsAgree,
  assertQuorumOutcome,
  deployProbe,
  mintProbeHandle,
  operatorSet,
} from './probe';
import { assertRunValidity } from './validity';

const ENABLE_PROBE = process.env.RUN_CONSENSUS_PROBE === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const LABEL = process.env.PROBE_LABEL ?? 'probe';
// `boundary` (default) adds two trivially encrypted values persisted in an
// earlier transaction; `local` recomputes them inside the consuming
// transaction. Same values and opcode, different operand provenance.
const SHAPE: ProbeShape = process.env.PROBE_SHAPE === 'local' ? 'local' : 'boundary';
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const GATEWAY_CONFIG_ADDRESS = process.env.GATEWAY_CONFIG_ADDRESS ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const MARKER = `[${LABEL}] PROBE COMPLETE`;

/**
 * The quorum expectation, as a mode rather than a boolean.
 *
 * A boolean called `expectQuorum` defaulted to `0` could not distinguish
 * "quorum must NOT form" from "quorum was not checked", and callers used the
 * same `0` for both -- including on a 2-of-3 topology where quorum should form.
 * `not_checked` now says so out loud, and it is only legitimate for a caller
 * whose case is about something else.
 */
function quorumMode(): QuorumMode {
  const raw = process.env.PROBE_QUORUM ?? 'not_checked';
  if (raw === 'required' || raw === 'forbidden' || raw === 'not_checked') return raw;
  throw new Error(`PROBE_QUORUM must be required, forbidden or not_checked; got ${raw}`);
}

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
  this.timeout(20 * 60_000);

  let databaseUrls: string[] = [];
  let contract: ProbeContract;

  before(async function () {
    if (!ENABLE_PROBE) this.skip();
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // Gate before measuring. A caller heals its fault and then asks this probe
    // whether the operators agree; if the stack is wedged or unprovisioned the
    // honest answer is "this run cannot tell you", not a green. The held-out
    // operators are excluded, because a caller that keeps one down on purpose
    // must not have its own fault reported back as an invalid run.
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
    console.info(
      `[${LABEL}] shape=${SHAPE} minted ${handle}; comparing operators ${operators.join(',')}` +
        (excluded.length ? ` (holding ${excluded.join(',')} out)` : ''),
    );

    // Bytes, type/version, compute digest, durable SNS digest, key identity,
    // producing operation and normalized provenance, all in one comparison, all
    // required from every compared operator.
    const report = await assertOperatorsAgree(databaseUrls, operators, handle);
    console.info(
      `[${LABEL}] AGREE ct=${report.ciphertextDigest.slice(0, 16)} sns=${report.snsDigest.slice(0, 16)} ` +
        `provenance=${report.provenance.join(',')} compared=${report.compared.join(',')}`,
    );

    // The canary, opt-in. A caller that invokes this probe many times over one
    // stack pays for a falsification once rather than per call, so the
    // orchestrator arms it on its baseline invocation.
    if (process.env.PROBE_CANARY === '1') {
      const outcome = await assertCanaryFires(
        databaseUrls,
        operators,
        await mintProbeHandle(contract, SHAPE),
        LABEL,
      );
      expect(outcome.kind, 'the canary must fire as a compute-digest mismatch').to.eq('compute-digest');
    }

    const mode = quorumMode();
    if (mode === 'not_checked') {
      console.info(`[${LABEL}] quorum NOT CHECKED by this caller`);
    } else {
      const membership = await readGatewayMembership(
        GATEWAY_RPC_URL,
        GATEWAY_CONFIG_ADDRESS || GATEWAY_RPC_URL,
      );
      const outcome = await assertQuorumOutcome({
        mode,
        gatewayRpcUrl: GATEWAY_RPC_URL,
        ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS,
        handle,
        authorizedSenders: membership.txSenders,
        threshold: membership.threshold,
        label: LABEL,
      });
      console.info(`[${LABEL}] ${outcome.detail}`);
    }

    console.info(MARKER);
  });
});
