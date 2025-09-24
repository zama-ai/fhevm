-- Use a default context ID of 1 as this is the first one used by the GW contract.
-- Note that if this migration is run after context ID has changed, the proof requests still pending would fail.
-- Therefore, we assume that this migration runs when either:
--  * the table is empty
--  * context ID is currently 1
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS coprocessor_context_id
BYTEA NOT NULL DEFAULT decode('0100000000000000000000000000000000000000000000000000000000000000', 'hex');
