-- Backfill heights for already-materialized branch ciphertexts from their
-- producer metadata. Branchless rows intentionally stay NULL.

UPDATE ciphertexts_branch c
SET block_number = src.block_number
FROM (
    SELECT output_handle AS handle, producer_block_hash, MIN(block_number) AS block_number
    FROM computations_branch
    WHERE producer_block_hash <> ''::BYTEA
      AND block_number IS NOT NULL
    GROUP BY output_handle, producer_block_hash
) src
WHERE c.handle = src.handle
  AND c.producer_block_hash = src.producer_block_hash
  AND c.producer_block_hash <> ''::BYTEA
  AND c.block_number IS NULL;

UPDATE ciphertexts128_branch c
SET block_number = src.block_number
FROM (
    SELECT handle, producer_block_hash, MIN(block_number) AS block_number
    FROM pbs_computations_branch
    WHERE producer_block_hash <> ''::BYTEA
      AND block_number IS NOT NULL
    GROUP BY handle, producer_block_hash
) src
WHERE c.handle = src.handle
  AND c.producer_block_hash = src.producer_block_hash
  AND c.producer_block_hash <> ''::BYTEA
  AND c.block_number IS NULL;
