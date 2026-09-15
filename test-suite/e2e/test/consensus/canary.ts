/**
 * The seeded-divergence canary, shared by every suite class.
 *
 * The coverage inventory makes this a standing rule rather than one test: each
 * suite runs one deliberately-poisoned arm and must reject it. This prevents a
 * comparison of one stream against itself or empty evidence from passing the
 * suite without exercising its disagreement detector.
 *
 * What makes a canary worth having is *which* comparator it falsifies, and
 * *how* that comparator failed. Querying digests directly and checking they
 * differ proves only that the canary's own query works; accepting any thrown
 * error means a database outage or a timeout counts as a successfully detected
 * poisoning. So this drives the comparison the other tests actually call, and
 * requires the specific mismatch class the tamper should produce.
 *
 * The tamper is at the consensus layer, not the data layer: it flips a byte of
 * one operator's stored compute *digest*, so the operator still holds correct
 * ciphertext and merely reports the wrong thing about it. That is the shape of
 * the divergence the detector exists for, and it restores cleanly.
 *
 * It runs on an already-published handle on purpose. Poisoning a value before
 * publication would be a test of bad submissions, and it can do irreversible
 * damage on the gateway side; poisoning after publication cannot, because the
 * commitment is already final.
 */
import { expect } from 'chai';
import { Pool } from 'pg';

import { type MismatchKind, ComparisonMismatch } from './mismatch';
import { assertOperatorsAgree } from './probe';
import { saveDigestRecovery, restoreRecordedDigest } from './abortRecovery';

const handleBytes = (handle: string) => Buffer.from(handle.replace(/^0x/, ''), 'hex');

async function withPool<T>(databaseUrl: string, fn: (pool: Pool) => Promise<T>): Promise<T> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1, connectionTimeoutMillis: 10_000, query_timeout: 10_000, statement_timeout: 10_000 });
  try {
    return await fn(pool);
  } finally {
    await pool.end();
  }
}

/**
 * Wait for this operator's own publication, then poison under the same row lock.
 * Local digest readiness or majority quorum can precede the victim's submission.
 * Never expose poisoned bytes to a sender that is still allowed to publish them.
 */
async function tamperDigestWithPublication(
  databaseUrl: string,
  handle: string,
  published: boolean,
  options: { timeoutMs?: number; pollIntervalMs?: number } = {},
): Promise<Buffer> {
  const deadline = Date.now() + (options.timeoutMs ?? 6 * 60_000);
  let uncertainCommitOriginal: Buffer | undefined;
  try {
    return await withPool(databaseUrl, async (pool) => {
      const client = await pool.connect();
      let inTransaction = false;
      try {
        for (;;) {
          await client.query('BEGIN');
          inTransaction = true;
          const current = await client.query<{ ciphertext: Buffer; txn_is_sent: boolean }>(
            'SELECT ciphertext, txn_is_sent FROM ciphertext_digest WHERE handle = $1 FOR UPDATE',
            [handleBytes(handle)],
          );
          if (current.rowCount !== 1) {
            throw new Error(`expected one digest row for ${handle}, found ${current.rowCount}`);
          }
          if (!published && current.rows[0].txn_is_sent !== false) {
            throw new Error(`operator already published ${handle}; detector fault must precede submission`);
          }
          if (current.rows[0].txn_is_sent === published) {
            const original = current.rows[0].ciphertext;
            if (!Buffer.isBuffer(original) || original.length !== 32) {
              throw new Error(`published compute digest for ${handle} is not bytes32`);
            }
            const poisoned = Buffer.from(original);
            poisoned[0] ^= 0xff;
            saveDigestRecovery(databaseUrl, handle, original);
            uncertainCommitOriginal = original;
            await client.query('UPDATE ciphertext_digest SET ciphertext = $2 WHERE handle = $1', [
              handleBytes(handle), poisoned,
            ]);
            await client.query('COMMIT');
            inTransaction = false;
            return original;
          }
          // The sender needs this lock to persist its receipt. Release it before
          // retrying, including at the deadline, so waiting cannot prevent publication.
          await client.query('ROLLBACK');
          inTransaction = false;
          if (Date.now() >= deadline) {
            throw new Error(`operator has not published ${handle}; refusing to expose canary poison to its sender`);
          }
          await new Promise((resolve) => setTimeout(resolve, options.pollIntervalMs ?? 2_000));
        }
      } finally {
        try {
          if (inTransaction) await client.query('ROLLBACK');
        } finally {
          client.release();
        }
      }
    });
  } catch (error) {
    // A COMMIT reply can be lost after PostgreSQL persisted the poison. The
    // caller has not received the original yet, so repair it here using a new
    // connection before reporting the failure.
    if (uncertainCommitOriginal) {
      try { await restoreDigest(databaseUrl, handle, uncertainCommitOriginal); }
      catch (cleanupError) {
        throw new Error(`canary mutation failed (${String(error)}); restoring ${handle} also failed: ${String(cleanupError)}`);
      }
    }
    throw error;
  }
}

/** A comparator canary must wait for this operator's own completed publication. */
export async function tamperDigest(
  databaseUrl: string, handle: string, options: { timeoutMs?: number; pollIntervalMs?: number } = {},
): Promise<Buffer> {
  return tamperDigestWithPublication(databaseUrl, handle, true, options);
}

/** Detector fault only: sender must be held, and the original is journaled
 * under the row lock before the pending submission can be poisoned. */
export async function tamperUnsubmittedDigest(databaseUrl: string, handle: string): Promise<Buffer> {
  return tamperDigestWithPublication(databaseUrl, handle, false);
}

export async function restoreDigest(databaseUrl: string, handle: string, original: Buffer): Promise<void> {
  await restoreRecordedDigest(databaseUrl, handle, original);
}

export interface CanaryOutcome {
  /** The mismatch class the poisoned arm produced. */
  kind: MismatchKind;
  label: string;
}

/**
 * The general form: poison one operator's digest and require *the caller's own
 * comparison* to reject it with an expected classification.
 *
 * Not every suite can use the shared comparator with every field enabled. The
 * fork suite compares bytes and digests but not provenance, because an operator
 * that followed the other branch legitimately attributes the handle to a
 * different transaction and block -- so a full comparison would fail there by
 * design, and a canary aimed at the full comparator would falsify something
 * that suite never calls. Pass the suite's own comparison as `compare` and the
 * canary falsifies that instead.
 *
 * `expectedKinds` is what turns this from "something went wrong" into evidence.
 * A `ComparisonMismatch` of the wrong class, a timeout, or a database error all
 * fail the canary, because none of them is the comparator detecting the
 * poisoning.
 */
export async function assertCanaryFiresWith(
  databaseUrl: string,
  handle: string,
  label: string,
  compare: (phase: 'clean' | 'poisoned' | 'restored') => Promise<void>,
  expectedKinds: readonly MismatchKind[] = ['compute-digest'],
): Promise<CanaryOutcome> {
  // Agreement first: starting from a disagreeing fleet would prove nothing
  // about the poison.
  await compare('clean');

  // When the poison went in and what it was aimed at. The runner records this
  // case's result, and a PASS that names neither cannot be told apart from a
  // run where the tamper never happened -- which is precisely what a canary
  // exists to rule out.
  const original = await tamperDigest(databaseUrl, handle);
  const poisonedAt = new Date().toISOString();
  console.info(`[${label}] canary poisoned ${handle} at ${poisonedAt}`);
  let observed: unknown;
  try {
    try {
      await compare('poisoned');
    } catch (error) {
      observed = error;
    }
    expect(
      observed,
      `${label}: a poisoned digest MUST be rejected by this suite's own comparison. It was not, ` +
        'which means the comparison is not comparing and every other green in this suite is vacuous',
    ).to.not.eq(undefined);
    if (!(observed instanceof ComparisonMismatch)) {
      throw new Error(
        `${label}: the poisoned arm failed, but not as a detected divergence: ` +
          `${observed instanceof Error ? `${observed.name}: ${observed.message}` : String(observed)}. ` +
          'A timeout or a failed read is not the comparator catching a poisoned digest, and accepting it ' +
          'here would let an unreachable database masquerade as a working canary',
      );
    }
    expect(
      expectedKinds.includes(observed.kind),
      `${label}: the poisoned arm was rejected as [${observed.kind}], but this canary poisons a compute ` +
        `digest and must be caught as one of [${expectedKinds.join(', ')}]`,
    ).to.eq(true);
    console.info(`[${label}] canary fired: poisoned digest rejected as [${observed.kind}]`);
  } finally {
    await restoreDigest(databaseUrl, handle, original);
  }

  // And it must pass again, so a later assertion is not reading damage the
  // canary left behind. This is the check that has to outlast a detector's
  // detect-revert-recompute cycle.
  await compare('restored');
  return { kind: (observed as ComparisonMismatch).kind, label };
}

/**
 * Poisons one operator's digest for `handle` and requires the shared comparator
 * to reject the fleet, then restores it and requires agreement again.
 *
 * `victim` must be one of `operators` — poisoning an operator the comparator
 * was never going to look at would produce a canary that cannot fire, which is
 * the exact failure this whole rule guards against.
 */
export async function assertCanaryFires(
  databaseUrls: readonly string[],
  operators: readonly number[],
  handle: string,
  label: string,
  victim = operators[operators.length - 1],
): Promise<CanaryOutcome> {
  if (!operators.includes(victim)) {
    throw new Error(
      `canary victim ${victim} is not among the compared operators ${operators.join(',')}; ` +
        'the canary could not fire and would pass vacuously',
    );
  }
  if (operators.length < 2) {
    throw new Error('a canary needs at least two operators to disagree');
  }

  // Two different deadlines, for two different questions.
  //
  // The poisoned check wants a short one: it expects rejection, and the
  // rejection it wants is a classified digest mismatch that the comparator
  // reaches immediately once every operator holds its evidence.
  //
  // The agreement checks either side of it need a generous one, because a
  // consensus-detector reacts to the poison: it raises a drift signal, reverts
  // the row and lets the operator recompute it. The evidence wait must allow
  // the full revert and recomputation cycle before checking agreement again.
  return assertCanaryFiresWith(
    databaseUrls[victim],
    handle,
    label,
    (phase) =>
      assertOperatorsAgree([...databaseUrls], [...operators], handle, {
        timeoutMs: phase === 'poisoned' ? 60_000 : 6 * 60_000,
      }).then(() => undefined),
    ['compute-digest'],
  );
}
