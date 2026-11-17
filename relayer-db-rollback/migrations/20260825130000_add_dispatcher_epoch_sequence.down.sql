-- Down for 20260825130000_add_dispatcher_epoch_sequence.
--
-- Drops the sequence that mints owner_epoch values, and its comment with it.
--
-- It also resets every owner_epoch to 0. Without the sequence nothing can mint
-- an epoch, so 0 is the only value the column can hold in the state this
-- restores. Leaving the stamped values in place would break a later upgrade:
-- the recreated sequence restarts at 1, and a dispatcher holding epoch 1 can
-- neither claim nor write a row stamped with a higher epoch, so in-flight
-- requests would hang and the chain cursor would stop advancing.
--
-- The rows that are reset get a new updated_at, through the table triggers.

UPDATE user_decrypt_req SET owner_epoch = 0 WHERE owner_epoch <> 0;

UPDATE public_decrypt_req SET owner_epoch = 0 WHERE owner_epoch <> 0;

UPDATE input_proof_req SET owner_epoch = 0 WHERE owner_epoch <> 0;

UPDATE gateway_chain_cursor SET owner_epoch = 0 WHERE owner_epoch <> 0;

DROP SEQUENCE dispatcher_epoch_seq;
