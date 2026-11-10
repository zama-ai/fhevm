-- Down for 20260904120000_add_active_status_indexes.
--
-- Drops the three active-status indexes. No data is affected.

DROP INDEX idx_public_decrypt_req_active;

DROP INDEX idx_user_decrypt_req_active;

DROP INDEX idx_input_proof_req_active;
