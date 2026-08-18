-- Decouple SNS claim from result-write so squashed-noise work can be dispatched
-- per item instead of per batch.
--
-- Today the worker claims rows with SELECT ... FOR UPDATE SKIP LOCKED and must
-- hold that transaction until every result is written, so commit granularity is
-- welded to fetch granularity. That forces a barrier: each item waits for the
-- slowest task in its batch, measured at ~24% of the stage's throughput
-- (batch_efficiency 0.76 from a 482/884/1232 ms per-task spread).
--
-- claimed_at lets the claim commit immediately and release the lock, so results
-- can be written independently as each item finishes. A claim older than the
-- reclaim window is retried, which is how a crashed worker's in-flight rows come
-- back without a separate reaper.
--
-- Backwards compatible: NULL means unclaimed, so a worker that predates this
-- column keeps working (it simply ignores the field). Rollout note — mixing old
-- and new workers is only safe while the old ones are the FOR UPDATE kind, since
-- they hold locks that the new claim path respects.
ALTER TABLE pbs_computations
    ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMP WITHOUT TIME ZONE NULL;

-- The hot predicate is (is_completed = FALSE AND claim is free-or-stale) ordered
-- by created_at, so index that shape directly.
CREATE INDEX IF NOT EXISTS pbs_computations_unclaimed_idx
    ON pbs_computations (created_at)
    WHERE is_completed = FALSE;
