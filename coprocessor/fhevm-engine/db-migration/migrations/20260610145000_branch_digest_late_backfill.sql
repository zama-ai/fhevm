-- Sweep legacy allowed_handles/ciphertext_digest into branchless rows after
-- installing the mirror triggers. These statements are intentionally guarded so
-- re-running them does not churn every branchless row on an online database.

UPDATE allowed_handles_branch b
SET tenant_id = a.tenant_id,
    event_type = a.event_type,
    txn_is_sent = b.txn_is_sent OR a.txn_is_sent,
    txn_limited_retries_count = GREATEST(
        b.txn_limited_retries_count,
        a.txn_limited_retries_count
    ),
    txn_unlimited_retries_count = GREATEST(
        b.txn_unlimited_retries_count,
        a.txn_unlimited_retries_count
    ),
    txn_hash = COALESCE(b.txn_hash, a.txn_hash),
    txn_block_number = COALESCE(b.txn_block_number, a.txn_block_number),
    allowed_at = LEAST(b.allowed_at, a.allowed_at),
    transaction_id = COALESCE(b.transaction_id, a.transaction_id),
    host_chain_id = a.host_chain_id,
    block_number = COALESCE(b.block_number, a.block_number),
    txn_last_error = COALESCE(a.txn_last_error, b.txn_last_error),
    txn_last_error_at = NULLIF(
        GREATEST(
            COALESCE(b.txn_last_error_at, '-infinity'::TIMESTAMP),
            COALESCE(a.txn_last_error_at, '-infinity'::TIMESTAMP)
        ),
        '-infinity'::TIMESTAMP
    )
FROM allowed_handles a
WHERE b.handle = a.handle
  AND b.account_address = a.account_address
  AND b.host_chain_id = a.host_chain_id
  AND b.block_number IS NOT DISTINCT FROM a.block_number
  AND b.transaction_id IS NOT DISTINCT FROM a.transaction_id
  AND (
       b.producer_block_hash <> ''::BYTEA
       OR b.block_hash <> ''::BYTEA
  )
  AND (
       b.event_type IS DISTINCT FROM a.event_type
       OR (NOT b.txn_is_sent AND a.txn_is_sent)
       OR b.txn_limited_retries_count < a.txn_limited_retries_count
       OR b.txn_unlimited_retries_count < a.txn_unlimited_retries_count
       OR (b.txn_hash IS NULL AND a.txn_hash IS NOT NULL)
       OR (b.txn_block_number IS NULL AND a.txn_block_number IS NOT NULL)
       OR b.allowed_at > a.allowed_at
       OR (b.transaction_id IS NULL AND a.transaction_id IS NOT NULL)
       OR b.host_chain_id IS DISTINCT FROM a.host_chain_id
       OR (b.block_number IS NULL AND a.block_number IS NOT NULL)
       OR b.txn_last_error IS DISTINCT FROM COALESCE(a.txn_last_error, b.txn_last_error)
       OR COALESCE(b.txn_last_error_at, '-infinity'::TIMESTAMP)
            < COALESCE(a.txn_last_error_at, '-infinity'::TIMESTAMP)
  );

DELETE FROM allowed_handles_branch branchless
USING allowed_handles a
WHERE branchless.handle = a.handle
  AND branchless.account_address = a.account_address
  AND branchless.producer_block_hash = ''::BYTEA
  AND branchless.block_hash = ''::BYTEA
  AND EXISTS (
        SELECT 1
          FROM allowed_handles_branch branchful
         WHERE branchful.handle = a.handle
           AND branchful.account_address = a.account_address
           AND branchful.host_chain_id = a.host_chain_id
           AND branchful.block_number IS NOT DISTINCT FROM a.block_number
           AND branchful.transaction_id IS NOT DISTINCT FROM a.transaction_id
           AND (
                branchful.producer_block_hash <> ''::BYTEA
                OR branchful.block_hash <> ''::BYTEA
           )
  );

INSERT INTO allowed_handles_branch (
    tenant_id,
    handle,
    account_address,
    event_type,
    txn_is_sent,
    txn_limited_retries_count,
    txn_last_error,
    txn_last_error_at,
    txn_unlimited_retries_count,
    txn_hash,
    txn_block_number,
    allowed_at,
    transaction_id,
    host_chain_id,
    block_number,
    producer_block_hash,
    block_hash
)
SELECT
    a.tenant_id,
    a.handle,
    a.account_address,
    a.event_type,
    a.txn_is_sent,
    a.txn_limited_retries_count,
    a.txn_last_error,
    a.txn_last_error_at,
    a.txn_unlimited_retries_count,
    a.txn_hash,
    a.txn_block_number,
    a.allowed_at,
    a.transaction_id,
    a.host_chain_id,
    a.block_number,
    ''::BYTEA,
    ''::BYTEA
FROM allowed_handles a
LEFT JOIN allowed_handles_branch b
  ON b.handle = a.handle
 AND b.account_address = a.account_address
 AND b.producer_block_hash = ''::BYTEA
 AND b.block_hash = ''::BYTEA
WHERE (
        b.handle IS NULL
        OR b.event_type IS DISTINCT FROM a.event_type
        OR (NOT b.txn_is_sent AND a.txn_is_sent)
        OR b.txn_limited_retries_count < a.txn_limited_retries_count
        OR b.txn_unlimited_retries_count < a.txn_unlimited_retries_count
        OR (b.txn_hash IS NULL AND a.txn_hash IS NOT NULL)
        OR (b.txn_block_number IS NULL AND a.txn_block_number IS NOT NULL)
        OR b.allowed_at > a.allowed_at
        OR (b.transaction_id IS NULL AND a.transaction_id IS NOT NULL)
        OR b.host_chain_id IS DISTINCT FROM a.host_chain_id
        OR (b.block_number IS NULL AND a.block_number IS NOT NULL)
        OR b.txn_last_error IS DISTINCT FROM COALESCE(a.txn_last_error, b.txn_last_error)
        OR COALESCE(b.txn_last_error_at, '-infinity'::TIMESTAMP)
             < COALESCE(a.txn_last_error_at, '-infinity'::TIMESTAMP)
   )
  AND NOT EXISTS (
        SELECT 1
          FROM allowed_handles_branch branchful
         WHERE branchful.handle = a.handle
           AND branchful.account_address = a.account_address
           AND branchful.host_chain_id = a.host_chain_id
           AND branchful.block_number IS NOT DISTINCT FROM a.block_number
           AND branchful.transaction_id IS NOT DISTINCT FROM a.transaction_id
           AND (
                branchful.producer_block_hash <> ''::BYTEA
                OR branchful.block_hash <> ''::BYTEA
           )
   )
ON CONFLICT (handle, account_address, producer_block_hash, block_hash) DO UPDATE
SET event_type = EXCLUDED.event_type,
    txn_is_sent = allowed_handles_branch.txn_is_sent OR EXCLUDED.txn_is_sent,
    txn_limited_retries_count = GREATEST(
        allowed_handles_branch.txn_limited_retries_count,
        EXCLUDED.txn_limited_retries_count
    ),
    txn_unlimited_retries_count = GREATEST(
        allowed_handles_branch.txn_unlimited_retries_count,
        EXCLUDED.txn_unlimited_retries_count
    ),
    txn_hash = COALESCE(allowed_handles_branch.txn_hash, EXCLUDED.txn_hash),
    txn_block_number = COALESCE(allowed_handles_branch.txn_block_number, EXCLUDED.txn_block_number),
    allowed_at = LEAST(allowed_handles_branch.allowed_at, EXCLUDED.allowed_at),
    transaction_id = COALESCE(allowed_handles_branch.transaction_id, EXCLUDED.transaction_id),
    host_chain_id = EXCLUDED.host_chain_id,
    block_number = COALESCE(allowed_handles_branch.block_number, EXCLUDED.block_number),
    txn_last_error = COALESCE(EXCLUDED.txn_last_error, allowed_handles_branch.txn_last_error),
    txn_last_error_at = NULLIF(
        GREATEST(
            COALESCE(allowed_handles_branch.txn_last_error_at, '-infinity'::TIMESTAMP),
            COALESCE(EXCLUDED.txn_last_error_at, '-infinity'::TIMESTAMP)
        ),
        '-infinity'::TIMESTAMP
    )
WHERE allowed_handles_branch.event_type IS DISTINCT FROM EXCLUDED.event_type
   OR (NOT allowed_handles_branch.txn_is_sent AND EXCLUDED.txn_is_sent)
   OR allowed_handles_branch.txn_limited_retries_count < EXCLUDED.txn_limited_retries_count
   OR allowed_handles_branch.txn_unlimited_retries_count < EXCLUDED.txn_unlimited_retries_count
   OR (allowed_handles_branch.txn_hash IS NULL AND EXCLUDED.txn_hash IS NOT NULL)
   OR (allowed_handles_branch.txn_block_number IS NULL AND EXCLUDED.txn_block_number IS NOT NULL)
   OR allowed_handles_branch.allowed_at > EXCLUDED.allowed_at
   OR (allowed_handles_branch.transaction_id IS NULL AND EXCLUDED.transaction_id IS NOT NULL)
   OR allowed_handles_branch.host_chain_id IS DISTINCT FROM EXCLUDED.host_chain_id
   OR (allowed_handles_branch.block_number IS NULL AND EXCLUDED.block_number IS NOT NULL)
   OR allowed_handles_branch.txn_last_error IS DISTINCT FROM COALESCE(
        EXCLUDED.txn_last_error,
        allowed_handles_branch.txn_last_error
   )
   OR COALESCE(allowed_handles_branch.txn_last_error_at, '-infinity'::TIMESTAMP)
        < COALESCE(EXCLUDED.txn_last_error_at, '-infinity'::TIMESTAMP);

INSERT INTO ciphertext_digest_branch (
    tenant_id,
    handle,
    ciphertext,
    ciphertext128,
    txn_is_sent,
    txn_limited_retries_count,
    txn_last_error,
    txn_last_error_at,
    txn_unlimited_retries_count,
    ciphertext128_format,
    txn_hash,
    txn_block_number,
    transaction_id,
    created_at,
    host_chain_id,
    key_id_gw,
    s3_format_version,
    producer_block_hash,
    block_number,
    block_hash
)
SELECT
    d.tenant_id,
    d.handle,
    d.ciphertext,
    d.ciphertext128,
    d.txn_is_sent,
    d.txn_limited_retries_count,
    d.txn_last_error,
    d.txn_last_error_at,
    d.txn_unlimited_retries_count,
    d.ciphertext128_format,
    d.txn_hash,
    d.txn_block_number,
    d.transaction_id,
    d.created_at,
    d.host_chain_id,
    d.key_id_gw,
    d.s3_format_version,
    ''::BYTEA,
    NULL::BIGINT,
    ''::BYTEA
FROM ciphertext_digest d
LEFT JOIN ciphertext_digest_branch b
  ON b.handle = d.handle
 AND b.producer_block_hash = ''::BYTEA
 AND b.block_hash = ''::BYTEA
WHERE b.handle IS NULL
   OR (b.ciphertext IS NULL AND d.ciphertext IS NOT NULL)
   OR (b.ciphertext128 IS NULL AND d.ciphertext128 IS NOT NULL)
   OR (
        b.ciphertext128 IS NULL
        AND b.ciphertext128_format IS DISTINCT FROM d.ciphertext128_format
   )
   OR (b.s3_format_version IS NULL AND d.s3_format_version IS NOT NULL)
   OR (NOT b.txn_is_sent AND d.txn_is_sent)
   OR b.txn_limited_retries_count < d.txn_limited_retries_count
   OR b.txn_unlimited_retries_count < d.txn_unlimited_retries_count
   OR (b.txn_hash IS NULL AND d.txn_hash IS NOT NULL)
   OR (b.txn_block_number IS NULL AND d.txn_block_number IS NOT NULL)
   OR (b.transaction_id IS NULL AND d.transaction_id IS NOT NULL)
   OR b.txn_last_error IS DISTINCT FROM COALESCE(d.txn_last_error, b.txn_last_error)
   OR COALESCE(b.txn_last_error_at, '-infinity'::TIMESTAMP)
        < COALESCE(d.txn_last_error_at, '-infinity'::TIMESTAMP)
   OR b.created_at > d.created_at
   OR b.host_chain_id IS DISTINCT FROM d.host_chain_id
   OR b.key_id_gw IS DISTINCT FROM d.key_id_gw
ON CONFLICT (handle, producer_block_hash, block_hash) DO UPDATE
SET ciphertext = COALESCE(ciphertext_digest_branch.ciphertext, EXCLUDED.ciphertext),
    ciphertext128 = COALESCE(ciphertext_digest_branch.ciphertext128, EXCLUDED.ciphertext128),
    ciphertext128_format = CASE
        WHEN ciphertext_digest_branch.ciphertext128 IS NULL
        THEN EXCLUDED.ciphertext128_format
        ELSE ciphertext_digest_branch.ciphertext128_format
    END,
    s3_format_version = COALESCE(ciphertext_digest_branch.s3_format_version, EXCLUDED.s3_format_version),
    txn_is_sent = ciphertext_digest_branch.txn_is_sent OR EXCLUDED.txn_is_sent,
    txn_limited_retries_count = GREATEST(
        ciphertext_digest_branch.txn_limited_retries_count,
        EXCLUDED.txn_limited_retries_count
    ),
    txn_unlimited_retries_count = GREATEST(
        ciphertext_digest_branch.txn_unlimited_retries_count,
        EXCLUDED.txn_unlimited_retries_count
    ),
    txn_hash = COALESCE(ciphertext_digest_branch.txn_hash, EXCLUDED.txn_hash),
    txn_block_number = COALESCE(ciphertext_digest_branch.txn_block_number, EXCLUDED.txn_block_number),
    transaction_id = COALESCE(ciphertext_digest_branch.transaction_id, EXCLUDED.transaction_id),
    txn_last_error = COALESCE(EXCLUDED.txn_last_error, ciphertext_digest_branch.txn_last_error),
    txn_last_error_at = NULLIF(
        GREATEST(
            COALESCE(ciphertext_digest_branch.txn_last_error_at, '-infinity'::TIMESTAMP),
            COALESCE(EXCLUDED.txn_last_error_at, '-infinity'::TIMESTAMP)
        ),
        '-infinity'::TIMESTAMP
    ),
    created_at = LEAST(ciphertext_digest_branch.created_at, EXCLUDED.created_at),
    host_chain_id = EXCLUDED.host_chain_id,
    key_id_gw = EXCLUDED.key_id_gw
WHERE (ciphertext_digest_branch.ciphertext IS NULL AND EXCLUDED.ciphertext IS NOT NULL)
   OR (ciphertext_digest_branch.ciphertext128 IS NULL AND EXCLUDED.ciphertext128 IS NOT NULL)
   OR (
        ciphertext_digest_branch.ciphertext128 IS NULL
        AND ciphertext_digest_branch.ciphertext128_format IS DISTINCT FROM EXCLUDED.ciphertext128_format
   )
   OR (ciphertext_digest_branch.s3_format_version IS NULL AND EXCLUDED.s3_format_version IS NOT NULL)
   OR (NOT ciphertext_digest_branch.txn_is_sent AND EXCLUDED.txn_is_sent)
   OR ciphertext_digest_branch.txn_limited_retries_count < EXCLUDED.txn_limited_retries_count
   OR ciphertext_digest_branch.txn_unlimited_retries_count < EXCLUDED.txn_unlimited_retries_count
   OR (ciphertext_digest_branch.txn_hash IS NULL AND EXCLUDED.txn_hash IS NOT NULL)
   OR (ciphertext_digest_branch.txn_block_number IS NULL AND EXCLUDED.txn_block_number IS NOT NULL)
   OR (ciphertext_digest_branch.transaction_id IS NULL AND EXCLUDED.transaction_id IS NOT NULL)
   OR ciphertext_digest_branch.txn_last_error IS DISTINCT FROM COALESCE(
        EXCLUDED.txn_last_error,
        ciphertext_digest_branch.txn_last_error
   )
   OR COALESCE(ciphertext_digest_branch.txn_last_error_at, '-infinity'::TIMESTAMP)
        < COALESCE(EXCLUDED.txn_last_error_at, '-infinity'::TIMESTAMP)
   OR ciphertext_digest_branch.created_at > EXCLUDED.created_at
   OR ciphertext_digest_branch.host_chain_id IS DISTINCT FROM EXCLUDED.host_chain_id
   OR ciphertext_digest_branch.key_id_gw IS DISTINCT FROM EXCLUDED.key_id_gw;
