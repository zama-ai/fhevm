-- Serves the sweep claim in store::sql::repositories. Nothing indexed it before: the
-- pre-existing `idx_*_timeout_check` indexes read as req_status coverage but are partial on
-- `receipt_received`, which the sweep never matches, so the holder sequentially scanned every
-- request table twice a second at the default 500 ms interval.
--
-- Partial on the live statuses because the terminal ones accumulate without bound, which sizes
-- the index to the in-flight backlog rather than to history. Leading on `owner_epoch`, the term
-- that rules rows out: a claimed row carries the current epoch, so the predicate matches
-- nothing in steady state.
--
-- `id` trails it so the claim's `ORDER BY owner_epoch, id` is the index's own order. Postgres
-- then walks the index and stops at the batch bound, with no sort and no reading of the
-- eligible set beyond what it returns. `id` is the insertion sequence, so one epoch's rows come
-- back oldest first.
--
-- Built without CONCURRENTLY, so this holds SHARE against each table - blocking writes, not
-- reads - for one sequential scan. Measured at 51 ms over a 1M-row table, and the alternative
-- costs more than it saves here: CONCURRENTLY cannot run in a transaction, which both splits
-- this into a file per index and deadlocks the per-test-schema harness, which runs migrations
-- concurrently.

CREATE INDEX idx_user_decrypt_req_sweep_claim
ON user_decrypt_req (owner_epoch, id)
WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);

CREATE INDEX idx_public_decrypt_req_sweep_claim
ON public_decrypt_req (owner_epoch, id)
WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);

CREATE INDEX idx_input_proof_req_sweep_claim
ON input_proof_req (owner_epoch, id)
WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);
