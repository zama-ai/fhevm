-- First block, per chain, whose S3 objects only this epoch's stack uploaded:
-- one past the highest block known when it cut over. NULL until cutover, while
-- the stack's uploader is parked. S3 attestations are not bound to an epoch,
-- so healing takes an attestation target for a finding of this epoch only at
-- or above this block. `legacy` has no window and uploads from block 0.
ALTER TABLE consensus_epoch_block_window
    ADD COLUMN upload_start_block BIGINT NULL
        CHECK (upload_start_block IS NULL OR upload_start_block >= start_block);
