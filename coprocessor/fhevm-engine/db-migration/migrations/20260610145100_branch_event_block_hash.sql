-- Branch-context wave 1: mirror legacy digest rows into event-scoped branch
-- contexts.
--
-- The branch tables are created with both producer_block_hash and block_hash
-- from the start. This migration only installs online triggers/functions so
-- newly written legacy digest rows are mirrored into the matching
-- pbs_computations_branch contexts. It intentionally does not repair historical
-- rows, delete orphaned rows, or mutate legacy reader tables.

-- The CREATE TRIGGER statements below take a brief ACCESS EXCLUSIVE lock on the
-- hot ciphertext_digest table. Bound the wait so a contended attach fails fast
-- and is retried instead of convoying queries behind it. Transaction-local.
SET LOCAL lock_timeout = '3s';

CREATE OR REPLACE FUNCTION upsert_ciphertext_digest_branch_from_legacy(
    _digest ciphertext_digest,
    _producer_block_hash BYTEA,
    _block_number BIGINT,
    _block_hash BYTEA
)
RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
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
        _digest.tenant_id,
        _digest.handle,
        _digest.ciphertext,
        _digest.ciphertext128,
        _digest.txn_is_sent,
        _digest.txn_limited_retries_count,
        _digest.txn_last_error,
        _digest.txn_last_error_at,
        _digest.txn_unlimited_retries_count,
        _digest.ciphertext128_format,
        _digest.txn_hash,
        _digest.txn_block_number,
        _digest.transaction_id,
        _digest.created_at,
        _digest.host_chain_id,
        _digest.key_id_gw,
        _digest.s3_format_version,
        _producer_block_hash,
        CASE
            WHEN _producer_block_hash = ''::BYTEA THEN NULL
            ELSE _block_number
        END,
        _block_hash
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
END;
$$;

CREATE OR REPLACE FUNCTION mirror_ciphertext_digest_branchless()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
    _context RECORD;
    _context_count BIGINT;
BEGIN
    -- Serialize all ciphertext_digest_branch writes for this (chain, handle)
    -- against the pbs-context trigger below, which writes the same rows while
    -- reading ciphertext_digest. Without this common lock the two triggers can
    -- acquire ciphertext_digest_branch row locks in opposite order and deadlock.
    -- Transaction-scoped: released automatically at commit/rollback.
    PERFORM pg_advisory_xact_lock(
        hashtextextended(
            COALESCE(NEW.host_chain_id, OLD.host_chain_id)::text
                || ':' || encode(COALESCE(NEW.handle, OLD.handle), 'hex'),
            0
        )
    );

    IF TG_OP = 'DELETE' THEN
        DELETE FROM ciphertext_digest_branch
         WHERE handle = OLD.handle
           AND host_chain_id = OLD.host_chain_id
           AND producer_block_hash = ''::BYTEA
           AND block_hash = ''::BYTEA;
        RETURN OLD;
    END IF;

    IF TG_OP = 'UPDATE'
       AND (
            OLD.handle IS DISTINCT FROM NEW.handle
            OR OLD.host_chain_id IS DISTINCT FROM NEW.host_chain_id
       )
    THEN
        DELETE FROM ciphertext_digest_branch
         WHERE handle = OLD.handle
           AND host_chain_id = OLD.host_chain_id
           AND producer_block_hash = ''::BYTEA
           AND block_hash = ''::BYTEA;
    END IF;

    SELECT COUNT(*)
      INTO _context_count
      FROM pbs_computations_branch
     WHERE host_chain_id = NEW.host_chain_id
       AND handle = NEW.handle;

    IF _context_count = 0 THEN
        PERFORM upsert_ciphertext_digest_branch_from_legacy(
            NEW,
            ''::BYTEA,
            NULL,
            ''::BYTEA
        );
    ELSE
        FOR _context IN
            SELECT DISTINCT producer_block_hash, block_number, block_hash
              FROM pbs_computations_branch
             WHERE host_chain_id = NEW.host_chain_id
               AND handle = NEW.handle
        LOOP
            PERFORM upsert_ciphertext_digest_branch_from_legacy(
                NEW,
                _context.producer_block_hash,
                _context.block_number,
                _context.block_hash
            );
        END LOOP;

        IF EXISTS (
            SELECT 1
              FROM pbs_computations_branch
             WHERE host_chain_id = NEW.host_chain_id
               AND handle = NEW.handle
               AND (
                    producer_block_hash <> ''::BYTEA
                    OR block_hash <> ''::BYTEA
               )
        )
        AND NOT EXISTS (
            SELECT 1
              FROM pbs_computations_branch
             WHERE host_chain_id = NEW.host_chain_id
               AND handle = NEW.handle
               AND producer_block_hash = ''::BYTEA
               AND block_hash = ''::BYTEA
        ) THEN
            DELETE FROM ciphertext_digest_branch
             WHERE handle = NEW.handle
               AND host_chain_id = NEW.host_chain_id
               AND producer_block_hash = ''::BYTEA
               AND block_hash = ''::BYTEA;
        END IF;
    END IF;

    RETURN NEW;
END;
$$;

CREATE OR REPLACE FUNCTION mirror_ciphertext_digest_for_pbs_context()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
    _digest ciphertext_digest%ROWTYPE;
BEGIN
    -- Same (chain, handle) advisory lock as mirror_ciphertext_digest_branchless()
    -- so the two ciphertext_digest_branch writers never deadlock on opposite-order
    -- row-lock acquisition. Transaction-scoped; released at commit/rollback.
    PERFORM pg_advisory_xact_lock(
        hashtextextended(
            COALESCE(NEW.host_chain_id, OLD.host_chain_id)::text
                || ':' || encode(COALESCE(NEW.handle, OLD.handle), 'hex'),
            0
        )
    );

    IF TG_OP = 'DELETE' THEN
        DELETE FROM ciphertext_digest_branch
         WHERE handle = OLD.handle
           AND host_chain_id = OLD.host_chain_id
           AND producer_block_hash = OLD.producer_block_hash
           AND block_hash = OLD.block_hash;
        RETURN OLD;
    END IF;

    IF TG_OP = 'UPDATE'
       AND (
            OLD.handle IS DISTINCT FROM NEW.handle
            OR OLD.host_chain_id IS DISTINCT FROM NEW.host_chain_id
            OR OLD.producer_block_hash IS DISTINCT FROM NEW.producer_block_hash
            OR OLD.block_hash IS DISTINCT FROM NEW.block_hash
       )
    THEN
        DELETE FROM ciphertext_digest_branch
         WHERE handle = OLD.handle
           AND host_chain_id = OLD.host_chain_id
           AND producer_block_hash = OLD.producer_block_hash
           AND block_hash = OLD.block_hash;
    END IF;

    SELECT *
      INTO _digest
      FROM ciphertext_digest
     WHERE host_chain_id = NEW.host_chain_id
       AND handle = NEW.handle;

    IF FOUND THEN
        PERFORM upsert_ciphertext_digest_branch_from_legacy(
            _digest,
            NEW.producer_block_hash,
            NEW.block_number,
            NEW.block_hash
        );

        IF NEW.producer_block_hash <> ''::BYTEA
           OR NEW.block_hash <> ''::BYTEA
        THEN
            IF NOT EXISTS (
                SELECT 1
                  FROM pbs_computations_branch
                 WHERE host_chain_id = NEW.host_chain_id
                   AND handle = NEW.handle
                   AND producer_block_hash = ''::BYTEA
                   AND block_hash = ''::BYTEA
            ) THEN
                DELETE FROM ciphertext_digest_branch
                 WHERE handle = NEW.handle
                   AND host_chain_id = NEW.host_chain_id
                   AND producer_block_hash = ''::BYTEA
                   AND block_hash = ''::BYTEA;
            END IF;
        END IF;
    END IF;

    RETURN NEW;
END;
$$;

-- Split per-operation so the expensive context fan-out (COUNT + DISTINCT loop
-- + N upserts) does not re-run on UPDATEs that change nothing it mirrors. A
-- single combined trigger cannot carry this WHEN clause because it would
-- reference OLD on INSERT / NEW on DELETE. INSERT and DELETE always fire.
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_trigger ON ciphertext_digest;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_ins ON ciphertext_digest;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_del ON ciphertext_digest;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_upd ON ciphertext_digest;

CREATE TRIGGER mirror_ciphertext_digest_branchless_ins
AFTER INSERT ON ciphertext_digest
FOR EACH ROW
EXECUTE FUNCTION mirror_ciphertext_digest_branchless();

CREATE TRIGGER mirror_ciphertext_digest_branchless_del
AFTER DELETE ON ciphertext_digest
FOR EACH ROW
EXECUTE FUNCTION mirror_ciphertext_digest_branchless();

-- Fire only when a column the branch row actually mirrors changed. Pure
-- status-bookkeeping UPDATEs that touch none of these are skipped, avoiding a
-- needless context fan-out. (statement_timeout on the writer pools remains the
-- hard bound on the worst-case reorg fan-out; this only trims redundant fires.)
CREATE TRIGGER mirror_ciphertext_digest_branchless_upd
AFTER UPDATE ON ciphertext_digest
FOR EACH ROW
WHEN (
     OLD.handle              IS DISTINCT FROM NEW.handle
  OR OLD.host_chain_id        IS DISTINCT FROM NEW.host_chain_id
  OR OLD.ciphertext           IS DISTINCT FROM NEW.ciphertext
  OR OLD.ciphertext128        IS DISTINCT FROM NEW.ciphertext128
  OR OLD.ciphertext128_format IS DISTINCT FROM NEW.ciphertext128_format
  OR OLD.s3_format_version    IS DISTINCT FROM NEW.s3_format_version
  OR OLD.txn_is_sent          IS DISTINCT FROM NEW.txn_is_sent
  OR OLD.txn_hash             IS DISTINCT FROM NEW.txn_hash
  OR OLD.txn_block_number     IS DISTINCT FROM NEW.txn_block_number
  OR OLD.key_id_gw            IS DISTINCT FROM NEW.key_id_gw
)
EXECUTE FUNCTION mirror_ciphertext_digest_branchless();

DROP TRIGGER IF EXISTS mirror_ciphertext_digest_pbs_context_trigger ON pbs_computations_branch;

CREATE TRIGGER mirror_ciphertext_digest_pbs_context_trigger
AFTER INSERT OR UPDATE OR DELETE
ON pbs_computations_branch
FOR EACH ROW
EXECUTE FUNCTION mirror_ciphertext_digest_for_pbs_context();
