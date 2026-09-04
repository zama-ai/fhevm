/**
 * The seeded-divergence canary, shared by every suite class.
 *
 * The coverage register makes this a standing rule rather than one test: each
 * suite runs one deliberately-poisoned arm and must go red, every time, or that
 * suite's greens count for nothing. The rule exists because this repository has
 * already shipped two comparisons that compared nothing — a GPU byte-gate that
 * measured one stream against itself, and a fork precondition that compared
 * zero against zero. Both were green. Both were meaningless.
 *
 * What makes a canary worth having is *which* comparator it falsifies. Querying
 * digests directly and checking they differ proves only that the canary's own
 * query works. So this drives `assertOperatorsAgree` — the assertion the other
 * tests actually call — and requires it to reject the poisoned fleet. If the
 * comparator ever stops comparing, this fails first and loudly.
 *
 * The tamper is at the consensus layer, not the data layer: it flips a byte of
 * one operator's stored compute *digest*, so the operator still holds correct
 * ciphertext and merely reports the wrong thing about it. That is the shape of
 * the divergence the detector exists for, and it restores cleanly.
 */
import { expect } from 'chai';
import { Pool } from 'pg';

import { assertOperatorsAgree } from './probe';

const handleBytes = (handle: string) => Buffer.from(handle.slice(2), 'hex');

async function withPool<T>(databaseUrl: string, fn: (pool: Pool) => Promise<T>): Promise<T> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    return await fn(pool);
  } finally {
    await pool.end();
  }
}

/** Flips one byte of an operator's stored compute digest, returning the original. */
export async function tamperDigest(databaseUrl: string, handle: string): Promise<Buffer> {
  return withPool(databaseUrl, async (pool) => {
    const current = await pool.query<{ ciphertext: Buffer }>(
      'SELECT ciphertext FROM ciphertext_digest WHERE handle = $1',
      [handleBytes(handle)],
    );
    if (current.rowCount !== 1) {
      throw new Error(`expected one digest row for ${handle}, found ${current.rowCount}`);
    }
    const original = current.rows[0].ciphertext;
    const poisoned = Buffer.from(original);
    poisoned[0] = poisoned[0] ^ 0xff;
    await pool.query('UPDATE ciphertext_digest SET ciphertext = $2 WHERE handle = $1', [
      handleBytes(handle),
      poisoned,
    ]);
    return original;
  });
}

export async function restoreDigest(databaseUrl: string, handle: string, original: Buffer): Promise<void> {
  await withPool(databaseUrl, (pool) =>
    pool.query('UPDATE ciphertext_digest SET ciphertext = $2 WHERE handle = $1', [handleBytes(handle), original]),
  );
}

/**
 * The general form: poison one operator's digest and require *the caller's own
 * comparison* to reject it.
 *
 * Not every suite can use the shared comparator. The fork suite compares only
 * ciphertext and digest, because an operator that followed the other branch
 * legitimately attributes the handle to a different transaction and block --
 * so `assertOperatorsAgree` would fail there by design, and the suite has its
 * own narrower comparison. A canary aimed at the shared comparator would
 * therefore falsify something that suite never calls, which is exactly the
 * hollow reassurance this rule exists to prevent. Pass the suite's own
 * comparison as `compare` and the canary falsifies that instead.
 */
export async function assertCanaryFiresWith(
  databaseUrl: string,
  handle: string,
  label: string,
  compare: (phase: 'clean' | 'poisoned' | 'restored') => Promise<void>,
): Promise<void> {
  // Agreement first: starting from a disagreeing fleet would prove nothing
  // about the poison.
  await compare('clean');

  const original = await tamperDigest(databaseUrl, handle);
  try {
    let rejected = false;
    try {
      await compare('poisoned');
    } catch {
      rejected = true;
    }
    expect(
      rejected,
      `${label}: a poisoned digest MUST be rejected by this suite's own comparison. It was not, ` +
        'which means the comparison is not comparing and every other green in this suite is vacuous',
    ).to.eq(true);
    console.info(`[${label}] canary fired: poisoned digest rejected by the comparison this suite relies on`);
  } finally {
    await restoreDigest(databaseUrl, handle, original);
  }

  // And it must pass again, so a later assertion is not reading damage the
  // canary left behind. This is the check that has to outlast a detector's
  // detect-revert-recompute cycle.
  await compare('restored');
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
): Promise<void> {
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
  // The poisoned check wants a short one: it expects rejection, and a timeout
  // *is* a rejection, so waiting long would only slow the suite down.
  //
  // The agreement checks either side of it need a generous one, because a
  // consensus-detector reacts to the poison: it raises a drift signal, reverts
  // the row and lets the operator recompute it. A full cycle was measured at
  // ~2m16s on this stack and the register documents drift auto-recovery at
  // 203s, so the 30s these once used could not survive a topology that
  // actually deploys the detector -- C2a failed exactly that way, timing out in
  // `waitForOperatorRow` while operator 1 was mid-revert.
  await assertCanaryFiresWith(
    databaseUrls[victim],
    handle,
    label,
    (phase) =>
      assertOperatorsAgree(
        [...databaseUrls],
        [...operators],
        handle,
        phase === 'poisoned' ? 30_000 : 6 * 60_000,
      ).then(() => undefined),
  );
}
