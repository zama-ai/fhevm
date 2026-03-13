-- no-transaction
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_computations_transaction
    ON computations (transaction_id, block_hash);
