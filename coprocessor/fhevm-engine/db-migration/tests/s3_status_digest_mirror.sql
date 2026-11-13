\set ON_ERROR_STOP on

-- Run only against an empty, disposable database with all migrations applied.
-- Separate transactions release the fixture INSERTs' advisory locks before
-- checking which locks each UPDATE itself acquires.
CREATE TEMP TABLE mirror_test_handles (handle bytea PRIMARY KEY);
INSERT INTO mirror_test_handles VALUES
    (decode(repeat('a1', 32), 'hex')),
    (decode(repeat('a2', 32), 'hex'));

CREATE FUNCTION pg_temp.check_test(ok boolean, description text)
RETURNS void LANGUAGE plpgsql AS $$
BEGIN
    IF ok IS DISTINCT FROM TRUE THEN
        RAISE EXCEPTION 'FAIL: %', description;
    END IF;
    RAISE NOTICE 'PASS: %', description;
END;
$$;

INSERT INTO pbs_computations_branch
    (handle, host_chain_id, producer_block_hash, block_number, block_hash)
VALUES
    (decode(repeat('a1', 32), 'hex'), 1, '\x01', 1, '\x01'),
    (decode(repeat('a1', 32), 'hex'), 1, '\x02', 2, '\x02');

INSERT INTO ciphertext_digest
    (handle, host_chain_id, ciphertext, s3_format_version, s3_migration_failure_count)
VALUES
    (decode(repeat('a1', 32), 'hex'), 1, '\x11', 0, 2),
    (decode(repeat('a2', 32), 'hex'), 1, '\x11', NULL, 0);

SELECT pg_temp.check_test(
    (SELECT count(*) = 2 FROM ciphertext_digest_branch
     WHERE handle = decode(repeat('a1', 32), 'hex')),
    'INSERT still mirrors into every branch context');

BEGIN;
-- Match the migration's real lock lifetime: lock the row, then update status.
SELECT 1 FROM ciphertext_digest
WHERE handle = decode(repeat('a1', 32), 'hex') FOR UPDATE SKIP LOCKED;
UPDATE ciphertext_digest
SET s3_format_version = 1,
    s3_migration_failure_count = 0,
    s3_migration_last_error = NULL,
    s3_migration_last_error_at = NULL
WHERE handle = decode(repeat('a1', 32), 'hex');
SELECT pg_temp.check_test(
    NOT EXISTS (SELECT 1 FROM pg_locks
                WHERE pid = pg_backend_pid() AND locktype = 'advisory'),
    '0-to-1 migration takes no advisory lock despite retaining its row lock');
SELECT pg_temp.check_test(
    (SELECT s3_format_version = 1 AND s3_migration_failure_count = 0
     FROM ciphertext_digest WHERE handle = decode(repeat('a1', 32), 'hex')),
    'migration status is updated on the authoritative legacy row');
ROLLBACK;

BEGIN;
UPDATE ciphertext_digest
SET s3_format_version = 0
WHERE s3_format_version IS NULL
  AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL);
SELECT pg_temp.check_test(
    NOT EXISTS (SELECT 1 FROM pg_locks
                WHERE pid = pg_backend_pid() AND locktype = 'advisory'),
    'NULL-to-0 startup backfill takes no advisory lock');
ROLLBACK;

BEGIN;
UPDATE ciphertext_digest
SET s3_migration_failure_count = s3_migration_failure_count + 1,
    s3_migration_last_error = 'test failure',
    s3_migration_last_error_at = now()
WHERE handle = decode(repeat('a1', 32), 'hex');
SELECT pg_temp.check_test(
    NOT EXISTS (SELECT 1 FROM pg_locks
                WHERE pid = pg_backend_pid() AND locktype = 'advisory'),
    'failure bookkeeping still takes no advisory lock');
ROLLBACK;

BEGIN;
UPDATE ciphertext_digest
SET ciphertext128 = '\x22', s3_format_version = 1
WHERE handle = decode(repeat('a1', 32), 'hex');
SELECT pg_temp.check_test(
    EXISTS (SELECT 1 FROM pg_locks
            WHERE pid = pg_backend_pid() AND locktype = 'advisory' AND granted),
    'an upload changing a digest still acquires the mirror lock');
SELECT pg_temp.check_test(
    (SELECT count(*) = 2 FROM ciphertext_digest_branch
     WHERE handle = decode(repeat('a1', 32), 'hex') AND ciphertext128 = '\x22'),
    'digest changes still reach every branch context when status also changes');
ROLLBACK;

BEGIN;
UPDATE ciphertext_digest
SET txn_is_sent = TRUE, txn_hash = '\x33', txn_block_number = 3
WHERE handle = decode(repeat('a1', 32), 'hex');
SELECT pg_temp.check_test(
    (SELECT count(*) = 2 FROM ciphertext_digest_branch
     WHERE handle = decode(repeat('a1', 32), 'hex')
       AND txn_is_sent AND txn_hash = '\x33' AND txn_block_number = 3),
    'transaction-sender updates still reach every branch context');
ROLLBACK;

BEGIN;
DELETE FROM ciphertext_digest WHERE handle = decode(repeat('a2', 32), 'hex');
SELECT pg_temp.check_test(
    NOT EXISTS (SELECT 1 FROM ciphertext_digest_branch
                WHERE handle = decode(repeat('a2', 32), 'hex')),
    'DELETE still removes the branchless mirror');
ROLLBACK;

DELETE FROM pbs_computations_branch
WHERE handle IN (SELECT handle FROM mirror_test_handles);
DELETE FROM ciphertext_digest
WHERE handle IN (SELECT handle FROM mirror_test_handles);
