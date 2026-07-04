-- Bound the advisory-lock footprint of the digest mirror triggers.
--
-- Both mirror triggers took one pg_advisory_xact_lock per (chain, handle),
-- held until commit. Advisory locks live in the shared lock table
-- (max_locks_per_transaction * max_connections slots, ~6400 by default): a
-- deep-reorg cleanup deleting tens of thousands of digest rows in one
-- transaction fires the trigger per row, exhausts the table ("out of shared
-- memory"), and stalls every transaction on the instance.
--
-- Stripe the lock space instead: the key is (chain, hash(handle) mod 256), so
-- one transaction holds at most 256 advisory locks no matter how many rows it
-- touches. The property the lock exists for is preserved — both triggers
-- compute the same stripe for the same (chain, handle), so the two
-- ciphertext_digest_branch writers still serialize per handle and cannot
-- deadlock on opposite-order row locks for one handle. Unrelated handles
-- sharing a stripe serialize spuriously (short trigger bodies, 256 stripes:
-- negligible), and cross-stripe deadlocks between multi-row transactions
-- remain possible exactly as they were with per-handle locks — Postgres
-- detects those and the writers' retry paths re-run.
--
-- The branchless mirror additionally gets the exception isolation the wave1
-- pre-merge review asked for: it fires from the hot legacy ciphertext_digest
-- writers (sns-worker, transaction-sender), and an error in the best-effort
-- branch mirror must not abort the authoritative legacy write. Cancellation
-- and deadlock/serialization states are re-raised so statement_timeout and
-- the writers' retry machinery keep working. The pbs-context mirror stays
-- strict: it fires from branch-row writes, where the branch state IS the
-- payload.
--
-- SET LOCAL lock_timeout is not needed: CREATE OR REPLACE FUNCTION does not
-- lock the tables the triggers are attached to.

CREATE OR REPLACE FUNCTION mirror_ciphertext_digest_branchless()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
    _context RECORD;
    _context_count BIGINT;
BEGIN
    -- Serialize all ciphertext_digest_branch writes for this (chain, handle
    -- stripe) against the pbs-context trigger below, which writes the same
    -- rows while reading ciphertext_digest. Without this common lock the two
    -- triggers can acquire ciphertext_digest_branch row locks in opposite
    -- order and deadlock. Striped (mod 256) so bulk transactions hold a
    -- bounded number of advisory locks. Transaction-scoped: released
    -- automatically at commit/rollback. Kept OUTSIDE the exception block: the
    -- lock must survive the subtransaction to keep ordering with the
    -- pbs-context trigger.
    PERFORM pg_advisory_xact_lock(
        hashtextextended(
            COALESCE(NEW.host_chain_id, OLD.host_chain_id)::text
                || ':digest-mirror:'
                || abs(
                    hashtextextended(
                        encode(COALESCE(NEW.handle, OLD.handle), 'hex'),
                        0
                    ) % 256
                )::text,
            0
        )
    );

    BEGIN
        IF TG_OP = 'DELETE' THEN
            DELETE FROM ciphertext_digest_branch
             WHERE handle = OLD.handle
               AND host_chain_id = OLD.host_chain_id
               AND producer_block_hash = ''::BYTEA
               AND block_hash = ''::BYTEA;
        ELSE
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
        END IF;
    EXCEPTION
        WHEN OTHERS THEN
            -- Cancellation must still abort (statement_timeout self-heal),
            -- and deadlock/serialization must reach the writers' retry
            -- machinery; everything else degrades to a warning so the
            -- authoritative legacy write commits.
            IF SQLSTATE IN ('57014', '40P01', '40001') THEN
                RAISE;
            END IF;
            RAISE WARNING
                'ciphertext_digest branch mirror skipped for handle %: % (%)',
                encode(COALESCE(NEW.handle, OLD.handle), 'hex'),
                SQLERRM,
                SQLSTATE;
    END;

    RETURN COALESCE(NEW, OLD);
END;
$$;

CREATE OR REPLACE FUNCTION mirror_ciphertext_digest_for_pbs_context()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
    _digest ciphertext_digest%ROWTYPE;
BEGIN
    -- Same (chain, handle stripe) advisory lock as
    -- mirror_ciphertext_digest_branchless() so the two
    -- ciphertext_digest_branch writers never deadlock on opposite-order
    -- row-lock acquisition for one handle. Transaction-scoped; released at
    -- commit/rollback.
    PERFORM pg_advisory_xact_lock(
        hashtextextended(
            COALESCE(NEW.host_chain_id, OLD.host_chain_id)::text
                || ':digest-mirror:'
                || abs(
                    hashtextextended(
                        encode(COALESCE(NEW.handle, OLD.handle), 'hex'),
                        0
                    ) % 256
                )::text,
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
