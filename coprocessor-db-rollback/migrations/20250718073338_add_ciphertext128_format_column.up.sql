-- Step 1: Add a new column using SMALLINT (2 bytes)
ALTER TABLE ciphertext_digest
ADD COLUMN ciphertext128_format smallint NOT NULL DEFAULT 10;

/*
0 - Unknown
10 - UncompressedOnCpu
11 - CompressedOnCpu
20 - UncompressedOnGpu
21 - CompressedOnGpu
*/

ALTER TABLE ciphertext_digest
ADD CONSTRAINT ciphertext128_format_valid CHECK (ciphertext128_format IN (0, 10, 11, 20, 21));
