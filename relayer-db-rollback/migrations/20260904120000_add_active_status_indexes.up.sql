-- Serves the Retry-After ETA: it counts queue depth and position from `req_status`, so both pods
-- of an HA pair agree. See store::sql::repositories::queue_depth.
--
-- Separate from `idx_*_sweep_claim` over the same rows: that one leads on `owner_epoch` to rule
-- rows out, this one needs `req_status` leading.
--
-- No CONCURRENTLY: the predicate covers only the live tail, so the build is one 125 ms scan, and
-- CONCURRENTLY deadlocks the harness that migrates several test schemas at once.
CREATE INDEX IF NOT EXISTS idx_input_proof_req_active
    ON input_proof_req (req_status, id)
    WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);

CREATE INDEX IF NOT EXISTS idx_user_decrypt_req_active
    ON user_decrypt_req (req_status, id)
    WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);

CREATE INDEX IF NOT EXISTS idx_public_decrypt_req_active
    ON public_decrypt_req (req_status, id)
    WHERE req_status IN ('queued'::req_status, 'processing'::req_status, 'tx_in_flight'::req_status);
