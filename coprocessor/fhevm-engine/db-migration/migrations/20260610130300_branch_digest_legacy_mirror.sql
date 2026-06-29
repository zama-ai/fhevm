-- Keep branchless branch rows in sync with legacy tables during wave 1. This
-- covers rolling deploys where old host-listener/SNS/transaction-sender
-- processes are still writing legacy tables after branch tables exist.

CREATE OR REPLACE FUNCTION mirror_allowed_handles_branchless()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        DELETE FROM allowed_handles_branch
         WHERE handle = OLD.handle
           AND account_address = OLD.account_address
           AND host_chain_id = OLD.host_chain_id
           AND producer_block_hash = ''::BYTEA
           AND block_hash = ''::BYTEA;
        RETURN OLD;
    END IF;

    IF TG_OP = 'UPDATE'
       AND (
            OLD.handle IS DISTINCT FROM NEW.handle
            OR OLD.account_address IS DISTINCT FROM NEW.account_address
            OR OLD.host_chain_id IS DISTINCT FROM NEW.host_chain_id
       )
    THEN
        DELETE FROM allowed_handles_branch
         WHERE handle = OLD.handle
           AND account_address = OLD.account_address
           AND host_chain_id = OLD.host_chain_id
           AND producer_block_hash = ''::BYTEA
           AND block_hash = ''::BYTEA;
    END IF;

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
    VALUES (
        NEW.tenant_id,
        NEW.handle,
        NEW.account_address,
        NEW.event_type,
        NEW.txn_is_sent,
        NEW.txn_limited_retries_count,
        NEW.txn_last_error,
        NEW.txn_last_error_at,
        NEW.txn_unlimited_retries_count,
        NEW.txn_hash,
        NEW.txn_block_number,
        NEW.allowed_at,
        NEW.transaction_id,
        NEW.host_chain_id,
        NEW.block_number,
        ''::BYTEA,
        ''::BYTEA
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

    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS mirror_allowed_handles_branchless_trigger ON allowed_handles;

CREATE TRIGGER mirror_allowed_handles_branchless_trigger
AFTER INSERT OR UPDATE OR DELETE
ON allowed_handles
FOR EACH ROW
EXECUTE FUNCTION mirror_allowed_handles_branchless();

CREATE OR REPLACE FUNCTION mirror_ciphertext_digest_branchless()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        DELETE FROM ciphertext_digest_branch
         WHERE handle = OLD.handle
           AND producer_block_hash = ''::BYTEA
           AND block_hash = ''::BYTEA;
        RETURN OLD;
    END IF;

    IF TG_OP = 'UPDATE' AND OLD.handle IS DISTINCT FROM NEW.handle THEN
        DELETE FROM ciphertext_digest_branch
         WHERE handle = OLD.handle
           AND producer_block_hash = ''::BYTEA
           AND block_hash = ''::BYTEA;
    END IF;

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
    VALUES (
        NEW.tenant_id,
        NEW.handle,
        NEW.ciphertext,
        NEW.ciphertext128,
        NEW.txn_is_sent,
        NEW.txn_limited_retries_count,
        NEW.txn_last_error,
        NEW.txn_last_error_at,
        NEW.txn_unlimited_retries_count,
        NEW.ciphertext128_format,
        NEW.txn_hash,
        NEW.txn_block_number,
        NEW.transaction_id,
        NEW.created_at,
        NEW.host_chain_id,
        NEW.key_id_gw,
        NEW.s3_format_version,
        ''::BYTEA,
        NULL,
        ''::BYTEA
    )
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

    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_trigger ON ciphertext_digest;

CREATE TRIGGER mirror_ciphertext_digest_branchless_trigger
AFTER INSERT OR UPDATE OR DELETE
ON ciphertext_digest
FOR EACH ROW
EXECUTE FUNCTION mirror_ciphertext_digest_branchless();
