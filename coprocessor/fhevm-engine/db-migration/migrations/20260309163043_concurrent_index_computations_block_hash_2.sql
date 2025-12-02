-- no-transaction
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_computations_handle_transaction
    ON computations (output_handle, transaction_id, block_hash);
