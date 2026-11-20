-- Down for 20261112120000_pad_ct64.
-- Drops the index and the table. 64-bit pad ciphertexts are lost.

DROP INDEX IF EXISTS idx_pad_ct64_by_age;
DROP TABLE IF EXISTS pad_ct64;
