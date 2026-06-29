-- Branch-context wave 1: backfill pre-existing legacy state as branchless rows.
--
-- Rows produced before the branch tables existed live only in the legacy
-- tables, so they are mirrored here as "branchless" rows
-- (producer_block_hash = block_hash = ''): valid on every branch and never
-- orphan-cleaned.
--
-- Idempotent: ON CONFLICT skips rows that already have a branchless twin, so
-- the migration is safe to re-run.
--
-- Not backfilled on purpose:
--   * computations / pbs_computations — work below the cutover block is
--     drained by the legacy pipeline; wave-2 workers only process rows at or
--     above FHEVM_BRANCH_CUTOVER_BLOCK.
--   * ciphertexts / ciphertexts128 bytes — handles keep landing in the legacy
--     tables until the legacy pipeline drains, so wave-2 readers fall back to
--     the legacy tables at read time for branchless handles instead of
--     relying on a one-shot copy.

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
ON CONFLICT (handle, account_address, producer_block_hash, block_hash) DO NOTHING;

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
ON CONFLICT (handle, producer_block_hash, block_hash) DO NOTHING;
