/**
 * The consensus probe: one small computation, then the question every test in
 * this directory ultimately asks — do all the operators hold the same bytes?
 *
 * This exists because of a rule the coverage register sets for the failure
 * matrix: *every cell's test ends in the consensus assertion*, not in "the
 * service came back". A restart that recovers into different bytes is the
 * failure these tests are for, and a liveness check would sail straight past
 * it. Rather than restate that assertion in every cell, each one calls this.
 *
 * It is deliberately cheap. The materialization gate is the thorough oracle
 * and takes ten minutes; a matrix of dozens of cells cannot pay that per cell,
 * so this does one add over two persisted operands and compares raw bytes,
 * compute and SNS digests, format and provenance across the operator databases.
 *
 * Fault injection is NOT done here. The test container cannot reach the Docker
 * socket, and giving it that access to stop other services would be a strange
 * privilege for a test runner to hold. Faults are injected host-side and this
 * probe is invoked between the steps.
 */
import { expect } from 'chai';

import { type CanonicalOutputRow, queryCanonicalOutputs } from './helpers';

export const PROBE_GAS_LIMIT = 10_000_000;

export interface ProbeContract {
  getAddress(): Promise<string>;
  waitForDeployment(): Promise<unknown>;
  produceInputs(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  combineFromStorage(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  combineLocal(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  combined(): Promise<string>;
  combinedLocal(): Promise<string>;
}

/**
 * Which operand shape the probe exercises.
 *
 * `boundary` is what every measurement so far used: two trivially encrypted
 * values persisted in an earlier transaction, then added as cross-transaction
 * boundaries. That is an unusual combination — trivially encrypted ciphertexts
 * are noiseless, and consuming them through the compressed boundary
 * representation is not what most traffic does. If the SNS divergence depends
 * on the noise profile of its input, this is exactly where it would show and
 * ordinary traffic would not.
 *
 * `local` recomputes the same operands inside the consuming transaction, so
 * they are forwarded in memory and fold zero boundary bits. Same values, same
 * opcode, different provenance.
 */
export type ProbeShape = 'boundary' | 'local';

/**
 * Deploys the probe fixture and seeds its two operands.
 *
 * The operands are minted in their own transaction on purpose: the add that
 * follows then consumes them as persisted boundary values, which is the path
 * that actually crosses the materialization boundary. An add over two operands
 * minted in the same transaction would forward them in memory and never test
 * the compressed representation the operators must agree on.
 */
export async function deployProbe(owner: unknown): Promise<{ contract: ProbeContract; address: string }> {
  const { ethers } = await import('hardhat');
  const factory = await ethers.getContractFactory('AliasFixture');
  const contract = (await factory.connect(owner as never).deploy()) as unknown as ProbeContract;
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  await (await contract.produceInputs({ gasLimit: PROBE_GAS_LIMIT })).wait();
  return { contract, address };
}

/** Mints one fresh handle and returns it. Repeatable: each call mints a new one. */
export async function mintProbeHandle(
  contract: ProbeContract,
  shape: ProbeShape = 'boundary',
): Promise<string> {
  if (shape === 'local') {
    await (await contract.combineLocal({ gasLimit: PROBE_GAS_LIMIT })).wait();
    return (await contract.combinedLocal()).toLowerCase();
  }
  await (await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT })).wait();
  return (await contract.combined()).toLowerCase();
}

/**
 * Waits for one operator to hold a complete, non-error row for a handle.
 *
 * Absence is only meaningful after the deadline, so a probe that queried once
 * would report ordinary ingestion lag as a consensus failure — which after a
 * deliberately injected fault is exactly the wrong conclusion to jump to.
 */
export async function waitForOperatorRow(
  databaseUrl: string,
  handle: string,
  timeoutMs = 5 * 60_000,
  awaitSnsDigest = false,
): Promise<CanonicalOutputRow> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  let lastRow: CanonicalOutputRow | undefined;
  while (Date.now() < deadline) {
    try {
      const rows = await queryCanonicalOutputs(databaseUrl, [handle]);
      if (rows.length === 1) {
        // The compute row lands before the SNS digest does. Callers that want
        // to compare digests keep waiting rather than compare against a value
        // that simply has not been written yet.
        if (!awaitSnsDigest || rows[0].snsCiphertextDigest !== null) return rows[0];
        lastRow = rows[0];
      }
      if (rows.length > 1) {
        // More than one row is not automatically wrong. Two transactions with
        // identical sourcing alias to the same handle, and the handle then has
        // one value attributed to several producing transactions -- that is
        // the minted-in-transaction discriminant working, not a duplicate.
        // What would be wrong is those rows disagreeing about the value.
        const distinct = new Set(rows.map((row) => row.ciphertext.toString('hex')));
        if (distinct.size === 1) return rows[0];
        throw new Error(
          `${databaseUrl} holds ${rows.length} rows for ${handle} with ${distinct.size} DIFFERENT ciphertexts; ` +
            'one handle must denote one value',
        );
      }
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 2_000));
  }
  // A complete compute row whose digest never arrived is still the best
  // evidence available: the transaction sender clears `ciphertext128` once its
  // AddCiphertextMaterial transaction lands, so a missing digest can mean
  // "already submitted" rather than "never produced". Return it and let the
  // caller decide what it can still compare.
  if (lastRow) return lastRow;
  throw new Error(`timed out waiting for ${handle} in ${databaseUrl}${lastError ? `: ${String(lastError)}` : ''}`);
}

export interface AgreementReport {
  handle: string;
  operators: number[];
  ciphertextDigest: string;
  /** False only when too few operators still held a digest to compare. */
  snsDigestsChecked: boolean;
  snsDigestsAgree: boolean;
}

/**
 * The assertion itself: every named operator holds byte-identical ciphertext,
 * the same compute digest, the same SNS digest, and the same provenance for the
 * handle.
 *
 * The SNS digest was previously reported rather than asserted, because
 * operators had been seen agreeing on compute bytes while disagreeing on it.
 * That turned out to be two workers serving one operator queue on the test host
 * -- a CPU container racing a CUDA host worker, each writing its own backend's
 * bytes (Consensus Defect Log, B-1, closed as not-a-bug; the harness now
 * refuses such a stack). With that gone there is no reason to weaken the
 * probe: the SNS digest is exactly what a unanimous topology must agree on to
 * reach quorum, so it is asserted.
 *
 * It is skipped, not failed, when fewer than two operators still hold a digest.
 * The transaction sender clears `ciphertext128` once its AddCiphertextMaterial
 * transaction lands, so an absent digest is not evidence of disagreement.
 */
export async function assertOperatorsAgree(
  databaseUrls: string[],
  operators: number[],
  handle: string,
  timeoutMs?: number,
): Promise<AgreementReport> {
  const rows = await Promise.all(
    operators.map((index) => waitForOperatorRow(databaseUrls[index], handle, timeoutMs, true)),
  );
  const reference = rows[0];

  for (let i = 1; i < rows.length; i += 1) {
    const operator = operators[i];
    expect(
      rows[i].ciphertext.equals(reference.ciphertext),
      `operator ${operator} holds different ciphertext bytes for ${handle}`,
    ).to.eq(true);
    expect(
      rows[i].ciphertextDigest?.equals(reference.ciphertextDigest!),
      `operator ${operator} holds a different compute digest for ${handle}`,
    ).to.eq(true);
    expect(
      rows[i].fheOperation,
      `operator ${operator} recorded a different producing operation for ${handle}`,
    ).to.eq(reference.fheOperation);
    expect(
      rows[i].transactionId.equals(reference.transactionId),
      `operator ${operator} attributed ${handle} to a different transaction`,
    ).to.eq(true);
    expect(
      rows[i].blockNumber,
      `operator ${operator} attributed ${handle} to a different block`,
    ).to.eq(reference.blockNumber);
  }

  const withDigest = rows
    .map((row, position) => ({ operator: operators[position], digest: row.snsCiphertextDigest, format: row.ciphertext128Format }))
    .filter((entry): entry is { operator: number; digest: Buffer; format: number } => entry.digest !== null);

  const snsDigestsChecked = withDigest.length >= 2;
  if (snsDigestsChecked) {
    const snsReference = withDigest[0];
    for (const entry of withDigest.slice(1)) {
      expect(
        entry.digest.equals(snsReference.digest),
        `operator ${entry.operator} holds a different SNS digest for ${handle} than operator ${snsReference.operator}`,
      ).to.eq(true);
      // The format byte records which backend squashed it (RFC-023: 11 is
      // compressed on CPU, 21 compressed on GPU), so operators disagreeing here
      // is a backend split rather than a bad squash -- worth naming distinctly.
      expect(
        entry.format,
        `operator ${entry.operator} squashed ${handle} in a different format than operator ${snsReference.operator}`,
      ).to.eq(snsReference.format);
    }
  }

  return {
    handle,
    operators,
    ciphertextDigest: reference.ciphertextDigest!.toString('hex'),
    snsDigestsChecked,
    snsDigestsAgree: snsDigestsChecked,
  };
}

/** Operator indexes 0..count-1, minus any the caller knows are deliberately down. */
export function operatorSet(count: number, excluded: number[] = []): number[] {
  return Array.from({ length: count }, (_, index) => index).filter((index) => !excluded.includes(index));
}
