-- Partial index over the three in-flight statuses, on each request table, for two readers:
--   1. The Retry-After ETA, which counts queue depth and position from `req_status` so both pods
--      of an HA pair agree. See store::sql::repositories::queue_depth.
--   2. The dispatcher sweep, whose `req_status IN (...)` filter had no index at all and cost a
--      sequential scan per 500 ms tick.
--
-- `(req_status, id)`: equality on status, then `id` as arrival order. Serves a position count
-- (`req_status = $1 AND id < $2`) and a depth count (`req_status = $1`) as index-only scans.
--
-- `attempts` and `owner_epoch` stay out, including as INCLUDE columns - the sweep writes both on
-- every claim, and indexing either would make each claim a non-HOT update.
--
-- Built without CONCURRENTLY on purpose. The predicate matches only the live tail - a few
-- thousand rows - so the index is tiny and the build is bounded by one table scan: 125 ms over a
-- 1 000 000-row, 322 MB table, for a 64 kB index. A concurrent build would trade that brief
-- ACCESS SHARE-blocking window for two table scans, an INVALID index to clean up by hand if it
-- fails, and - because CREATE INDEX CONCURRENTLY waits on every other session's snapshot - a
-- deadlock whenever the integration suite migrates several test schemas at once.
CREATE INDEX IF NOT EXISTS idx_input_proof_req_active
    ON input_proof_req (req_status, id)
    WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);

-- Unlike input proofs, a decrypt request uses both stages this index covers: it enters `queued`
-- (readiness queue) and moves to `processing` (TX queue), so its ETA reads a count from each.
CREATE INDEX IF NOT EXISTS idx_user_decrypt_req_active
    ON user_decrypt_req (req_status, id)
    WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);

CREATE INDEX IF NOT EXISTS idx_public_decrypt_req_active
    ON public_decrypt_req (req_status, id)
    WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);

-- Autovacuum tuning ships with the indexes because they only pay off while the visibility map is
-- fresh. The ETA's counts are index-only scans, and "index-only" is conditional: for any heap
-- page the map does not mark all-visible, Postgres still fetches it, so the scan degrades into a
-- heap scan with an unchanged EXPLAIN shape - visible only as latency.
--
-- These tables fall behind fast. Every status transition rewrites `req_status`, an indexed
-- column, so none can be a HOT update; the sweep's claim adds more by rewriting `owner_epoch` and
-- `attempts`. Only VACUUM sets visibility-map bits, so at the stock 20% scale factor the map is
-- stale most of the time. Analyze moves with it: the planner only picks the index-only scan while
-- its row estimates for these statuses are current.
ALTER TABLE input_proof_req SET (
    autovacuum_vacuum_scale_factor = 0.02,
    autovacuum_vacuum_threshold = 200,
    autovacuum_analyze_scale_factor = 0.02,
    autovacuum_analyze_threshold = 200
);

ALTER TABLE user_decrypt_req SET (
    autovacuum_vacuum_scale_factor = 0.02,
    autovacuum_vacuum_threshold = 200,
    autovacuum_analyze_scale_factor = 0.02,
    autovacuum_analyze_threshold = 200
);

ALTER TABLE public_decrypt_req SET (
    autovacuum_vacuum_scale_factor = 0.02,
    autovacuum_vacuum_threshold = 200,
    autovacuum_analyze_scale_factor = 0.02,
    autovacuum_analyze_threshold = 200
);
