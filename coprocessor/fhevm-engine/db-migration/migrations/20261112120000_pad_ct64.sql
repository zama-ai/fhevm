-- The ct64 of a complete row, for Unmask. The GC clears it after `PAD_CT64_MAX_AGE`, and keeps
-- `ciphertext_digest`: the lane can compute it again.
ALTER TABLE pads ADD COLUMN IF NOT EXISTS ciphertext64 BYTEA NULL;

-- For the GC of the ct64.
CREATE INDEX IF NOT EXISTS idx_pads_ct64_by_age
    ON pads (created_at)
    WHERE ciphertext64 IS NOT NULL;
