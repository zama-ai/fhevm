-- S3 migration changes publication status, not ciphertext identity. Mirroring a
-- status-only update takes the digest-mirror advisory stripe and can block on
-- unrelated handles whose upload transactions retain that stripe across S3 I/O.
--
-- v0.14 still uses branch state for producer resolution and reorg cleanup, so
-- retain the INSERT/DELETE mirrors and every other UPDATE condition. No v0.14
-- consumer reads ciphertext_digest_branch.s3_format_version; v0.15 retires all
-- branch-state consumers. That mirror column may therefore retain an older
-- publication status until the deprecated schema is removed.
--
-- This is a forward migration: do not edit the already-applied trigger-attach
-- migration. Replacing the trigger is atomic and needs no table scan/backfill.
-- Bound the DDL lock wait on this hot table; a timeout rolls the migration back
-- and it can be retried. CREATE OR REPLACE avoids DROP's stronger table lock.
SET LOCAL lock_timeout = '3s';

CREATE OR REPLACE TRIGGER mirror_ciphertext_digest_branchless_upd
AFTER UPDATE ON ciphertext_digest
FOR EACH ROW
WHEN (
     OLD.handle              IS DISTINCT FROM NEW.handle
  OR OLD.host_chain_id        IS DISTINCT FROM NEW.host_chain_id
  OR OLD.ciphertext           IS DISTINCT FROM NEW.ciphertext
  OR OLD.ciphertext128        IS DISTINCT FROM NEW.ciphertext128
  OR OLD.ciphertext128_format IS DISTINCT FROM NEW.ciphertext128_format
  OR OLD.txn_is_sent          IS DISTINCT FROM NEW.txn_is_sent
  OR OLD.txn_hash             IS DISTINCT FROM NEW.txn_hash
  OR OLD.txn_block_number     IS DISTINCT FROM NEW.txn_block_number
  OR OLD.key_id_gw            IS DISTINCT FROM NEW.key_id_gw
)
EXECUTE FUNCTION mirror_ciphertext_digest_branchless();
