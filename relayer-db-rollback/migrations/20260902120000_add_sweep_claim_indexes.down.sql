-- Down for 20260902120000_add_sweep_claim_indexes.
--
-- Drops the three sweep-claim indexes. No data is affected.

DROP INDEX idx_input_proof_req_sweep_claim;

DROP INDEX idx_public_decrypt_req_sweep_claim;

DROP INDEX idx_user_decrypt_req_sweep_claim;
