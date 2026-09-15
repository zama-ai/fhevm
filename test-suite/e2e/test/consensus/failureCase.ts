import { emitAssertions } from './assertionEvidence';
import { requireDriftBaseline, requireIngestionTarget } from './failureEvidence';
/**
 * Service-specific failure cases: the workload half.
 *
 * The failure matrix used to be twenty-two variations on one sentence: kill a
 * service, heal it, mint a fresh handle, compare bytes. That establishes that a
 * restarted fleet computes new work correctly, which is worth knowing and is
 * not what the cells claimed. None of them exercised work that was IN FLIGHT
 * when the service went away, so none of them could detect work lost,
 * duplicated or stranded across the outage -- which is the failure mode the
 * matrix exists for.
 *
 * A case therefore has two phases and one workload that spans them:
 *
 *   arm     create the workload, observe that it is genuinely PENDING on the
 *           service that is about to be (or already is) faulted, and publish
 *           its identifiers.
 *   verify  read those identifiers back and require THAT work to have
 *           completed, with the assertions the service's failure mode calls
 *           for.
 *
 * `run-failure-matrix.sh` owns the fault injection and sequences the phases.
 * The workload kinds below are chosen per service, because "an add over two
 * trivially encrypted operands" does not exercise proof ingestion, does not
 * produce a noisy SNS input, and does not involve the relayer or the KMS at
 * all.
 *
 * Environment:
 *   RUN_FAILURE_CASE=1        opt in
 *   FAILURE_CASE_ID           inventory case id, for the log and the record
 *   FAILURE_WORKLOAD          workload kind (see WORKLOADS below)
 *   FAILURE_PHASE             arm | release | submitted | verify
 *   FAILURE_VICTIM_OPERATOR   operator whose service is faulted, default 1
 *   COPROCESSOR_COUNT         fleet size, default 3
 */
import { expect } from 'chai';
import { tamperUnsubmittedDigest } from './canary';
import { markDetectorPublicationStarted, completeDetectorRecovery } from './abortRecovery';

import { queryStorageRowCount } from './comparator';
import { type FaultAcknowledgement, publishHandshake, readHandshake } from './handshake';
import {
  requireQuorumConfiguration,
  getCoprocessorDbUrls,
  queryCanonicalOutputs,
  readGatewayMembership,
  waitForDatabaseReadiness,
} from './helpers';
import {
  type ProbeContract,
  assertOperatorsAgree,
  assertQuorumOutcome,
  deployProbe,
  operatorSet,
} from './probe';
import { assertRunValidity, withDeadline } from './validity';
import { INSTALL_PROOF_OUTCOME_AUDIT, DROP_PROOF_OUTCOME_AUDIT, waitForOriginalProofSucceeded, explicitProofRejection, invalidAuxiliaryProof, recoveredVerifierOutcomes, type OriginalProofInput, type ProofOutcome } from './proofRecovery';

const ENABLE = process.env.RUN_FAILURE_CASE === '1';
const CASE_ID = process.env.FAILURE_CASE_ID ?? 'unnamed';
const WORKLOAD = process.env.FAILURE_WORKLOAD ?? 'compute-chain';
const PHASE = process.env.FAILURE_PHASE ?? 'arm';
const VICTIM = Number.parseInt(process.env.FAILURE_VICTIM_OPERATOR ?? '1', 10);
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const GATEWAY_CONFIG_ADDRESS = process.env.GATEWAY_CONFIG_ADDRESS ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const GAS_LIMIT = 10_000_000;
const MARKER = `[failure-case/${CASE_ID}/${PHASE}] CASE COMPLETE`;

/** The workload kinds a cell can ask for. */
export const WORKLOADS = [
  // A producer and a cross-block dependent, so the fault lands on work with a
  // gate as well as on a computation.
  'compute-chain',
  // A genuinely encrypted (noisy) input, computed and squashed: the SNS input
  // shape a trivial encrypt does not produce.
  'sns-noisy',
  // A real externally encrypted input with its proof, pending verification,
  // plus an invalid-proof control.
  'zkproof-input',
  // Work minted while an ingestion path is down, so its blocks are pending
  // delivery to that path.
  'ingestion',
  // Material complete, submission pending, because the sender is down.
  'submission',
  // A squash whose upload is blocked because object storage is down.
  'storage',
  // A real relayer client request issued before the relayer is faulted.
  // A real decryption request whose id is recorded before the KMS connector is
  // faulted.
  // A deliberately divergent pre-consensus submission, plus an agreeing control.
  'detector-drift',
  // Fresh work after the heal. Labeled smoke, and it says so.
  'smoke',
] as const;

let databaseUrls: string[] = [];

async function withPool<T>(databaseUrl: string, fn: (pool: import('pg').Pool) => Promise<T>): Promise<T> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1, statement_timeout: 30_000 } as never);
  try {
    return await withDeadline(fn(pool), 60_000, 'failure-case query');
  } finally {
    await pool.end().catch(() => undefined);
  }
}

const handleBytes = (handle: string) => Buffer.from(handle.replace(/^0x/, ''), 'hex');

/**
 * Whether an operator has produced the compute row for a handle.
 *
 * Deliberately NOT `queryCanonicalOutputs`, which asks whether the output is
 * publishable and throws when the gateway key id is absent. With the SNS worker
 * down -- which is this stage's whole point -- the key id IS absent, so the
 * strict query reported "not publishable" and the case recorded INVALID for a
 * precondition its own fault had created. The stage needs the weaker fact: the
 * computation exists and succeeded.
 */
async function computeRowPresent(databaseUrl: string, handle: string): Promise<boolean> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ present: boolean }>(
      `SELECT EXISTS (
         SELECT 1
           FROM computations c
           JOIN ciphertexts ct ON ct.handle = c.output_handle
          WHERE c.output_handle = $1
            AND c.is_completed
            AND NOT c.is_error
       ) AS present`,
      [handleBytes(handle)],
    );
    return result.rows[0]?.present === true;
  });
}

/** Whether an operator has written the SNS digest for a handle yet. */
async function snsDigestPresent(databaseUrl: string, handle: string): Promise<boolean> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ present: boolean }>(
      'SELECT ciphertext128 IS NOT NULL AS present FROM ciphertext_digest WHERE handle = $1',
      [handleBytes(handle)],
    );
    return result.rows[0]?.present === true;
  });
}

/** Whether an operator has submitted the handle's material yet. */
async function submissionSent(databaseUrl: string, handle: string): Promise<boolean | null> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ txn_is_sent: boolean }>(
      'SELECT txn_is_sent FROM ciphertext_digest WHERE handle = $1',
      [handleBytes(handle)],
    );
    return result.rows[0]?.txn_is_sent ?? null;
  });
}

/** Drift/revert signals on one operator. */
async function driftSignals(databaseUrl: string): Promise<number> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ count: string }>('SELECT COUNT(*)::text AS count FROM drift_revert_signal');
    return Number.parseInt(result.rows[0].count, 10);
  });
}

/** The poller's cursor, used as an ingestion watermark. */
async function pollerCursor(databaseUrl: string, chainId: string): Promise<number | null> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ block: string | null }>(
      'SELECT MAX(last_caught_up_block)::text AS block FROM host_listener_poller_state WHERE chain_id = $1',
      [chainId],
    );
    const value = result.rows[0]?.block;
    return value === null || value === undefined ? null : Number.parseInt(value, 10);
  });
}

interface ArmedWorkload {
  caseId: string;
  workload: string;
  victim: number;
  contractAddress: string;
  /** Output handles whose fate the verify phase asserts. */
  handles: string[];
  /** Producing transaction hashes. */
  transactionHashes: string[];
  /**
   * Work this case identifies by something other than a handle.
   *
   * The proof case arms by leaving specific `verify_proofs` rows unverified,
   * and those rows have ids, not handles. Without a field for them the case
   * published nothing the runner could name, and the runner refused it --
   * correctly, since "recovery" of unnamed work is not a claim.
   */
  identifiers?: string[];
  /** Free-form, per workload kind. */
  detail?: Record<string, unknown>;
}

describe('Service failure case', function () {
  this.timeout(40 * 60_000);

  let contract: ProbeContract;
  let contractAddress: string;

  before(async function () {
    if (!ENABLE) this.skip();
    requireQuorumConfiguration(WORKLOAD, GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS, CIPHERTEXT_COMMITS_ADDRESS);
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // The faulted operator is held out of the gates: its own injected fault
    // must not be reported back as an invalid run. The deferred gauge is not
    // checked at all here -- a fault in flight is the point.
    console.info(
      `[failure-case/${CASE_ID}] validity gates: ${await assertRunValidity({
        databaseUrls,
        rpcUrl: process.env.RPC_URL,
        operators: operatorSet(COPROCESSOR_COUNT, PHASE === 'arm' ? [VICTIM] : []),
        checkDeferred: false,
      })}`,
    );

    const { getSigners, initSigners } = await import('../signers');
    await initSigners(2);
    const signers = await getSigners();
    if (PHASE === 'arm' || WORKLOAD === 'smoke') {
      const deployment = await deployProbe(signers.alice);
      contract = deployment.contract;
      contractAddress = deployment.address;
    } else {
      const record = readHandshake<ArmedWorkload>('failure-workload').payload;
      contractAddress = record.contractAddress;
      const { ethers } = await import('hardhat');
      contract = (await ethers.getContractAt('AliasFixture', contractAddress, signers.alice)) as unknown as ProbeContract;
    }
  });

  it('arms the workload at the stage the fault is aimed at', async function () {
    if (PHASE !== 'arm') this.skip();
    if (['ingestion', 'submission', 'zkproof-input'].includes(WORKLOAD)) {
      const fault = readHandshake<FaultAcknowledgement & { caseId: string }>('failure-fault').payload;
      expect(fault.caseId).to.eq(CASE_ID);
      expect(fault.applied, 'host must acknowledge the fault before this workload is minted').to.eq(true);
      expect(fault.faultObservedAt).to.be.a('string').and.not.empty;
    }
    const operators = operatorSet(COPROCESSOR_COUNT);
    const armed: ArmedWorkload = {
      caseId: CASE_ID,
      workload: WORKLOAD,
      victim: VICTIM,
      contractAddress,
      handles: [],
      transactionHashes: [],
      detail: {},
    };

    const send = async (method: 'combineFromStorage' | 'combineFromStorageAgain' | 'consumeCombined') => {
      const contractAny = contract as unknown as Record<string, (overrides: unknown) => Promise<{ wait(): Promise<import('ethers').TransactionReceipt | null> }>>;
      const sent = await contractAny[method]({ gasLimit: GAS_LIMIT });
      const receipt = await sent.wait();
      if (!receipt) throw new Error(`${method} produced no receipt`);
      armed.transactionHashes.push(receipt.hash.toLowerCase());
      return receipt;
    };

    switch (WORKLOAD) {
      case 'compute-chain': {
        // A producer and a cross-block dependent. The child's gate is what
        // makes this more than one computation: a fault here can strand work
        // rather than merely delay it.
        await send('combineFromStorage');
        const producer = ((await contract.combined()) as string).toLowerCase();
        await send('consumeCombined');
        const child = ((await (contract as unknown as { consumed(): Promise<string> }).consumed()) as string).toLowerCase();
        armed.handles = [producer, child];
        armed.detail = { producer, child };
        // Observe the exact queued rows before the fault is applied. Database
        // cases freeze all workers; a worker stall freezes only its victim.
        const heldOperators = CASE_ID.startsWith('FMDB-') ? operators : [VICTIM];
        const deadline = Date.now() + 4 * 60_000;
        for (const operator of heldOperators) {
          for (;;) {
            const pending = await withPool(databaseUrls[operator], async (pool) => {
              const result = await pool.query<{ count: string }>(
                `SELECT COUNT(DISTINCT output_handle)::text AS count FROM computations
                 WHERE output_handle = ANY($1) AND NOT is_completed AND NOT is_error`,
                [armed.handles.map(handleBytes)],
              );
              return Number(result.rows[0].count);
            });
            if (pending === armed.handles.length) break;
            if (Date.now() >= deadline) throw new Error(`operator ${operator} did not retain both selected computations pending`);
            await new Promise((resolve) => setTimeout(resolve, 1_000));
          }
        }
        console.info(`[failure-case/${CASE_ID}] armed producer ${producer} and cross-block child ${child}`);
        break;
      }

      case 'sns-noisy': {
        // A genuinely encrypted operand, so the value the SNS worker squashes
        // carries real noise. A trivial encrypt is noiseless and is not
        // representative of every SNS input shape.
        const { createInstances } = await import('../instance');
        const { getSigners } = await import('../signers');
        const signers = await getSigners();
        const instances = await createInstances(signers);
        // A real client encryption, so the value carries genuine noise rather
        // than the noiseless output of an on-chain trivial encrypt.
        const encrypted = await instances.alice.encryptUint64({
          value: 9n,
          contractAddress,
          userAddress: signers.alice.address,
        });
        const { hexlify } = await import('ethers');
        const transaction = await contract.consumeExternal(hexlify(encrypted.handles[0]), hexlify(encrypted.inputProof), { gasLimit: GAS_LIMIT });
        const receipt = await transaction.wait() as { hash: string };
        const handle = (await contract.consumed()).toLowerCase();
        armed.transactionHashes.push(receipt.hash.toLowerCase());
        armed.handles = [handle];
        armed.detail = { encryptedInputHandle: hexlify(encrypted.handles[0]), expectedPlaintext: '16' };
        // The stage: the compute row exists and the SNS digest does NOT, so a
        // fault on the SNS worker lands on work it has not finished.
        const deadline = Date.now() + 4 * 60_000;
        for (;;) {
          const computed = await computeRowPresent(databaseUrls[VICTIM], handle);
          const present = computed ? await snsDigestPresent(databaseUrls[VICTIM], handle) : false;
          if (computed && !present) break;
          if (computed && present) {
            throw new Error(
              `operator ${VICTIM} squashed ${handle} before the fault could be aimed at the SNS worker; ` +
                'this case needs the squash still outstanding, so the runner must fault the worker first',
            );
          }
          if (Date.now() >= deadline) {
            throw new Error(`operator ${VICTIM} never produced a compute row for ${handle} to squash`);
          }
          await new Promise((resolve) => setTimeout(resolve, 2_000));
        }
        console.info(`[failure-case/${CASE_ID}] armed ${handle} with its squash outstanding on operator ${VICTIM}`);
        break;
      }

      case 'zkproof-input': {
        const { createInstances } = await import('../instance');
        const { getSigners } = await import('../signers');
        const { hexlify } = await import('ethers');
        const signers = await getSigners();
        const instances = await createInstances(signers);
        await withPool(databaseUrls[VICTIM], (pool) => pool.query(INSTALL_PROOF_OUTCOME_AUDIT));
        // Keep this original request alive across the fault. The host starts
        // this phase concurrently and heals only after its pending IDs appear.
        const encryption = instances.alice.encryptUint64({
          value: 21n, contractAddress, userAddress: signers.alice.address,
        }).then(value => ({ ok: true as const, value }), error => ({ ok: false as const, error }));
        const deadline = Date.now() + 4 * 60_000;
        let pendingProofs: OriginalProofInput[] = [];
        for (;;) {
          pendingProofs = await withPool(databaseUrls[VICTIM], async (pool) => {
            const result = await pool.query<OriginalProofInput>(
              `SELECT zk_proof_id::text AS "zkProofId", encode(input, 'hex') AS "inputHex",
                      chain_id::text AS "chainId", contract_address AS "contractAddress", user_address AS "userAddress"
                 FROM verify_proofs WHERE verified IS NULL AND input IS NOT NULL
                  AND lower(contract_address) = lower($1) AND lower(user_address) = lower($2)`,
              [contractAddress, signers.alice.address],
            );
            return result.rows;
          });
          if (pendingProofs.length > 0) break;
          if (Date.now() >= deadline) throw new Error('the original encryption left no identifiable pending proof');
          await new Promise(resolve => setTimeout(resolve, 1_000));
        }
        expect(pendingProofs, 'one original SDK encryption must name exactly one proof').to.have.length(1);
        // Validate the retained original before acknowledging its pending IDs.
        invalidAuxiliaryProof(pendingProofs[0]);
        const pendingIds = pendingProofs.map(proof => proof.zkProofId);
        armed.identifiers = pendingIds.map(id => `zk_proof_id:${id}`);
        armed.detail = { pendingIds, originalProof: pendingProofs[0] };
        publishHandshake('failure-workload', armed);
        emitAssertions(CASE_ID, ['precondition'], 'Exactly one original SDK proof was identified pending verification with its retained validated input bytes.');
        const recoveryDeadline = Date.now() + 8 * 60_000;
        for (;;) {
          const ack = readHandshake<FaultAcknowledgement & { caseId: string }>('failure-fault').payload;
          expect(ack.caseId).to.eq(CASE_ID);
          expect(ack.applied).to.eq(true);
          if (ack.recoveryObservedAt) break;
          if (Date.now() >= recoveryDeadline) throw new Error('host did not acknowledge proof-worker recovery');
          await new Promise(resolve => setTimeout(resolve, 500));
        }
        const result = await withDeadline(encryption, 6 * 60_000, 'the original interrupted encryption');
        if (!result.ok) throw new Error(`the original encryption failed: ${String(result.error)}`);
        publishHandshake('failure-proof-result', {
          caseId: CASE_ID,
          handles: result.value.handles.map(handle => hexlify(handle)),
          inputProof: hexlify(result.value.inputProof),
        });
        break;
      }

      case 'ingestion': {
        // The ingestion path is already down. Record its watermark, then mint:
        // the blocks the workload lands in are then pending delivery to it.
        const { ethers } = await import('hardhat');
        const chainId = (await ethers.provider.getNetwork()).chainId.toString();
        const watermark = await pollerCursor(databaseUrls[VICTIM], chainId);
        const receipt = await send('combineFromStorage');
        const handle = ((await contract.combined()) as string).toLowerCase();
        armed.handles = [handle];
        const blockNumber = receipt.blockNumber;
        armed.detail = { watermarkWhenArmed: watermark, blockNumber, chainId };
        expect(
          watermark === null || watermark < blockNumber,
          `operator ${VICTIM}'s ingestion watermark (${watermark}) is already past the workload's block ` +
            `(${blockNumber}), so nothing is pending on the path under test`,
        ).to.eq(true);
        console.info(
          `[failure-case/${CASE_ID}] armed ${handle} in block ${blockNumber} with operator ${VICTIM}'s ` +
            `ingestion watermark at ${watermark}`,
        );
        break;
      }

      case 'submission': {
        // The sender is already down. The stage is: material complete, nothing
        // submitted.
        await send('combineFromStorage');
        const handle = ((await contract.combined()) as string).toLowerCase();
        armed.handles = [handle];
        const deadline = Date.now() + 6 * 60_000;
        for (;;) {
          const sent = await submissionSent(databaseUrls[VICTIM], handle);
          const digest = await snsDigestPresent(databaseUrls[VICTIM], handle);
          if (digest && sent === false) break;
          if (sent === true) {
            throw new Error(
              `operator ${VICTIM} already submitted ${handle}; this case needs the submission outstanding, ` +
                'so the runner must fault the transaction sender first',
            );
          }
          if (Date.now() >= deadline) {
            throw new Error(`operator ${VICTIM} never produced complete material for ${handle} to submit`);
          }
          await new Promise((resolve) => setTimeout(resolve, 3_000));
        }
        console.info(`[failure-case/${CASE_ID}] armed ${handle} with its submission outstanding on operator ${VICTIM}`);
        break;
      }

      case 'storage': {
        // This case arms BEFORE its fault, unlike the others, because the fault
        // is stopping the container whose network namespace this one shares:
        // once object storage is down this suite has no network at all. The
        // runner freezes the squash workers across this phase, so the work is
        // computed and its upload still outstanding when the store goes away --
        // the stage the fault is aimed at, constructed rather than raced.
        //
        // The weaker query for the same reason as the SNS stage: with the
        // squash frozen the output is not publishable yet, and asking the
        // strict question would report the arranged stage as a broken
        // precondition.
        await send('combineFromStorage');
        const handle = ((await contract.combined()) as string).toLowerCase();
        armed.handles = [handle];
        const deadline = Date.now() + 5 * 60_000;
        for (;;) {
          if (await computeRowPresent(databaseUrls[VICTIM], handle)) break;
          if (Date.now() >= deadline) throw new Error(`operator ${VICTIM} never computed ${handle}`);
          await new Promise((resolve) => setTimeout(resolve, 3_000));
        }
        // And the stage itself: the squash must still be outstanding, or the
        // outage lands on work that was already safely uploaded.
        if (await snsDigestPresent(databaseUrls[VICTIM], handle)) {
          throw new Error(
            `operator ${VICTIM} had already squashed ${handle} before the object-storage outage was applied; ` +
              'this case needs the upload still outstanding, so the runner must hold the squash back',
          );
        }
        console.info(`[failure-case/${CASE_ID}] armed ${handle} with object storage unavailable`);
        break;
      }

      case 'detector-drift': {
        // A divergent PRE-consensus submission. The victim's transaction sender
        // is stalled by the runner, so its digest can be poisoned before it
        // submits; when the sender resumes it submits the poisoned digest, the
        // gateway's consensus verdict disagrees with the victim's local row,
        // and that is exactly the input the drift detector consumes.
        await send('combineFromStorage');
        const handle = ((await contract.combined()) as string).toLowerCase();
        await send('combineFromStorageAgain');
        const control = (await contract.combinedSecond()).toLowerCase();
        armed.handles = [handle, control];
        const deadline = Date.now() + 6 * 60_000;
        for (;;) {
          const digest = await snsDigestPresent(databaseUrls[VICTIM], handle);
          const sent = await submissionSent(databaseUrls[VICTIM], handle);
          if (digest && sent === false) break;
          if (sent === true) {
            throw new Error(
              `operator ${VICTIM} submitted ${handle} before it could be poisoned; the runner must stall its ` +
                'transaction sender first',
            );
          }
          if (Date.now() >= deadline) throw new Error(`operator ${VICTIM} never produced material for ${handle}`);
          await new Promise((resolve) => setTimeout(resolve, 3_000));
        }
        const signalsBeforeByOperator = await Promise.all(databaseUrls.map(driftSignals));
        const signalsBefore = signalsBeforeByOperator[VICTIM];
        const original = await tamperUnsubmittedDigest(databaseUrls[VICTIM], handle);
        armed.detail = { originalDigest: original.toString('hex'), signalsBefore, signalsBeforeByOperator, control };
        console.info(
          `[failure-case/${CASE_ID}] poisoned operator ${VICTIM}'s pre-submission digest for ${handle}; ` +
            `control handle ${control}; ${signalsBefore} drift signal(s) before`,
        );
        break;
      }

      case 'smoke': {
        await send('combineFromStorage');
        armed.handles = [((await contract.combined()) as string).toLowerCase()];
        console.warn(
          `[failure-case/${CASE_ID}] SMOKE workload: fresh work minted after the heal. A green here does NOT ` +
            'establish anything about work in flight during the fault',
        );
        break;
      }

      default:
        throw new Error(`unknown FAILURE_WORKLOAD ${WORKLOAD}; expected one of ${WORKLOADS.join(', ')}`);
    }

    publishHandshake('failure-workload', armed);
    emitAssertions(CASE_ID, ['precondition'], 'The selected workload was minted, identified, and its required pending fault stage was observed.');
    void operators;
    console.info(MARKER);
  });

  it('durably records possible detector publication before the host releases its sender', async function () {
    if (PHASE !== 'release') this.skip();
    const record = readHandshake<ArmedWorkload>('failure-workload').payload;
    expect(record.caseId).to.eq(CASE_ID);
    expect(record.workload).to.eq('detector-drift');
    expect(record.handles).to.have.length(2);
    markDetectorPublicationStarted(databaseUrls[record.victim], record.handles[0]);
    console.info(MARKER);
  });

  it('observes the poisoned submission before honest senders resume', async function () {
    if (PHASE !== 'submitted') this.skip();
    const record = readHandshake<ArmedWorkload>('failure-workload').payload;
    const { ethers } = await import('ethers');
    const provider = new ethers.JsonRpcProvider(GATEWAY_RPC_URL);
    const commits = new ethers.Contract(CIPHERTEXT_COMMITS_ADDRESS, [
      'event AddCiphertextMaterial(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address coprocessorTxSender)',
      'event AddCiphertextMaterialConsensus(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address[] coprocessorTxSenders)',
    ], provider);
    const deadline = Date.now() + 120_000;
    try {
      for (;;) {
        const submissions = await commits.queryFilter(commits.filters.AddCiphertextMaterial(record.handles[0]));
        if (submissions.length > 0) {
          expect(submissions).to.have.length(1);
          const poisoned = Buffer.from(String(record.detail?.originalDigest), 'hex');
          poisoned[0] ^= 0xff;
          expect((submissions[0] as import('ethers').EventLog).args.ciphertextDigest.toLowerCase()).to.eq(`0x${poisoned.toString('hex')}`);
          expect(await commits.queryFilter(commits.filters.AddCiphertextMaterialConsensus(record.handles[0]))).to.have.length(0);
          break;
        }
        if (Date.now() >= deadline) throw new Error('poisoned submission did not reach the undecided gateway handle');
        await new Promise((resolve) => setTimeout(resolve, 1_000));
      }
    } finally { provider.destroy(); }
    console.info(MARKER);
  });

  it('requires the armed workload itself to have recovered', async function () {
    if (PHASE !== 'verify') this.skip();
    const record = readHandshake<ArmedWorkload>('failure-workload').payload;
    expect(record.caseId, 'the verify phase must be reading its own case\'s workload').to.eq(CASE_ID);
    const fault = readHandshake<FaultAcknowledgement & { caseId: string }>('failure-fault').payload;
    expect(fault.caseId).to.eq(CASE_ID);
    expect(fault.applied, 'the host must have verified the fault throughout workload arming').to.eq(true);
    expect(fault.faultObservedAt).to.be.a('string').and.not.empty;
    expect(fault.recoveryObservedAt).to.be.a('string').and.not.empty;
    expect(Date.parse(fault.recoveryObservedAt!)).to.be.at.least(Date.parse(fault.faultObservedAt!));
    if (CASE_ID.endsWith('-CRASH')) {
      expect(fault.processBefore).to.be.a('string').and.not.empty;
      expect(fault.processAfter).to.be.a('string').and.not.empty;
      expect(fault.processAfter, 'crash recovery requires a replacement process').not.to.eq(fault.processBefore);
    }
    const operators = operatorSet(COPROCESSOR_COUNT);
    let driftVerification: { signalsBefore: number; signalsAfter: number } | undefined;

    // Common contract for every non-smoke case: the SAME handles complete, on
    // every operator, byte-identically, exactly once.
    //
    // The drift case is the exception, and necessarily so: it poisons one
    // operator's digest on purpose, so requiring the fleet to agree on that
    // handle would assert against the fault the case exists to create. It
    // asserts agreement on its untouched control handle instead, below.
    const agreementHandles = record.workload === 'detector-drift' ? [] : record.handles;
    if (agreementHandles.length > 0) {
      for (const handle of agreementHandles) {
        await assertOperatorsAgree(databaseUrls, operators, handle, { timeoutMs: 12 * 60_000 });
        for (const operator of operators) {
          expect(
            await queryStorageRowCount(databaseUrls[operator], handle),
            `operator ${operator} holds a duplicate storage row for ${handle} after the fault`,
          ).to.eq(1);
        }
      }
      emitAssertions(CASE_ID, ['bytes', 'liveness', 'safety'], 'Every original armed handle completed with canonical fleet bytes/digests and exactly one storage row per operator.');
      console.info(
        `[failure-case/${CASE_ID}] the ${agreementHandles.length} armed handle(s) completed and agree fleet-wide`,
      );
    }

    switch (record.workload) {
      case 'compute-chain': {
        // The child's gate must have been released and the child must be
        // complete: a fault that stranded it would leave the gate closed
        // forever, which byte agreement over the producer alone would hide.
        const child = String(record.detail?.child ?? '');
        expect(child, 'the arming phase must have published the child handle').to.not.eq('');
        for (const operator of operators) {
          const rows = await queryCanonicalOutputs(databaseUrls[operator], [child]);
          expect(rows.length, `operator ${operator} never completed the gated child ${child}`).to.be.greaterThan(0);
        }
        break;
      }

      case 'sns-noisy':
      case 'storage': {
        // Every expected participant's SNS digest, which the shared comparison
        // already requires; stated here so a failure names the service.
        for (const handle of record.handles) {
          for (const operator of operators) {
            expect(
              await snsDigestPresent(databaseUrls[operator], handle),
              `operator ${operator} still holds no SNS digest for ${handle} after the ` +
                `${record.workload === 'storage' ? 'object-storage outage' : 'SNS worker fault'}`,
            ).to.eq(true);
          }
        }
        if (record.workload === 'storage') {
          // Retrievable and digest-bound, and decryptable: an upload that
          // "completed" into an unreadable object is not recovery.
          const { createInstances } = await import('../instance');
          const { getSigners } = await import('../signers');
          const signers = await getSigners();
          const instances = await createInstances(signers);
          const plaintext = await withDeadline(
            instances.alice.userDecryptSingleHandle({
              handle: record.handles[0],
              contractAddress: record.contractAddress,
              signer: signers.alice,
            }) as Promise<unknown>,
            8 * 60_000,
            'representative decryption after the storage outage',
          );
          expect(String(plaintext), 'recovered object plaintext').to.eq('12');
          emitAssertions(CASE_ID, ['correctness'], 'Recovered object material was retrievable and decrypted to plaintext12.');
          console.info(`[failure-case/${CASE_ID}] representative decryption returned ${String(plaintext)}`);
        }
        break;
      }

      case 'zkproof-input': {
        const { createInstances } = await import('../instance');
        const { getSigners } = await import('../signers');
        const signers = await getSigners();
        const instances = await createInstances(signers);
        const encrypted = readHandshake<{ caseId: string; handles: string[]; inputProof: string }>('failure-proof-result').payload;
        expect(encrypted.caseId).to.eq(CASE_ID);
        expect(encrypted.handles).to.have.length(1);
        const armedIds = (record.identifiers ?? []).map(id => id.replace(/^zk_proof_id:/, ''));
        expect(armedIds).to.have.length(1);
        const outcomes = (id: string | number) => withPool(databaseUrls[record.victim], async pool => {
          const result = await pool.query<ProofOutcome>(
            `SELECT verified, encode(handles, 'hex') AS handles, host(client_address) AS "clientAddress",
                    observed_at::text AS "observedAt", operation
             FROM public.consensus_test_proof_outcomes WHERE zk_proof_id = $1 ORDER BY observed_at`, [id]);
          return recoveredVerifierOutcomes(result.rows, fault.recoveredWorkerAddress, fault.faultObservedAt);
        });
        let controlId: number | undefined;
        try {
          for (const id of armedIds) await waitForOriginalProofSucceeded(() => outcomes(id), encrypted.handles);
          // A verifier success must have materialized THIS request's input on
          // every operator, including the interrupted one.
          for (const database of databaseUrls) {
            const materialized = await withPool(database, async pool => {
              const result = await pool.query<{ count: string }>(
                'SELECT count(*)::text AS count FROM ciphertexts WHERE handle = $1', [handleBytes(encrypted.handles[0])]);
              return Number(result.rows[0].count);
            });
            expect(materialized, 'the original verified input must be materialized').to.eq(1);
          }
          // This is the same serialized proof whose committed success was
          // established above. Changing only its bound user keeps decoding
          // valid and requires actual cryptographic verification to reject it.
          const original = record.detail?.originalProof as OriginalProofInput;
          const control = invalidAuxiliaryProof(original);
          expect(original.zkProofId, 'the control must reuse the interrupted proof').to.eq(armedIds[0]);
          expect(original.contractAddress.toLowerCase()).to.eq(record.contractAddress.toLowerCase());
          expect(original.userAddress.toLowerCase()).to.eq(signers.alice.address.toLowerCase());
          controlId = 9_000_000_000 + (Date.now() % 1_000_000);
          await withPool(databaseUrls[record.victim], pool => pool.query(
            'INSERT INTO public.consensus_test_proof_controls(zk_proof_id) VALUES ($1)', [controlId]));
          await withPool(databaseUrls[record.victim], pool => pool.query(
            'INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, input, verified) VALUES ($1,$2,$3,$4,$5,NULL)',
            [controlId, control.chainId, control.contractAddress, control.userAddress, Buffer.from(control.inputHex, 'hex')],
          ));
          const deadline = Date.now() + 5 * 60_000;
          while (!explicitProofRejection(await outcomes(controlId))) {
            if (Date.now() >= deadline) throw new Error('no committed rejection of the original proof with mismatched auxiliary data');
            await new Promise(resolve => setTimeout(resolve, 250));
          }
          const tx = await contract.consumeExternal(encrypted.handles[0], encrypted.inputProof, { gasLimit: GAS_LIMIT });
          await tx.wait();
          const producedHandle = (await contract.consumed()).toLowerCase();
          record.handles = [producedHandle];
          await assertOperatorsAgree(databaseUrls, operators, producedHandle, { timeoutMs: 8 * 60_000 });
          const plaintext = await withDeadline(instances.alice.userDecryptSingleHandle({
            handle: producedHandle, contractAddress: record.contractAddress, signer: signers.alice,
          }), 8 * 60_000, 'consumption of the original recovered proof input');
          expect(String(plaintext), 'original encrypted 21 plus fixture input 7').to.eq('28');
          emitAssertions(CASE_ID, ['liveness', 'bytes', 'safety'], 'Original SDK proof succeeded under the recovered verifier, materialized once, matched fleet bytes, passed downstream plaintext28, and wrong-aux control was durably rejected.');
          console.info(`[failure-case/${CASE_ID}] original proof ${armedIds[0]} verified, materialized and consumed as ${producedHandle}`);
        } finally {
          await withPool(databaseUrls[record.victim], async pool => {
            if (controlId !== undefined) await pool.query('DELETE FROM verify_proofs WHERE zk_proof_id = $1', [controlId]);
            await pool.query(DROP_PROOF_OUTCOME_AUDIT);
          });
        }
        break;
      }

      case 'ingestion': {
        // The path under test must have caught up past the workload's block,
        // and nothing may have been duplicated by the catch-up.
        const { blockNumber, chainId } = requireIngestionTarget(record.detail);
        const deadline = Date.now() + 8 * 60_000;
        for (;;) {
          const cursor = await pollerCursor(databaseUrls[record.victim], chainId);
          if (cursor !== null && cursor >= blockNumber) {
            console.info(
              `[failure-case/${CASE_ID}] operator ${record.victim}'s ingestion path caught up to ${cursor} ` +
                `(workload block ${blockNumber})`,
            );
            break;
          }
          if (Date.now() >= deadline) {
            throw new Error(
              `operator ${record.victim}'s ingestion watermark is ${cursor} and never reached the workload's ` +
                `block ${blockNumber}; the events pending on the faulted path were not delivered`,
            );
          }
          await new Promise((resolve) => setTimeout(resolve, 5_000));
        }
        emitAssertions(CASE_ID, ['liveness'], 'The victim ingestion cursor caught up past the exact recorded workload block on the recorded chain.');
        if (CASE_ID === 'FM-BROKER-OUTAGE') emitAssertions(CASE_ID, ['scope'], 'The original work completed through the surviving ingestion path; this is redundancy evidence, not broker delivery attribution.');
        break;
      }

      case 'submission': {
        // The sender must have submitted the material it was holding, and the
        // commitment must have formed from authorized members.
        for (const handle of record.handles) {
          const deadline = Date.now() + 8 * 60_000;
          for (;;) {
            if ((await submissionSent(databaseUrls[record.victim], handle)) === true) break;
            if (Date.now() >= deadline) {
              throw new Error(
                `operator ${record.victim} never submitted ${handle} after its transaction sender recovered`,
              );
            }
            await new Promise((resolve) => setTimeout(resolve, 5_000));
          }
        }
        break;
      }

      case 'detector-drift': {
        // The poisoned submission must have produced exactly one drift/revert
        // signal on the victim, and the agreeing control must have produced
        // none anywhere.
        const signalsBeforeByOperator = requireDriftBaseline(record.detail, record.victim, COPROCESSOR_COUNT);
        const signalsBefore = signalsBeforeByOperator[record.victim];
        const deadline = Date.now() + 8 * 60_000;
        let signals = signalsBefore;
        for (;;) {
          signals = await driftSignals(databaseUrls[record.victim]);
          if (signals > signalsBefore) break;
          if (Date.now() >= deadline) {
            throw new Error(
              `operator ${record.victim} submitted a digest that disagreed with the gateway's consensus verdict ` +
                `and raised no drift signal (still ${signals}); the detector did not react to the divergence ` +
                'it exists for',
            );
          }
          await new Promise((resolve) => setTimeout(resolve, 5_000));
        }
        expect(signals - signalsBefore, 'exactly one signal for the divergent workload').to.eq(1);
        console.info(
          `[failure-case/${CASE_ID}] the divergent submission raised ${signals - signalsBefore} drift signal(s) ` +
            `on operator ${record.victim}`,
        );
        for (const operator of operators.filter((entry) => entry !== record.victim)) {
          expect(
            await driftSignals(databaseUrls[operator]),
            `operator ${operator} agreed with the consensus verdict and must not have raised a drift signal`,
          ).to.eq(signalsBeforeByOperator[operator]);
        }

        // The signal starts an asynchronous revert, including its configured grace
        // period. Comparing immediately would observe the deliberately poisoned
        // row before recovery has had a chance to replace it.
        for (;;) {
          const status = await withPool(databaseUrls[record.victim], async (pool) => {
            const result = await pool.query<{ status: string }>(
              'SELECT status FROM drift_revert_signal ORDER BY id DESC LIMIT 1',
            );
            return result.rows[0]?.status;
          });
          if (status === 'done') break;
          if (status === 'failed' || Date.now() >= deadline) {
            throw new Error(`operator ${record.victim}'s drift recovery did not finish (status ${status})`);
          }
          await new Promise((resolve) => setTimeout(resolve, 2_000));
        }

        // The healthy control: an untouched handle from the same window must
        // still agree fleet-wide and raise nothing. Without it, "a signal
        // appeared" could be a detector that signals on everything.
        const control = String(record.detail?.control ?? '');
        expect(control, 'the arming phase must have published a control handle').to.not.eq('');
        await assertOperatorsAgree(databaseUrls, operators, control, { timeoutMs: 10 * 60_000 });
        for (const handle of record.handles) {
          await assertOperatorsAgree(databaseUrls, operators, handle, { timeoutMs: 10 * 60_000 });
        }
        const signalsAfter = await driftSignals(databaseUrls[record.victim]);
        expect(signalsAfter - signalsBefore,
          'recovery and the agreeing control must not create another signal').to.eq(1);
        await completeDetectorRecovery(databaseUrls[record.victim], record.handles[0], Buffer.from(String(record.detail?.originalDigest), 'hex'));
        driftVerification = { signalsBefore, signalsAfter };
        console.info(`[failure-case/${CASE_ID}] the agreeing control ${control} raised no signal anywhere`);
        break;
      }

      case 'smoke':
        emitAssertions(CASE_ID, ['scope'], 'The checked handles were deliberately minted after recovery; no in-flight recovery claim is made.');
        break;

      default:
        throw new Error(`unknown workload ${record.workload} in the armed record`);
    }

    // Every non-smoke case ends in the quorum question too, judged against the
    // gateway's own threshold.
    if (record.workload !== 'smoke') {
      const membership = await readGatewayMembership(GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS);
      // For the drift case this is the control handle: the poisoned one is
      // exactly the handle whose commitment the case made disagree.
      const quorumHandle =
        record.workload === 'detector-drift' ? String(record.detail?.control ?? '') : record.handles[0];
      // An absent handle is a harness error, and a loud one: `queryFilter` with
      // an undefined topic matches every event ever emitted, so the case would
      // report "duplicate consensus events for undefined" -- a system-sounding
      // failure caused entirely by asking about nothing.
      expect(
        quorumHandle,
        `${CASE_ID}: reached the quorum assertion with no handle to ask about; the ${record.workload} ` +
          'workload must record the handle whose commitment it expects',
      ).to.match(/^0x[0-9a-f]{64}$/);
      const outcome = await assertQuorumOutcome({
        mode: 'required',
        gatewayRpcUrl: GATEWAY_RPC_URL,
        ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS,
        handle: quorumHandle,
        authorizedSenders: membership.txSenders,
        threshold: membership.threshold,
        timeoutMs: 8 * 60_000,
        label: CASE_ID,
      });
      console.info(`[failure-case/${CASE_ID}] ${outcome.detail}`);
      emitAssertions(CASE_ID, ['quorum', 'safety'], 'The exact original handle formed one consensus event from distinct authorized gateway members.');
    }

    if (record.workload === 'detector-drift') {
      if (!driftVerification) throw new Error('detector recovery produced no verification evidence');
      emitAssertions(CASE_ID, ['safety', 'liveness', 'scope'], 'Exactly one victim signal reached done; original material recovered and every honest/control baseline stayed unchanged.');
      publishHandshake('failure-verification', {
        runId: process.env.CONSENSUS_RUN_ID, caseId: CASE_ID, workload: record.workload, driftDetected: true, driftRecovered: true,
        ...driftVerification,
        outcomes: [
          { name: 'safety', outcome: 'pass', detail: 'Recovered output bytes and untouched control agree across every operator; required gateway quorum verified.' },
          { name: 'liveness', outcome: 'pass', detail: 'Exactly one new drift signal reached done and the original divergent output recovered.' },
          { name: 'scope', outcome: 'pass', detail: 'Every honest operator retained its recorded baseline; the agreeing control caused no additional signal.' },
        ],
      });
    }
    console.info(MARKER);
  });
});
