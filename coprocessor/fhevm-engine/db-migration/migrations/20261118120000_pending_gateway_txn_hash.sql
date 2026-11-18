-- This column holds the hash of a Gateway submission that executed but was not
-- in a sealed block when the transaction-sender deadline expired. Arbitrum can
-- return such a receipt from eth_sendRawTransactionSync and retract it later.
-- The next attempt reads the receipt of this transaction before it sends again.
-- This rule also applies after a restart. The value is NULL when no submission
-- is unconfirmed.
--
-- This column is separate from ciphertext_digest.txn_hash on purpose. A trigger
-- copies txn_hash into ciphertext_digest_branch. For this reason, txn_hash must
-- only hold a sealed submission. The trigger does not watch this column, so a write to this
-- column does not start the trigger. GCS copies that use
-- (LIKE public.<table> INCLUDING ALL) also get this column.
--
-- A nullable column without a default changes only the catalog. But it needs a
-- short ACCESS EXCLUSIVE lock on the busy ciphertext_digest table. The lock
-- timeout limits the wait. If the lock is not available, the migration stops
-- quickly, and the next run tries again.
SET LOCAL lock_timeout = '3s';

ALTER TABLE verify_proofs ADD COLUMN IF NOT EXISTS pending_txn_hash BYTEA;
ALTER TABLE ciphertext_digest ADD COLUMN IF NOT EXISTS pending_txn_hash BYTEA;
